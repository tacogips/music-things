//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ChromeOxide/ChromeOxideProc.cpp
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.
#![allow(clippy::approx_constant)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "ChromeOxide",
    upstream_name: "ChromeOxide",
    description: "ChromeOxide is an alternate path to vibey old tape sonics.",
    params: &[
        ParamInfo {
            name: "Intense",
            label: "",
            default: 0.5,
        },
        ParamInfo {
            name: "Bias",
            label: "",
            default: 0.5,
        },
    ],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ChromeOxide::new(rate, seed))
}
struct ChromeOxide {
    rate: f32,
    seed: u32,
    params: [f32; 2],
    iir: [[f64; 4]; 2],
    delay: [[f64; 4]; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl ChromeOxide {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            rate,
            seed,
            params: [0.5; 2],
            iir: [[0.0; 4]; 2],
            delay: [[0.0; 4]; 2],
            flip: false,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for ChromeOxide {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(index), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, index: usize) -> String {
        self.params
            .get(index)
            .map_or_else(String::new, |&v| float2string(v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = 1.0_f64;
        scale /= 44100.0;
        scale *= self.rate as f64;
        let mut bias = self.params[1] as f64 / 1.31578947368421;
        let intensity = 0.9 + (self.params[0] as f64).powf(2.0);
        let amount = (1.0 - (intensity / (10.0 + (bias * 4.0)))).powf(2.0) / scale;
        let mut treble = 1.0;
        let mut drive = 1.0;
        let density = (intensity * 80.0) + 1.0;
        let mut noise = intensity / (1.0 + bias);
        let mut bass_trim = 1.0;
        let mut glitch = 0.0;
        bias *= scale;
        noise *= scale;
        if intensity > 1.0 {
            glitch = intensity - 1.0;
            drive = intensity * intensity;
            bass_trim /= intensity * intensity;
            treble = (intensity + 1.0) / 2.0;
        }
        for (((&left, &right), out_l), out_r) in l.iter().zip(r).zip(ol).zip(or) {
            let mut output = [0.0_f32; 2];
            for (c, sample) in [left, right].into_iter().enumerate() {
                let mut input = sample as f64;
                if input.abs() < 1.18e-23 {
                    input = self.fpd[c] as f64 * 1.18e-17;
                }
                input *= drive;
                let mut bass = input;
                let hi = if self.flip { 0 } else { 1 };
                self.iir[c][hi] = (self.iir[c][hi] * (1.0 - amount)) + (input * amount);
                input -= self.iir[c][hi];
                bass -= input * (input.abs() * glitch) * (input.abs() * glitch);
                let lo = if self.flip { 2 } else { 3 };
                self.iir[c][lo] = (self.iir[c][lo] * (1.0 - amount)) + (bass * amount);
                bass = self.iir[c][lo];
                let random = bias + ((self.fpd[c] as f64 / u32::MAX as f64) * noise);
                let mut bridge = input;
                let d = self.delay[c];
                if (0.0..1.0).contains(&random) {
                    bridge = (input * random) + (d[0] * (1.0 - random));
                }
                if (1.0..2.0).contains(&random) {
                    bridge = (d[0] * (random - 1.0)) + (d[1] * (2.0 - random));
                }
                if (2.0..3.0).contains(&random) {
                    bridge = (d[1] * (random - 2.0)) + (d[2] * (3.0 - random));
                }
                if (3.0..4.0).contains(&random) {
                    bridge = (d[2] * (random - 3.0)) + (d[3] * (4.0 - random));
                }
                self.delay[c] = [input, d[0], d[1], d[2]];
                input = bridge;
                bridge = input.abs() * density;
                if bridge > 1.57079633 {
                    bridge = 1.57079633;
                }
                bridge = bridge.sin();
                if input > 0.0 {
                    input = bridge / density;
                } else {
                    input = -bridge / density;
                }
                input *= treble;
                bass *= bass_trim;
                input += bass;
                output[c] = fpd_dither(input, &mut self.fpd[c]) as f32;
            }
            self.flip = !self.flip;
            *out_l = output[0];
            *out_r = output[1];
        }
    }
}
