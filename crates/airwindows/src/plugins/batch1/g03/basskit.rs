//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Port of airwindows/plugins/LinuxVST/src/BassKit at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Drive",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Voicing",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "BassOut",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "SubOut",
        label: "",
        default: 0.5,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "BassKit",
    upstream_name: "BassKit",
    description: "BassKit is centered bass reinforcement with subs fill. Clean and controllable.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(BassKit::new(sample_rate, seed))
}
struct BassKit {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
    negative: bool,
    octave: bool,
    bflip: usize,
    drive: [f64; 6],
    iir: [f64; 26],
    head: [f64; 3],
    sub: [f64; 3],
    gate: f64,
}
impl BassKit {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5; 4],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            negative: false,
            octave: false,
            bflip: 0,
            drive: [0.0; 6],
            iir: [0.0; 26],
            head: [0.0; 3],
            sub: [0.0; 3],
            gate: 1.0,
        }
    }
}
impl AirwindowsEffect for BassKit {
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
        match self.params.get(i) {
            Some(&v) if i < 2 => float2string(v),
            Some(&v) => float2string((v as f64 * 2.0 - 1.0) as f32),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let overallscale = (1.0 / 44100.0) * self.sample_rate as f64;
        let bass_gain = self.params[0] as f64 * 0.1;
        let freq = (self.params[1] as f64 * 0.1 + 0.02) / overallscale;
        let amount = freq / 44.1;
        let bass = (self.params[2] as f64 * 2.0 - 1.0) * (self.params[2] as f64 * 2.0 - 1.0).abs();
        let sub =
            (self.params[3] as f64 * 2.0 - 1.0) * (self.params[3] as f64 * 2.0 - 1.0).abs() * 4.0;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let mut low = (l + r) / 2.0;
            for i in 0..2 {
                self.drive[i] = self.drive[i] * (1.0 - freq) + low * freq;
                low = self.drive[i];
            }
            self.gate += (low * 10.0).abs();
            self.gate -= 0.001;
            self.gate = self.gate.clamp(0.0, 1.0);
            let clamp = (1.0 - self.gate) * 0.00001;
            if low > 0.0 {
                if self.negative {
                    self.octave = !self.octave;
                }
                self.negative = false;
            } else {
                self.negative = true;
            }
            let mut randy = (self.fpd_l as f64 / u32::MAX as f64) * 0.111;
            let inv = 1.0 - randy;
            randy /= 2.0;
            for i in 0..22 {
                self.iir[i] = self.iir[i] * (1.0 - amount) + low * amount;
                low -= self.iir[i];
            }
            let mut head = 0.0;
            if (1..=3).contains(&self.bflip) {
                let i = self.bflip - 1;
                self.head[i] += low * bass_gain;
                self.head[i] -= self.head[i] * self.head[i] * self.head[i] * freq;
                self.head[i] = if i == 0 {
                    inv * self.head[0] + randy * self.head[1] + randy * self.head[2]
                } else if i == 1 {
                    randy * self.head[0] + inv * self.head[1] + randy * self.head[2]
                } else {
                    randy * self.head[0] + randy * self.head[1] + inv * self.head[2]
                };
                if self.head[i] > 0.0 {
                    self.head[i] -= clamp;
                }
                if self.head[i] < 0.0 {
                    self.head[i] += clamp;
                }
                head = self.head[i];
            }
            for i in 22..24 {
                self.iir[i] = self.iir[i] * (1.0 - amount) + head * amount;
                head -= self.iir[i];
            }
            let mut s = head;
            self.iir[24] = self.iir[24] * (1.0 - amount) + s * amount;
            s -= self.iir[24];
            for i in 2..4 {
                self.drive[i] = self.drive[i] * (1.0 - freq) + s * freq;
                s = self.drive[i];
            }
            s = s.abs();
            if !self.octave {
                s = -s;
            }
            if (1..=3).contains(&self.bflip) {
                let i = self.bflip - 1;
                self.sub[i] += s;
                self.sub[i] -= self.sub[i] * self.sub[i] * self.sub[i] * freq;
                self.sub[i] = if i == 0 {
                    inv * self.sub[0] + randy * self.sub[1] + randy * self.sub[2]
                } else if i == 1 {
                    randy * self.sub[0] + inv * self.sub[1] + randy * self.sub[2]
                } else {
                    randy * self.sub[0] + randy * self.sub[1] + inv * self.sub[2]
                };
                if self.sub[i] > 0.0 {
                    self.sub[i] -= clamp;
                }
                if self.sub[i] < 0.0 {
                    self.sub[i] += clamp;
                }
                s = self.sub[i];
            }
            self.iir[25] = self.iir[25] * (1.0 - freq) + s * freq;
            s = self.iir[25];
            for i in 4..6 {
                self.drive[i] = self.drive[i] * (1.0 - amount) + s * amount;
                s = self.drive[i];
            }
            l += head * bass;
            l += s * sub;
            r += head * bass;
            r += s * sub;
            self.bflip += 1;
            if self.bflip > 3 {
                self.bflip = 1;
            }
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
