//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Highpass2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Hipass",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Ls/Tite",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Poles",
        label: "",
        default: 0.25,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Highpass2",
    upstream_name: "Highpass2",
    description: "Highpass2 is an unusual-sounding variable-slope highpass filter.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Highpass2::new(sample_rate, seed))
}
struct Highpass2 {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    state: [[[f64; 2]; 4]; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl Highpass2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 0.5, 0.25, 1.0],
            sample_rate,
            seed,
            state: [[[0.0; 2]; 4]; 2],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Highpass2 {
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
            0 | 3 => float2string(self.params[i]),
            1 => float2string((self.params[1] as f64 * 2.0 - 1.0) as f32),
            2 => float2string(self.params[2] * 4.0),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let amount = (a as f64).clamp(0.0000001, 1.0);
        let mut tight = b as f64 * 2.0 - 1.0;
        if tight < 0.0 {
            tight *= 0.5;
        }
        let stages = c as f64 * 4.0;
        let wets = [
            stages.clamp(0.0, 1.0),
            (stages - 1.0).clamp(0.0, 1.0),
            (stages - 2.0).clamp(0.0, 1.0),
            (stages - 3.0).clamp(0.0, 1.0),
        ];
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut out = [0.0; 2];
            for (ch, input) in [l, r].into_iter().enumerate() {
                let mut x = input as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = x;
                let offset = if tight > 0.0 {
                    (1.0 - tight) + x.abs() * tight
                } else {
                    (1.0 + tight) + (1.0 - x.abs()) * tight
                }
                .clamp(0.0000001, 1.0);
                for (stage, wet) in wets.into_iter().enumerate() {
                    if wet > 0.0 {
                        let s = &mut self.state[ch][stage][usize::from(!self.flip)];
                        *s = *s * (1.0 - offset * amount) + x * (offset * amount);
                        x = (x - *s) * wet + x * (1.0 - wet);
                    }
                }
                if d != 1.0 {
                    x = x * d as f64 + dry * (1.0 - d as f64);
                }
                out[ch] = fpd_dither(x, &mut self.fpd[ch]) as f32;
            }
            self.flip = !self.flip;
            *outl = out[0];
            *outr = out[1];
        }
    }
}
