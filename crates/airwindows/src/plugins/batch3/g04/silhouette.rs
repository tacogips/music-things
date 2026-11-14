//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Silhouette, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)] // Preserve upstream DSP constants and state ordering.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Dry/Wet",
    label: "",
    default: 0.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Silhouette",
    upstream_name: "Silhouette",
    description: "Silhouette replaces the sound with raw noise sculpted to match its dynamics.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Silhouette::new(sample_rate, seed))
}
struct Silhouette {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    last: [f64; 2],
    out: [f64; 2],
}
impl Silhouette {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0],
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            last: [0.0; 2],
            out: [0.0; 2],
        }
    }
}
impl AirwindowsEffect for Silhouette {
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
        let wet = (self.params[0] as f64).powf(5.0);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut input = [l as f64, r as f64];
            for (ch, x) in input.iter_mut().enumerate() {
                if x.abs() < 1.18e-23 {
                    *x = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *x;
                let mut bridge = x.abs() * 1.57079633;
                if bridge > 1.57079633 {
                    bridge = 1.57079633;
                }
                bridge = 1.0 - bridge.cos();
                *x = if *x > 0.0 { bridge } else { -bridge };
                let mut silhouette = self.fpd[ch] as f64 / u32::MAX as f64;
                silhouette -= 0.5;
                silhouette *= 2.0;
                silhouette *= x.abs();
                xorshift(&mut self.fpd[ch]);
                let mut smoother = self.fpd[ch] as f64 / u32::MAX as f64;
                smoother -= 0.5;
                smoother *= 2.0;
                smoother *= self.last[ch].abs();
                self.last[ch] = *x;
                silhouette += smoother;
                bridge = silhouette.abs() * 1.57079633;
                if bridge > 1.57079633 {
                    bridge = 1.57079633;
                }
                bridge = bridge.sin();
                silhouette = if silhouette > 0.0 { bridge } else { -bridge };
                *x = (silhouette + self.out[ch]) / 2.0;
                self.out[ch] = silhouette;
                if wet != 1.0 {
                    *x = (*x * wet) + (dry * (1.0 - wet));
                }
                *x = fpd_dither(*x, &mut self.fpd[ch]);
            }
            *out_l = input[0] as f32;
            *out_r = input[1] as f32;
        }
    }
}
