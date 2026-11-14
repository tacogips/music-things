//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Channel7/Channel7.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.0.
use crate::display::{float2string, int2string};
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
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
    name: "Channel7",
    upstream_name: "Channel7",
    description: "Channel7 improves slew clipping and lets you blend Channel 6 and 5.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Channel7::new(rate, seed))
}
struct Channel7 {
    params: [f32; 3],
    rate: f32,
    seed: u32,
    flip: bool,
    iir: [[f64; 2]; 2],
    last: [f64; 2],
    amount: f64,
    threshold: f64,
    fpd: [u32; 2],
}
impl Channel7 {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.0, 1.0],
            rate,
            seed,
            flip: false,
            iir: [[0.0; 2]; 2],
            last: [0.0; 2],
            amount: 0.005832,
            threshold: 0.33362176,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    fn update(&mut self) {
        (self.amount, self.threshold) = match (self.params[0] as f64 * 2.999) as i32 {
            0 => (0.005832, 0.33362176),
            1 => (0.004096, 0.59969536),
            _ => (0.004913, 0.84934656),
        };
    }
}
impl AirwindowsEffect for Channel7 {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(v)) {
            *p = v;
            self.update();
        }
    }
    fn param_display(&self, i: usize) -> String {
        match i {
            0 => match (self.params[0] as f64 * 2.999) as i32 {
                0 => "Neve",
                1 => "API",
                _ => "SSL",
            }
            .into(),
            1 => int2string((self.params[1] as f64 * 200.0) as i32),
            2 => float2string(self.params[2]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
        self.update();
    }
    // Preserve upstream constants for parity.
    #[allow(clippy::approx_constant)]
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let scale = (1.0 / 44100.0) * self.rate as f64;
        let amount = self.amount / scale;
        let threshold = self.threshold;
        let density = (self.params[1] as f64) * 2.0;
        let phattity = (density - 1.0).max(0.0);
        let density = density.min(1.0);
        for (((&l, &r), ol), or) in l.iter().zip(r).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            let bank = usize::from(!self.flip);
            for (c, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[c] as f64 * 1.18e-17;
                }
                self.iir[bank][c] = self.iir[bank][c] * (1.0 - amount) + *sample * amount;
                *sample -= self.iir[bank][c];
                let dry = *sample;
                *sample = sample.clamp(-1.0, 1.0);
                let phat = (*sample * 1.57079633).sin();
                *sample *= 1.2533141373155;
                let dist = (*sample * sample.abs()).sin()
                    / if sample.abs() == 0.0 {
                        1.0
                    } else {
                        sample.abs()
                    };
                *sample = dist;
                if density < 1.0 {
                    *sample = dry * (1.0 - density) + dist * density;
                }
                if phattity > 0.0 {
                    *sample = *sample * (1.0 - phattity) + phat * phattity;
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
                *sample = fpd_dither(*sample, &mut self.fpd[c]);
            }
            self.flip = !self.flip;
            *ol = samples[0] as f32;
            *or = samples[1] as f32;
        }
    }
}
