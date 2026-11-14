//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Slew, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
#![allow(clippy::excessive_precision, clippy::needless_range_loop)]
use crate::dither::fpd_from_seed;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Clamping",
    label: "",
    default: 0.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Slew",
    upstream_name: "Slew",
    description: "Slew is a slew clipper, which darkens treble in an unusual way.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Slew::new(sample_rate, seed))
}
struct Slew {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    state: [f64; 2],
    fpd: [u32; 2],
}
impl Slew {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0],
            sample_rate,
            seed,
            state: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Slew {
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
        if i == 0 {
            crate::display::float2string(self.params[0])
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], out_r_buf: &mut [f32]) {
        let overallscale = (1.0 / 44100.0) * self.sample_rate as f64;
        let threshold = ((1.0f32 - self.params[0]) as f64).powf(4.0) / overallscale;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(out_r_buf) {
            let mut input = [l as f64, r as f64];
            for ch in 0..2 {
                if input[ch].abs() < 1.18e-23 {
                    input[ch] = self.fpd[ch] as f64 * 1.18e-17;
                }
                let clamp = input[ch] - self.state[ch];
                if clamp > threshold {
                    input[ch] = self.state[ch] + threshold;
                }
                if -clamp > threshold {
                    input[ch] = self.state[ch] - threshold;
                }
                self.state[ch] = input[ch];
            }
            *out_l = input[0] as f32;
            *out_r = input[1] as f32;
        }
    }
}
