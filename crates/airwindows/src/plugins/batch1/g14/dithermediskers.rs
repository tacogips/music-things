//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/DitherMeDiskers, b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact; measured max-abs 0.0.
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "DitherMeDiskers",
    upstream_name: "DitherMeDiskers",
    description: "DitherMeDiskers is dark deep 'dither' in a 16 bit version.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DitherMeDiskers::new(sample_rate, seed))
}
struct DitherMeDiskers {
    sample_rate: f32,
    seed: u32,
    last: [f64; 2],
    last2: [f64; 2],
    shaping: [f64; 2],
}
impl DitherMeDiskers {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            sample_rate,
            seed,
            last: [0.0; 2],
            last2: [0.0; 2],
            shaping: [0.0; 2],
        }
    }
    fn channel(&mut self, channel: usize, sample: f32) -> f32 {
        let input = sample as f64 * 32768.0;
        self.last[channel] -= self.shaping[channel] * 0.125;
        let mut output = if self.last[channel] + self.last[channel] >= input + self.last2[channel] {
            self.last[channel].floor()
        } else {
            (self.last[channel] + 1.0).floor()
        };
        self.last2[channel] = self.last[channel];
        self.last[channel] = input;
        self.shaping[channel] += output;
        self.shaping[channel] -= self.last[channel];
        if output > 32760.0 {
            output = 32760.0;
            self.shaping[channel] *= 0.5;
        }
        if output < -32760.0 {
            output = -32760.0;
            self.shaping[channel] *= 0.5;
        }
        (output / 32768.0) as f32
    }
}
impl AirwindowsEffect for DitherMeDiskers {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, _: usize) -> f32 {
        0.0
    }
    fn set_param(&mut self, _: usize, _: f32) {}
    fn param_display(&self, _: usize) -> String {
        String::new()
    }
    fn reset(&mut self) {
        *self = Self::new(self.sample_rate, self.seed);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        for (((l, r), o_l), o_r) in il.iter().zip(ir).zip(ol).zip(or) {
            *o_l = self.channel(0, *l);
            *o_r = self.channel(1, *r);
        }
    }
}
