//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/HighImpact, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Impact",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Output Level",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "HighImpact",
    upstream_name: "HighImpact",
    description: "HighImpact is distorted grit and punch without fatness.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(HighImpact::new(sample_rate, seed))
}
struct HighImpact {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    last: [f64; 2],
    fpd: [u32; 2],
}
impl HighImpact {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0; 3],
            sample_rate,
            seed,
            last: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for HighImpact {
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
            .map_or_else(String::new, |p| float2string(*p))
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c] = self.params;
        let density = a as f64 * 5.0;
        let out = density / 5.0;
        let sustain = 1.0 - 1.0 / (1.0 + density * a as f64);
        let threshold = 1.25 - out;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut output = [0.0; 2];
            for (ch, input) in [l, r].into_iter().enumerate() {
                let mut x = input as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = x;
                let mut count = density;
                while count > 1.0 {
                    let bridge = (x.abs() * 1.57079633).min(1.57079633).sin();
                    x = if x > 0.0 { bridge } else { -bridge };
                    count -= 1.0;
                }
                let bridge = (x.abs() * 1.57079633).min(1.57079633);
                let bridge = if density > 0.0 {
                    bridge.sin()
                } else {
                    1.0 - bridge.cos()
                };
                x = if x > 0.0 {
                    x * (1.0 - out) + bridge * out
                } else {
                    x * (1.0 - out) - bridge * out
                };
                let bridge = 1.0 - (x.abs() * 1.57079633).min(1.57079633).cos();
                x = if x > 0.0 {
                    x * (1.0 - sustain) + bridge * sustain
                } else {
                    x * (1.0 - sustain) - bridge * sustain
                };
                let clamp = x - self.last[ch];
                if clamp > threshold {
                    x = self.last[ch] + threshold;
                }
                if -clamp > threshold {
                    x = self.last[ch] - threshold;
                }
                self.last[ch] = x;
                if b < 1.0 {
                    x *= b as f64;
                }
                if c < 1.0 {
                    x = dry * (1.0 - c as f64) + x * c as f64;
                }
                output[ch] = fpd_dither(x, &mut self.fpd[ch]) as f32;
            }
            *outl = output[0];
            *outr = output[1];
        }
    }
}
