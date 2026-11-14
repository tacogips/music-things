//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PurestWarm2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)] // Upstream coefficients are intentionally rounded.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Dry/Pos",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Neg",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PurestWarm2",
    upstream_name: "PurestWarm2",
    description: "PurestWarm2 adds dual dry/wet controls for each side of the waveform.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PurestWarm2::new(sample_rate, seed))
}
struct PurestWarm2 {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
}
impl PurestWarm2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 0.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for PurestWarm2 {
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
        let pos = self.params[0] as f64;
        let neg = self.params[1] as f64;
        let cutoff = (25000.0 / self.sample_rate as f64).min(0.49);
        let k = (std::f64::consts::PI * cutoff).tan();
        let norm = 1.0 / (1.0 + k / 0.70710678 + k * k);
        let a0 = k * k * norm;
        let a1 = 2.0 * a0;
        let a2 = a0;
        let b1 = 2.0 * (k * k - 1.0) * norm;
        let b2 = (1.0 - k / 0.70710678 + k * k) * norm;
        let (mut sl1, mut sl2, mut sr1, mut sr2) = (0.0, 0.0, 0.0, 0.0);
        for (((&l, &r), ol), orr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let o = l * a0 + sl1;
            sl1 = l * a1 - o * b1 + sl2;
            sl2 = l * a2 - o * b2;
            l = o;
            if l > 0.0 {
                l = (l * 1.57079634 * pos).sin() / 1.57079634 + l * (1.0 - pos);
            }
            if l < 0.0 {
                l = (l * 1.57079634 * neg).sin() / 1.57079634 + l * (1.0 - neg);
            }
            let o = r * a0 + sr1;
            sr1 = r * a1 - o * b1 + sr2;
            sr2 = r * a2 - o * b2;
            r = o;
            if r > 0.0 {
                r = (r * 1.57079634 * pos).sin() / 1.57079634 + r * (1.0 - pos);
            }
            if r < 0.0 {
                r = (r * 1.57079634 * neg).sin() / 1.57079634 + r * (1.0 - neg);
            }
            *ol = fpd_dither(l, &mut self.fpd_l) as f32;
            *orr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
