//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/PocketVerbs, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
mod chamber_l;
mod chamber_r;
mod room_l;
mod room_r;
mod setup;
mod spring_l;
mod spring_r;
mod state;
mod stretch_l;
mod stretch_r;
mod tiled_l;
mod tiled_r;
mod zarathustra_l;
mod zarathustra_r;

use crate::display::float2string;
use crate::dither::fpd_dither;
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
use state::State;

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Type",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Size",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Gating",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.5,
    },
];

fn param_display(params: &[f32; 4], index: usize) -> String {
    match index {
        0 => ["Chamber", "Spring", "Tiled", "Room", "Stretch", "Zarathu"]
            .get((params[0] as f64 * 5.999) as usize)
            .copied()
            .unwrap_or("")
            .to_owned(),
        1..=3 => float2string(params[index]),
        _ => String::new(),
    }
}

const INFO: PluginInfo = PluginInfo {
    name: "PocketVerbs",
    upstream_name: "PocketVerbs",
    description: "PocketVerbs is my popular old special effects reverbs plugin!",
    params: PARAMS,
};

struct PocketVerbs {
    params: [f32; 4],
    seed: u32,
    state: State,
}

impl PocketVerbs {
    fn new(_sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 0.5, 0.0, 0.5],
            seed,
            state: State::new(seed),
        }
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
        param_display(&self.params, index)
    }

    fn reset(&mut self) {
        self.state.reset(self.seed);
    }
}

pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PocketVerbs::new(sample_rate, seed))
}
impl AirwindowsEffect for PocketVerbs {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.get_param(index)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        self.set_param(index, value);
    }
    fn param_display(&self, index: usize) -> String {
        self.param_display(index)
    }
    fn reset(&mut self) {
        self.reset();
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let verbtype = (a as f64 * 5.999) as i32 + 1;
        let roomsize = (b as f64).powi(2) * 1.9 + 0.1;
        let release = 0.00008 * (c as f64).powi(3);
        if release == 0.0 {
            self.state.peakL = 1.0;
            self.state.peakR = 1.0;
        }
        let target = d as f64;
        let dryness = 1.0 - target;
        let mut wetness = target;
        let const_allpass = 0.618_033_988_749_894_9_f64;
        let gain = 0.5 + target * 0.5;
        setup::process(&mut self.state, roomsize);
        for (((&left, &right), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = left as f64;
            let mut r = right as f64;
            if l.abs() < 1.18e-23 {
                l = self.state.fpdL as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.state.fpdR as f64 * 1.18e-17;
            }
            let mut dry_l = l;
            let mut dry_r = r;
            self.state.peakL -= release;
            if self.state.peakL < (l * 2.0).abs() {
                self.state.peakL = (l * 2.0).abs();
            }
            if self.state.peakL > 1.0 {
                self.state.peakL = 1.0;
            }
            self.state.peakR -= release;
            if self.state.peakR < (r * 2.0).abs() {
                self.state.peakR = (r * 2.0).abs();
            }
            if self.state.peakR > 1.0 {
                self.state.peakR = 1.0;
            }
            l *= gain;
            let bridge = l.abs().sin();
            l = if l > 0.0 { bridge } else { -bridge };
            r *= gain;
            let bridge = r.abs().sin();
            r = if r > 0.0 { bridge } else { -bridge };
            l = match verbtype {
                1 => chamber_l::process(&mut self.state, l, const_allpass),
                2 => spring_l::process(&mut self.state, l, const_allpass),
                3 => tiled_l::process(&mut self.state, l, const_allpass),
                4 => room_l::process(&mut self.state, l, dry_l, const_allpass, dryness, wetness),
                5 => stretch_l::process(&mut self.state, l, const_allpass),
                6 => zarathustra_l::process(&mut self.state, l, const_allpass),
                _ => l,
            };
            r = match verbtype {
                1 => chamber_r::process(&mut self.state, r, const_allpass),
                2 => spring_r::process(&mut self.state, r, const_allpass),
                3 => tiled_r::process(&mut self.state, r, const_allpass),
                4 => room_r::process(&mut self.state, r, dry_r, const_allpass, dryness, wetness),
                5 => stretch_r::process(&mut self.state, r, const_allpass),
                6 => zarathustra_r::process(&mut self.state, r, const_allpass),
                _ => r,
            };
            let bridge = 1.0 - l.abs().cos();
            if l > 0.0 {
                l -= bridge;
            } else {
                l += bridge;
            }
            l /= gain;
            let bridge = 1.0 - r.abs().cos();
            if r > 0.0 {
                r -= bridge;
            } else {
                r += bridge;
            }
            r /= gain;
            wetness = target;
            wetness *= self.state.peakL;
            dry_l *= 1.0 - wetness;
            l *= wetness;
            l += dry_l;
            wetness = target;
            wetness *= self.state.peakR;
            dry_r *= 1.0 - wetness;
            r *= wetness;
            r += dry_r;
            *out_l = fpd_dither(l, &mut self.state.fpdL) as f32;
            *out_r = fpd_dither(r, &mut self.state.fpdR) as f32;
        }
    }
}
