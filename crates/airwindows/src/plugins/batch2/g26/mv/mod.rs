//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/MV, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
mod process;
mod state;
use crate::display::float2string;
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
use state::Mv;
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Depth",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Bright",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Regen",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
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
    name: "MV",
    upstream_name: "MV",
    description: "MV is a dual-mono reverb based on BitShiftGain and the old Midiverbs.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(_sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Mv::new(seed))
}
impl AirwindowsEffect for Mv {
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
        self.clear();
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        self.render(il, ir, ol, or);
    }
}
