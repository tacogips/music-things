//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Dubly, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Dubly",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Dubly",
    upstream_name: "Dubly",
    description: "Dubly is a retro vibe and atmosphere maker.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Dubly::new(rate, seed))
}
struct Dubly {
    params: [f32; 2],
    rate: f32,
    seed: u32,
    iir: [[f64; 2]; 2],
    fpd: [u32; 2],
}
impl Dubly {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 1.0],
            rate,
            seed,
            iir: [[0.0; 2]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn encode(sample: f64, iir: &mut f64, amount: f64) -> f64 {
    *iir = *iir * (1.0 - amount) + sample * amount;
    let mut difference = (sample - *iir).clamp(-1.0, 1.0);
    if difference > 0.0 {
        difference = (1.0 + 255.0 * difference.abs()).ln() / 2.40823996531;
    }
    if difference < 0.0 {
        difference = -(1.0 + 255.0 * difference.abs()).ln() / 2.40823996531;
    }
    difference
}
impl AirwindowsEffect for Dubly {
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
        self.params
            .get(i)
            .map_or_else(String::new, |&v| float2string(v))
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let a = self.params[0] as f64;
        let wet = self.params[1] as f64;
        let dubly = a.powi(3) * 0.16;
        let outly = a.powi(3) * 0.160618;
        let gain = outly + 1.0;
        let amount = 0.4275 / ((1.0 / 44100.0) * self.rate as f64);
        for (((&l, &r), ol), or) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *sample;
                *sample += encode(*sample, &mut self.iir[0][ch], amount) * dubly;
                *sample *= gain;
                *sample = sample.sin();
                *sample /= gain;
                *sample -= encode(*sample, &mut self.iir[1][ch], amount) * outly;
                if wet != 1.0 {
                    *sample = *sample * wet + dry * (1.0 - wet);
                }
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            *ol = samples[0] as f32;
            *or = samples[1] as f32;
        }
    }
}
