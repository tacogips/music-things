//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ClipSoftly/ClipSoftlyProc.cpp
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.
#![allow(clippy::approx_constant)]
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "ClipSoftly",
    upstream_name: "ClipSoftly",
    description: "ClipSoftly is ClipOnly2 but as a softclipper: a very handy building block.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ClipSoftly::new(rate, seed))
}
struct ClipSoftly {
    rate: f32,
    seed: u32,
    last: [f64; 2],
    intermediate: [[f64; 17]; 2],
    fpd: [u32; 2],
}
impl ClipSoftly {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            rate,
            seed,
            last: [0.0; 2],
            intermediate: [[0.0; 17]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    fn channel(&mut self, sample: f32, channel: usize, spacing: usize) -> f32 {
        let mut input = sample as f64;
        if input.abs() < 1.18e-23 {
            input = self.fpd[channel] as f64 * 1.18e-17;
        }
        let mut speed = input.abs();
        if speed < 1.0 {
            speed = 1.0;
        } else {
            speed = 1.0 / speed;
        }
        input = input.clamp(-1.57079633, 1.57079633);
        input = input.sin() * 0.9549925859;
        input = input * speed + self.last[channel] * (1.0 - speed);
        self.intermediate[channel][spacing] = input;
        input = self.last[channel];
        for x in (1..=spacing).rev() {
            self.intermediate[channel][x - 1] = self.intermediate[channel][x];
        }
        self.last[channel] = self.intermediate[channel][0];
        fpd_dither(input, &mut self.fpd[channel]) as f32
    }
}
impl AirwindowsEffect for ClipSoftly {
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
        *self = Self::new(self.rate, self.seed);
    }
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = 1.0_f64;
        scale /= 44100.0;
        scale *= self.rate as f64;
        let spacing = (scale.floor() as i32).clamp(1, 16) as usize;
        for (((&l, &r), ol), or) in l.iter().zip(r).zip(ol).zip(or) {
            *ol = self.channel(l, 0, spacing);
            *or = self.channel(r, 1, spacing);
        }
    }
}
