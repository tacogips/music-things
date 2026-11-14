//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Infinity2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use super::infinity::Reverb;
use crate::display::float2string;
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Filter",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Size",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Damping",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Allpass",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Feedback",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Infinity2",
    upstream_name: "Infinity2",
    description:
        "Infinity2 expands Infinity with more tone shaping and a much-requested kill-switch!",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Infinity2 {
        params: [1.0, 0.5, 0.0, 1.0, 1.0, 1.0],
        core: Reverb::new(sample_rate, seed),
    })
}
struct Infinity2 {
    params: [f32; 6],
    core: Reverb,
}
impl AirwindowsEffect for Infinity2 {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(v)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        self.params
            .get(i)
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        self.core.reset();
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        self.core
            .process(il, ir, ol, or, self.params.map(f64::from), true);
    }
}
