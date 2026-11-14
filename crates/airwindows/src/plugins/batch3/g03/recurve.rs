//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Recurve, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Recurve",
    upstream_name: "Recurve",
    description: "Recurve is a special buss compressor with no threshold point.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Recurve::new(sample_rate, seed))
}
struct Recurve {
    sample_rate: f32,
    seed: u32,
    gain: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl Recurve {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            gain: 2.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Recurve {
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
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            l *= 0.5;
            r *= 0.5;
            self.gain = self.gain.clamp(0.0078125, 2.0);
            l *= self.gain;
            r *= self.gain;
            self.gain += (if (l * 4.0).abs() > 1.0 {
                4.0
            } else {
                (l * 4.0).abs()
            })
            .sin()
                * l.powf(4.0);
            self.gain += (if (r * 4.0).abs() > 1.0 {
                4.0
            } else {
                (r * 4.0).abs()
            })
            .sin()
                * r.powf(4.0);
            l *= 2.0;
            r *= 2.0;
            l = l.clamp(-0.9999, 0.9999);
            r = r.clamp(-0.9999, 0.9999);
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
