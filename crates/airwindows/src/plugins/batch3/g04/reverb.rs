//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Reverb, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::needless_range_loop)] // Preserve upstream DSP constants and state ordering.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::AwRand;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Big",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Wet",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo { name: "Reverb", upstream_name: "Reverb", description: "Reverb is simplified and tuned MatrixVerb, adjusting everything for each optimum verb size.", params: PARAMS };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Reverb::new(sample_rate, seed))
}
const LENGTHS: [usize; 13] = [
    8111, 7511, 7311, 6911, 6311, 6111, 5511, 4911, 4511, 4311, 3911, 3311, 3111,
];
const BASE: [usize; 13] = [79, 73, 71, 67, 61, 59, 53, 47, 43, 41, 37, 31, 29];
const DEPTH: [f64; 8] = [
    0.003251, 0.002999, 0.002917, 0.002749, 0.002503, 0.002423, 0.002146, 0.002088,
];
struct Reverb {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    buffers: [[Box<[f64]>; 13]; 2],
    count: [usize; 13],
    feedback: [[f64; 8]; 2],
    vib: [[f64; 8]; 2],
    biquad: [[f64; 11]; 3],
    fpd: [u32; 2],
}
impl Reverb {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut result = Self {
            params: [0.5; 2],
            sample_rate,
            seed,
            buffers: std::array::from_fn(|_| {
                std::array::from_fn(|i| vec![0.0; LENGTHS[i]].into_boxed_slice())
            }),
            count: [1; 13],
            feedback: [[0.0; 8]; 2],
            vib: [[0.0; 8]; 2],
            biquad: [[0.0; 11]; 3],
            fpd: [0; 2],
        };
        result.reset();
        result
    }
}
fn filter(x: f64, b: &mut [f64; 11], ch: usize) -> f64 {
    let i = 7 + ch * 2;
    let out = (x * b[2]) + b[i];
    b[i] = (x * b[3]) - (out * b[5]) + b[i + 1];
    b[i + 1] = (x * b[4]) - (out * b[6]);
    out
}
impl AirwindowsEffect for Reverb {
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
        for channel in &mut self.buffers {
            for buffer in channel {
                buffer.fill(0.0);
            }
        }
        self.count = [1; 13];
        self.feedback = [[0.0; 8]; 2];
        self.biquad = [[0.0; 11]; 3];
        let mut rng = AwRand::new(self.seed);
        for ch in 0..2 {
            for i in 0..8 {
                self.vib[ch][i] = (rng.next() as i32).wrapping_mul(-2147483647) as f64;
            }
        }
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let big = self.params[0] as f64;
        let wet = self.params[1] as f64;
        let size = (big.powf(2.0) * 75.0) + 25.0;
        let depth_factor = 1.0 - (1.0 - (0.82 - (((1.0 - big) * 0.7) + (size * 0.002)))).powf(4.0);
        let blend = 0.955 - (size * 0.007);
        let regen = depth_factor * 0.5;
        let delay: [usize; 13] = std::array::from_fn(|i| (BASE[i] as f64 * size) as usize);
        for (i, q) in [1.618033988749895, 0.6180339887498948, 0.5]
            .into_iter()
            .enumerate()
        {
            let b = &mut self.biquad[i];
            b[0] = (10000.0 - (big * wet * 3000.0)) / self.sample_rate as f64;
            b[1] = q;
            let k = (std::f64::consts::PI * b[0]).tan();
            let norm = 1.0 / (1.0 + k / b[1] + k * k);
            b[2] = k * k * norm;
            b[3] = 2.0 * b[2];
            b[4] = b[2];
            b[5] = 2.0 * (k * k - 1.0) * norm;
            b[6] = (1.0 - k / b[1] + k * k) * norm;
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut x = [l as f64, r as f64];
            for ch in 0..2 {
                if x[ch].abs() < 1.18e-23 {
                    x[ch] = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            let dry = x;
            for ch in 0..2 {
                self.buffers[ch][12][self.count[12]] = x[ch];
            }
            self.count[12] += 1;
            if self.count[12] > delay[12] {
                self.count[12] = 0;
            }
            for ch in 0..2 {
                x[ch] = self.buffers[ch][12][self.count[12]];
                x[ch] = filter(x[ch], &mut self.biquad[0], ch);
                x[ch] *= wet;
                x[ch] = x[ch].sin();
            }
            let mut all = [[0.0; 4]; 2];
            for i in 0..4 {
                let line = 8 + i;
                let mut next = self.count[line] + 1;
                if next > delay[line] {
                    next = 0;
                }
                for ch in 0..2 {
                    all[ch][i] = x[ch];
                    all[ch][i] -= self.buffers[ch][line][next] * 0.5;
                    self.buffers[ch][line][self.count[line]] = all[ch][i];
                    all[ch][i] *= 0.5;
                }
                self.count[line] += 1;
                if self.count[line] > delay[line] {
                    self.count[line] = 0;
                }
                for ch in 0..2 {
                    all[ch][i] += self.buffers[ch][line][self.count[line]];
                }
            }
            for i in 0..8 {
                let source = [3, 2, 1, 0, 0, 1, 2, 3][i];
                for ch in 0..2 {
                    self.buffers[ch][i][self.count[i]] = all[ch][source] + self.feedback[ch][i];
                }
                self.count[i] += 1;
                if self.count[i] > delay[i] {
                    self.count[i] = 0;
                }
            }
            let mut interp = [[0.0; 8]; 2];
            for ch in 0..2 {
                for i in 0..8 {
                    self.vib[ch][i] += DEPTH[i] * 0.1;
                    let offset = (self.vib[ch][i].sin() + 1.0) * 7.0;
                    let working = (self.count[i] as f64 + offset) as usize;
                    let first = working - if working > delay[i] { delay[i] + 1 } else { 0 };
                    let second = working + 1
                        - if working + 1 > delay[i] {
                            delay[i] + 1
                        } else {
                            0
                        };
                    let frac = offset - offset.floor();
                    interp[ch][i] = self.buffers[ch][i][first] * (1.0 - frac);
                    interp[ch][i] += self.buffers[ch][i][second] * frac;
                    interp[ch][i] =
                        ((1.0 - blend) * interp[ch][i]) + (self.buffers[ch][i][first] * blend);
                }
            }
            for ch in 0..2 {
                for start in [0, 4] {
                    for i in start..start + 4 {
                        let others = match i - start {
                            0 => [start + 1, start + 2, start + 3],
                            1 => [start, start + 2, start + 3],
                            2 => [start, start + 1, start + 3],
                            _ => [start, start + 1, start + 2],
                        };
                        self.feedback[ch][i] = (interp[ch][i]
                            - (interp[ch][others[0]]
                                + interp[ch][others[1]]
                                + interp[ch][others[2]]))
                            * regen;
                    }
                }
                x[ch] = (interp[ch][0]
                    + interp[ch][1]
                    + interp[ch][2]
                    + interp[ch][3]
                    + interp[ch][4]
                    + interp[ch][5]
                    + interp[ch][6]
                    + interp[ch][7])
                    / 8.0;
                x[ch] = filter(x[ch], &mut self.biquad[1], ch);
                x[ch] = x[ch].clamp(-1.0, 1.0);
                x[ch] = x[ch].asin();
                x[ch] = filter(x[ch], &mut self.biquad[2], ch);
                if wet != 1.0 {
                    x[ch] += dry[ch] * (1.0 - wet);
                }
                x[ch] = fpd_dither(x[ch], &mut self.fpd[ch]);
            }
            *out_l = x[0] as f32;
            *out_r = x[1] as f32;
        }
    }
}
