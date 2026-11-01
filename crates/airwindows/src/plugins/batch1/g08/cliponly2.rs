//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ClipOnly2, b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Exact; measured max-abs 0.
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo { name: "ClipOnly2", upstream_name: "ClipOnly2", description: "ClipOnly2 suppresses the brightness of digital clipping without affecting unclipped samples, at any sample rate.", params: &[] };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ClipOnly2::new(sample_rate, seed))
}
#[derive(Default)]
struct Channel {
    last: f64,
    intermediate: [f64; 17],
    positive: bool,
    negative: bool,
}
impl Channel {
    fn process(&mut self, sample: f32, spacing: usize) -> f32 {
        let mut x = sample.clamp(-4.0, 4.0);
        if self.positive {
            self.last = if (x as f64) < self.last {
                0.7058208 + x as f64 * 0.2609148
            } else {
                0.2491717 + self.last * 0.7390851
            };
        }
        self.positive = false;
        if x as f64 > 0.9549925859 {
            self.positive = true;
            x = (0.7058208 + self.last * 0.2609148) as f32;
        }
        if self.negative {
            self.last = if (x as f64) > self.last {
                -0.7058208 + x as f64 * 0.2609148
            } else {
                -0.2491717 + self.last * 0.7390851
            };
        }
        self.negative = false;
        if (x as f64) < -0.9549925859 {
            self.negative = true;
            x = (-0.7058208 + self.last * 0.2609148) as f32;
        }
        self.intermediate[spacing] = x as f64;
        let out = self.last as f32;
        for i in (1..=spacing).rev() {
            self.intermediate[i - 1] = self.intermediate[i];
        }
        self.last = self.intermediate[0];
        out
    }
}
struct ClipOnly2 {
    sample_rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
}
impl ClipOnly2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            sample_rate,
            seed,
            left: Channel::default(),
            right: Channel::default(),
        }
    }
}
impl AirwindowsEffect for ClipOnly2 {
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
        let spacing = ((1.0_f64 / 44100.0 * self.sample_rate as f64).floor() as usize).clamp(1, 16);
        for (((&l, &r), lo), ro) in il.iter().zip(ir).zip(ol).zip(or) {
            *lo = self.left.process(l, spacing);
            *ro = self.right.process(r, spacing);
        }
    }
}
