//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/MidAmp, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
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
    name: "MidAmp",
    upstream_name: "MidAmp",
    description:
        "MidAmp is a clean amp sim meant to work like a loud Twin or something of that nature.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(MidAmp::new(sample_rate, seed))
}
struct MidAmp {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    last_sample: [f64; 2],
    store_sample: [f64; 2],
    iir: [[f64; 10]; 2],
    odd: [[f64; 257]; 2],
    even: [[f64; 257]; 2],
    flip: bool,
    count: i32,
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
impl MidAmp {
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
            odd: [[0.0; 257]; 2],
            even: [[0.0; 257]; 2],
            flip: false,
            count: 0,
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
impl AirwindowsEffect for MidAmp {
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
        let basstrim = self.params[1] as f64;
        let outputlevel = self.params[2] as f64;
        let wet = self.params[3] as f64;
        let samplerate = self.sample_rate as f64;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= samplerate;
        let cycle_end = (overallscale.floor() as usize).clamp(1, 4);
        if self.cycle > cycle_end - 1 {
            self.cycle = cycle_end - 1;
        }
        let bleed = outputlevel / 6.0;
        let EQ = (basstrim / samplerate) * 22050.0;
        let BEQ = (bleed / samplerate) * 44100.0;
        let side = ((0.0006092985 * samplerate) as i32).min(126);
        let down = side + 1;
        let inputlevel = self.params[0] as f64 * 3.0;
        let inputlevel = (inputlevel + (inputlevel * basstrim)) / 2.0;
        for (((&left, &right), out_l), out_r) in
            il.iter().zip(ir).zip(ol.iter_mut()).zip(orr.iter_mut())
        {
            let mut samples = [left as f64, right as f64];
            let fpd = [self.fpd_l, self.fpd_r];
            for ch in 0..2 {
                if samples[ch].abs() < 1.18e-23 {
                    samples[ch] = fpd[ch] as f64 * 1.18e-17;
                }
            }
            let dry = samples;
            if self.count < 0 || self.count > 128 {
                self.count = 128;
            }
            for ch in 0..2 {
                let mut x = samples[ch];
                let idx = if ch == 0 { 7 } else { 9 };
                let filter = |x: f64, f: &mut [f64; 11]| {
                    let y = x * f[2] + f[idx];
                    f[idx] = x * f[3] - y * f[5] + f[idx + 1];
                    f[idx + 1] = x * f[4] - y * f[6];
                    y
                };
                x = filter(x, &mut self.fix[0]);
                let mut skew = x - self.last_sample[ch];
                self.last_sample[ch] = x;
                let bridge = skew.abs().min(3.1415926).sin();
                skew = if skew > 0.0 { bridge } else { -bridge };
                skew *= x;
                x = filter(x, &mut self.fix[1]);
                x *= inputlevel;
                let mut offset = ((1.0 - EQ) + (x.abs() * EQ)).clamp(0.0, 1.0);
                self.iir[ch][0] = self.iir[ch][0] * (1.0 - offset * EQ) + x * (offset * EQ);
                x -= self.iir[ch][0] * 0.8;
                let bridge = (x.abs() + skew).min(1.57079633).sin() * 1.57079633 + skew;
                x = if x > 0.0 {
                    x * (-0.57079633 + skew) + bridge * (1.57079633 + skew)
                } else {
                    x * (-0.57079633 + skew) - bridge * (1.57079633 + skew)
                };
                self.iir[ch][2] = self.iir[ch][2] * (1.0 - offset * EQ) + x * (offset * EQ);
                x -= self.iir[ch][2] * 0.6;
                skew /= 1.57079633;
                x = filter(x, &mut self.fix[2]);
                x *= inputlevel;
                offset = (1.0 + offset) / 2.0;
                let bridge = ((x.abs() + skew).min(1.57079633).sin() * 1.57079633 + skew)
                    .min(1.57079633)
                    .sin()
                    * 1.57079633;
                x = if x > 0.0 {
                    x * (-0.57079633 + skew) + bridge * (1.57079633 + skew)
                } else {
                    x * (-0.57079633 + skew) - bridge * (1.57079633 + skew)
                };
                self.iir[ch][6] = self.iir[ch][6] * (1.0 - EQ) + x * EQ;
                x -= self.iir[ch][6] * 0.4;
                self.iir[ch][3] = self.iir[ch][3] * (1.0 - offset * EQ) + x * (offset * EQ);
                x = self.iir[ch][3];
                skew /= 1.57079633;
                x = filter(x, &mut self.fix[3]);
                x *= inputlevel;
                let bridge = (x.abs() + skew).min(1.57079633).sin() * 1.57079633;
                x = if x > 0.0 {
                    x * (-0.57079633 + skew) + bridge * (1.57079633 + skew)
                } else {
                    x * (-0.57079633 + skew) - bridge * (1.57079633 + skew)
                };
                self.iir[ch][4] = self.iir[ch][4] * (1.0 - EQ) + x * EQ;
                x = self.iir[ch][4];
                x = filter(x, &mut self.fix[4]);
                self.iir[ch][5] = self.iir[ch][5] * (1.0 - offset * BEQ) + x * (offset * BEQ);
                let history = if self.flip {
                    &mut self.odd[ch]
                } else {
                    &mut self.even[ch]
                };
                history[(self.count + 128) as usize] = self.iir[ch][5];
                history[self.count as usize] = self.iir[ch][5];
                let result =
                    history[(self.count + down) as usize] + history[(self.count + side) as usize];
                self.iir[ch][1] = self.iir[ch][1] * (1.0 - offset * BEQ) + result * (offset * BEQ);
                x += self.iir[ch][1] * bleed;
                self.iir[ch][7] = self.iir[ch][7] * (1.0 - offset * BEQ) + x * (offset * BEQ);
                x += self.iir[ch][7];
                x = (x * outputlevel).sin();
                let randy = (fpd[ch] as f64 / u32::MAX as f64) * 0.047;
                x = ((x * (1.0 - randy)) + (self.store_sample[ch] * randy)) * outputlevel;
                self.store_sample[ch] = x;
                x = filter(x, &mut self.fix[5]);
                if wet != 1.0 {
                    x = x * wet + dry[ch] * (1.0 - wet);
                }
                samples[ch] = x;
            }
            self.count -= 1;
            self.flip = !self.flip;
            let mut inputSampleL = samples[0];
            let mut inputSampleR = samples[1];
            let drySampleL = dry[0];
            let drySampleR = dry[1];
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
                    let randy = (fpd as f64 / u32::MAX as f64) * 0.05;
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
