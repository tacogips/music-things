/* Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
 * Upstream: airwindows/plugins/LinuxVST/src/C5RawChannel/C5RawChannel.h
 * Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
 * Parity: Libm; measured max-abs 0.0.
 */
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

static PARAMS: [ParamInfo; 1] = [ParamInfo {
    name: "Center",
    label: "",
    default: 0.0,
}];
pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "C5RawChannel",
    upstream_name: "C5RawChannel",
    description:
        "C5RawChannel is the original Console5 algorithm, with optional very gentle DC suppression.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(C5RawChannel::new(sample_rate, seed))
}
struct C5RawChannel {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    last_fx: [f64; 2],
    last_sample: [f64; 2],
}
impl C5RawChannel {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0],
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            last_fx: [0.0; 2],
            last_sample: [0.0; 2],
        }
    }
    #[allow(clippy::approx_constant)]
    fn sample(&mut self, sample: f32, channel: usize, centering: f64) -> f32 {
        let mut input = sample as f64;
        if input.abs() < 1.18e-23 {
            input = self.fpd[channel] as f64 * 1.18e-17;
        }
        let difference = (self.last_sample[channel] - input).clamp(-1.0, 1.0);
        self.last_sample[channel] = input;
        input = self.last_fx[channel] + difference.asin();
        self.last_fx[channel] = input;
        if centering < 1.0 {
            self.last_fx[channel] *= centering;
        }
        self.last_fx[channel] = self.last_fx[channel].clamp(-1.0, 1.0);
        fpd_dither(
            input.clamp(-1.57079633, 1.57079633).sin(),
            &mut self.fpd[channel],
        ) as f32
    }
}
impl AirwindowsEffect for C5RawChannel {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(index), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, index: usize) -> String {
        if index == 0 {
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
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let centering = 1.0 - (self.params[0] as f64 * 0.5).powf(5.0);
        for (((l, r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            *ol = self.sample(*l, 0, centering);
            *or = self.sample(*r, 1, centering);
        }
    }
}
