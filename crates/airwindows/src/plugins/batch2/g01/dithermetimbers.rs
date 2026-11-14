//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/DitherMeTimbers, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "DitherMeTimbers",
    upstream_name: "DitherMeTimbers",
    description: "DitherMeTimbers is dark deep 'dither' in a 24 bit version.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(_: f32, _: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DitherMeTimbers::default())
}
#[derive(Default)]
struct Channel {
    noise_shaping: f64,
    last: f64,
    last2: f64,
}
impl Channel {
    fn process(&mut self, input: f32) -> f32 {
        let input = input as f64 * 8388608.0;
        self.last -= self.noise_shaping * 0.125;
        let mut output = if self.last + self.last >= input + self.last2 {
            self.last.floor()
        } else {
            (self.last + 1.0).floor()
        };
        self.last2 = self.last;
        self.last = input;
        self.noise_shaping += output;
        self.noise_shaping -= self.last;
        if output > 8388600.0 {
            output = 8388600.0;
            self.noise_shaping *= 0.5;
        }
        if output < -8388600.0 {
            output = -8388600.0;
            self.noise_shaping *= 0.5;
        }
        (output / 8388608.0) as f32
    }
}
#[derive(Default)]
struct DitherMeTimbers {
    left: Channel,
    right: Channel,
}
impl AirwindowsEffect for DitherMeTimbers {
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
        *self = Self::default();
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        for (((&l, &r), ol), or) in il.iter().zip(ir).zip(ol).zip(or) {
            *ol = self.left.process(l);
            *or = self.right.process(r);
        }
    }
}
