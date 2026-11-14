//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/NotJustAnotherDither, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Quant",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "DeRez",
        label: "",
        default: 0.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "NotJustAnotherDither",
    upstream_name: "NotJustAnotherDither",
    description: "NotJustAnotherDither is a next-gen wordlength reducers for 24 bit with DeRez.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(NotJustAnotherDither::new(sample_rate, seed))
}
struct NotJustAnotherDither {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    bins: [[f64; 11]; 2],
    shaping: [f64; 2],
}
impl NotJustAnotherDither {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.0],
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            bins: [[
                1000.0, 301.0, 176.0, 125.0, 97.0, 79.0, 67.0, 58.0, 51.0, 46.0, 1000.0,
            ]; 2],
            shaping: [0.0; 2],
        }
    }
}
fn score(mut value: f64, bins: &mut [f64; 11]) -> (usize, f64, bool) {
    while value >= 1.0 {
        value /= 10.0;
    }
    while value < 1.0 && value > 0.0000001 {
        value *= 10.0;
    }
    let bin = value.floor() as i32;
    if !(1..10).contains(&bin) {
        return (10, 0.0, false);
    }
    let bin = bin as usize;
    bins[bin] += 1.0;
    let cut = bins[bin] > 982.0;
    let target = [301.0, 176.0, 125.0, 97.0, 79.0, 67.0, 58.0, 51.0, 46.0];
    let mut total = 0.0;
    for i in 0..9 {
        total += target[i] - bins[i + 1];
    }
    bins[bin] -= 1.0;
    (bin, total, cut)
}
impl AirwindowsEffect for NotJustAnotherDither {
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
        match i {
            0 => {
                if self.params[0] as f64 * 1.999 < 1.0 {
                    "CD 16".into()
                } else {
                    "HD 24".into()
                }
            }
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = if (self.params[0] as f64 * 1.999) as i32 == 1 {
            8388608.0_f32
        } else {
            32768.0_f32
        };
        if self.params[1] > 0.0 {
            scale = (scale as f64 * (1.0 - self.params[1] as f64).powf(6.0)) as f32;
        }
        scale = scale.max(0.0001);
        let outscale = scale.max(8.0);
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            for (i, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[i] as f64 * 1.18e-17;
                }
                xorshift(&mut self.fpd[i]);
                *sample *= scale as f64;
                let dry = *sample;
                *sample -= self.shaping[i];
                let (a, total_a, cut_a) = score(sample.floor(), &mut self.bins[i]);
                let (b, total_b, cut_b) = score(sample.ceil(), &mut self.bins[i]);
                let output = if total_a < total_b {
                    self.bins[i][a] += 1.0;
                    sample.floor()
                } else {
                    self.bins[i][b] += 1.0;
                    (*sample + 1.0).floor()
                };
                if cut_a || cut_b {
                    for bin in &mut self.bins[i][1..=10] {
                        *bin *= 0.99;
                    }
                }
                self.shaping[i] += output - dry;
                self.shaping[i] = self.shaping[i].clamp(-sample.abs(), sample.abs());
                *sample = (*sample / outscale as f64).clamp(-1.0, 1.0);
            }
            *outl = samples[0] as f32;
            *outr = samples[1] as f32;
        }
    }
}
