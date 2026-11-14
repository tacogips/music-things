//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/XNotch, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)] // Preserve upstream coefficients.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Gain",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Freq",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Nuke",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "XNotch",
    upstream_name: "XNotch",
    description: "XNotch is a distorted digital EQ, not as glitchy as the others.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(XNotch::new(sample_rate, seed))
}
struct XNotch {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    filters: [[f64; 15]; 5],
    fpd_l: u32,
    fpd_r: u32,
}
impl XNotch {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 1.0, 0.0, 1.0],
            sample_rate,
            seed,
            filters: [[0.0; 15]; 5],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn stage(b: &mut [f64; 15], input: f64, right: bool) -> f64 {
    let i = if right { 11 } else { 7 };
    let out = b[2] * input + b[3] * b[i] + b[4] * b[i + 1] - b[5] * b[i + 2] - b[6] * b[i + 3];
    b[i + 1] = b[i];
    b[i] = input;
    b[i + 3] = b[i + 2];
    b[i + 2] = out;
    out
}
fn sine(x: f64) -> f64 {
    x.clamp(-1.57079633, 1.57079633).sin()
}
impl AirwindowsEffect for XNotch {
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
            .map_or_else(String::new, |&p| float2string(p))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let gain = (a as f64 + 0.5).powf(4.0);
        let freq = ((b as f64).powf(3.0) * 20000.0 / self.sample_rate as f64).max(0.0003);
        let clip = 1.0 + (c as f64).powf(3.0) * 2.0;
        let k = (std::f64::consts::PI * freq).tan();
        let norm = 1.0 / (1.0 + k / 0.7071 + k * k);
        for filter in &mut self.filters {
            filter[0] = freq;
            filter[2] = (1.0 + k * k) * norm;
            filter[3] = 2.0 * (k * k - 1.0) * norm;
            filter[4] = filter[2];
            filter[5] = filter[3];
            filter[6] = (1.0 - k / 0.7071 + k * k) * norm;
        }
        let nuke = c as f64 * 4.0;
        let wets = [
            nuke.clamp(0.0, 1.0),
            (nuke - 1.0).clamp(0.0, 1.0),
            (nuke - 2.0).clamp(0.0, 1.0),
            (nuke - 3.0).clamp(0.0, 1.0),
        ];
        let wet = d as f64;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            if gain != 1.0 {
                l *= gain;
                r *= gain;
            }
            l = stage(&mut self.filters[0], sine(l), false);
            r = stage(&mut self.filters[0], sine(r), true);
            for (i, amount) in wets.into_iter().enumerate() {
                if amount > 0.0 {
                    let next_l = stage(&mut self.filters[i + 1], sine(l * clip), false);
                    let next_r = stage(&mut self.filters[i + 1], sine(r * clip), true);
                    l = next_l * amount + l * (1.0 - amount);
                    r = next_r * amount + r * (1.0 - amount);
                }
            }
            l = sine(l);
            r = sine(r);
            if wet < 1.0 {
                l = dry_l * (1.0 - wet) + l * wet;
                r = dry_r * (1.0 - wet) + r * wet;
            }
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
