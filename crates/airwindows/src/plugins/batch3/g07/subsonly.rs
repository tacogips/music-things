//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/SubsOnly, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::dither::fpd_from_seed;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "SubsOnly",
    upstream_name: "SubsOnly",
    description: "SubsOnly is a mix check plugin that shows you only the extreme lows.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(SubsOnly::new(sample_rate, seed))
}
struct SubsOnly {
    sample_rate: f32,
    seed: u32,
    left: [f64; 26],
    right: [f64; 26],
    fpd_l: u32,
    fpd_r: u32,
}
impl SubsOnly {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            sample_rate,
            seed,
            left: [0.0; 26],
            right: [0.0; 26],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn filter(mut sample: f64, stages: &mut [f64; 26], amount: f64, alt: f64) -> f64 {
    let mut gain = 1.42;
    for (i, stage) in stages.iter_mut().enumerate() {
        sample *= gain;
        gain = ((gain - 1.0) * 0.75) + 1.0;
        if i != 0 {
            sample = sample.clamp(-1.0, 1.0);
        }
        *stage = (*stage * alt) + (sample * amount);
        sample = *stage;
    }
    sample.clamp(-1.0, 1.0)
}
impl AirwindowsEffect for SubsOnly {
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
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let mut amount = 2250.0 / 44100.0;
        amount /= overallscale;
        let alt = 1.0 - amount;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            *outl = filter(l, &mut self.left, amount, alt) as f32;
            *outr = filter(r, &mut self.right, amount, alt) as f32;
        }
    }
}
