//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/SlewOnly, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
#![allow(clippy::excessive_precision, clippy::needless_range_loop)]
use crate::dither::fpd_from_seed;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "SlewOnly",
    upstream_name: "SlewOnly",
    description: "SlewOnly is a mix check plugin that shows you only the extreme highs.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(SlewOnly::new(sample_rate, seed))
}
struct SlewOnly {
    params: [f32; 0],
    sample_rate: f32,
    seed: u32,
    state: [f64; 2],
    fpd: [u32; 2],
}
impl SlewOnly {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [],
            sample_rate,
            seed,
            state: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for SlewOnly {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), crate::params::sanitize_param(v)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        let _ = i;
        String::new()
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], out_r_buf: &mut [f32]) {
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(out_r_buf) {
            let mut input = [l as f64, r as f64];
            for ch in 0..2 {
                if input[ch].abs() < 1.18e-23 {
                    input[ch] = self.fpd[ch] as f64 * 1.18e-17;
                }
                let output =
                    ((input[ch] - self.state[ch]) * 2.302585092994045684017991).clamp(-1.0, 1.0);
                self.state[ch] = input[ch];
                input[ch] = output;
            }
            *out_l = input[0] as f32;
            *out_r = input[1] as f32;
        }
    }
}
