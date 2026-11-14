//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Slew2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
#![allow(clippy::excessive_precision, clippy::needless_range_loop)]
use crate::dither::fpd_from_seed;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Clamping",
    label: " ",
    default: 0.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Slew2",
    upstream_name: "Slew2",
    description: "Slew2 works like a de-esser or acceleration limiter: controls extreme highs.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Slew2::new(sample_rate, seed))
}
#[derive(Clone, Copy, Default)]
struct Channel {
    last1: f64,
    last2: f64,
    last3: f64,
    a: f64,
    b: f64,
    prev_diff: f64,
    flip: bool,
    last: f64,
}
impl Channel {
    fn process(&mut self, mut input: f64, threshold: f64) -> f64 {
        let dry = input;
        let mut half =
            (input + self.last1 + ((-self.last2 + self.last3) * 0.0414213562373095048801688)) / 2.0;
        let half_dry = half;
        self.last3 = self.last2;
        self.last2 = self.last1;
        self.last1 = input;
        let clamp = half - half_dry;
        if clamp > threshold {
            half = self.last + threshold;
        }
        if -clamp > threshold {
            half = self.last - threshold;
        }
        self.last = half;
        let half_diff = self.filter(half - half_dry);
        let clamp = input - self.last;
        if clamp > threshold {
            input = self.last + threshold;
        }
        if -clamp > threshold {
            input = self.last - threshold;
        }
        self.last = input;
        let diff = self.filter(input - dry);
        input = dry;
        input += (diff + half_diff + self.prev_diff) / 0.734;
        self.prev_diff = diff / 2.0;
        input
    }
    fn filter(&mut self, mut c: f64) -> f64 {
        let decay = 0.915965594177219015;
        if self.flip {
            self.a *= decay;
            self.b *= decay;
            self.a += c;
            self.b -= c;
            c = self.a;
        } else {
            self.b *= decay;
            self.a *= decay;
            self.b += c;
            self.a -= c;
            c = self.b;
        }
        self.flip = !self.flip;
        c * decay
    }
}
struct Slew2 {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    channels: [Channel; 2],
    fpd: [u32; 2],
}
impl Slew2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0],
            sample_rate,
            seed,
            channels: [Channel::default(); 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Slew2 {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), crate::params::sanitize_param(v)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i == 0 {
            crate::display::float2string(self.params[0])
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let overallscale = (2.0 / 44100.0) * self.sample_rate as f64;
        let threshold = ((1.0f32 - self.params[0]) as f64).powf(4.0) / overallscale;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut samples = [l as f64, r as f64];
            for (ch, input) in samples.iter_mut().enumerate() {
                if input.abs() < 1.18e-23 {
                    *input = self.fpd[ch] as f64 * 1.18e-17;
                }
                *input = self.channels[ch].process(*input, threshold);
            }
            *out_l = samples[0] as f32;
            *out_r = samples[1] as f32;
        }
    }
}
