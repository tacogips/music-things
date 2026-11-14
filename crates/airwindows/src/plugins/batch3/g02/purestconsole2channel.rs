//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PurestConsole2Channel, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PurestConsole2Channel",
    upstream_name: "PurestConsole2Channel",
    description: "PurestConsole2Channel is the distributed filter version of PurestConsole.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PurestConsole2Channel::new(sample_rate, seed))
}
struct PurestConsole2Channel {
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    biquad: [f64; 15],
}
impl PurestConsole2Channel {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            biquad: [0.0; 15],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for PurestConsole2Channel {
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
        b[1] = 0.618_033_988_749_894_9;
        let k = (std::f64::consts::PI * b[0]).tan();
        let norm = 1.0 / (1.0 + k / b[1] + k * k);
        b[2] = k * k * norm;
        b[3] = 2.0 * b[2];
        b[4] = b[2];
        b[5] = 2.0 * (k * k - 1.0) * norm;
        b[6] = (1.0 - k / b[1] + k * k) * norm;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut input = [l as f64, r as f64];
            for (i, sample) in input.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[i] as f64 * 1.18e-17;
                }
                if b[0] < 0.49999 {
                    let offset = 7 + i * 4;
                    let temp = b[2] * *sample + b[3] * b[offset] + b[4] * b[offset + 1]
                        - b[5] * b[offset + 2]
                        - b[6] * b[offset + 3];
                    b[offset + 1] = b[offset];
                    b[offset] = *sample;
                    *sample = temp;
                    b[offset + 3] = b[offset + 2];
                    b[offset + 2] = *sample;
                }
                #[allow(clippy::approx_constant)]
                {
                    *sample = sample.clamp(-1.57079633, 1.57079633);
                }
                *sample = (*sample).sin();
                *sample = fpd_dither(*sample, &mut self.fpd[i]);
            }
            *out_l = input[0] as f32;
            *out_r = input[1] as f32;
        }
    }
}
