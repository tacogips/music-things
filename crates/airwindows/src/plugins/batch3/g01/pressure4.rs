//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Pressure4, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Pressure",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Speed",
        label: " ",
        default: 0.2,
    },
    ParamInfo {
        name: "Mewiness",
        label: " ",
        default: 1.0,
    },
    ParamInfo {
        name: "Output Gain",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo { name: "Pressure4", upstream_name: "Pressure4", description: "Pressure4 is a compressor adjustable between vari-mu and ‘new york’ peak-retaining behaviors.", params: PARAMS };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Pressure4::new(sample_rate, seed))
}
struct Pressure4 {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    speed: f64,
    coefficient: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl Pressure4 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.2, 1.0, 1.0],
            sample_rate,
            seed,
            speed: 10000.0,
            coefficient: 1.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Pressure4 {
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
        match i {
            2 => float2string((self.params[2] as f64 * 2.0 - 1.0) as f32),
            0 | 1 | 3 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let threshold = 1.0 - self.params[0] as f64 * 0.95;
        let makeup = 1.0 / threshold;
        let mut release = (1.28 - self.params[1] as f64).powf(5.0) * 32768.0;
        release /= overallscale;
        let fastest = release.sqrt();
        let mut mewiness = self.params[2] as f64 * 2.0 - 1.0;
        let positive = mewiness >= 0.0;
        if !positive {
            mewiness = -mewiness;
        }
        let unmewiness = 1.0 - mewiness;
        let gain = self.params[3] as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            l *= makeup;
            r *= makeup;
            let sense = l.abs().max(r.abs());
            if sense > threshold {
                let vary = threshold / sense;
                let attack = self.speed.abs().sqrt();
                self.coefficient *= attack - 1.0;
                self.coefficient += if vary < threshold { threshold } else { vary };
                self.coefficient /= attack;
            } else {
                self.coefficient *= (self.speed * self.speed) - 1.0;
                self.coefficient += 1.0;
                self.coefficient /= self.speed * self.speed;
            }
            let mut new_speed = self.speed * (self.speed - 1.0);
            new_speed += (sense * release).abs() + fastest;
            self.speed = new_speed / self.speed;
            let coefficient = if positive {
                self.coefficient.powf(2.0)
            } else {
                self.coefficient.sqrt()
            };
            let coefficient = coefficient * mewiness + self.coefficient * unmewiness;
            l *= coefficient;
            r *= coefficient;
            if gain != 1.0 {
                l *= gain;
                r *= gain;
            }
            #[allow(clippy::approx_constant)] // Preserve the upstream clipping threshold.
            let bridge = if l.abs() > 1.57079633 {
                1.0
            } else {
                l.abs().sin()
            };
            l = if l > 0.0 { bridge } else { -bridge };
            #[allow(clippy::approx_constant)] // Preserve the upstream clipping threshold.
            let bridge = if r.abs() > 1.57079633 {
                1.0
            } else {
                r.abs().sin()
            };
            r = if r > 0.0 { bridge } else { -bridge };
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
