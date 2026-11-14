//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Verbity, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Bigness",
        label: "",
        default: 0.25,
    },
    ParamInfo {
        name: "Longness",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Darkness",
        label: "",
        default: 0.25,
    },
    ParamInfo {
        name: "Wetness",
        label: "",
        default: 0.25,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Verbity",
    upstream_name: "Verbity",
    description: "Verbity is a dual-mono reverb, which uses feedforward reverb topology.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Verbity::new(sample_rate, seed))
}
struct Verbity {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    delays: [[Box<[f64]>; 12]; 2],
    counts: [usize; 12],
    iir: [[f64; 2]; 2],
    feedback: [[f64; 4]; 2],
    previous: [[f64; 4]; 2],
    last: [[f64; 7]; 2],
    thunder: [f64; 2],
    cycle: usize,
    fpd: [u32; 2],
}
impl Verbity {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        let lengths = [
            6480, 3660, 1720, 680, 9700, 6000, 2320, 940, 15220, 8460, 4540, 3200,
        ];
        Self {
            params: [0.25, 0.0, 0.25, 0.25],
            sample_rate,
            seed,
            delays: std::array::from_fn(|_| {
                std::array::from_fn(|i| vec![0.0; lengths[i]].into_boxed_slice())
            }),
            counts: [1; 12],
            iir: [[0.0; 2]; 2],
            feedback: [[0.0; 4]; 2],
            previous: [[0.0; 4]; 2],
            last: [[0.0; 7]; 2],
            thunder: [0.0; 2],
            cycle: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn mix(v: [f64; 4]) -> [f64; 4] {
    [
        v[0] - (v[1] + v[2] + v[3]),
        v[1] - (v[0] + v[2] + v[3]),
        v[2] - (v[0] + v[1] + v[3]),
        v[3] - (v[0] + v[1] + v[2]),
    ]
}
impl AirwindowsEffect for Verbity {
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
        for channel in &mut self.delays {
            for delay in channel {
                delay.fill(0.0);
            }
        }
        self.counts = [1; 12];
        self.iir = [[0.0; 2]; 2];
        self.feedback = [[0.0; 4]; 2];
        self.previous = [[0.0; 4]; 2];
        self.last = [[0.0; 7]; 2];
        self.thunder = [0.0; 2];
        self.cycle = 0;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    #[allow(clippy::excessive_precision, clippy::needless_range_loop)] // Upstream literals and channel order.
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let mut overall = 1.0;
        overall /= 44100.0;
        overall *= self.sample_rate as f64;
        let cycle_end = (overall.floor() as usize).clamp(1, 4);
        if self.cycle > cycle_end - 1 {
            self.cycle = cycle_end - 1;
        }
        let size = a as f64 * 1.77 + 0.1;
        let regen = 0.0625 + b as f64 * 0.03125;
        let lowpass = (1.0 - (c as f64).powf(2.0)) / overall.sqrt();
        let interpolate = (c as f64).powf(2.0) * 0.618033988749894848204586;
        let thunder = (0.3 - b as f64 * 0.22) * c as f64 * 0.1;
        let wet = (d as f64 * 2.0).clamp(0.0, 1.0);
        let dry = (2.0 - d as f64 * 2.0).clamp(0.0, 1.0);
        let sizes = [
            3407.0, 1823.0, 859.0, 331.0, 4801.0, 2909.0, 1153.0, 461.0, 7607.0, 4217.0, 2269.0,
            1597.0,
        ];
        let lengths = sizes.map(|n| (n * size) as usize);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            for ch in 0..2 {
                if input[ch].abs() < 1.18e-23 {
                    input[ch] = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            let dry_sample = input;
            for (ch, sample) in input.iter_mut().enumerate() {
                if self.iir[ch][0].abs() < 1.18e-37 {
                    self.iir[ch][0] = 0.0;
                }
                self.iir[ch][0] = self.iir[ch][0] * (1.0 - lowpass) + *sample * lowpass;
                *sample = self.iir[ch][0];
            }
            self.cycle += 1;
            if self.cycle == cycle_end {
                for ch in 0..2 {
                    for i in 0..4 {
                        self.feedback[ch][i] = self.feedback[ch][i] * (1.0 - interpolate)
                            + self.previous[ch][i] * interpolate;
                        self.previous[ch][i] = self.feedback[ch][i];
                    }
                    self.thunder[ch] = self.thunder[ch] * 0.99 - self.feedback[ch][0] * thunder;
                }
                let mut stage = [[0.0; 4]; 2];
                for ch in 0..2 {
                    for i in 0..4 {
                        let fb = if i == 0 {
                            self.feedback[ch][i] + self.thunder[ch]
                        } else {
                            self.feedback[ch][i]
                        };
                        self.delays[ch][i][self.counts[i]] = input[ch] + fb * regen;
                    }
                }
                for i in 0..4 {
                    self.counts[i] += 1;
                    if self.counts[i] > lengths[i] {
                        self.counts[i] = 0;
                    }
                    for (ch, channel) in stage.iter_mut().enumerate() {
                        channel[i] = self.delays[ch][i][self.counts[i]];
                    }
                }
                for level in 1..3 {
                    let mixed = [mix(stage[0]), mix(stage[1])];
                    for ch in 0..2 {
                        for i in 0..4 {
                            let idx = level * 4 + i;
                            self.delays[ch][idx][self.counts[idx]] = mixed[ch][i];
                        }
                    }
                    for i in 0..4 {
                        let idx = level * 4 + i;
                        self.counts[idx] += 1;
                        if self.counts[idx] > lengths[idx] {
                            self.counts[idx] = 0;
                        }
                        for (ch, channel) in stage.iter_mut().enumerate() {
                            channel[i] = self.delays[ch][idx][self.counts[idx]];
                        }
                    }
                }
                for ch in 0..2 {
                    self.feedback[ch] = mix(stage[ch]);
                    let value = (stage[ch][0] + stage[ch][1] + stage[ch][2] + stage[ch][3]) / 8.0;
                    let refs = &mut self.last[ch];
                    match cycle_end {
                        4 => {
                            refs[0] = refs[4];
                            refs[2] = (refs[0] + value) / 2.0;
                            refs[1] = (refs[0] + refs[2]) / 2.0;
                            refs[3] = (refs[2] + value) / 2.0;
                            refs[4] = value;
                        }
                        3 => {
                            refs[0] = refs[3];
                            refs[2] = (refs[0] + refs[0] + value) / 3.0;
                            refs[1] = (refs[0] + value + value) / 3.0;
                            refs[3] = value;
                        }
                        2 => {
                            refs[0] = refs[2];
                            refs[1] = (refs[0] + value) / 2.0;
                            refs[2] = value;
                        }
                        _ => refs[0] = value,
                    }
                }
                self.cycle = 0;
            }
            for (ch, sample) in input.iter_mut().enumerate() {
                *sample = self.last[ch][self.cycle];
                if self.iir[ch][1].abs() < 1.18e-37 {
                    self.iir[ch][1] = 0.0;
                }
                self.iir[ch][1] = self.iir[ch][1] * (1.0 - lowpass) + *sample * lowpass;
                *sample = self.iir[ch][1];
                if wet < 1.0 {
                    *sample *= wet;
                }
                let mut ds = dry_sample[ch];
                if dry < 1.0 {
                    ds *= dry;
                }
                *sample += ds;
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            *out_l = input[0] as f32;
            *out_r = input[1] as f32;
        }
    }
}
