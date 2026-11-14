//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Dynamics, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Comp",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Speed",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Gate",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Dynamics",
    upstream_name: "Dynamics",
    description: "Dynamics is a combination compressor gate.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Dynamics::new(sample_rate, seed))
}
struct Channel {
    target: [f64; 2],
    control: [[f64; 2]; 2],
    was_negative: bool,
    zero_cross: f64,
    roller: f64,
    gate: f64,
}
impl Default for Channel {
    fn default() -> Self {
        Self {
            target: [1.0; 2],
            control: [[1.0; 2]; 2],
            was_negative: false,
            zero_cross: 0.0,
            roller: 0.0,
            gate: 0.0,
        }
    }
}
impl Channel {
    fn process(
        &mut self,
        dry: f64,
        gain: f64,
        divisor: f64,
        remainder: f64,
        on: f64,
        flip: bool,
    ) -> f64 {
        let mut input = dry * gain;
        let mut pos = input + 1.0;
        if pos < 0.0 {
            pos = 0.0;
        }
        let mut outpos = pos / 2.0;
        if outpos > 1.0 {
            outpos = 1.0;
        }
        pos *= pos;
        self.target[0] *= divisor;
        self.target[0] += pos * remainder;
        let calcpos = 1.0 / self.target[0];
        let mut neg = -input + 1.0;
        if neg < 0.0 {
            neg = 0.0;
        }
        let mut outneg = neg / 2.0;
        if outneg > 1.0 {
            outneg = 1.0;
        }
        neg *= neg;
        self.target[1] *= divisor;
        self.target[1] += neg * remainder;
        let calcneg = 1.0 / self.target[1];
        let index = if flip { 0 } else { 1 };
        let side = if input > 0.0 { 0 } else { 1 };
        self.control[index][side] *= divisor;
        self.control[index][side] += if side == 0 {
            calcpos * remainder
        } else {
            calcneg * remainder
        };
        let multiplier = self.control[index][0] * outpos + self.control[index][1] * outneg;
        input *= multiplier;
        input /= gain;
        if dry > 0.0 {
            if self.was_negative {
                self.zero_cross = 220.9 * 0.3;
            }
            self.was_negative = false;
        } else {
            self.zero_cross += 1.0;
            self.was_negative = true;
        }
        if self.zero_cross > 220.9 {
            self.zero_cross = 220.9;
        }
        if self.gate == 0.0 {
            if dry.abs() > on {
                if self.roller == 0.0 {
                    self.roller = self.zero_cross;
                } else {
                    self.roller -= 0.028331119964586;
                }
            } else {
                self.roller -= 0.028331119964586;
            }
        } else if dry.abs() > on * 1.1 {
            if self.roller < self.zero_cross {
                self.roller = self.zero_cross;
            } else {
                self.roller -= 0.028331119964586;
            }
        } else {
            self.roller -= 0.028331119964586;
        }
        if self.roller < 0.0 {
            self.roller = 0.0;
        }
        if self.roller < 1.0 {
            self.gate = self.roller;
            let bridge = 1.0 - input.abs().cos();
            if input > 0.0 {
                input = input * self.gate + bridge * (1.0 - self.gate);
            } else {
                input = input * self.gate - bridge * (1.0 - self.gate);
            }
            if self.gate == 0.0 {
                input = 0.0;
            }
        } else {
            self.gate = 1.0;
        }
        input
    }
}
struct Dynamics {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl Dynamics {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.5, 0.0, 1.0],
            sample_rate,
            seed,
            left: Channel::default(),
            right: Channel::default(),
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Dynamics {
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
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let scale = 1.0 / 44100.0 * self.sample_rate as f64;
        let gain = (a as f64).powf(5.0) * 35.0 + 1.0;
        let remainder = ((b as f64).powf(4.0) * 0.01 + 0.0005) / scale;
        let divisor = 1.0 - remainder;
        let on = (c as f64).powf(3.0) / 3.0 + 0.00018;
        let wet = d as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            l = self
                .left
                .process(l, gain, divisor, remainder, on, self.flip);
            r = self
                .right
                .process(r, gain, divisor, remainder, on, self.flip);
            self.flip = !self.flip;
            if wet != 1.0 {
                l = l * wet + dry_l * (1.0 - wet);
                r = r * wet + dry_r * (1.0 - wet);
            }
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
