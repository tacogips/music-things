//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/DitherFloat, b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact; measured max-abs pending.
use crate::display::{float2string, int2string};
use crate::dither::{fpd_from_seed, frexpf_exponent, pow2, xorshift};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Offset",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dither",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo { name: "DitherFloat", upstream_name: "DitherFloat", description: "DitherFloat is a demo of floating point truncation: can also dither to 32 bit float output.", params: PARAMS };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DitherFloat::new(sample_rate, seed))
}
struct DitherFloat {
    sample_rate: f32,
    seed: u32,
    params: [f32; 2],
    fpd: [u32; 2],
}
impl DitherFloat {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            params: [0.0, 1.0],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    fn channel(&mut self, channel: usize, sample: f32, gain: f64) -> f32 {
        let mut value = sample as f64 + (gain - 1.0);
        let expon = frexpf_exponent(value as f32);
        xorshift(&mut self.fpd[channel]);
        value += self.fpd[channel] as f64 * 3.4e-36 * pow2(expon + 62) * self.params[1] as f64;
        value = value as f32 as f64;
        (value - (gain - 1.0)) as f32
    }
}
impl AirwindowsEffect for DitherFloat {
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
        match index {
            0 => int2string((self.params[0] * 32.0) as i32),
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let gain = pow2((self.params[0] * 32.0) as i32);
        for (((l, r), o_l), o_r) in il.iter().zip(ir).zip(ol).zip(or) {
            *o_l = self.channel(0, *l, gain);
            *o_r = self.channel(1, *r, gain);
        }
    }
}
