//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/IronOxideClassic2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
#![allow(clippy::approx_constant, clippy::excessive_precision)]

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
        name: "Tape Speed",
        label: "ips",
        default: 0.562341325190349,
    },
    ParamInfo {
        name: "Output Trim",
        label: "dB",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "IronOxideClassic2",
    upstream_name: "IronOxideClassic2",
    description: "IronOxideClassic2 is my bandpassy tape sim, updated for high sample rate and aliasing control.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(IronOxideClassic2::new(sample_rate, seed))
}
struct IronOxideClassic2 {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    biquad: [[[f64; 4]; 2]; 2],
    delay: [[f64; 263]; 2],
    refs: [[f64; 6]; 2],
    iir: [[f64; 2]; 2],
    fast: [[f64; 2]; 2],
    slow: [[f64; 2]; 2],
    cycle: usize,
    gcount: usize,
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl IronOxideClassic2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.562341325190349, 0.5],
            sample_rate,
            seed,
            biquad: [[[0.0; 4]; 2]; 2],
            delay: [[0.0; 263]; 2],
            refs: [[0.0; 6]; 2],
            iir: [[0.0; 2]; 2],
            fast: [[0.0; 2]; 2],
            slow: [[0.0; 2]; 2],
            cycle: 0,
            gcount: 0,
            flip: true,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn filter(x: f64, s: &mut [f64; 4], c: [f64; 5]) -> f64 {
    let out = c[0] * x + c[1] * s[0] + c[2] * s[1] - c[3] * s[2] - c[4] * s[3];
    s[1] = s[0];
    s[0] = x;
    s[3] = s[2];
    s[2] = out;
    out
}
fn tape_sum(d: &[f64; 263], count: usize) -> f64 {
    let mut t = d[count + 127];
    for offset in [113, 109, 107, 103, 101, 97, 89, 83] {
        t += d[count + offset];
    }
    t /= 2.0;
    for offset in [79, 73, 71, 67, 61, 59, 53, 47, 43, 41, 37, 31, 29] {
        t += d[count + offset];
    }
    t /= 2.0;
    for offset in [23, 19, 17, 13, 11] {
        t += d[count + offset];
    }
    t /= 2.0;
    for offset in [7, 5, 3] {
        t += d[count + offset];
    }
    t /= 2.0;
    t += d[count + 2];
    t += d[count + 1];
    t
}
fn saturate(x: f64) -> f64 {
    let v = x.abs().min(1.57079633).sin();
    if x > 0.0 {
        v
    } else {
        -v
    }
}
impl AirwindowsEffect for IronOxideClassic2 {
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
            0 | 2 => float2string((self.params[i] as f64 * 36.0 - 18.0) as f32),
            1 => {
                let b = self.params[1];
                float2string((((b * b) * (b * b)) as f64 * 148.5 + 1.5) as f32)
            }
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c] = self.params;
        let scale = self.sample_rate as f64 / 44100.0;
        let end = (scale.floor() as usize).clamp(1, 4);
        self.cycle = self.cycle.min(end - 1);
        let ingain = 10.0f64.powf((a as f64 * 36.0 - 18.0) / 20.0);
        let outgain = 10.0f64.powf((c as f64 * 36.0 - 18.0) / 20.0);
        let mut ips = (((b * b) * (b * b)) as f64 * 148.5 + 1.5) * 1.1;
        if !(1.0..=200.0).contains(&ips) {
            ips = 33.0;
        }
        let iir = ips / 430.0 / scale;
        let fast = ips / 15.0 / scale * end as f64 + 1.0;
        let slow = 2.0 / (ips * ips) / scale * end as f64 + 1.0;
        let freq = 24000.0 / self.sample_rate as f64;
        let k = (std::f64::consts::PI * freq).tan();
        let mut coeff = [[0.0; 5]; 2];
        for (v, q) in coeff
            .iter_mut()
            .zip([1.618033988749894848204586, 0.618033988749894848204586])
        {
            let norm = 1.0 / (1.0 + k / q + k * k);
            let a0 = k * k * norm;
            *v = [
                a0,
                2.0 * a0,
                a0,
                2.0 * (k * k - 1.0) * norm,
                (1.0 - k / q + k * k) * norm,
            ];
        }
        for (((&left, &right), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut x = [left as f64, right as f64];
            for (ch, fpd) in [&mut self.fpd_l, &mut self.fpd_r].into_iter().enumerate() {
                if x[ch].abs() < 1.18e-23 {
                    x[ch] = *fpd as f64 * 1.18e-17;
                }
            }
            let phase = usize::from(!self.flip);
            for (ch, v) in x.iter_mut().enumerate() {
                let s = &mut self.iir[ch][phase];
                if s.abs() < 1.18e-37 {
                    *s = 0.0;
                }
                *s = *s * (1.0 - iir) + *v * iir;
                *v -= *s;
                if freq < 0.49999 {
                    *v = filter(*v, &mut self.biquad[0][ch], coeff[0]);
                }
                if ingain != 1.0 {
                    *v *= ingain;
                }
                *v = saturate(*v);
            }
            self.cycle += 1;
            if self.cycle == end {
                for (ch, v) in x.iter_mut().enumerate() {
                    self.delay[ch][self.gcount] = *v;
                    self.delay[ch][self.gcount + 131] = *v;
                    let f = &mut self.fast[ch][phase];
                    let s = &mut self.slow[ch][phase];
                    if f.abs() < 1.18e-37 {
                        *f = 0.0;
                    }
                    if s.abs() < 1.18e-37 {
                        *s = 0.0;
                    }
                    *f = *f / fast + self.delay[ch][self.gcount];
                    *s /= slow;
                    *s += tape_sum(&self.delay[ch], self.gcount + 3) / 128.0;
                    *v = *f - *s / slow;
                    let r = &mut self.refs[ch];
                    match end {
                        4 => {
                            r[0] = r[4];
                            r[2] = (r[0] + *v) / 2.0;
                            r[1] = (r[0] + r[2]) / 2.0;
                            r[3] = (r[2] + *v) / 2.0;
                            r[4] = *v;
                        }
                        3 => {
                            r[0] = r[3];
                            r[2] = (r[0] + r[0] + *v) / 3.0;
                            r[1] = (r[0] + *v + *v) / 3.0;
                            r[3] = *v;
                        }
                        2 => {
                            r[0] = r[2];
                            r[1] = (r[0] + *v) / 2.0;
                            r[2] = *v;
                        }
                        _ => r[0] = *v,
                    }
                }
                self.cycle = 0;
            }
            for (ch, v) in x.iter_mut().enumerate() {
                *v = saturate(self.refs[ch][self.cycle]);
                if freq < 0.49999 {
                    *v = filter(*v, &mut self.biquad[1][ch], coeff[1]);
                }
                if outgain != 1.0 {
                    *v *= outgain;
                }
            }
            self.flip = !self.flip;
            *outl = fpd_dither(x[0], &mut self.fpd_l) as f32;
            *outr = fpd_dither(x[1], &mut self.fpd_r) as f32;
        }
    }
}
