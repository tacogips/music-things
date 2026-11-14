//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Gringer, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Gringer",
    upstream_name: "Gringer",
    description: "Gringer is a full-wave rectifier plugin, like a Green Ringer guitar effect.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Gringer::new(sample_rate, seed))
}
struct Gringer {
    sample_rate: f32,
    seed: u32,
    in_l: [f64; 9],
    in_r: [f64; 9],
    out_l: [f64; 9],
    out_r: [f64; 9],
    fpd_l: u32,
    fpd_r: u32,
}
impl Gringer {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            in_l: [0.0; 9],
            in_r: [0.0; 9],
            out_l: [0.0; 9],
            out_r: [0.0; 9],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn coefficients(b: &mut [f64; 9], scale: f64) {
    b[0] = 0.025 / scale;
    b[1] = 0.001;
    let k = (std::f64::consts::PI * b[0]).tan();
    let norm = 1.0 / (1.0 + k / b[1] + k * k);
    b[2] = k / b[1] * norm;
    b[4] = -b[2];
    b[5] = 2.0 * (k * k - 1.0) * norm;
    b[6] = (1.0 - k / b[1] + k * k) * norm;
}
fn filter(input: f64, b: &mut [f64; 9]) -> f64 {
    let temp = input * b[2] + b[7];
    b[7] = -(temp * b[5]) + b[8];
    b[8] = input * b[4] - temp * b[6];
    temp
}
impl AirwindowsEffect for Gringer {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        coefficients(&mut self.in_l, scale);
        coefficients(&mut self.in_r, scale);
        coefficients(&mut self.out_l, scale);
        coefficients(&mut self.out_r, scale);
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            l = filter(l.sin(), &mut self.in_l).abs();
            r = filter(r.sin(), &mut self.in_r).abs();
            l = filter(l, &mut self.out_l).clamp(-1.0, 1.0).asin();
            r = filter(r, &mut self.out_r).clamp(-1.0, 1.0).asin();
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
