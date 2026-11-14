//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/SoftGate, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Thresh",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Darken",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Silence",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "SoftGate",
    upstream_name: "SoftGate",
    description: "SoftGate is a gate that can mute hiss and smooth sample tails.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(SoftGate::new(sample_rate, seed))
}
struct SoftGate {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
    stored_l: [f64; 2],
    stored_r: [f64; 2],
    gate: f64,
}
impl SoftGate {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            stored_l: [0.0; 2],
            stored_r: [0.0; 2],
            gate: 1.1,
        }
    }
}
impl AirwindowsEffect for SoftGate {
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
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c] = self.params;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let threshold = (a as f64).powf(6.0);
        let mut recovery = (b as f64 * 0.5).powf(6.0);
        recovery /= overallscale;
        let baseline = (c as f64).powf(6.0);
        let invrec = 1.0 - recovery;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            self.stored_l[1] = self.stored_l[0];
            self.stored_l[0] = l;
            let diff_l = self.stored_l[0] - self.stored_l[1];
            self.stored_r[1] = self.stored_r[0];
            self.stored_r[0] = r;
            let diff_r = self.stored_r[0] - self.stored_r[1];
            if self.gate > 0.0 {
                self.gate = ((self.gate - baseline) * invrec) + baseline;
            }
            if diff_r.abs() > threshold || diff_l.abs() > threshold {
                self.gate = 1.1;
            } else {
                self.gate *= invrec;
                if threshold > 0.0 {
                    self.gate += (l.abs() / threshold) * recovery;
                    self.gate += (r.abs() / threshold) * recovery;
                }
            }
            if self.gate < 0.0 {
                self.gate = 0.0;
            }
            if self.gate < 1.0 {
                self.stored_l[0] = self.stored_l[1] + diff_l * self.gate;
                self.stored_r[0] = self.stored_r[1] + diff_r * self.gate;
            }
            if self.gate < 1.0 {
                l = l * self.gate + self.stored_l[0] * (1.0 - self.gate);
                r = r * self.gate + self.stored_r[0] * (1.0 - self.gate);
            }
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
