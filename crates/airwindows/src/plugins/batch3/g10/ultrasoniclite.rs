//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/UltrasonicLite, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "UltrasonicLite",
    upstream_name: "UltrasonicLite",
    description:
        "UltrasonicLite is just one stage of supersonic filter, for using inside digital mixes.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(UltrasonicLite::new(sample_rate, seed))
}
struct UltrasonicLite {
    sample_rate: f32,
    seed: u32,
    biquads: [[f64; 15]; 1],
    fpd_l: u32,
    fpd_r: u32,
}
impl UltrasonicLite {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            biquads: [[0.0; 15]; 1],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for UltrasonicLite {
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
    #[allow(clippy::approx_constant)] // Preserve the upstream filter Q.
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        self.biquads[0][0] = 24000.0 / self.sample_rate as f64;
        if self.sample_rate < 88000.0 {
            self.biquads[0][0] = 21000.0 / self.sample_rate as f64;
        }
        self.biquads[0][1] = 0.70710678;
        for b in &mut self.biquads {
            let k = (std::f64::consts::PI * b[0]).tan();
            let norm = 1.0 / (1.0 + k / b[1] + k * k);
            b[2] = k * k * norm;
            b[3] = 2.0 * b[2];
            b[4] = b[2];
            b[5] = 2.0 * (k * k - 1.0) * norm;
            b[6] = (1.0 - k / b[1] + k * k) * norm;
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            for b in &mut self.biquads {
                let out = b[2] * l + b[3] * b[7] + b[4] * b[8] - b[5] * b[9] - b[6] * b[10];
                b[8] = b[7];
                b[7] = l;
                l = out;
                b[10] = b[9];
                b[9] = l;
            }
            for b in &mut self.biquads {
                let out = b[2] * r + b[3] * b[11] + b[4] * b[12] - b[5] * b[13] - b[6] * b[14];
                b[12] = b[11];
                b[11] = r;
                r = out;
                b[14] = b[13];
                b[13] = r;
            }
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
