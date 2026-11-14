//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Preponderant, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Narrow",
        label: "",
        default: 0.25,
    },
    ParamInfo {
        name: "Medium",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Wide",
        label: "",
        default: 0.75,
    },
    ParamInfo {
        name: "Resonan",
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
    name: "Preponderant",
    upstream_name: "Preponderant",
    description: "Preponderant is Airwindows Anti-Soothe, a strange tone intensifier and balancer.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Preponderant::new(sample_rate, seed))
}
struct Preponderant {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    biquad: [[f64; 15]; 3],
    balance: [[f64; 3]; 2],
    track: [[f64; 3]; 2],
    quickness: [f64; 2],
    fpd: [u32; 2],
}
impl Preponderant {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.25, 0.5, 0.75, 0.5, 1.0],
            sample_rate,
            seed,
            biquad: [[0.0; 15]; 3],
            balance: [[1.0; 3]; 2],
            track: [[0.0; 3]; 2],
            quickness: [0.0001; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Preponderant {
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
        self.biquad = [[0.0; 15]; 3];
        self.balance = [[1.0; 3]; 2];
        self.track = [[0.0; 3]; 2];
        self.quickness = [0.0001; 2];
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    #[allow(
        clippy::excessive_precision,
        clippy::manual_clamp,
        clippy::needless_range_loop
    )]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let chase = 0.00005 / scale;
        let resonance = (self.params[3] as f64).powf(3.0) * 8.0 + 0.33;
        let compensation = resonance.sqrt();
        for band in 0..3 {
            let b = &mut self.biquad[band];
            b[0] = (self.params[band] as f64).powf(3.0 * scale.cbrt()) * 0.42;
            if b[0] < 0.0001 {
                b[0] = 0.0001;
            }
            b[1] = resonance;
            if band == 1 {
                b[1] /= 2.0;
            }
            if band == 2 {
                b[1] /= 4.0;
            }
            let k = (std::f64::consts::PI * b[0]).tan();
            let norm = 1.0 / (1.0 + k / b[1] + k * k);
            b[2] = k / b[1] * norm;
            b[3] = 0.0;
            b[4] = -b[2];
            b[5] = 2.0 * (k * k - 1.0) * norm;
            b[6] = (1.0 - k / b[1] + k * k) * norm;
        }
        let wet = self.params[4] as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut outputs = [0.0f32; 2];
            for (ch, sample) in [l, r].into_iter().enumerate() {
                let mut input = sample as f64;
                if input.abs() < 1.18e-23 {
                    input = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = sample as f64;
                let mut bands = [0.0; 3];
                for (band, value) in bands.iter_mut().enumerate() {
                    let b = &mut self.biquad[band];
                    let i = 7 + ch * 2;
                    let temp = (input * b[2]) + b[i];
                    b[i] = -(temp * b[5]) + b[i + 1];
                    b[i + 1] = (input * b[4]) - (temp * b[6]);
                    *value = temp;
                }
                for band in 0..3 {
                    if self.balance[ch][band] < 0.00125 {
                        self.balance[ch][band] = 0.00125;
                    }
                    if self.balance[ch][band] > 1.0 {
                        self.balance[ch][band] = 1.0;
                    }
                    bands[band] *= self.balance[ch][band];
                    if self.track[ch][band] < bands[band].abs() {
                        self.track[ch][band] = bands[band].abs();
                    }
                    if self.track[ch][band] > self.quickness[ch] {
                        self.track[ch][band] -= self.quickness[ch];
                    }
                }
                let mut disparity = 0.0;
                for (a, b) in [(0, 1), (0, 2), (1, 2)] {
                    let diff = (self.track[ch][a] - self.track[ch][b]).abs();
                    if disparity < diff {
                        disparity = diff;
                    }
                }
                self.quickness[ch] *= 1.0 - chase;
                self.quickness[ch] += (1.0 + disparity) * 0.0001 * chase;
                for (a, b) in [(0, 1), (1, 2), (2, 0)] {
                    if self.track[ch][a] > self.track[ch][b] {
                        self.balance[ch][a] -= self.quickness[ch];
                        self.balance[ch][b] += self.quickness[ch];
                    } else {
                        self.balance[ch][a] += self.quickness[ch];
                        self.balance[ch][b] -= self.quickness[ch];
                    }
                }
                for band in [2, 1, 0] {
                    if self.balance[ch][band] > 0.618033988749894848204586 {
                        self.balance[ch][band] -= self.quickness[ch] * 0.001;
                    } else {
                        self.balance[ch][band] += self.quickness[ch] * 0.001;
                    }
                }
                input = (bands[0] + bands[1] + bands[2]) * compensation;
                if wet != 1.0 {
                    input = (input * wet) + (dry * (1.0 - wet));
                }
                outputs[ch] = fpd_dither(input, &mut self.fpd[ch]) as f32;
            }
            *out_l = outputs[0];
            *out_r = outputs[1];
        }
    }
}
