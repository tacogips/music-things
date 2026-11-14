//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/LilAmp, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
#![allow(
    non_snake_case,
    unused_parens,
    clippy::approx_constant,
    clippy::assign_op_pattern,
    clippy::manual_clamp,
    clippy::excessive_precision
)]
mod cabinet;
use crate::display::float2string;
use crate::dither::fpd_from_seed;
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Gain",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Tone",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 0.8,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "LilAmp",
    upstream_name: "LilAmp",
    description:
        "LilAmp is a tiny amp sim, like a little bitty practice amp without that much gain.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(LilAmp::new(sample_rate, seed))
}
struct LilAmp {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    last_sample: [f64; 2],
    store_sample: [f64; 2],
    iir: [[f64; 10]; 2],
    odd: [[f64; 4]; 2],
    even: [[f64; 4]; 2],
    flip: bool,
    cab: [[f64; 90]; 2],
    last_cab: [f64; 2],
    smooth_cab_a: [f64; 2],
    smooth_cab_b: [f64; 2],
    last_ref: [[f64; 10]; 2],
    cycle: usize,
    fix: [[f64; 11]; 6],
    fpd_l: u32,
    fpd_r: u32,
}
impl LilAmp {
    // Upstream processReplacing's six fixed ultrasonic low-pass sections.
    fn setup_fixed_filters(&mut self) {
        let cutoff = ((15000.0 + self.params[1] as f64 * 10000.0) / self.sample_rate as f64)
            .clamp(0.001, 0.49);
        for (filter, resonance) in self.fix.iter_mut().zip([
            4.46570214, 1.51387132, 0.93979296, 0.70710678, 0.52972649, 0.50316379,
        ]) {
            filter[0] = cutoff;
            filter[1] = resonance;
            let k = (std::f64::consts::PI * filter[0]).tan();
            let norm = 1.0 / (1.0 + k / filter[1] + k * k);
            filter[2] = k * k * norm;
            filter[3] = 2.0 * filter[2];
            filter[4] = filter[2];
            filter[5] = 2.0 * (k * k - 1.0) * norm;
            filter[6] = (1.0 - k / filter[1] + k * k) * norm;
        }
    }
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.5, 0.8, 1.0],
            sample_rate,
            seed,
            last_sample: [0.0; 2],
            store_sample: [0.0; 2],
            iir: [[0.0; 10]; 2],
            odd: [[0.0; 4]; 2],
            even: [[0.0; 4]; 2],
            flip: false,
            cab: [[0.0; 90]; 2],
            last_cab: [0.0; 2],
            smooth_cab_a: [0.0; 2],
            smooth_cab_b: [0.0; 2],
            last_ref: [[0.0; 10]; 2],
            cycle: 0,
            fix: [[0.0; 11]; 6],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for LilAmp {
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
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        self.setup_fixed_filters();
        let inputlevel = self.params[0] as f64 * 6.0;
        let basstrim = self.params[1] as f64;
        let outputlevel = self.params[2] as f64;
        let wet = self.params[3] as f64;
        let eq = ((self.params[1] / self.sample_rate) * 22050.0) as f64;
        let skewlevel = basstrim.powf(2.0) * outputlevel;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let cycle_end = (overallscale.floor() as usize).clamp(1, 4);
        if self.cycle > cycle_end - 1 {
            self.cycle = cycle_end - 1;
        }
        for (((&left, &right), out_l), out_r) in
            il.iter().zip(ir).zip(ol.iter_mut()).zip(orr.iter_mut())
        {
            let mut inputSampleL = left as f64;
            let mut inputSampleR = right as f64;
            if inputSampleL.abs() < 1.18e-23 {
                inputSampleL = self.fpd_l as f64 * 1.18e-17;
            }
            if inputSampleR.abs() < 1.18e-23 {
                inputSampleR = self.fpd_r as f64 * 1.18e-17;
            }
            let drySampleL = inputSampleL;
            let drySampleR = inputSampleR;
            let mut outSample = (inputSampleL * self.fix[0][2]) + self.fix[0][7];
            self.fix[0][7] =
                (inputSampleL * self.fix[0][3]) - (outSample * self.fix[0][5]) + self.fix[0][8];
            self.fix[0][8] = (inputSampleL * self.fix[0][4]) - (outSample * self.fix[0][6]);
            inputSampleL = outSample;
            outSample = (inputSampleR * self.fix[0][2]) + self.fix[0][9];
            self.fix[0][9] =
                (inputSampleR * self.fix[0][3]) - (outSample * self.fix[0][5]) + self.fix[0][10];
            self.fix[0][10] = (inputSampleR * self.fix[0][4]) - (outSample * self.fix[0][6]);
            inputSampleR = outSample;

            let mut samples = [inputSampleL, inputSampleR];
            for (c, sample) in samples.iter_mut().enumerate() {
                let mut x = *sample;
                let skew = x - self.last_sample[c];
                self.last_sample[c] = x;
                let rect = skew.abs().min(3.1415926).sin();
                let skew = (if skew > 0.0 { rect } else { -rect }) * x * skewlevel;
                x *= basstrim;
                x *= inputlevel;
                let mut offset = ((1.0 - eq) + (x.abs() * eq)).clamp(0.0, 1.0);
                self.iir[c][0] = (self.iir[c][0] * (1.0 - (offset * eq))) + (x * (offset * eq));
                x -= self.iir[c][0];
                let mut rect = x.abs() + skew;
                if rect > 1.57079633 {
                    rect = 1.57079633;
                }
                rect = (rect.sin() * 1.57079633) + skew;
                x = if x > 0.0 {
                    (x * (-0.57079633 + skew)) + (rect * (1.57079633 + skew))
                } else {
                    (x * (-0.57079633 + skew)) - (rect * (1.57079633 + skew))
                };
                self.iir[c][2] = (self.iir[c][2] * (1.0 - (offset * eq))) + (x * (offset * eq));
                x = self.iir[c][2];
                x = fixed(&mut self.fix[1], c, x);
                x *= inputlevel;
                offset = (1.0 + offset) / 2.0;
                self.iir[c][1] = (self.iir[c][1] * (1.0 - (offset * eq))) + (x * (offset * eq));
                x -= self.iir[c][1];
                rect = x.abs() + skew;
                if rect > 1.57079633 {
                    rect = 1.57079633;
                }
                rect = (rect.sin() * 1.57079633) + skew;
                x = if x > 0.0 {
                    (x * (-0.57079633 + skew)) + (rect * (1.57079633 + skew))
                } else {
                    (x * (-0.57079633 + skew)) - (rect * (1.57079633 + skew))
                };
                self.iir[c][3] = (self.iir[c][3] * (1.0 - (offset * eq))) + (x * (offset * eq));
                x = self.iir[c][3];
                let history = if self.flip {
                    &mut self.odd[c]
                } else {
                    &mut self.even[c]
                };
                history.copy_within(0..3, 1);
                history[0] = x;
                x = (history[0] + history[1] + history[2] + history[3]) / 4.0;
                x = fixed(&mut self.fix[2], c, x);
                x *= inputlevel;
                rect = x.abs() + skew;
                if rect > 1.57079633 {
                    rect = 1.57079633;
                }
                rect = rect.sin() * 1.57079633;
                x = if x > 0.0 {
                    (x * -0.57079633) + (rect * 1.57079633)
                } else {
                    (x * -0.57079633) - (rect * 1.57079633)
                };
                self.iir[c][4] = (self.iir[c][4] * (1.0 - eq)) + (x * eq);
                x = fixed(&mut self.fix[3], c, self.iir[c][4]);
                x = (x * outputlevel).sin();
                x = fixed(&mut self.fix[4], c, x);
                let fpd = if c == 0 { self.fpd_l } else { self.fpd_r };
                let randy = (fpd as f64 / u32::MAX as f64) * 0.034;
                x = ((x * (1.0 - randy)) + (self.store_sample[c] * randy)) * outputlevel;
                self.store_sample[c] = x;
                *sample = fixed(&mut self.fix[5], c, x);
            }
            inputSampleL = samples[0];
            inputSampleR = samples[1];
            self.flip = !self.flip;

            if (wet != 1.0) {
                inputSampleL = (inputSampleL * wet) + (drySampleL * (1.0 - wet));
                inputSampleR = (inputSampleR * wet) + (drySampleR * (1.0 - wet));
            }

            self.cycle += 1;
            if self.cycle == cycle_end {
                for channel in 0..2 {
                    let sample = if channel == 0 {
                        inputSampleL
                    } else {
                        inputSampleR
                    };
                    let dry = if channel == 0 { drySampleL } else { drySampleR };
                    let fpd = if channel == 0 { self.fpd_l } else { self.fpd_r };
                    let temp = (sample + self.smooth_cab_a[channel]) / 3.0;
                    self.smooth_cab_a[channel] = sample;
                    let sample = cabinet::convolve(&mut self.cab[channel], temp);
                    let temp = (sample + self.smooth_cab_b[channel]) / 3.0;
                    self.smooth_cab_b[channel] = sample;
                    let sample = temp / 4.0;
                    let randy = (fpd as f64 / u32::MAX as f64) * 0.085;
                    let result = ((((sample * (1.0 - randy)) + (self.last_cab[channel] * randy))
                        * wet)
                        + (dry * (1.0 - wet)))
                        * outputlevel;
                    self.last_cab[channel] = sample;
                    let refs = &mut self.last_ref[channel];
                    match cycle_end {
                        4 => {
                            refs[0] = refs[4];
                            refs[2] = (refs[0] + result) / 2.0;
                            refs[1] = (refs[0] + refs[2]) / 2.0;
                            refs[3] = (refs[2] + result) / 2.0;
                            refs[4] = result;
                        }
                        3 => {
                            refs[0] = refs[3];
                            refs[2] = (refs[0] + refs[0] + result) / 3.0;
                            refs[1] = (refs[0] + result + result) / 3.0;
                            refs[3] = result;
                        }
                        2 => {
                            refs[0] = refs[2];
                            refs[1] = (refs[0] + result) / 2.0;
                            refs[2] = result;
                        }
                        _ => refs[0] = result,
                    }
                }
                self.cycle = 0;
            }
            inputSampleL = self.last_ref[0][self.cycle];
            inputSampleR = self.last_ref[1][self.cycle];
            for stage in (2..=cycle_end).rev() {
                let idx = stage + 3;
                self.last_ref[0][8] = inputSampleL;
                inputSampleL = (inputSampleL + self.last_ref[0][idx]) * 0.5;
                self.last_ref[0][idx] = self.last_ref[0][8];
                self.last_ref[1][8] = inputSampleR;
                inputSampleR = (inputSampleR + self.last_ref[1][idx]) * 0.5;
                self.last_ref[1][idx] = self.last_ref[1][8];
            }
            *out_l = crate::dither::fpd_dither(inputSampleL, &mut self.fpd_l) as f32;
            *out_r = crate::dither::fpd_dither(inputSampleR, &mut self.fpd_r) as f32;
        }
    }
}
fn fixed(f: &mut [f64; 11], channel: usize, x: f64) -> f64 {
    let i = if channel == 0 { 7 } else { 9 };
    let out = (x * f[2]) + f[i];
    f[i] = (x * f[3]) - (out * f[5]) + f[i + 1];
    f[i + 1] = (x * f[4]) - (out * f[6]);
    out
}
