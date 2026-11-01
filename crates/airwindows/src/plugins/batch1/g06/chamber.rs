/* Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
 * Upstream: airwindows/plugins/LinuxVST/src/Chamber/Chamber.h
 * Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
 * Parity: Libm; measured max-abs 0.0.
 */
#![allow(clippy::needless_range_loop, clippy::excessive_precision)]

use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
static PARAMS: [ParamInfo; 5] = [
    ParamInfo {
        name: "Bigness",
        label: "",
        default: 0.35,
    },
    ParamInfo {
        name: "Longness",
        label: "",
        default: 0.35,
    },
    ParamInfo {
        name: "Liteness",
        label: "",
        default: 0.35,
    },
    ParamInfo {
        name: "Darkness",
        label: "",
        default: 0.35,
    },
    ParamInfo {
        name: "Wetness",
        label: "",
        default: 0.35,
    },
];
static INFO: PluginInfo = PluginInfo {
    name: "Chamber",
    upstream_name: "Chamber",
    description: "Chamber is a feedforward reverb based on the golden ratio.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Chamber::new(rate, seed))
}
struct Chamber {
    params: [f32; 5],
    rate: f32,
    seed: u32,
    fpd: [u32; 2],
    lines: [[Vec<f64>; 12]; 2],
    count: [usize; 12],
    cycle: usize,
    filter: [[f64; 3]; 2],
    feedback: [[f64; 4]; 2],
    previous: [[f64; 4]; 2],
    refs: [[f64; 9]; 2],
}
impl Chamber {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        let sizes = [
            2915, 1803, 1114, 688, 19999, 12360, 7639, 4721, 425, 263, 162, 100,
        ];
        Self {
            params: [0.35; 5],
            rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            lines: std::array::from_fn(|_| std::array::from_fn(|i| vec![0.0; sizes[i]])),
            count: [1; 12],
            cycle: 0,
            filter: [[0.0; 3]; 2],
            feedback: [[0.0; 4]; 2],
            previous: [[0.0; 4]; 2],
            refs: [[0.0; 9]; 2],
        }
    }
    fn reset_state(&mut self) {
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
        for side in &mut self.lines {
            for line in side {
                line.fill(0.0);
            }
        }
        self.count = [1; 12];
        self.cycle = 0;
        self.filter = [[0.0; 3]; 2];
        self.feedback = [[0.0; 4]; 2];
        self.previous = [[0.0; 4]; 2];
        self.refs = [[0.0; 9]; 2];
    }
    fn network(
        &mut self,
        mut x: [f64; 2],
        delay: [usize; 12],
        regen: f64,
        interpolate: f64,
        end: usize,
    ) -> [f64; 2] {
        for ch in 0..2 {
            for i in 0..4 {
                self.feedback[ch][i] = (self.feedback[ch][i] * (1.0 - interpolate))
                    + (self.previous[ch][i] * interpolate);
                self.previous[ch][i] = self.feedback[ch][i];
            }
        }
        let mut outputs = [[0.0; 4]; 2];
        for stage in 0..3 {
            let base = match stage {
                0 => 8,
                1 => 0,
                _ => 4,
            };
            for ch in 0..2 {
                for i in 0..4 {
                    let v = if stage == 0 {
                        x[ch] + self.feedback[ch][i] * regen
                    } else {
                        let v = outputs[ch];
                        match i {
                            0 => v[0] - (v[1] + v[2] + v[3]),
                            1 => v[1] - (v[0] + v[2] + v[3]),
                            2 => v[2] - (v[0] + v[1] + v[3]),
                            _ => v[3] - (v[0] + v[1] + v[2]),
                        }
                    };
                    self.lines[ch][base + i][self.count[base + i]] = v;
                }
            }
            for i in 0..4 {
                let k = base + i;
                self.count[k] += 1;
                if self.count[k] > delay[k] {
                    self.count[k] = 0;
                }
            }
            for ch in 0..2 {
                for i in 0..4 {
                    outputs[ch][i] = self.lines[ch][base + i][self.count[base + i]];
                }
            }
        }
        for ch in 0..2 {
            let o = outputs[ch];
            self.feedback[ch] = [
                o[0] - (o[1] + o[2] + o[3]),
                o[1] - (o[0] + o[2] + o[3]),
                o[2] - (o[0] + o[1] + o[3]),
                o[3] - (o[0] + o[1] + o[2]),
            ];
            x[ch] = (o[0] + o[1] + o[2] + o[3]) / 8.0;
            let r = &mut self.refs[ch];
            match end {
                4 => {
                    r[0] = r[4];
                    r[2] = (r[0] + x[ch]) / 2.0;
                    r[1] = (r[0] + r[2]) / 2.0;
                    r[3] = (r[2] + x[ch]) / 2.0;
                    r[4] = x[ch];
                }
                3 => {
                    r[0] = r[3];
                    r[2] = (r[0] + r[0] + x[ch]) / 3.0;
                    r[1] = (r[0] + x[ch] + x[ch]) / 3.0;
                    r[3] = x[ch];
                }
                2 => {
                    r[0] = r[2];
                    r[1] = (r[0] + x[ch]) / 2.0;
                    r[2] = x[ch];
                }
                _ => r[0] = x[ch],
            }
        }
        self.cycle = 0;
        [self.refs[0][0], self.refs[1][0]]
    }
}
impl AirwindowsEffect for Chamber {
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
            .map_or_else(String::new, |v| crate::display::float2string(*v))
    }
    fn reset(&mut self) {
        self.reset_state()
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let overall = (1.0 / 44100.0) * self.rate as f64;
        let end = (overall.floor() as usize).clamp(1, 4);
        self.cycle = self.cycle.min(end - 1);
        let size = (self.params[0] as f64).powf(2.0) * 0.9 + 0.1;
        let regen = (1.0 - (1.0 - self.params[1] as f64).powf(6.0)) * 0.123;
        let hi = (self.params[2] as f64).powf(2.0) / overall.sqrt();
        let lo = (1.0 - (self.params[3] as f64).powf(2.0)) / overall.sqrt();
        let interpolate = size * 0.381966011250105;
        let wet = (self.params[4] as f64 * 2.0).clamp(0.0, 1.0);
        let dry = (2.0 - self.params[4] as f64 * 2.0).clamp(0.0, 1.0);
        let mut delay = [0usize; 12];
        delay[4] = (19900.0 * size) as usize;
        for (prev, next) in [
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 0),
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 8),
            (8, 9),
            (9, 10),
            (10, 11),
        ] {
            delay[next] = (delay[prev] as f64 * 0.618033988749894848204586) as usize;
        }
        for (((l, r), a), b) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut x = [*l as f64, *r as f64];
            for ch in 0..2 {
                if x[ch].abs() < 1.18e-23 {
                    x[ch] = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            let original = x;
            for ch in 0..2 {
                self.filter[ch][2] = self.filter[ch][2] * (1.0 - hi) + x[ch] * hi;
                x[ch] -= self.filter[ch][2];
                self.filter[ch][0] = self.filter[ch][0] * (1.0 - lo) + x[ch] * lo;
                x[ch] = self.filter[ch][0];
            }
            self.cycle += 1;
            if self.cycle == end {
                x = self.network(x, delay, regen, interpolate, end);
            } else {
                x = [self.refs[0][self.cycle], self.refs[1][self.cycle]];
            }
            for ch in 0..2 {
                for k in (2..=end).rev() {
                    let idx = k + 3;
                    let t = x[ch];
                    x[ch] = (x[ch] + self.refs[ch][idx]) * 0.5;
                    self.refs[ch][idx] = t;
                }
                self.filter[ch][1] = self.filter[ch][1] * (1.0 - lo) + x[ch] * lo;
                x[ch] = self.filter[ch][1];
                if wet < 1.0 {
                    x[ch] *= wet;
                }
                let mut d = original[ch];
                if dry < 1.0 {
                    d *= dry;
                }
                x[ch] += d;
            }
            *a = fpd_dither(x[0], &mut self.fpd[0]) as f32;
            *b = fpd_dither(x[1], &mut self.fpd[1]) as f32;
        }
    }
}
