//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/FathomFive, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::fpd_from_seed;
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Root Note",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "SubOctave",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Frequency",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "FathomFive",
    upstream_name: "FathomFive",
    description: "FathomFive is a way of supplementing extreme bass that’s not just EQ.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(FathomFive::new(rate, seed))
}
#[derive(Default)]
struct Channel {
    negative: bool,
    octave: bool,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}
struct FathomFive {
    params: [f32; 4],
    rate: f32,
    seed: u32,
    channels: [Channel; 2],
    fpd: [u32; 2],
}
impl FathomFive {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.0, 0.5, 1.0],
            rate,
            seed,
            channels: [Channel::default(), Channel::default()],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn channel(
    input: f64,
    state: &mut Channel,
    eq: f64,
    root: f64,
    sub: f64,
    wet: f64,
    dry: f64,
) -> f64 {
    if input > 0.0 {
        if state.negative {
            state.octave = !state.octave;
        }
        state.negative = false;
    } else {
        state.negative = true;
    }
    state.d = state.d * (1.0 - eq) + input * eq;
    let mut temp = if state.octave {
        state.d.abs() * sub
    } else {
        -state.d.abs() * sub
    };
    temp += input * root;
    state.a += temp * eq;
    state.a -= state.a * state.a * state.a * eq;
    if state.a > 0.0 {
        state.a -= eq / 320.0;
    } else {
        state.a += eq / 320.0;
    }
    temp = state.a * ((0.01 / eq) + 1.0);
    state.b = state.b * (1.0 - eq) + temp * eq;
    state.c = state.c * (1.0 - eq) + state.b * eq;
    input * dry + state.c * wet
}
impl AirwindowsEffect for FathomFive {
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
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let eq = 0.01 + ((c as f64).powf(4.0) / self.rate as f64) * 32000.0;
        let wet = (d as f64 * 2.0).min(1.0);
        let dry = (2.0 - d as f64 * 2.0).min(1.0);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd[0] as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd[1] as f64 * 1.18e-17;
            }
            *out_l = channel(l, &mut self.channels[0], eq, a as f64, b as f64, wet, dry) as f32;
            *out_r = channel(r, &mut self.channels[1], eq, a as f64, b as f64, wet, dry) as f32;
        }
    }
}
