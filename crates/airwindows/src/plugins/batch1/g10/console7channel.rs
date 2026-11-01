//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: plugins/LinuxVST/src/Console7Channel, b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
// Upstream literal constants are intentionally preserved for parity.
#![allow(clippy::approx_constant, clippy::excessive_precision)]
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Console7Channel",
    upstream_name: "Console7Channel",
    description: "Console7Channel adds anti-alias filtering and special saturation curves.",
    params: &[ParamInfo {
        name: "Fader",
        label: "",
        default: 0.772,
    }],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Console7Channel::new(sample_rate, seed))
}
struct Console7Channel {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    gainchase: f64,
    chasespeed: f64,
    a: [f64; 15],
    fpd_l: u32,
    fpd_r: u32,
}
impl Console7Channel {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.772],
            sample_rate,
            seed,
            gainchase: -1.0,
            chasespeed: 64.0,
            a: [0.0; 15],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
    fn filter(b: &mut [f64; 15], input: f64, i: usize) -> f64 {
        let output =
            b[2] * input + b[3] * b[i] + b[4] * b[i + 1] - b[5] * b[i + 2] - b[6] * b[i + 3];
        b[i + 1] = b[i];
        b[i] = input;
        b[i + 3] = b[i + 2];
        b[i + 2] = output;
        output
    }
    fn coeff(b: &mut [f64; 15], rate: f32, q: f64) {
        b[0] = 20000.0 / f64::from(rate);
        b[1] = q;
        let k = (std::f64::consts::PI * b[0]).tan();
        let norm = 1.0 / (1.0 + k / b[1] + k * k);
        b[2] = k * k * norm;
        b[3] = 2.0 * b[2];
        b[4] = b[2];
        b[5] = 2.0 * (k * k - 1.0) * norm;
        b[6] = (1.0 - k / b[1] + k * k) * norm;
    }
    fn saturate(x: f64) -> f64 {
        let x = x.clamp(-1.097, 1.097);
        let abs = x.abs();
        ((x * abs).sin() / if abs == 0.0 { 1.0 } else { abs }) * 0.8 + x.sin() * 0.2
    }
}
impl AirwindowsEffect for Console7Channel {
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
        if index == 0 {
            crate::display::float2string(self.params[0])
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let n = il.len().min(ir.len()).min(ol.len()).min(or.len());
        let gain = f64::from(self.params[0]) * 1.272019649514069;
        if self.gainchase != gain {
            self.chasespeed *= 2.0;
        }
        if self.chasespeed > n as f64 {
            self.chasespeed = n as f64;
        }
        if self.gainchase < 0.0 {
            self.gainchase = gain;
        }
        Self::coeff(&mut self.a, self.sample_rate, 1.618033988749894848204586);
        for i in 0..n {
            let mut l = f64::from(il[i]);
            let mut r = f64::from(ir[i]);
            if l.abs() < 1.18e-23 {
                l = f64::from(self.fpd_l) * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = f64::from(self.fpd_r) * 1.18e-17;
            }
            l = Self::filter(&mut self.a, l, 7);
            r = Self::filter(&mut self.a, r, 11);
            self.chasespeed *= 0.9999;
            self.chasespeed -= 0.01;
            if self.chasespeed < 64.0 {
                self.chasespeed = 64.0;
            }
            self.gainchase = ((self.gainchase * self.chasespeed) + gain) / (self.chasespeed + 1.0);
            if self.gainchase != 1.0 {
                l *= self.gainchase.powf(3.0);
                r *= self.gainchase.powf(3.0);
            }
            l = Self::saturate(l);
            r = Self::saturate(r);
            if self.gainchase != 1.0 && self.gainchase != 0.0 {
                l /= self.gainchase;
                r /= self.gainchase;
            }
            ol[i] = fpd_dither(l, &mut self.fpd_l) as f32;
            or[i] = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
