//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/NonlinearSpace, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0 (all three fixture renders).
mod process;
mod reset_state;
mod state;
use crate::display::float2string;
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
use state::State;

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "SmpRate",
        label: "",
        default: 0.3,
    },
    ParamInfo {
        name: "Livenes",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Treble",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Bass",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Nonlin",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];

struct NonlinearSpace {
    params: [f32; 6],
    sample_rate: f32,
    seed: u32,
    state: State,
}
impl NonlinearSpace {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.3, 0.5, 0.5, 0.5, 0.5, 1.0],
            sample_rate,
            seed,
            state: State::new(seed),
        }
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(param), Some(value)) = (self.params.get_mut(i), sanitize_param(value)) {
            *param = value;
        }
    }
    fn param_display(&self, i: usize) -> String {
        match i {
            0 => ["16K", "32K", "44.1K", "48K", "64K", "88.2K", "96K"]
                [(self.params[0] as f64 * 6.999) as usize]
                .into(),
            4 => float2string((self.params[4] as f64 * 2.0 - 1.0) as f32),
            1..=5 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
}

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "NonlinearSpace",
    upstream_name: "NonlinearSpace",
    description: "NonlinearSpace is a flexible reverb plugin.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(NonlinearSpace::new(sample_rate, seed))
}
impl AirwindowsEffect for NonlinearSpace {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        NonlinearSpace::get_param(self, index)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        NonlinearSpace::set_param(self, index, value);
    }
    fn param_display(&self, index: usize) -> String {
        NonlinearSpace::param_display(self, index)
    }
    fn reset(&mut self) {
        self.state.reset(self.seed);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        self.process_block(il, ir, ol, or);
    }
}
