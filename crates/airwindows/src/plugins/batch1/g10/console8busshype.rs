// Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
// Upstream: airwindows/plugins/LinuxVST/src/Console8BussHype/Console8BussHype.h, Console8BussHype.cpp, Console8BussHypeProc.cpp
// Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Libm, measured max-abs 0.0.
// Upstream literal constants are intentionally preserved for parity.
#![allow(clippy::approx_constant, clippy::excessive_precision)]
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) static INFO: PluginInfo = PluginInfo { name: "Console8BussHype", upstream_name: "Console8BussHype", description: "Console8BussHype swaps out just the In section of Console8BussIn, for a brighter sparklier circuit.", params: &[] };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Console8BussHype::new(sample_rate, seed))
}
struct Console8BussHype {
    sample_rate: f32,
    seed: u32,
    iir_a: [f64; 2],
    iir_b: [f64; 2],
    fix: [[f64; 2]; 2],
    soft: [[f64; 11]; 2],
    fpd: [u32; 2],
}
impl Console8BussHype {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        let _ = replay_fpd_seed_loop(&mut rng);
        let _ = replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            iir_a: [0.0; 2],
            iir_b: [0.0; 2],
            fix: [[0.0; 2]; 2],
            soft: [[0.0; 11]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Console8BussHype {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, _: usize) -> f32 {
        0.0
    }
    fn set_param(&mut self, _: usize, _: f32) {}
    fn param_display(&self, _: usize) -> String {
        String::new()
    }
    fn reset(&mut self) {
        *self = Self::new(self.sample_rate, self.seed);
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let rate = self.sample_rate as f64;
        let iir_amount = 12.66 / rate;
        for channel in 0..2 {
            if self.iir_a[channel].abs() < 1.18e-37 {
                self.iir_a[channel] = 0.0;
            }
            if self.iir_b[channel].abs() < 1.18e-37 {
                self.iir_b[channel] = 0.0;
            }
        }
        let k = (std::f64::consts::PI * (24000.0 / rate)).tan();
        let norm = 1.0 / (1.0 + k / 0.5 + k * k);
        let a0 = k * k * norm;
        let a1 = 2.0 * a0;
        let a2 = a0;
        let b1 = 2.0 * (k * k - 1.0) * norm;
        let b2 = (1.0 - k / 0.5 + k * k) * norm;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= rate;
        let mut cycle = overallscale.floor() as i32;
        if cycle < 1 {
            cycle = 1;
        }
        if cycle == 3 {
            cycle = 4;
        }
        if cycle > 4 {
            cycle = 4;
        }
        let len = in_l.len().min(in_r.len()).min(out_l.len()).min(out_r.len());
        for i in 0..len {
            for ch in 0..2 {
                let mut sample = if ch == 0 {
                    in_l[i] as f64
                } else {
                    in_r[i] as f64
                };
                if sample.abs() < 1.18e-23 {
                    sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                self.iir_a[ch] = (self.iir_a[ch] * (1.0 - iir_amount)) + (sample * iir_amount);
                let amount_b = self.iir_a[ch].abs() + 0.00001;
                self.iir_b[ch] = (self.iir_b[ch] * (1.0 - amount_b)) + (self.iir_a[ch] * amount_b);
                sample -= self.iir_b[ch];
                let soft = &mut self.soft[ch];
                if cycle == 4 {
                    for n in (1..=8).rev() {
                        soft[n] = soft[n - 1];
                    }
                    soft[0] = sample;
                }
                if cycle == 2 {
                    for n in [8, 6, 4, 2] {
                        soft[n] = soft[n - 2];
                    }
                    soft[0] = sample;
                }
                if cycle == 1 {
                    soft[8] = soft[4];
                    soft[4] = soft[0];
                    soft[0] = sample;
                }
                soft[9] = (soft[0] - soft[4]) - (soft[4] - soft[8]);
                soft[9] = soft[9].clamp(-1.57079633, 1.57079633);
                sample = soft[8] + (soft[9].sin() * 0.61803398);
                if rate > 49000.0 {
                    let out = (sample * a0) + self.fix[ch][0];
                    self.fix[ch][0] = (sample * a1) - (out * b1) + self.fix[ch][1];
                    self.fix[ch][1] = (sample * a2) - (out * b2);
                    sample = out;
                }
                sample = sample.clamp(-1.0, 1.0).asin();
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
