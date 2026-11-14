//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Shape, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)] // Preserve upstream DSP constants and state ordering.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Shape",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Neg/Pos",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Shape",
    upstream_name: "Shape",
    description: "Shape is an asymmetrical waveshaper for peak manipulating.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Shape::new(sample_rate, seed))
}
struct Shape {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
}
impl Shape {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Shape {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let shape = -((self.params[0] as f64 * 2.0) - 1.0);
        let mut gain = shape.abs() + 0.01;
        let offset = (self.params[1] as f64 * 2.0) - 1.0;
        let mut post = 0.0;
        if shape > 0.0 {
            gain += 0.99;
            post = offset.sin();
        }
        if shape < 0.0 {
            post = offset.asin();
        }
        let cutoff = (25000.0 / self.sample_rate as f64).min(0.49);
        let k = (std::f64::consts::PI * cutoff).tan();
        let norm = 1.0 / (1.0 + k / 0.70710678 + k * k);
        let a0 = k * k * norm;
        let a1 = 2.0 * a0;
        let a2 = a0;
        let b1 = 2.0 * (k * k - 1.0) * norm;
        let b2 = (1.0 - k / 0.70710678 + k * k) * norm;
        let mut state = [[0.0_f64; 2]; 2];
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut input = [l as f64, r as f64];
            if input[0].abs() < 1.18e-23 {
                input[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if input[1].abs() < 1.18e-23 {
                input[1] = self.fpd_r as f64 * 1.18e-17;
            }
            for ch in 0..2 {
                let dry = input[ch];
                let out = input[ch] * a0 + state[ch][0];
                state[ch][0] = (input[ch] * a1) - (out * b1) + state[ch][1];
                state[ch][1] = (input[ch] * a2) - (out * b2);
                let mut x = out;
                x *= gain;
                x += offset;
                x = x.clamp(-1.0, 1.0);
                if shape > 0.0 {
                    x = x.sin();
                }
                if shape < 0.0 {
                    x = x.asin();
                }
                x -= post;
                input[ch] = ((x / gain) * shape.abs()) + (dry * (1.0 - shape.abs()));
            }
            *out_l = fpd_dither(input[0], &mut self.fpd_l) as f32;
            *out_r = fpd_dither(input[1], &mut self.fpd_r) as f32;
        }
    }
}
