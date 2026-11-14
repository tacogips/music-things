//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PeaksOnly, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PeaksOnly",
    upstream_name: "PeaksOnly",
    description: "PeaksOnly is a transformative mix check tool.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PeaksOnly::new(sample_rate, seed))
}
const STAGE_LEN: usize = 1503;
struct PeaksOnly {
    sample_rate: f32,
    seed: u32,
    left: Box<[f64]>,
    right: Box<[f64]>,
    indices: [usize; 4],
    fpd_l: u32,
    fpd_r: u32,
}
impl PeaksOnly {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            left: vec![0.0; 4 * STAGE_LEN].into_boxed_slice(),
            right: vec![0.0; 4 * STAGE_LEN].into_boxed_slice(),
            indices: [1; 4],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for PeaksOnly {
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
        self.left.fill(0.0);
        self.right.fill(0.0);
        self.indices = [1; 4];
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= f64::from(self.sample_rate);
        let maxima =
            [149.0, 179.0, 191.0, 223.0].map(|x| ((x * overallscale) as usize).min(STAGE_LEN - 1));
        for (((&in_l, &in_r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = f64::from(in_l);
            let mut r = f64::from(in_r);
            if l.abs() < 1.18e-23 {
                l = f64::from(self.fpd_l) * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = f64::from(self.fpd_r) * 1.18e-17;
            }
            for (stage, &max) in maxima.iter().enumerate() {
                l = l.clamp(-1.0, 1.0).asin();
                r = r.clamp(-1.0, 1.0).asin();
                let idx = self.indices[stage];
                let prev = if idx == 0 || idx - 1 > max {
                    max
                } else {
                    idx - 1
                };
                let offset = stage * STAGE_LEN;
                l -= self.left[offset + prev] * 0.5;
                r -= self.right[offset + prev] * 0.5;
                self.left[offset + idx] = l;
                self.right[offset + idx] = r;
                l *= 0.5;
                r *= 0.5;
                let next = if idx == 0 || idx - 1 > max {
                    max
                } else {
                    idx - 1
                };
                self.indices[stage] = next;
                l += self.left[offset + next];
                r += self.right[offset + next];
            }
            l = l.clamp(-1.0, 1.0).asin();
            r = r.clamp(-1.0, 1.0).asin();
            l *= 0.63679;
            r *= 0.63679;
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
