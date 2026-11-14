//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Exciter, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.6,
    },
    ParamInfo {
        name: "Excite",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Exciter",
    upstream_name: "Exciter",
    description: "Exciter is an Aural Exciter plugin that can be both subtle and extreme.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Exciter::new(rate, seed))
}
struct Exciter {
    params: [f32; 2],
    rate: f32,
    seed: u32,
    biquad: [f64; 11],
    l: u32,
    r: u32,
}
impl Exciter {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.6, 0.0],
            rate,
            seed,
            biquad: [0.0; 11],
            l: fpd_from_seed(seed, 0),
            r: fpd_from_seed(seed, 1),
        }
    }
}
fn channel(input: f64, filter: &mut [f64; 11], offset: usize, boost: f64) -> f64 {
    let mut output = input * filter[2] + filter[offset];
    filter[offset] = input * filter[3] - output * filter[5] + filter[offset + 1];
    filter[offset + 1] = input * filter[4] - output * filter[6];
    output = (output * boost).clamp(-std::f64::consts::PI, std::f64::consts::PI);
    output -= output.sin();
    output = (output * boost)
        .clamp(-std::f64::consts::PI, std::f64::consts::PI)
        .sin();
    (input - output).clamp(-1.0, 1.0)
}
impl AirwindowsEffect for Exciter {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(value)) {
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
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    #[allow(clippy::approx_constant)] // Preserve the upstream decimal coefficient.
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let [a, b] = self.params;
        self.biquad[0] = (a as f64 * 7000.0 + 8000.0) / self.rate as f64;
        self.biquad[1] = (a + b) as f64 + 0.7071;
        let boost = (b as f64).powf(2.0) * 16.0;
        let k = (std::f64::consts::PI * self.biquad[0]).tan();
        let norm = 1.0 / (1.0 + k / self.biquad[1] + k * k);
        self.biquad[2] = k / 0.7071 * norm;
        self.biquad[4] = -self.biquad[2];
        self.biquad[5] = 2.0 * (k * k - 1.0) * norm;
        self.biquad[6] = (1.0 - k / self.biquad[1] + k * k) * norm;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.r as f64 * 1.18e-17;
            }
            *out_l = fpd_dither(channel(l, &mut self.biquad, 7, boost), &mut self.l) as f32;
            *out_r = fpd_dither(channel(r, &mut self.biquad, 9, boost), &mut self.r) as f32;
        }
    }
}
