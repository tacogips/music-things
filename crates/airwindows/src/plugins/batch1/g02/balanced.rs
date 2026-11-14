//! Balanced, ported from airwindows/plugins/LinuxVST/src/Balanced at
//! b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Parity: Exact; measured max-abs 0.0.
use crate::display::int2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "Balanced",
    upstream_name: "Balanced",
    description:
        "Balanced lets you run an XLR input into a stereo input, sum it to mono, and use it.",
    params: &[ParamInfo {
        name: "BitShift",
        label: "bits",
        default: 0.0,
    }],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Balanced::new(sample_rate, seed))
}
struct Balanced {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
}
impl Balanced {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Balanced {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(param), Some(value)) = (self.params.get_mut(index), sanitize_param(value)) {
            *param = value;
        }
    }
    fn param_display(&self, index: usize) -> String {
        if index == 0 {
            int2string((self.params[0] * 8.0) as i32)
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let gain =
            [0.5, 1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0][(self.params[0] * 8.0) as usize];
        for (((&l, &r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let side = (l - r) * gain;
            *ol = fpd_dither(side, &mut self.fpd_l) as f32;
            *or = fpd_dither(side, &mut self.fpd_r) as f32;
        }
    }
}
