//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Coils2/Coils2Proc.cpp, b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Coils2",
    upstream_name: "Coils2",
    description: "Coils2 is a transformer overdrive emulator.",
    params: &[
        ParamInfo {
            name: "Saturate",
            label: "",
            default: 0.0,
        },
        ParamInfo {
            name: "Cheapness",
            label: "",
            default: 0.0,
        },
        ParamInfo {
            name: "Dry/Wet",
            label: "",
            default: 1.0,
        },
    ],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Coils2::new(rate, seed))
}
struct Coils2 {
    rate: f32,
    seed: u32,
    params: [f32; 3],
    a: [f64; 15],
    b: [f64; 15],
    h: [f64; 2],
    fpd: [u32; 2],
}
impl Coils2 {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            rate,
            seed,
            params: [0.0, 0.0, 1.0],
            a: [0.0; 15],
            b: [0.0; 15],
            h: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Coils2 {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(v)) {
            *p = v
        }
    }
    fn param_display(&self, i: usize) -> String {
        self.params
            .get(i)
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let scale = 1.0 / 44100.0 * self.rate as f64;
        let dist = (1.0 - self.params[0] as f64).powf(2.0).max(0.0001);
        self.a[0] = 600.0 / self.rate as f64;
        self.a[1] = 0.01 + ((self.params[1] as f64).powf(2.0) * 0.5);
        let iir = self.a[1] / scale;
        let k = (std::f64::consts::PI * self.a[0]).tan();
        let norm = 1.0 / (1.0 + k / self.a[1] + k * k);
        self.a[2] = k / self.a[1] * norm;
        self.a[4] = -self.a[2];
        self.a[5] = 2.0 * (k * k - 1.0) * norm;
        self.a[6] = (1.0 - k / self.a[1] + k * k) * norm;
        self.b[0] = (21890.0 - self.a[1] * 890.0) / self.rate as f64;
        self.b[1] = 0.89;
        let k = (std::f64::consts::PI * self.b[0]).tan();
        let norm = 1.0 / (1.0 + k / self.b[1] + k * k);
        self.b[2] = k * k * norm;
        self.b[3] = 2.0 * self.b[2];
        self.b[4] = self.b[2];
        self.b[5] = 2.0 * (k * k - 1.0) * norm;
        self.b[6] = (1.0 - k / self.b[1] + k * k) * norm;
        let wet = self.params[2] as f64;
        for (((&left, &right), out_l), out_r) in l.iter().zip(r).zip(ol).zip(orr) {
            let mut out = [0.0; 2];
            for (ch, raw) in [left, right].into_iter().enumerate() {
                let mut x = raw as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[ch] as f64 * 1.18e-17
                }
                let dry = x;
                let idx = 7 + ch * 2;
                if self.a[0] < 0.49999 {
                    let t = x * self.a[2] + self.a[idx];
                    self.a[idx] = -(t * self.a[5]) + self.a[idx + 1];
                    self.a[idx + 1] = x * self.a[4] - t * self.a[6];
                    x = t;
                }
                let mut diff = (dry - x) / dist;
                if self.b[0] < 0.49999 {
                    let t = diff * self.b[2] + self.b[idx];
                    self.b[idx] = diff * self.b[3] - t * self.b[5] + self.b[idx + 1];
                    self.b[idx + 1] = diff * self.b[4] - t * self.b[6];
                    diff = t;
                }
                self.h[ch] = self.h[ch] * (1.0 - iir) + diff * iir;
                if self.h[ch].abs() < 1.18e-37 {
                    self.h[ch] = 0.0
                } else {
                    diff -= self.h[ch]
                }
                diff = diff.clamp(-1.571, 1.571);
                self.h[ch] = self.h[ch].clamp(-1.571, 1.571);
                x += (diff.sin() - self.h[ch].sin()) * dist;
                if wet != 1.0 {
                    x = x * wet + dry * (1.0 - wet)
                }
                out[ch] = fpd_dither(x, &mut self.fpd[ch]) as f32;
            }
            *out_l = out[0];
            *out_r = out[1];
        }
    }
}
