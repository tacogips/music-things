//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: plugins/LinuxVST/src/DCVoltage, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact; measured max-abs 0.0.

use crate::display::float2string;
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "DCVoltage",
    upstream_name: "DCVoltage",
    description: "DCVoltage is literally a DC control voltage, in a plugin.",
    params: &[ParamInfo {
        name: "Voltage",
        label: " ",
        default: 0.5,
    }],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DCVoltage {
        params: [0.5],
        sample_rate,
        seed,
    })
}
struct DCVoltage {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
}
impl AirwindowsEffect for DCVoltage {
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
            float2string((f64::from(self.params[0]) * 2.0 - 1.0) as f32)
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let _ = (self.sample_rate, self.seed);
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let voltage = f64::from(self.params[0]) * 2.0 - 1.0;
        for (((l, r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            *ol = (f64::from(*l) + voltage) as f32;
            *or = (f64::from(*r) + voltage) as f32;
        }
    }
}
