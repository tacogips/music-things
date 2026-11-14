//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/StudioTan, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.0.
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Quantzr",
    label: "",
    default: 0.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "StudioTan",
    upstream_name: "StudioTan",
    description: "StudioTan is all the 'non-dither' dithers, up to date and convenient.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(StudioTan::new(sample_rate, seed))
}

struct Channel {
    bins: [f64; 13],
    shaping: f64,
    last: f64,
    last2: f64,
}
impl Channel {
    fn new() -> Self {
        Self {
            bins: [
                1000.0, 301.0, 176.0, 125.0, 97.0, 79.0, 67.0, 58.0, 51.0, 46.0, 1000.0, 0.0, 0.0,
            ],
            shaping: 0.0,
            last: 0.0,
            last2: 0.0,
        }
    }
    fn benford(&mut self, mut input: f64) -> (f64, f64) {
        let dry = input;
        input -= self.shaping;
        let mut cut = false;
        let mut score = |mut value: f64| {
            while value >= 1.0 {
                value /= 10.0;
            }
            for _ in 0..5 {
                if value < 1.0 {
                    value *= 10.0;
                }
            }
            let index = value.floor() as i32;
            if !(1..10).contains(&index) {
                return (10usize, 0.0);
            }
            let index = index as usize;
            self.bins[index] += 1.0;
            if self.bins[index] > 982.0 {
                cut = true;
            }
            let mut total = 0.0;
            for (i, expected) in [301.0, 176.0, 125.0, 97.0, 79.0, 67.0, 58.0, 51.0, 46.0]
                .iter()
                .enumerate()
            {
                total += expected - self.bins[i + 1];
            }
            self.bins[index] -= 1.0;
            (index, total)
        };
        let (a, ta) = score(input.floor());
        let (b, tb) = score(input.ceil());
        let output = if ta < tb {
            self.bins[a] += 1.0;
            input.floor()
        } else {
            self.bins[b] += 1.0;
            (input + 1.0).floor()
        };
        if cut {
            for bin in &mut self.bins[1..=10] {
                *bin *= 0.99;
            }
        }
        self.shaping += output - dry;
        (input, output)
    }
    fn process(&mut self, input: f64, benford: bool, bright: bool) -> f64 {
        let (input, output) = if benford {
            self.benford(input)
        } else {
            self.last -= self.shaping * if bright { 0.8 } else { 0.11 };
            let down = if bright {
                self.last + self.last <= input + self.last2
            } else {
                self.last + self.last >= input + self.last2
            };
            let output = if down {
                self.last.floor()
            } else {
                (self.last + 1.0).floor()
            };
            self.shaping += output;
            self.shaping -= self.last;
            self.last2 = self.last;
            self.last = input;
            (input, output)
        };
        let suppress = input.abs();
        if self.shaping > suppress {
            self.shaping = suppress;
        }
        if self.shaping < -suppress {
            self.shaping = -suppress;
        }
        output
    }
}
struct StudioTan {
    params: [f32; 1],
    left: Channel,
    right: Channel,
}
impl StudioTan {
    fn new(_sample_rate: f32, _seed: u32) -> Self {
        Self {
            params: [0.0],
            left: Channel::new(),
            right: Channel::new(),
        }
    }
}
impl AirwindowsEffect for StudioTan {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i != 0 {
            return String::new();
        }
        ["ST 24", "DMT 24", "NJAD 24", "ST 16", "DMT 16", "NJAD 16"]
            [(self.params[0] as f64 * 5.999) as usize]
            .into()
    }
    fn reset(&mut self) {
        self.left = Channel::new();
        self.right = Channel::new();
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mode = (self.params[0] as f64 * 5.999) as i32;
        let high = mode < 3;
        let benford = mode == 2 || mode == 5;
        let bright = mode != 1 && mode != 4;
        let scale = if high { 8388608.0 } else { 32768.0 };
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let l = self.left.process(l as f64 * scale, benford, bright);
            let r = self.right.process(r as f64 * scale, benford, bright);
            *out_l = ((l / scale) as f32).clamp(-1.0, 1.0);
            *out_r = ((r / scale) as f32).clamp(-1.0, 1.0);
        }
    }
}
