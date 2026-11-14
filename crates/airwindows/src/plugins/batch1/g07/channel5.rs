//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Channel5/Channel5.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.0.
use crate::display::{float2string, int2string};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Console Type",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Drive",
        label: "%",
        default: 0.0,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Channel5",
    upstream_name: "Channel5",
    description: "Channel5 is Channel for 2018, with new subtleties.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Channel5::new(rate, seed))
}
struct Channel5 {
    params: [f32; 3],
    rate: f32,
    seed: u32,
    shape: [f64; 2],
    flip: bool,
    iir: [[f64; 2]; 2],
    last: [f64; 2],
    amount: f64,
    threshold: f64,
}
impl Channel5 {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 0.0, 1.0],
            rate,
            seed,
            shape: [0.0; 2],
            flip: true,
            iir: [[0.0; 2]; 2],
            last: [0.0; 2],
            amount: 0.005832,
            threshold: 0.33362176,
        }
    }
}
impl AirwindowsEffect for Channel5 {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(slot), Some(value)) = (self.params.get_mut(index), sanitize_param(value)) {
            *slot = value;
            (self.amount, self.threshold) = match (self.params[0] as f64 * 2.999) as i32 {
                0 => (0.005832, 0.33362176),
                1 => (0.004096, 0.59969536),
                2 => (0.004913, 0.84934656),
                _ => return,
            };
        }
    }
    fn param_display(&self, index: usize) -> String {
        match index {
            0 => match (self.params[0] as f64 * 2.999) as i32 {
                0 => "Neve",
                1 => "API",
                _ => "SSL",
            }
            .into(),
            1 => int2string((self.params[1] * 100.0) as i32),
            2 => float2string(self.params[2]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        for (i, value) in params.into_iter().enumerate() {
            self.set_param(i, value);
        }
    }
    // Preserve upstream constants for parity.
    #[allow(clippy::approx_constant)]
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let scale = (1.0 / 44100.0) * self.rate as f64;
        let amount = self.amount / scale;
        let threshold = self.threshold / scale;
        let density = (self.params[1] as f64).powf(2.0);
        for (((&l, &r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            let mut s = [l as f64, r as f64];
            let bank = usize::from(!self.flip);
            for (c, sample) in s.iter_mut().enumerate() {
                self.iir[bank][c] = (self.iir[bank][c] * (1.0 - amount)) + (*sample * amount);
                *sample -= self.iir[bank][c];
                let mut rect = sample.abs() * 1.57079633;
                if rect > 1.57079633 {
                    rect = 1.0;
                } else {
                    rect = rect.sin();
                }
                if *sample > 0.0 {
                    *sample = (*sample * (1.0 - density)) + (rect * density);
                } else {
                    *sample = (*sample * (1.0 - density)) - (rect * density);
                }
                let clamp = *sample - self.last[c];
                if clamp > threshold {
                    *sample = self.last[c] + threshold;
                }
                if -clamp > threshold {
                    *sample = self.last[c] - threshold;
                }
                self.last[c] = *sample;
                if self.params[2] < 1.0 {
                    *sample *= self.params[2] as f64;
                }
                let temp = *sample as f32;
                self.shape[c] += *sample - temp as f64;
                *sample += self.shape[c];
            }
            self.flip = !self.flip;
            *ol = s[0] as f32;
            *or = s[1] as f32;
        }
        self.shape[0] *= 0.999999;
        self.shape[1] *= 0.999999;
    }
}
