//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Nikola, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
mod input;
mod output;
mod waveform;

use crate::display::float2string;
use crate::dither::fpd_from_seed;
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
use input::Channel;

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Voltage",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Nikola",
    upstream_name: "Nikola",
    description: "Nikola is an Audio Tesla Coil plugin! Note: audio tesla coils don't sound nice.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Nikola::new(sample_rate, seed))
}
struct Nikola {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    fpd_l: u32,
    fpd_r: u32,
}
impl Nikola {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 1.0],
            sample_rate,
            seed,
            left: Channel::default(),
            right: Channel::default(),
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Nikola {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        self.params
            .get(i)
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let a = self.params[0] as f64;
        let decay = a.powf(3.0) * 32767.0;
        let highs_boost = 16 - (a.powf(2.0) * 16.0) as i32;
        let wet = self.params[1] as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let left = self.left.render(l, decay, highs_boost);
            let right = self.right.render(r, decay, highs_boost);
            *out_l = output::finish(left, l, wet, &mut self.fpd_l);
            *out_r = output::finish(right, r, wet, &mut self.fpd_r);
        }
    }
}
