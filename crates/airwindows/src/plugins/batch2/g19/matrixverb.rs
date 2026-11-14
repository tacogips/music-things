//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/MatrixVerb, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
#![allow(clippy::excessive_precision, clippy::needless_range_loop)]

use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::AwRand;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Filter",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Damping",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Speed",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Vibrato",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "RmSize",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Flavor",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "MatrixVerb",
    upstream_name: "MatrixVerb",
    description:
        "MatrixVerb is a wildly adjustable, strangely colorful reverb for deep and flexible spaces.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(MatrixVerb::new(sample_rate, seed))
}
const SIZES: [usize; 13] = [
    8111, 7511, 7311, 6911, 6311, 6111, 5511, 4911, 4511, 4311, 3911, 3311, 3111,
];
const LENGTHS: [f64; 12] = [
    79.0, 73.0, 71.0, 67.0, 61.0, 59.0, 53.0, 47.0, 43.0, 41.0, 37.0, 31.0,
];
const DEPTHS: [f64; 8] = [
    0.003251, 0.002999, 0.002917, 0.002749, 0.002503, 0.002423, 0.002146, 0.002088,
];
struct MatrixVerb {
    params: [f32; 7],
    sample_rate: f32,
    seed: u32,
    delay: [[Box<[f64]>; 13]; 2],
    count: [usize; 13],
    feedback: [[f64; 8]; 2],
    vib: [[f64; 8]; 2],
    filters: [[f64; 11]; 3],
    fpd: [u32; 2],
}
impl MatrixVerb {
    fn vibrato(seed: u32) -> [[f64; 8]; 2] {
        let mut rng = AwRand::new(seed);
        let mut vib = [[0.0; 8]; 2];
        for channel in &mut vib {
            for v in channel {
                *v = (rng.next() as i32).wrapping_mul(-2147483647) as f64;
            }
        }
        vib
    }
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.0, 0.0, 0.0, 0.5, 0.5, 1.0],
            sample_rate,
            seed,
            delay: std::array::from_fn(|_| {
                std::array::from_fn(|i| vec![0.0; SIZES[i]].into_boxed_slice())
            }),
            count: [1; 13],
            feedback: [[0.0; 8]; 2],
            vib: Self::vibrato(seed),
            filters: [[0.0; 11]; 3],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn filter(sample: f64, b: &mut [f64; 11], ch: usize) -> f64 {
    let i = 7 + ch * 2;
    let out = sample * b[2] + b[i];
    b[i] = sample * b[3] - out * b[5] + b[i + 1];
    b[i + 1] = sample * b[4] - out * b[6];
    out
}
impl AirwindowsEffect for MatrixVerb {
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
        for ch in &mut self.delay {
            for d in ch {
                d.fill(0.0);
            }
        }
        self.count = [1; 13];
        self.feedback = [[0.0; 8]; 2];
        self.vib = Self::vibrato(self.seed);
        self.filters = [[0.0; 11]; 3];
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d, e, f, g] = self.params;
        let k =
            (std::f64::consts::PI * ((a as f64 * 9000.0 + 1000.0) / self.sample_rate as f64)).tan();
        for (filter, q) in self.filters.iter_mut().zip([
            1.618033988749894848204586,
            0.618033988749894848204586,
            0.5,
        ]) {
            let norm = 1.0 / (1.0 + k / q + k * k);
            filter[2] = k * k * norm;
            filter[3] = 2.0 * filter[2];
            filter[4] = filter[2];
            filter[5] = 2.0 * (k * k - 1.0) * norm;
            filter[6] = (1.0 - k / q + k * k) * norm;
        }
        let vib_speed = 0.06 + c as f64;
        let vib_depth = (0.027 + (d as f64).powf(3.0)) * 100.0;
        let size = (e as f64).powf(2.0) * 90.0 + 10.0;
        let depth_factor = 1.0 - (1.0 - (0.82 - (b as f64 * 0.5 + size * 0.002))).powf(4.0);
        let blend = 0.955 - size * 0.007;
        let crossmod = ((f as f64 - 0.5) * 2.0).powf(3.0) * 0.5;
        let regen = depth_factor * (0.5 - crossmod.abs() * 0.031);
        let wet = g as f64;
        let mut lengths = [0usize; 13];
        for i in 0..12 {
            lengths[i] = (LENGTHS[i] * size) as usize;
        }
        lengths[12] = (29.0 * size - 56.0 * size * crossmod.abs()) as usize;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut input = [l as f64, r as f64];
            for (v, fp) in input.iter_mut().zip(self.fpd) {
                if v.abs() < 1.18e-23 {
                    *v = fp as f64 * 1.18e-17;
                }
            }
            let dry = input;
            for ch in 0..2 {
                self.delay[ch][12][self.count[12]] = input[ch];
            }
            self.count[12] += 1;
            if self.count[12] > lengths[12] {
                self.count[12] = 0;
            }
            for ch in 0..2 {
                input[ch] = filter(self.delay[ch][12][self.count[12]], &mut self.filters[0], ch);
                input[ch] = (input[ch] * wet).sin();
            }
            let mut allpass = [[0.0; 4]; 2];
            for n in 0..4 {
                let i = 8 + n;
                let next = if self.count[i] + 1 > lengths[i] {
                    0
                } else {
                    self.count[i] + 1
                };
                for ch in 0..2 {
                    allpass[ch][n] = input[ch] - self.delay[ch][i][next] * 0.5;
                    self.delay[ch][i][self.count[i]] = allpass[ch][n];
                    allpass[ch][n] *= 0.5;
                }
                self.count[i] += 1;
                if self.count[i] > lengths[i] {
                    self.count[i] = 0;
                }
                for ch in 0..2 {
                    allpass[ch][n] += self.delay[ch][i][self.count[i]];
                }
            }
            for i in 0..8 {
                let source = [3, 2, 1, 0, 0, 1, 2, 3][i];
                for ch in 0..2 {
                    self.delay[ch][i][self.count[i]] = allpass[ch][source] + self.feedback[ch][i];
                }
                self.count[i] += 1;
                if self.count[i] > lengths[i] {
                    self.count[i] = 0;
                }
            }
            for ch in 0..2 {
                let mut v = [0.0; 8];
                for i in 0..8 {
                    self.vib[ch][i] += DEPTHS[i] * vib_speed;
                    let offset = (self.vib[ch][i].sin() + 1.0) * vib_depth;
                    let working = (self.count[i] as f64 + offset) as usize;
                    let idx = if working > lengths[i] {
                        working - lengths[i] - 1
                    } else {
                        working
                    };
                    let next = if working + 1 > lengths[i] {
                        working - lengths[i]
                    } else {
                        working + 1
                    };
                    let frac = offset - offset.floor();
                    let base = self.delay[ch][i][idx];
                    let interp = base * (1.0 - frac) + self.delay[ch][i][next] * frac;
                    v[i] = (1.0 - blend) * interp + base * blend;
                }
                v[0] = v[0] * (1.0 - crossmod.abs()) + v[4] * crossmod;
                v[4] = v[4] * (1.0 - crossmod.abs()) + v[0] * crossmod;
                for n in [0, 4] {
                    for i in 0..4 {
                        let other = match i {
                            0 => v[n + 1] + v[n + 2] + v[n + 3],
                            1 => v[n] + v[n + 2] + v[n + 3],
                            2 => v[n] + v[n + 1] + v[n + 3],
                            _ => v[n] + v[n + 1] + v[n + 2],
                        };
                        self.feedback[ch][n + i] = (v[n + i] - other) * regen;
                    }
                }
                let mut sample = (v[0] + v[1] + v[2] + v[3] + v[4] + v[5] + v[6] + v[7]) / 8.0;
                sample = filter(sample, &mut self.filters[1], ch)
                    .clamp(-1.0, 1.0)
                    .asin();
                sample = filter(sample, &mut self.filters[2], ch);
                if wet != 1.0 {
                    sample += dry[ch] * (1.0 - wet);
                }
                input[ch] = fpd_dither(sample, &mut self.fpd[ch]);
            }
            *out_l = input[0] as f32;
            *out_r = input[1] as f32;
        }
    }
}
