//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Flipity, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Flipity",
    label: "",
    default: 0.0,
}];
const INFO: PluginInfo = PluginInfo {
    name: "Flipity",
    upstream_name: "Flipity",
    description: "Flipity is a channel phase flipper/swapper utility.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(_: f32, _: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Flipity { value: 0.0 })
}
struct Flipity {
    value: f32,
}
impl AirwindowsEffect for Flipity {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        if i == 0 {
            self.value
        } else {
            0.0
        }
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if i == 0 {
            if let Some(v) = sanitize_param(v) {
                self.value = v;
            }
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i != 0 {
            return String::new();
        }
        [
            "Dry", "Flip L", "Flip R", "Flip LR", "Swap LR", "Swip L", "Swip R", "Swip LR",
        ][(self.value as f64 * 7.999) as usize]
            .into()
    }
    fn reset(&mut self) {}
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mode = (self.value as f64 * 7.999) as usize;
        for (((&l, &r), left), right) in il.iter().zip(ir).zip(ol).zip(or) {
            (*left, *right) = match mode {
                0 => (l, r),
                1 => (-l, r),
                2 => (l, -r),
                3 => (-l, -r),
                4 => (r, l),
                5 => (-r, l),
                6 => (r, -l),
                _ => (-r, -l),
            };
        }
    }
}
