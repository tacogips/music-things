//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Biquad2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Type",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Q",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Inv/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Biquad2",
    upstream_name: "Biquad2",
    description: "Biquad2 is the Airwindows biquad filter that's more sweepable and synthy.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Biquad2::new(sample_rate, seed))
}
struct Biquad2 {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    biquad: [f64; 15],
    bl: [f64; 11],
    br: [f64; 11],
    f: [f64; 11],
    frequency: f64,
    resonance: f64,
    output: f64,
    wet: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl Biquad2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 0.5, 0.5, 1.0, 1.0],
            sample_rate,
            seed,
            biquad: [0.0; 15],
            bl: [0.0; 11],
            br: [0.0; 11],
            f: [0.0; 11],
            frequency: 0.0015,
            resonance: 0.001,
            output: 1.0,
            wet: 1.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Biquad2 {
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
        let [a, b, c, d, e] = self.params;
        let v = match i {
            0 => (a as f64 * 3.999 + 0.00001).ceil() as f32,
            1 => ((b * b) as f64 * 0.9999 + 0.0001) as f32,
            2 => ((c * c) as f64 * 49.99 + 0.01) as f32,
            3 => d,
            4 => (e as f64 * 2.0 - 1.0) as f32,
            _ => return String::new(),
        };
        float2string(v)
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let scale = 1.0 / 44100.0 * self.sample_rate as f64;
        let typ = (a as f64 * 3.999 + 0.00001).ceil() as i32;
        let mut average = (b * b) as f64;
        let mut ft = average * 0.39 / scale;
        if ft < 0.0015 / scale {
            ft = 0.0015 / scale;
        }
        let mut rt = (c * c) as f64 * 49.99 + 0.01;
        if rt < 1.0 {
            rt = 1.0;
        }
        let mut ot = d as f64;
        if typ < 3 {
            ot /= rt.sqrt();
        }
        let wt = e as f64 * 2.0 - 1.0;
        let q = &mut self.biquad;
        let mut k = (std::f64::consts::PI * q[0]).tan();
        let mut norm = 1.0 / (1.0 + k / q[1] + k * k);
        average = (1.0 - average) * 10.0;
        if typ == 1 || typ == 3 {
            average = 1.0;
        }
        let mut gain = average;
        for f in &mut self.f[..10] {
            if gain > 1.0 {
                *f = 1.0;
                gain -= 1.0;
            } else {
                *f = gain;
                gain = 0.0;
            }
        }
        if average < 1.0 {
            average = 1.0;
        }
        for f in &mut self.f[..10] {
            *f /= average;
        }
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dryl = l;
            let dryr = r;
            let mut speed = 50000.0;
            if self.frequency < ft {
                speed = 500000.0;
            }
            speed /= self.resonance;
            speed *= scale;
            self.frequency = (self.frequency * speed + ft) / (speed + 1.0);
            let fast = 1000.0 * scale;
            self.resonance = (self.resonance * fast + rt) / (fast + 1.0);
            self.output = (self.output * fast + ot) / (fast + 1.0);
            self.wet = (self.wet * fast + wt) / (fast + 1.0);
            if q[0] != self.frequency {
                q[0] = self.frequency;
                k = (std::f64::consts::PI * q[0]).tan();
            }
            if q[1] != self.resonance {
                q[1] = self.resonance;
                norm = 1.0 / (1.0 + k / q[1] + k * k);
            }
            match typ {
                1 => {
                    q[2] = k * k * norm;
                    q[3] = 2.0 * q[2];
                    q[4] = q[2];
                    q[5] = 2.0 * (k * k - 1.0) * norm;
                }
                2 => {
                    q[2] = norm;
                    q[3] = -2.0 * q[2];
                    q[4] = q[2];
                    q[5] = 2.0 * (k * k - 1.0) * norm;
                }
                3 => {
                    q[2] = k / q[1] * norm;
                    q[3] = 0.0;
                    q[4] = -q[2];
                    q[5] = 2.0 * (k * k - 1.0) * norm;
                }
                4 => {
                    q[2] = (1.0 + k * k) * norm;
                    q[3] = 2.0 * (k * k - 1.0) * norm;
                    q[4] = q[2];
                    q[5] = q[3];
                }
                _ => {}
            }
            q[6] = (1.0 - k / q[1] + k * k) * norm;
            l = l.sin();
            r = r.sin();
            let tl = q[2] * l + q[3] * q[7] + q[4] * q[8] - q[5] * q[9] - q[6] * q[10];
            q[8] = q[7];
            q[7] = l;
            l = tl;
            q[10] = q[9];
            q[9] = l;
            let tr = q[2] * r + q[3] * q[11] + q[4] * q[12] - q[5] * q[13] - q[6] * q[14];
            q[12] = q[11];
            q[11] = r;
            r = tr;
            q[14] = q[13];
            q[13] = r;
            l = l.clamp(-1.0, 1.0);
            r = r.clamp(-1.0, 1.0);
            self.bl.copy_within(0..9, 1);
            self.br.copy_within(0..9, 1);
            self.bl[0] = l;
            self.br[0] = r;
            l *= self.f[0];
            r *= self.f[0];
            for i in 1..10 {
                l += self.bl[i] * self.f[i];
                r += self.br[i] * self.f[i];
            }
            l = l.clamp(-1.0, 1.0).asin().clamp(-1.0, 1.0);
            r = r.clamp(-1.0, 1.0).asin().clamp(-1.0, 1.0);
            if self.output < 1.0 {
                l *= self.output;
                r *= self.output;
            }
            if self.wet < 1.0 {
                l = l * self.wet + dryl * (1.0 - self.wet.abs());
                r = r * self.wet + dryr * (1.0 - self.wet.abs());
            }
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
