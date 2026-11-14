//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Drive, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Drive",
        label: "%",
        default: 0.0,
    },
    ParamInfo {
        name: "Highpass",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Out Level",
        label: " ",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Drive",
    upstream_name: "Drive",
    description: "Drive is the angry distortion!",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Drive::new(rate, seed))
}
struct Drive {
    params: [f32; 4],
    rate: f32,
    seed: u32,
    iir: [[f64; 2]; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl Drive {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 0.0, 1.0, 1.0],
            rate,
            seed,
            iir: [[0.0; 2]; 2],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Drive {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(v)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        match self.params.get(i) {
            Some(v) => float2string(if i == 0 {
                (*v as f64 * 100.0) as f32
            } else {
                *v
            }),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let drive = (a as f64 * 2.0).powi(2);
        let amount = (b as f64).powi(3) / ((1.0 / 44100.0) * self.rate as f64);
        let wet = d as f64;
        for (((&l, &r), ol), or) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *sample;
                let slot = usize::from(!self.flip);
                self.iir[slot][ch] = self.iir[slot][ch] * (1.0 - amount) + *sample * amount;
                *sample -= self.iir[slot][ch];
                *sample = sample.clamp(-1.0, 1.0);
                let mut out = drive;
                while out > 0.60 {
                    out -= 0.60;
                    *sample -= *sample * (sample.abs() * 0.60) * (sample.abs() * 0.60);
                    *sample *= 1.60;
                }
                *sample -= *sample * (sample.abs() * out) * (sample.abs() * out);
                *sample *= 1.0 + out;
                if c < 1.0 {
                    *sample *= c as f64;
                }
                if wet < 1.0 {
                    *sample = dry * (1.0 - wet) + *sample * wet;
                }
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            self.flip = !self.flip;
            *ol = samples[0] as f32;
            *or = samples[1] as f32;
        }
    }
}
