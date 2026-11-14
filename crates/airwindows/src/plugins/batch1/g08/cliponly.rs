//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ClipOnly/ClipOnlyProc.cpp
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Exact; measured max-abs 0.
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo { name: "ClipOnly", upstream_name: "ClipOnly", description: "ClipOnly is a clipper plugin that suppresses the brightness of digital clipping without affecting unclipped samples.", params: &[] };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(_: f32, _: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ClipOnly::default())
}
#[derive(Default)]
struct ClipOnly {
    last: [f64; 2],
    pos: [bool; 2],
    neg: [bool; 2],
}
impl ClipOnly {
    fn channel(&mut self, sample: f32, n: usize) -> f32 {
        let hard = 0.7390851332151606;
        let soft = 1.0 - hard;
        let clip = 0.9549925859;
        let mut input = sample.clamp(-4.0, 4.0);
        if self.pos[n] {
            self.last[n] = if (input as f64) < self.last[n] {
                clip * hard + input as f64 * soft
            } else {
                self.last[n] * hard + clip * soft
            };
        }
        self.pos[n] = false;
        if input as f64 > clip {
            self.pos[n] = true;
            input = (clip * hard + self.last[n] * soft) as f32;
        }
        if self.neg[n] {
            self.last[n] = if (input as f64) > self.last[n] {
                -clip * hard + input as f64 * soft
            } else {
                self.last[n] * hard + -clip * soft
            };
        }
        self.neg[n] = false;
        if (input as f64) < -clip {
            self.neg[n] = true;
            input = (-clip * hard + self.last[n] * soft) as f32;
        }
        let output = self.last[n] as f32;
        self.last[n] = input as f64;
        output
    }
}
impl AirwindowsEffect for ClipOnly {
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
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], or: &mut [f32]) {
        for (((&l, &r), ol), or) in l.iter().zip(r).zip(ol).zip(or) {
            *ol = self.channel(l, 0);
            *or = self.channel(r, 1);
        }
    }
}
