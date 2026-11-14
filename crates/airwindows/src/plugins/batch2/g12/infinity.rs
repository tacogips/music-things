//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Infinity, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
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
        name: "Size",
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
    name: "Infinity",
    upstream_name: "Infinity",
    description:
        "Infinity is a MatrixVerb specifically designed to sustain and layer sounds forever.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Infinity::new(sample_rate, seed))
}
struct Infinity {
    params: [f32; 4],
    core: Reverb,
}
impl Infinity {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.0, 0.5, 1.0],
            core: Reverb::new(rate, seed),
        }
    }
}
impl AirwindowsEffect for Infinity {
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
        self.core.reset();
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d] = self.params;
        self.core.process(
            il,
            ir,
            ol,
            or,
            [a as f64, b as f64, c as f64, d as f64, 1.0, 1.0],
            false,
        );
    }
}
pub(super) struct Reverb {
    rate: f32,
    seed: u32,
    filters: [[f64; 11]; 3],
    lines: [[Box<[f64]>; 12]; 2],
    counts: [usize; 12],
    feedback: [[f64; 8]; 2],
    fpd: [u32; 2],
}
impl Reverb {
    pub(super) fn new(rate: f32, seed: u32) -> Self {
        let sizes = [
            8111, 7511, 7311, 6911, 6311, 6111, 5511, 4911, 4511, 4311, 3911, 3311,
        ];
        Self {
            rate,
            seed,
            filters: [[0.0; 11]; 3],
            lines: std::array::from_fn(|_| {
                std::array::from_fn(|i| vec![0.0; sizes[i]].into_boxed_slice())
            }),
            counts: [1; 12],
            feedback: [[0.0; 8]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    pub(super) fn reset(&mut self) {
        self.filters = [[0.0; 11]; 3];
        self.counts = [1; 12];
        self.feedback = [[0.0; 8]; 2];
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
        for channel in &mut self.lines {
            for line in channel {
                line.fill(0.0);
            }
        }
    }
    #[allow(clippy::needless_range_loop)]
    pub(super) fn process(
        &mut self,
        il: &[f32],
        ir: &[f32],
        ol: &mut [f32],
        or: &mut [f32],
        p: [f64; 6],
        second: bool,
    ) {
        let frequency = (p[0].powf(2.0) * 9900.0 + 100.0) / self.rate as f64;
        for (filter, resonance) in self.filters.iter_mut().zip([
            0.618_033_988_749_894_9,
            p[0] * 0.5 + 0.118_033_988_749_894_85,
            0.5,
        ]) {
            filter[0] = frequency;
            filter[1] = resonance;
            let k = (std::f64::consts::PI * frequency).tan();
            let norm = 1.0 / (1.0 + k / resonance + k * k);
            filter[2] = k * k * norm;
            filter[3] = 2.0 * filter[2];
            filter[4] = filter[2];
            filter[5] = 2.0 * (k * k - 1.0) * norm;
            filter[6] = (1.0 - k / resonance + k * k) * norm;
        }
        let (size, damping, raw, feedback, wet) = if second {
            (
                p[1].powf(2.0) * 99.0 + 1.0,
                p[2].powf(2.0) * 0.5,
                p[3],
                1.0 - (1.0 - p[4]).powf(4.0),
                p[5],
            )
        } else {
            (
                p[2].powf(2.0) * 90.0 + 10.0,
                p[1].powf(2.0) * 0.5,
                1.0,
                1.0,
                p[3],
            )
        };
        let factors = [
            79.0, 73.0, 71.0, 67.0, 61.0, 59.0, 53.0, 47.0, 43.0, 41.0, 37.0, 31.0,
        ];
        let delays: [usize; 12] = std::array::from_fn(|i| (factors[i] * size) as usize);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut input = [l as f64, r as f64];
            for (ch, sample) in input.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            let dry = input;
            filter_stereo(&mut self.filters[0], &mut input);
            if !second {
                for sample in &mut input {
                    *sample *= wet;
                    *sample *= 0.5;
                }
            }
            let mut allpass = [[0.0; 4]; 2];
            for i in 0..4 {
                let line = i + 8;
                let next = if self.counts[line] + 1 > delays[line] {
                    0
                } else {
                    self.counts[line] + 1
                };
                for ch in 0..2 {
                    let mut value = input[ch] - self.lines[ch][line][next] * 0.5;
                    self.lines[ch][line][self.counts[line]] = value;
                    value *= 0.5;
                    allpass[ch][i] = value;
                }
                self.counts[line] += 1;
                if self.counts[line] > delays[line] {
                    self.counts[line] = 0;
                }
                for ch in 0..2 {
                    allpass[ch][i] += self.lines[ch][line][self.counts[line]];
                }
            }
            if second && raw != 1.0 {
                for ch in 0..2 {
                    for x in &mut allpass[ch] {
                        *x = (*x * raw) + (dry[ch] * (1.0 - raw));
                    }
                }
            }
            let order = if second {
                [0, 1, 2, 3, 0, 1, 2, 3]
            } else {
                [3, 2, 1, 0, 0, 1, 2, 3]
            };
            for i in 0..8 {
                for ch in 0..2 {
                    self.lines[ch][i][self.counts[i]] =
                        allpass[ch][order[i]] + self.feedback[ch][i] * feedback;
                }
                self.counts[i] += 1;
                if self.counts[i] > delays[i] {
                    self.counts[i] = 0;
                }
            }
            let mut values = [[0.0; 8]; 2];
            for ch in 0..2 {
                for i in 0..8 {
                    let current = self.lines[ch][i][self.counts[i]];
                    values[ch][i] = if second || i == 0 || i == 4 {
                        let next = if self.counts[i] + 1 > delays[i] {
                            0
                        } else {
                            self.counts[i] + 1
                        };
                        current * (1.0 - (damping - damping.floor()))
                            + self.lines[ch][i][next] * (damping - damping.floor())
                    } else {
                        current
                    };
                }
                let x = &values[ch];
                let mut back_a = 0.5;
                let mut back_e = 0.5;
                let mut back_dry = 0.5;
                if x[0].abs() > 0.4 {
                    back_a -= (x[0].abs() - 0.4) * 0.2;
                }
                if x[4].abs() > 0.4 {
                    back_e -= (x[4].abs() - 0.4) * 0.2;
                }
                if dry[ch].abs() > 0.4 {
                    back_dry -= (dry[ch].abs() - 0.4) * 0.2;
                }
                for base in [0, 4] {
                    for i in 0..4 {
                        let scale = if i == 0 {
                            if base == 0 {
                                back_a
                            } else {
                                back_e
                            }
                        } else {
                            back_dry
                        };
                        let others = [[1, 2, 3], [0, 2, 3], [0, 1, 3], [0, 1, 2]][i];
                        self.feedback[ch][base + i] = (x[base + i]
                            - (x[base + others[0]] + x[base + others[1]] + x[base + others[2]]))
                            * scale;
                    }
                }
                input[ch] = (x[0] + x[1] + x[2] + x[3] + x[4] + x[5] + x[6] + x[7]) / 8.0;
            }
            filter_stereo(&mut self.filters[1], &mut input);
            for x in &mut input {
                *x = x.clamp(-1.0, 1.0).asin();
            }
            filter_stereo(&mut self.filters[2], &mut input);
            if wet != 1.0 {
                for ch in 0..2 {
                    input[ch] = input[ch] * wet + dry[ch] * (1.0 - wet);
                }
            }
            *out_l = fpd_dither(input[0], &mut self.fpd[0]) as f32;
            *out_r = fpd_dither(input[1], &mut self.fpd[1]) as f32;
        }
    }
}
#[allow(clippy::needless_range_loop)]
fn filter_stereo(b: &mut [f64; 11], input: &mut [f64; 2]) {
    for ch in 0..2 {
        let i = 7 + ch * 2;
        let temp = input[ch] * b[2] + b[i];
        b[i] = input[ch] * b[3] - temp * b[5] + b[i + 1];
        b[i + 1] = input[ch] * b[4] - temp * b[6];
        input[ch] = temp;
    }
}
