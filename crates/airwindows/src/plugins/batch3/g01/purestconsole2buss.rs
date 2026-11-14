//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PurestConsole2Buss, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PurestConsole2Buss",
    upstream_name: "PurestConsole2Buss",
    description: "PurestConsole2Buss is the distributed filter version of PurestConsole.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PurestConsole2Buss::new(sample_rate, seed))
}
struct PurestConsole2Buss {
    sample_rate: f32,
    seed: u32,
    biquad: [f64; 15],
    fpd_l: u32,
    fpd_r: u32,
}
impl PurestConsole2Buss {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            biquad: [0.0; 15],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for PurestConsole2Buss {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let b = &mut self.biquad;
        b[0] = 30000.0 / self.sample_rate as f64;
        b[1] = 1.618_033_988_749_895;
        let k = (std::f64::consts::PI * b[0]).tan();
        let norm = 1.0 / (1.0 + k / b[1] + k * k);
        b[2] = k * k * norm;
        b[3] = 2.0 * b[2];
        b[4] = b[2];
        b[5] = 2.0 * (k * k - 1.0) * norm;
        b[6] = (1.0 - k / b[1] + k * k) * norm;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            if b[0] < 0.49999 {
                let t = b[2] * l + b[3] * b[7] + b[4] * b[8] - b[5] * b[9] - b[6] * b[10];
                b[8] = b[7];
                b[7] = l;
                l = t;
                b[10] = b[9];
                b[9] = l;
                let t = b[2] * r + b[3] * b[11] + b[4] * b[12] - b[5] * b[13] - b[6] * b[14];
                b[12] = b[11];
                b[11] = r;
                r = t;
                b[14] = b[13];
                b[13] = r;
            }
            l = l.clamp(-1.0, 1.0).asin();
            r = r.clamp(-1.0, 1.0).asin();
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
