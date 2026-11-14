//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Isolator2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
#![allow(clippy::approx_constant)]

use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Freq",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Reso",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "High",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Low",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Isolator2",
    upstream_name: "Isolator2",
    description: "Isolator2 is Isolator, but with control smoothing and a new resonance booster.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Isolator2::new(sample_rate, seed))
}
struct Isolator2 {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    coeff: [[f64; 5]; 7],
    state: [[[f64; 2]; 2]; 7],
    high: f64,
    low: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl Isolator2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.0, 0.0, 1.0],
            sample_rate,
            seed,
            coeff: [[0.0; 5]; 7],
            state: [[[0.0; 2]; 2]; 7],
            high: 0.0,
            low: 1.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Isolator2 {
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
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let scale = self.sample_rate as f64 / 44100.0;
        let freq = (a as f64)
            .powf(2.0 * scale.sqrt())
            .mul_add(0.4999, 0.0)
            .max(0.0005);
        let reso = (b as f64).powi(2);
        let mut next = [[0.0; 5]; 7];
        for (coeff, r) in next.iter_mut().zip([
            4.46570214, 1.51387132, 0.93979296, 0.70710678, 0.59051105, 0.52972649, 0.50316379,
        ]) {
            let r = r * (reso + 1.0);
            let k = (std::f64::consts::PI * freq).tan();
            let norm = 1.0 / (1.0 + k / r + k * k);
            let a0 = k * k * norm;
            *coeff = [
                a0,
                2.0 * a0,
                a0,
                2.0 * (k * k - 1.0) * norm,
                (1.0 - k / r + k * k) * norm,
            ];
        }
        let high = c as f64 * (1.0 - reso);
        let low = d as f64 * (1.0 - reso);
        let bypass = a == 1.0 && high <= 0.0 && low >= 1.0;
        let n = il.len().min(ir.len()).min(ol.len()).min(orr.len());
        for i in 0..n {
            let mut l = il[i] as f64;
            let mut r = ir[i] as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            let t = (n - i - 1) as f64 / n as f64;
            for (j, next_coeff) in next.iter().enumerate() {
                let mut c = [0.0; 5];
                for (k, v) in c.iter_mut().enumerate() {
                    *v = self.coeff[j][k] * t + next_coeff[k] * (1.0 - t);
                }
                for (v, s) in [&mut l, &mut r].into_iter().zip(self.state[j].iter_mut()) {
                    let out = *v * c[0] + s[0];
                    s[0] = *v * c[1] - out * c[3] + s[1];
                    s[1] = *v * c[2] - out * c[4];
                    *v = out;
                }
            }
            if bypass {
                l = dry_l;
                r = dry_r;
            } else {
                let h = self.high * t + high * (1.0 - t);
                let lo = self.low * t + low * (1.0 - t);
                l = l * lo + (dry_l - l) * h;
                r = r * lo + (dry_r - r) * h;
            }
            ol[i] = fpd_dither(l, &mut self.fpd_l) as f32;
            orr[i] = fpd_dither(r, &mut self.fpd_r) as f32;
        }
        self.coeff = next;
        self.high = high;
        self.low = low;
    }
}
