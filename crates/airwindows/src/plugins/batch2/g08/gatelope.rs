//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Gatelope, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Thresh",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "TrebSus",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "BassSus",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "AttackS",
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
    name: "Gatelope",
    upstream_name: "Gatelope",
    description: "Gatelope is a special gate that applies filters.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Gatelope::new(sample_rate, seed))
}
struct Gatelope {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    low: [[f64; 2]; 2],
    high: [[f64; 2]; 2],
    treble: f64,
    bass: f64,
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl Gatelope {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 1.0, 0.5, 0.0, 1.0],
            sample_rate,
            seed,
            low: [[0.0; 2]; 2],
            high: [[0.0; 2]; 2],
            treble: 1.0,
            bass: 0.0,
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Gatelope {
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
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let threshold = (a as f64).powf(2.0);
        let mut trebledecay = (1.0 - b as f64).powf(2.0) / 4196.0;
        let mut bassdecay = (1.0 - c as f64).powf(2.0) / 8192.0;
        let mut slow_attack = (d as f64).powf(3.0) * 3.0 + 0.003;
        let wet = e as f64;
        slow_attack /= overallscale;
        trebledecay /= overallscale;
        bassdecay /= overallscale;
        trebledecay += 1.0;
        bassdecay += 1.0;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut input = [l as f64, r as f64];
            if input[0].abs() < 1.18e-23 {
                input[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if input[1].abs() < 1.18e-23 {
                input[1] = self.fpd_r as f64 * 1.18e-17;
            }
            let dry = input;
            let highest = if input[0].abs() > input[1].abs() {
                input[0].abs()
            } else {
                input[1].abs()
            };
            let mut attack = slow_attack - (highest * slow_attack * 0.5);
            if attack < 0.0 {
                attack = 0.0;
            }
            if highest > threshold {
                self.treble += attack;
                if self.treble > 2.0 {
                    self.treble = 2.0;
                }
                self.bass -= attack;
                self.bass -= attack;
                if self.bass < 0.0 {
                    self.bass = 0.0;
                }
                self.low = [input, input];
                self.high = [[0.0; 2]; 2];
            } else {
                self.treble -= self.bass;
                self.treble /= trebledecay;
                self.treble += self.bass;
                self.bass -= self.treble;
                self.bass /= bassdecay;
                self.bass += self.treble;
            }
            let bank = if self.flip { 0 } else { 1 };
            for (channel, &sample) in input.iter().enumerate() {
                if self.treble >= 1.0 {
                    self.low[bank][channel] = sample;
                } else {
                    self.low[bank][channel] =
                        self.low[bank][channel] * (1.0 - self.treble) + sample * self.treble;
                }
            }
            if self.bass > 1.0 {
                self.bass = 1.0;
            }
            for channel in 0..2 {
                if self.bass > 0.0 {
                    self.high[bank][channel] =
                        self.high[bank][channel] * (1.0 - self.bass) + input[channel] * self.bass;
                } else {
                    self.high[bank][channel] = 0.0;
                }
                input[channel] = if self.treble > self.bass {
                    self.low[bank][channel] - self.high[bank][channel]
                } else {
                    0.0
                };
                input[channel] = (1.0 - wet) * dry[channel] + wet * input[channel];
            }
            self.flip = !self.flip;
            *out_l = fpd_dither(input[0], &mut self.fpd_l) as f32;
            *out_r = fpd_dither(input[1], &mut self.fpd_r) as f32;
        }
    }
}
