//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/AQuickVoiceClip, b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Highpass",
    label: "hz",
    default: 0.42,
}];
pub(super) const INFO:PluginInfo=PluginInfo{name:"AQuickVoiceClip",upstream_name:"AQuickVoiceClip",description:"AQuickVoiceClip softens headset mic recordings that have been super hard clipped on capture.",params:PARAMS};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Quick::new(rate, seed))
}
#[derive(Default)]
struct Channel {
    history: [f64; 6],
    last: f64,
    diff: f64,
    overall: f64,
    iir: [f64; 4],
    outs: [f64; 3],
    depth: f64,
}
impl Channel {
    fn clip(&mut self, mut input: f64) -> f64 {
        let softness = 0.484416;
        let hardness = 1.0 - softness;
        let clip = 0.5;
        if self.last >= clip {
            self.depth += 0.1;
            self.last = if input < clip {
                clip * hardness + input * softness
            } else {
                clip
            };
        }
        if self.last <= -clip {
            self.depth += 0.1;
            self.last = if input > -clip {
                -clip * hardness + input * softness
            } else {
                -clip
            };
        }
        if input > clip {
            self.depth += 0.1;
            input = if self.last < clip {
                clip * hardness + self.last * softness
            } else {
                clip
            };
        }
        if input < -clip {
            self.depth += 0.1;
            input = if self.last > -clip {
                -clip * hardness + self.last * softness
            } else {
                -clip
            };
        }
        let result = self.last;
        self.last = input;
        result
    }
    fn sample(&mut self, input: f64, amount: f64, flip: bool, fpd: &mut u32) -> f64 {
        let dry = input;
        let mut pass = input;
        let mut half = (input
            + self.history[0]
            + self.history[1] * -0.646
            + self.history[2] * 0.311
            + self.history[3] * -0.093
            + self.history[4] * 0.057
            + self.history[5] * -0.023)
            / 2.0;
        self.history.copy_within(0..5, 1);
        self.history[0] = input;
        let mut max = self.history[5].abs();
        for &x in self.history[..5].iter().rev() {
            if x.abs() > max {
                max = x.abs();
            }
        }
        if input.abs() > max {
            max = input.abs();
        }
        max *= 2.0;
        if max > 1.0 {
            max = 1.0;
        }
        let halfdry = half;
        half -= self.overall;
        half = self.clip(half);
        let raw = self.clip(input - self.overall);
        let halfdry = dry * 0.114 + halfdry * 0.886;
        let halfdiff = (half - halfdry) / 2.0;
        let lastdiff = self.diff * 0.431;
        self.diff = (raw - dry) / 2.0;
        self.diff += halfdiff;
        self.diff -= lastdiff;
        let mut result = dry;
        result += self.diff;
        self.overall = self.overall * (1.0 - 0.0682276) + self.diff * 0.0682276;
        let a = if flip { 0 } else { 1 };
        let c = if flip { 2 } else { 3 };
        self.iir[a] = self.iir[a] * (1.0 - amount) + result * amount;
        result -= self.iir[a];
        self.iir[c] = self.iir[c] * (1.0 - amount) + pass * amount;
        pass -= self.iir[c];
        self.outs[2] = self.outs[1];
        self.outs[1] = self.outs[0];
        self.outs[0] = result;
        self.depth -= 0.0009;
        if self.depth > 0.0 {
            if self.depth > 1.0 {
                self.depth = 1.0;
            }
            result *= 1.0 - self.depth;
            result += ((self.outs[0] + self.outs[1] + self.outs[2]) / 3.6) * self.depth;
        }
        if self.depth < 0.0 {
            self.depth = 0.0;
        }
        result = fpd_dither(result, fpd);
        result *= 1.0 - max;
        result += pass * max;
        result.clamp(-0.98, 0.98)
    }
}
struct Quick {
    p: f32,
    rate: f32,
    seed: u32,
    ch: [Channel; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl Quick {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            p: 0.42,
            rate,
            seed,
            ch: [Channel::default(), Channel::default()],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Quick {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        if i == 0 {
            self.p
        } else {
            0.0
        }
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if i == 0 {
            if let Some(v) = sanitize_param(v) {
                self.p = v;
            }
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i == 0 {
            float2string(((self.p as f64).powf(3.0) * 2070.0 + 30.0) as f32)
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let p = self.p;
        *self = Self::new(self.rate, self.seed);
        self.p = p;
    }
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let overall = (1.0 / 44100.0) * self.rate as f64;
        let amount = ((self.p as f64).powf(3.0) * 2070.0 + 30.0) / 8000.0 / overall;
        for (((&l, &r), ol), orr) in l.iter().zip(r).zip(ol.iter_mut()).zip(orr.iter_mut()) {
            let mut input = [l as f64, r as f64];
            for (c, v) in input.iter_mut().enumerate() {
                if v.abs() < 1.18e-23 {
                    *v = self.fpd[c] as f64 * 1.18e-17;
                }
                *v = self.ch[c].sample(*v, amount, self.flip, &mut self.fpd[c]);
            }
            self.flip = !self.flip;
            *ol = input[0] as f32;
            *orr = input[1] as f32;
        }
    }
}
