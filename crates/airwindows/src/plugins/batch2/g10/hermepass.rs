//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Hermepass, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::excessive_precision)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Cutoff",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "Slope",
        label: " ",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Hermepass",
    upstream_name: "Hermepass",
    description: "Hermepass is a mastering highpass to set by ear only.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Hermepass::new(sample_rate, seed))
}
struct Hermepass {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    iir: [[f64; 8]; 2],
    flip: bool,
}
impl Hermepass {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5; 2],
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            iir: [[0.0; 8]; 2],
            flip: true,
        }
    }
}
impl AirwindowsEffect for Hermepass {
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
            .map_or_else(String::new, |p| float2string(*p))
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let old = 0.618033988749894848204586f64;
        let range = 0.1 / (self.sample_rate as f64 / 44100.0);
        let mut cutoff = (self.params[0] as f64).powf(3.0);
        let mut coeff = [0.0; 8];
        coeff[0] = cutoff * range;
        coeff[1] = coeff[0];
        coeff[2] = coeff[0];
        for c in &mut coeff[3..] {
            cutoff = cutoff * old + 0.00001 * (1.0 - old);
            *c = cutoff * range;
        }
        let mut slope = (self.params[1] as f64).powf(3.0) * 6.0;
        let mut poles = [0.0; 6];
        for p in &mut poles {
            *p = slope.min(1.0);
            slope = (slope - 1.0).max(0.0);
        }
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                let mut temp = *sample;
                let first = if self.flip { 0 } else { 1 };
                self.iir[ch][first] =
                    self.iir[ch][first] * (1.0 - coeff[first]) + temp * coeff[first];
                temp -= self.iir[ch][first];
                let mut correction = self.iir[ch][first];
                for i in 2..8 {
                    self.iir[ch][i] = self.iir[ch][i] * (1.0 - coeff[i]) + temp * coeff[i];
                    temp -= self.iir[ch][i];
                    if poles[i - 2] == 1.0 {
                        correction += self.iir[ch][i];
                    } else if poles[i - 2] > 0.0 && poles[i - 2] < 1.0 {
                        correction += self.iir[ch][i] * poles[i - 2];
                    }
                }
                *sample = fpd_dither(*sample - correction, &mut self.fpd[ch]);
            }
            self.flip = !self.flip;
            *outl = samples[0] as f32;
            *outr = samples[1] as f32;
        }
    }
}
