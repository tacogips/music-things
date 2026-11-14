//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Hype, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Hype",
    upstream_name: "Hype",
    description: "Hype is a treble softening experiment gone strangely awry :)",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Hype::new(sample_rate, seed))
}
struct Hype {
    sample_rate: f32,
    seed: u32,
    cycle_end: usize,
    soft: [[f64; 10]; 2],
    fpd: [u32; 2],
}
impl Hype {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut cycle_end = ((sample_rate as f64 / 44100.0).floor() as usize).clamp(1, 4);
        if cycle_end == 3 {
            cycle_end = 4;
        }
        Self {
            sample_rate,
            seed,
            cycle_end,
            soft: [[0.0; 10]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Hype {
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
    #[allow(clippy::approx_constant)] // Preserve the upstream literals.
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut out = [0.0; 2];
            for (ch, input) in [l, r].into_iter().enumerate() {
                let mut sample = input as f64;
                if sample.abs() < 1.18e-23 {
                    sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                let soft = &mut self.soft[ch];
                let step = 4 / self.cycle_end;
                for index in (1..=(8 / step)).rev() {
                    let i = index * step;
                    soft[i] = soft[i - step];
                }
                soft[0] = sample;
                soft[9] =
                    ((soft[0] - soft[4]) - (soft[4] - soft[8])).clamp(-1.57079633, 1.57079633);
                sample = soft[8] + soft[9].sin() * 0.61803398;
                out[ch] = fpd_dither(sample, &mut self.fpd[ch]) as f32;
            }
            *out_l = out[0];
            *out_r = out[1];
        }
    }
}
