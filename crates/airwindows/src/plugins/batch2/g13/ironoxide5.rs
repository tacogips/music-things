//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/IronOxide5, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
#![allow(clippy::approx_constant)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Input Trim",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Tape High",
        label: "ips",
        default: 0.562_341_33,
    },
    ParamInfo {
        name: "Tape Low",
        label: "ips",
        default: 0.562_341_33,
    },
    ParamInfo {
        name: "Flutter",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Noise",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output Trim",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Inv/Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "IronOxide5",
    upstream_name: "IronOxide5",
    description: "IronOxide5 is the old school, heavily colored Airwindows tape emulation.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(IronOxide5::new(sample_rate, seed))
}
struct IronOxide5 {
    params: [f32; 7],
    sample_rate: f32,
    seed: u32,
    input_iir: [[f64; 2]; 2],
    fast: [[f64; 2]; 2],
    slow: [[f64; 2]; 2],
    delay: [[f64; 264]; 2],
    flutter: [[f64; 99]; 2],
    flcount: i32,
    sweep: f64,
    rate: f64,
    next: f64,
    prev: [f64; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl IronOxide5 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.562_341_33, 0.562_341_33, 0.5, 0.5, 0.5, 1.0],
            sample_rate,
            seed,
            input_iir: [[0.0; 2]; 2],
            fast: [[0.0; 2]; 2],
            slow: [[0.0; 2]; 2],
            delay: [[0.0; 264]; 2],
            flutter: [[0.0; 99]; 2],
            flcount: 0,
            sweep: 0.0,
            rate: 0.5,
            next: 0.5,
            prev: [0.0; 2],
            flip: false,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn taper(d: &[f64; 264]) -> f64 {
    let count = 3;
    let mut t = d[count + 127];
    for i in [113, 109, 107, 103, 101, 97, 89, 83] {
        t += d[count + i];
    }
    t /= 2.0;
    for i in [79, 73, 71, 67, 61, 59, 53, 47, 43, 41, 37, 31, 29] {
        t += d[count + i];
    }
    t /= 2.0;
    for i in [23, 19, 17, 13, 11] {
        t += d[count + i];
    }
    t /= 2.0;
    for i in [7, 5, 3] {
        t += d[count + i];
    }
    t /= 2.0;
    t += d[count + 2];
    t += d[count + 1];
    t
}
fn speed(x: f32) -> f64 {
    let v = (((x * x) * (x * x)) as f64 * 148.5 + 1.5) * 1.1;
    if !(1.0..=200.0).contains(&v) {
        33.0
    } else {
        v
    }
}
impl AirwindowsEffect for IronOxide5 {
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
        match i {
            0 | 5 => float2string((self.params[i] as f64 * 36.0 - 18.0) as f32),
            1 | 2 => {
                let x = self.params[i];
                float2string((((x * x) * (x * x)) as f64 * 148.5 + 1.5) as f32)
            }
            3 | 4 => float2string(self.params[i]),
            6 => float2string((self.params[i] as f64 * 2.0 - 1.0) as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d, e, f, g] = self.params;
        let scale = self.sample_rate as f64 / 44100.0;
        let ingain = 10.0f64.powf((a as f64 * 36.0 - 18.0) / 20.0);
        let outgain = 10.0f64.powf((f as f64 * 36.0 - 18.0) / 20.0);
        let ips = speed(b);
        let lps = speed(c);
        let temp_randy = 0.04 + 0.11 / ips.sqrt();
        let mut iir = lps / 430.0;
        let mut fast = ips / 15.0;
        let mut slow = 2.0 / (lps * lps);
        let mut low = 5.0 / ips;
        let depth = (d as f64).powf(2.0) * scale;
        let fluttertrim = 0.00581 / scale;
        let sweeptrim = (0.0005 * depth) / scale;
        let newrate = 0.006 / scale;
        let oldrate = 1.0 - newrate;
        if scale == 0.0 {
            fast += 1.0;
            slow += 1.0;
        } else {
            iir /= scale;
            low *= scale;
            fast = 1.0 + fast / scale;
            slow = 1.0 + slow / scale;
        }
        let noise = e as f64 * 0.5;
        let wet = g as f64 * 2.0 - 1.0;
        let drygain = if wet > 0.0 { 1.0 - wet } else { 1.0 };
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut dry = [l as f64, r as f64];
            for (ch, x) in dry.iter_mut().enumerate() {
                if x.abs() < 1.18e-23 {
                    *x = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            if self.flcount < 0 || self.flcount > 30 {
                self.flcount = 30;
            }
            let count = self.flcount as usize;
            for (ch, x) in dry.iter().enumerate() {
                self.flutter[ch][count] = *x;
                self.flutter[ch][count + 31] = *x;
            }
            let offset = (1.0 + self.sweep.sin()) * depth;
            let frac = offset - offset.floor();
            let index = count + offset.floor() as usize;
            let mut output = [0.0; 2];
            for (ch, x) in output.iter_mut().enumerate() {
                *x = self.flutter[ch][index] * (1.0 - frac);
                *x += self.flutter[ch][index + 1] * frac;
            }
            self.rate = self.next * newrate + self.rate * oldrate;
            self.sweep += self.rate * fluttertrim;
            self.sweep += self.sweep * sweeptrim;
            if self.sweep >= std::f64::consts::PI * 2.0 {
                self.sweep = 0.0;
                self.next = 0.02 + self.fpd[0] as f64 / u32::MAX as f64 * 0.98;
            }
            self.flcount -= 1;
            let side = if self.flip { 0 } else { 1 };
            let randy =
                (0.55 + temp_randy + (self.fpd[1] as f64 / u32::MAX as f64 * temp_randy)) * noise;
            for (ch, x) in output.iter_mut().enumerate() {
                self.input_iir[ch][side] = self.input_iir[ch][side] * (1.0 - iir) + *x * iir;
                *x -= self.input_iir[ch][side];
                *x *= ingain;
                let rect = x.abs().min(1.57079633).sin();
                *x = if *x > 0.0 { rect } else { -rect };
                self.delay[ch][0] = *x;
                self.delay[ch][131] = *x;
                self.fast[ch][side] /= fast;
                self.slow[ch][side] /= slow;
                self.fast[ch][side] += self.delay[ch][0];
                self.slow[ch][side] += taper(&self.delay[ch]) / 128.0;
                *x = self.fast[ch][side] - self.slow[ch][side] / slow;
                *x /= fast;
                *x /= low;
                let rect = x.abs().min(1.57079633).sin();
                *x = if *x > 0.0 { rect } else { -rect };
                *x *= 1.0 - randy;
                *x += self.prev[ch] * randy;
                self.prev[ch] = dry[ch];
                if outgain != 1.0 {
                    *x *= outgain;
                }
                if wet != 1.0 {
                    *x *= wet;
                }
                if drygain != 1.0 {
                    dry[ch] *= drygain;
                }
                if dry[ch].abs() > 0.0 {
                    *x += dry[ch];
                }
                *x = fpd_dither(*x, &mut self.fpd[ch]);
            }
            self.flip = !self.flip;
            *outl = output[0] as f32;
            *outr = output[1] as f32;
        }
    }
}
