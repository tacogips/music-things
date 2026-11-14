//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Loud at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Boost",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Output Level",
        label: " ",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Loud",
    upstream_name: "Loud",
    description: "Loud is distortion and demolition of air molecules, modeled.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Loud::new(rate, seed))
}
struct Loud {
    rate: f32,
    seed: u32,
    params: [f32; 3],
    last: [f64; 2],
    fpd: [u32; 2],
}
impl Loud {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            rate,
            seed,
            params: [0.0, 1.0, 1.0],
            last: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Loud {
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
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c] = self.params;
        let boost = (a as f64 + 1.0).powi(5);
        let scale = self.rate as f64 / 44100.0;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut outputs = [0.0; 2];
            for (ch, s) in [l, r].into_iter().enumerate() {
                let mut input = s as f64;
                if input.abs() < 1.18e-23 {
                    input = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = input;
                input *= boost;
                let mut delta = input - self.last[ch];
                if delta > 0.0 {
                    input = (-(input - 1.0) * 1.2566108).clamp(0.0, 3.141527).sin() * scale;
                    if delta > input {
                        delta = input;
                    }
                }
                if delta < 0.0 {
                    input = ((input + 1.0) * 1.2566108).clamp(0.0, 3.141527).sin() * -scale;
                    if delta < input {
                        delta = input;
                    }
                }
                input = self.last[ch] + delta;
                self.last[ch] = input;
                if b < 1.0 {
                    input *= b as f64;
                }
                if c < 1.0 {
                    input = dry * (1.0 - c as f64) + input * c as f64;
                }
                outputs[ch] = fpd_dither(input, &mut self.fpd[ch]) as f32;
            }
            *outl = outputs[0];
            *outr = outputs[1];
        }
    }
}
