//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: plugins/LinuxVST/src/curve, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "curve",
    upstream_name: "curve",
    description: "curve is the simplest, purest form of Recurve with no extra boosts.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Curve::new(sample_rate, seed))
}
struct Curve {
    sample_rate: f32,
    seed: u32,
    gain: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl Curve {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rand = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rand);
        replay_fpd_seed_loop(&mut rand);
        Self {
            sample_rate,
            seed,
            gain: 1.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Curve {
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
        for (((l, r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            let mut l = f64::from(*l);
            let mut r = f64::from(*r);
            if l.abs() < 1.18e-23 {
                l = f64::from(self.fpd_l) * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = f64::from(self.fpd_r) * 1.18e-17;
            }
            l *= 0.5;
            r *= 0.5;
            self.gain = self.gain.clamp(0.0078125, 1.0);
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
            *ol = fpd_dither(l, &mut self.fpd_l) as f32;
            *or = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
