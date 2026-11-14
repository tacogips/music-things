//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ADT, b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.0.
// Preserve upstream constants and operation order.
#![allow(clippy::needless_range_loop)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Headrm",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "A Delay",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "A Level",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "B Delay",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "B Level",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 0.5,
    },
];
pub(super) const INFO: PluginInfo = PluginInfo {
    name: "ADT",
    upstream_name: "ADT",
    description: "ADT is a double short delay tap with saturation.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Adt::new(rate, seed))
}
struct Adt {
    params: [f32; 6],
    rate: f32,
    seed: u32,
    p: [Box<[f64]>; 2],
    offsets: [f64; 2],
    count: usize,
    fpd: [u32; 2],
}
impl Adt {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5; 6],
            rate,
            seed,
            p: [
                vec![0.0; 9999].into_boxed_slice(),
                vec![0.0; 9999].into_boxed_slice(),
            ],
            offsets: [9001.0; 2],
            count: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Adt {
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
        if i >= 6 {
            return String::new();
        }
        let p = self.params[i] as f64;
        float2string(match i {
            0 | 5 => p * 2.0,
            2 | 4 => p * 2.0 - 1.0,
            _ => p,
        } as f32)
    }
    fn reset(&mut self) {
        self.p[0].fill(0.0);
        self.p[1].fill(0.0);
        self.offsets = [9001.0; 2];
        self.count = 0;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let _ = self.rate;
        let gain = self.params[0] as f64 * 1.272;
        let targets = [
            (self.params[1] as f64).powf(4.0) * 4790.0,
            (self.params[3] as f64).powf(4.0) * 4790.0,
        ];
        let intensities = [self.params[2] as f64 - 0.5, self.params[4] as f64 - 0.5];
        let output = self.params[5] as f64 * 2.0;
        for (((&l, &r), ol), orr) in l.iter().zip(r).zip(ol.iter_mut()).zip(orr.iter_mut()) {
            let mut input = [l as f64, r as f64];
            for (c, v) in input.iter_mut().enumerate() {
                if v.abs() < 1.18e-23 {
                    *v = self.fpd[c] as f64 * 1.18e-17;
                }
            }
            for (offset, target) in self.offsets.iter_mut().zip(targets) {
                if (*offset - target).abs() > 1000.0 {
                    *offset = target;
                }
                *offset = (*offset * 999.0 + target) / 1000.0;
            }
            for v in &mut input {
                if gain > 0.0 {
                    *v /= gain;
                }
                *v = v.clamp(-1.2533141373155, 1.2533141373155);
                let a = v.abs();
                *v = (*v * a).sin() / if a == 0.0 { 1.0 } else { a };
            }
            if self.count < 1 || self.count > 4800 {
                self.count = 4800;
            }
            let mut totals = [0.0; 2];
            for c in 0..2 {
                self.p[c][self.count] = input[c];
                self.p[c][self.count + 4800] = input[c];
            }
            for t in 0..2 {
                if intensities[t] != 0.0 {
                    let floor = self.offsets[t].floor();
                    let fraction = self.offsets[t] - floor;
                    let minus = 1.0 - fraction;
                    let idx = self.count + floor as usize;
                    for (c, total) in totals.iter_mut().enumerate() {
                        let p = &self.p[c];
                        let mut v = p[idx] * minus;
                        v += p[idx + 1];
                        v += p[idx + 2] * fraction;
                        v -= ((p[idx] - p[idx + 1]) - (p[idx + 1] - p[idx + 2])) / 50.0;
                        *total += v * intensities[t];
                    }
                }
            }
            self.count -= 1;
            for c in 0..2 {
                input[c] += totals[c];
                input[c] = input[c].clamp(-1.0, 1.0).asin();
                input[c] *= gain;
                if output < 1.0 {
                    input[c] *= output;
                }
                input[c] = fpd_dither(input[c], &mut self.fpd[c]);
            }
            *ol = input[0] as f32;
            *orr = input[1] as f32;
        }
    }
}
