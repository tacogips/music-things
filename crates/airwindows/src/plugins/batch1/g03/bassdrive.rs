//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Port of airwindows/plugins/LinuxVST/src/BassDrive at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)] // Preserve upstream threshold.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Presnce",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "High",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Mid",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Low",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Drive",
        label: "",
        default: 0.5,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "BassDrive",
    upstream_name: "BassDrive",
    description: "BassDrive is an old secret weapon, like a kind of bass amp.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(BassDrive::new(sample_rate, seed))
}
struct BassDrive {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
    flip: bool,
    input: [[[[f64; 7]; 4]; 2]; 2],
    output: [[[[f64; 7]; 4]; 2]; 2],
}
impl BassDrive {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5; 5],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            flip: false,
            input: [[[[0.0; 7]; 4]; 2]; 2],
            output: [[[[0.0; 7]; 4]; 2]; 2],
        }
    }
    fn channel(
        &mut self,
        sample: f64,
        bank: usize,
        channel: usize,
        gains: [f64; 4],
        drive: f64,
    ) -> f64 {
        let mut sum = 0.0;
        for (band, gain) in gains.into_iter().enumerate() {
            let input = &mut self.input[bank][channel][band];
            let output = &mut self.output[bank][channel][band];
            for i in 0..6 {
                input[i] = input[i + 1];
            }
            if band == 0 {
                input[6] = sample * gain;
            } else {
                let mut bridge = (sample.abs() * gain).min(1.57079633).sin();
                if sample <= 0.0 {
                    bridge = -bridge;
                }
                input[6] = bridge;
                input[6] *= gain;
            }
            for i in 2..6 {
                output[i] = output[i + 1];
            }
            output[6] = match band {
                0 => {
                    (input[0] + input[6]) + 1.9152966321 * (input[1] + input[5])
                        - (input[2] + input[4])
                        - 3.8305932641 * input[3]
                        + (-0.2828214615 * output[2])
                        + (0.2613069963 * output[3])
                        + (-0.8628193852 * output[4])
                        + (0.5387164389 * output[5])
                }
                1 => {
                    (input[0] + input[6])
                        - 0.5141967433 * (input[1] + input[5])
                        - (input[2] + input[4])
                        + 1.0283934866 * input[3]
                        + (-0.2828214615 * output[2])
                        + (1.0195930909 * output[3])
                        + (-1.9633013869 * output[4])
                        + (2.1020162751 * output[5])
                }
                2 => {
                    (input[0] + input[6])
                        - 1.1790257790 * (input[1] + input[5])
                        - (input[2] + input[4])
                        + 2.3580515580 * input[3]
                        + (-0.6292082828 * output[2])
                        + (2.7785843605 * output[3])
                        + (-4.6638295236 * output[4])
                        + (3.5142515802 * output[5])
                }
                _ => {
                    (input[0] + input[6])
                        - 1.9193504547 * (input[1] + input[5])
                        - (input[2] + input[4])
                        + 3.8387009093 * input[3]
                        + (-0.9195964462 * output[2])
                        + (3.7538173833 * output[3])
                        + (-5.7487775603 * output[4])
                        + (3.9145559258 * output[5])
                }
            };
            if band < 2 {
                let bridge = output[6].abs().min(1.57079633).sin();
                if output[6] > 0.0 {
                    sum += bridge;
                } else {
                    sum -= bridge;
                }
            } else {
                sum += output[6];
            }
        }
        let result = (sum.abs() * drive).min(1.57079633).sin();
        if sum < 0.0 {
            -result
        } else {
            result
        }
    }
}
impl AirwindowsEffect for BassDrive {
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
            .map_or_else(String::new, |&v| float2string(v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let gains = [
            (self.params[0] as f64).powf(5.0) * 8.0,
            (self.params[1] as f64).powf(3.0) * 4.0,
            (self.params[2] as f64).powf(2.0),
            self.params[3] as f64 / 4.0,
        ];
        let drive = self.params[4] as f64 * 2.0;
        for (((&l, &r), ol), or) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let bank = usize::from(self.flip);
            l = self.channel(l, bank, 0, gains, drive);
            r = self.channel(r, bank, 1, gains, drive);
            self.flip = !self.flip;
            *ol = fpd_dither(l, &mut self.fpd_l) as f32;
            *or = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
