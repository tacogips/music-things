// Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
// Upstream: airwindows/plugins/LinuxVST/src/Console8BussOut/Console8BussOut.h, Console8BussOut.cpp, Console8BussOutProc.cpp
// Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Libm, measured max-abs 0.0.
// Upstream literal constants are intentionally preserved for parity.
#![allow(clippy::approx_constant, clippy::excessive_precision)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "Console8BusOut",
    upstream_name: "Console8BussOut",
    description:
        "Console8BussOut moves to a channel/submix/buss topology and adds analog simulation.",
    params: &[ParamInfo {
        name: "Fader",
        label: "",
        default: 0.5,
    }],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Console8BussOut::new(sample_rate, seed))
}
struct Console8BussOut {
    sample_rate: f32,
    seed: u32,
    params: [f32; 1],
    trim_a: f64,
    trim_b: f64,
    fix: [[f64; 2]; 2],
    last: [f64; 2],
    pos: [bool; 2],
    neg: [bool; 2],
    intermediate: [[f64; 17]; 2],
    fpd: [u32; 2],
}
impl Console8BussOut {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        let _ = replay_fpd_seed_loop(&mut rng);
        let _ = replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            params: [0.5],
            trim_a: 0.5,
            trim_b: 0.5,
            fix: [[0.0; 2]; 2],
            last: [0.0; 2],
            pos: [false; 2],
            neg: [false; 2],
            intermediate: [[0.0; 17]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Console8BussOut {
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
            float2string(self.params[0])
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let len = in_l.len().min(in_r.len()).min(out_l.len()).min(out_r.len());
        self.trim_a = self.trim_b;
        self.trim_b = self.params[0] as f64 * 2.0;
        let rate = self.sample_rate as f64;
        let k = (std::f64::consts::PI * (24000.0 / rate)).tan();
        let norm = 1.0 / (1.0 + k / 0.52110856 + k * k);
        let a0 = k * k * norm;
        let a1 = 2.0 * a0;
        let a2 = a0;
        let b1 = 2.0 * (k * k - 1.0) * norm;
        let b2 = (1.0 - k / 0.52110856 + k * k) * norm;
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= rate;
        let spacing = (scale.floor() as usize).clamp(1, 16);
        for i in 0..len {
            let position = (len - i - 1) as f64 / len as f64;
            let trim = self.trim_a * position + self.trim_b * (1.0 - position);
            for ch in 0..2 {
                let mut sample = if ch == 0 {
                    in_l[i] as f64
                } else {
                    in_r[i] as f64
                };
                if sample.abs() < 1.18e-23 {
                    sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                sample *= trim;
                sample = sample.clamp(-1.57079633, 1.57079633).sin();
                if rate > 49000.0 {
                    let out = sample * a0 + self.fix[ch][0];
                    self.fix[ch][0] = sample * a1 - out * b1 + self.fix[ch][1];
                    self.fix[ch][1] = sample * a2 - out * b2;
                    sample = out;
                }
                sample *= trim;
                sample = sample.clamp(-4.0, 4.0);
                if self.pos[ch] {
                    if sample < self.last[ch] {
                        self.last[ch] = 0.7058208 + sample * 0.2609148;
                    } else {
                        self.last[ch] = 0.2491717 + self.last[ch] * 0.7390851;
                    }
                }
                self.pos[ch] = false;
                if sample > 0.9549925859 {
                    self.pos[ch] = true;
                    sample = 0.7058208 + self.last[ch] * 0.2609148;
                }
                if self.neg[ch] {
                    if sample > self.last[ch] {
                        self.last[ch] = -0.7058208 + sample * 0.2609148;
                    } else {
                        self.last[ch] = -0.2491717 + self.last[ch] * 0.7390851;
                    }
                }
                self.neg[ch] = false;
                if sample < -0.9549925859 {
                    self.neg[ch] = true;
                    sample = -0.7058208 + self.last[ch] * 0.2609148;
                }
                self.intermediate[ch][spacing] = sample;
                sample = self.last[ch];
                for x in (1..=spacing).rev() {
                    self.intermediate[ch][x - 1] = self.intermediate[ch][x];
                }
                self.last[ch] = self.intermediate[ch][0];
                sample = fpd_dither(sample, &mut self.fpd[ch]);
                if ch == 0 {
                    out_l[i] = sample as f32;
                } else {
                    out_r[i] = sample as f32;
                }
            }
        }
    }
}
