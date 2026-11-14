//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Thunder, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::{db2string, float2string};
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Thunder",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Output Trim",
        label: "dB",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Thunder",
    upstream_name: "Thunder",
    description:
        "Thunder is a compressor that retains or exaggerates subsonic bass when you push it.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Thunder::new(sample_rate, seed))
}
struct Thunder {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
    speed: f64,
    coefficient: f64,
    gate: [f64; 2],
    a: [f64; 3],
    b: [f64; 3],
    cm: f64,
}
impl Thunder {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 1.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            speed: 10000.0,
            coefficient: 1.0,
            gate: [0.0; 2],
            a: [0.0; 3],
            b: [0.0; 3],
            cm: 0.0,
        }
    }
}
impl AirwindowsEffect for Thunder {
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
            0 => float2string(self.params[0]),
            1 => db2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn param_label(&self, i: usize) -> &'static str {
        match i {
            0 => " ",
            1 => "dB",
            _ => "",
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let overallscale = (1.0 / 44100.0) * self.sample_rate as f64;
        let thunder = self.params[0] as f64 * 0.4;
        let threshold = (1.0 - (thunder * 2.0)).max(0.01);
        let makeup = 1.0 / threshold;
        let mut release = (1.28 - thunder).powf(5.0) * 32768.0;
        release /= overallscale;
        let fastest = release.sqrt();
        let eq = (0.0275 / self.sample_rate as f64) * 32000.0;
        let dcblock = eq / 300.0;
        let basstrim = (0.01 / eq) + 1.0;
        let output = self.params[1] as f64;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            if input[0].abs() < 1.18e-23 {
                input[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if input[1].abs() < 1.18e-23 {
                input[1] = self.fpd_r as f64 * 1.18e-17;
            }
            input[0] *= makeup;
            input[1] *= makeup;
            for (i, &sample) in input.iter().enumerate() {
                if self.gate[i] < sample.abs() {
                    self.gate[i] = sample;
                } else {
                    self.gate[i] -= dcblock;
                }
                self.a[i] += sample * eq * thunder;
                self.a[i] -= self.a[i] * self.a[i] * self.a[i] * eq;
                if self.a[i] > self.gate[i] {
                    self.a[i] -= dcblock;
                }
                if self.a[i] < -self.gate[i] {
                    self.a[i] += dcblock;
                }
                let result = self.a[i] * basstrim;
                self.b[i] = (self.b[i] * (1.0 - eq)) + (result * eq);
            }
            self.a[2] += (input[0] + input[1]) * eq * thunder;
            self.a[2] -= self.a[2] * self.a[2] * self.a[2] * eq;
            let result = self.a[2] * basstrim;
            self.b[2] = (self.b[2] * (1.0 - eq)) + (result * eq);
            self.cm = (self.cm * (1.0 - eq)) + (self.b[2] * eq);
            let result_m = self.cm.abs();
            input[0] -= self.b[0] * thunder;
            input[1] -= self.b[1] * thunder;
            let sense = input[0].abs().max(input[1].abs());
            if sense > threshold {
                let vary = threshold / sense;
                let attack = self.speed.abs().sqrt();
                self.coefficient *= attack - 1.0;
                self.coefficient += if vary < threshold { threshold } else { vary };
                self.coefficient /= attack;
            } else {
                self.coefficient *= (self.speed * self.speed) - 1.0;
                self.coefficient += 1.0;
                self.coefficient /= self.speed * self.speed;
            }
            let mut new_speed = self.speed * (self.speed - 1.0);
            new_speed = new_speed + (sense * release).abs() + fastest;
            self.speed = new_speed / self.speed;
            let coefficient = self.coefficient.powf(2.0);
            input[0] *= coefficient;
            input[1] *= coefficient;
            input[0] += self.b[0] * result_m;
            input[1] += self.b[1] * result_m;
            if output != 1.0 {
                input[0] *= output;
                input[1] *= output;
            }
            *outl = fpd_dither(input[0], &mut self.fpd_l) as f32;
            *outr = fpd_dither(input[1], &mut self.fpd_r) as f32;
        }
    }
}
