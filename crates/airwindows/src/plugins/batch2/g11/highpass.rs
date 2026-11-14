//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Highpass, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Highpass",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Loose/Tight",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Highpass",
    upstream_name: "Highpass",
    description: "Highpass is a time warp, for retro midrangey sounds.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Highpass::new(sample_rate, seed))
}
struct Highpass {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    state: [[f64; 2]; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl Highpass {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 0.5, 1.0],
            sample_rate,
            seed,
            state: [[0.0; 2]; 2],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Highpass {
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
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c] = self.params;
        let mut amount = (a as f64).powf(3.0) / (self.sample_rate as f64 / 44100.0);
        let mut tight = b as f64 * 2.0 - 1.0;
        amount += amount * tight * tight;
        tight /= if tight > 0.0 { 1.5 } else { 3.0 };
        amount = amount.clamp(0.0, 1.0);
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut out = [0.0; 2];
            for (ch, input) in [l, r].into_iter().enumerate() {
                let mut x = input as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[ch] as f64 * 1.18e-17;
                }
                let offset = if tight > 0.0 {
                    (1.0 - tight) + x.abs() * tight
                } else {
                    (1.0 + tight) + (1.0 - x.abs()) * tight
                }
                .clamp(0.0, 1.0);
                let s = &mut self.state[ch][usize::from(!self.flip)];
                *s = *s * (1.0 - offset * amount) + x * (offset * amount);
                let mut y = x - *s;
                if c < 1.0 {
                    y = y * c as f64 + x * (1.0 - c as f64);
                }
                xorshift(&mut self.fpd[ch]);
                out[ch] = y as f32;
            }
            self.flip = !self.flip;
            *outl = out[0];
            *outr = out[1];
        }
    }
}
