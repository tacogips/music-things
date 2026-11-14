//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Lowpass at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Lowpass",
        label: " ",
        default: 1.0,
    },
    ParamInfo {
        name: "Soft/Hard",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Lowpass",
    upstream_name: "Lowpass",
    description: "Lowpass deepens the tone, leaves a gloss and textural modifications.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Lowpass::new(rate, seed))
}
struct Lowpass {
    rate: f32,
    seed: u32,
    params: [f32; 3],
    iir: [[f64; 2]; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl Lowpass {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            rate,
            seed,
            params: [1.0, 0.5, 1.0],
            iir: [[0.0; 2]; 2],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Lowpass {
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
        match i {
            0 | 2 => float2string(self.params[i]),
            1 => float2string((self.params[1] as f64 * 2.0 - 1.0) as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c] = self.params;
        let mut amount = ((a as f64).powi(2) + a as f64) / 2.0 / (self.rate as f64 / 44100.0);
        let mut tight = b as f64 * 2.0 - 1.0;
        amount += amount * tight * tight;
        tight /= if tight > 0.0 { 1.5 } else { 3.0 };
        amount = amount.clamp(0.0, 1.0);
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut outputs = [0.0; 2];
            for (ch, s) in [l, r].into_iter().enumerate() {
                let mut input = s as f64;
                if input.abs() < 1.18e-23 {
                    input = self.fpd[ch] as f64 * 1.18e-17;
                }
                let offset = if tight > 0.0 {
                    (1.0 - tight) + input.abs() * tight
                } else {
                    (1.0 + tight) + (1.0 - input.abs()) * tight
                };
                let offset = offset.clamp(0.0, 1.0);
                let state = &mut self.iir[ch][if self.flip { 0 } else { 1 }];
                *state = *state * (1.0 - offset * amount) + input * (offset * amount);
                let mut output = *state;
                if c < 1.0 {
                    output = output * c as f64 + input * (1.0 - c as f64);
                }
                xorshift(&mut self.fpd[ch]);
                outputs[ch] = output as f32;
            }
            self.flip = !self.flip;
            *outl = outputs[0];
            *outr = outputs[1];
        }
    }
}
