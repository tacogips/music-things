//! Desk, upstream airwindows/plugins/LinuxVST/src/Desk at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Parity: Libm, measured max-abs 0.0.

// Preserve the literal coefficient used by upstream.
#![allow(clippy::approx_constant)]
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};
pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "Desk",
    upstream_name: "Desk",
    description: "Desk is classic Airwindows subtle analog modeling.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Desk::new(rate, seed))
}
struct Desk {
    sample_rate: f32,
    seed: u32,
    last: [f64; 2],
    last_out: [f64; 2],
    last_slew: [f64; 2],
    fpd: [u32; 2],
}
impl Desk {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            last: [0.0; 2],
            last_out: [0.0; 2],
            last_slew: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Desk {
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
        let gain = 0.135;
        let mut slewgain = 0.208;
        let mut prevslew = 0.333;
        let mut balance_b = 0.0001;
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        slewgain *= scale;
        prevslew *= scale;
        balance_b /= scale;
        let balance_a = 1.0 - balance_b;
        for (((l, r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [*l as f64, *r as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *sample;
                let mut slew = *sample - self.last[ch];
                self.last[ch] = *sample;
                let mut rectifier = (slew * slewgain).abs();
                if rectifier > 1.57079633 {
                    rectifier = 1.0;
                } else {
                    rectifier = rectifier.sin();
                }
                if slew > 0.0 {
                    slew = rectifier / slewgain;
                } else {
                    slew = -(rectifier / slewgain);
                }
                *sample = (self.last_out[ch] * balance_a) + (self.last[ch] * balance_b) + slew;
                self.last_out[ch] = *sample;
                let mut comb = (dry * self.last[ch]).abs();
                if comb > 1.0 {
                    comb = 1.0;
                }
                *sample -= self.last_slew[ch] * comb * prevslew;
                self.last_slew[ch] = slew;
                *sample *= gain;
                rectifier = sample.abs();
                if rectifier > 1.57079633 {
                    rectifier = 1.0;
                } else {
                    rectifier = rectifier.sin();
                }
                if *sample > 0.0 {
                    *sample = rectifier;
                } else {
                    *sample = -rectifier;
                }
                *sample /= gain;
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            *out_l = samples[0] as f32;
            *out_r = samples[1] as f32;
        }
    }
}
