//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: FireAmp, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
#![allow(
    non_snake_case,
    unused_parens,
    clippy::approx_constant,
    clippy::assign_op_pattern,
    clippy::manual_clamp,
    clippy::excessive_precision,
    clippy::too_many_arguments
)]
mod cabinet;
mod stage;
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
    name: "FireAmp",
    upstream_name: "FireAmp",
    description: "FireAmp is a bright, loud, tubey amp sim for leads and dirt guitar.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(FireAmp::new(sample_rate, seed))
}
struct FireAmp {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    lastSampleL: f64,
    storeSampleL: f64,
    smoothAL: f64,
    smoothBL: f64,
    smoothCL: f64,
    smoothDL: f64,
    smoothEL: f64,
    smoothFL: f64,
    smoothGL: f64,
    smoothHL: f64,
    smoothIL: f64,
    smoothJL: f64,
    smoothKL: f64,
    smoothLL: f64,
    iirSampleAL: f64,
    iirSampleBL: f64,
    iirSampleCL: f64,
    iirSampleDL: f64,
    iirSampleEL: f64,
    iirSampleFL: f64,
    iirSampleGL: f64,
    iirSampleHL: f64,
    iirSampleIL: f64,
    iirSampleJL: f64,
    iirSampleKL: f64,
    iirSampleLL: f64,
    iirLowpassL: f64,
    iirSpkAL: f64,
    iirSpkBL: f64,
    iirSubL: f64,
    lastSampleR: f64,
    storeSampleR: f64,
    smoothAR: f64,
    smoothBR: f64,
    smoothCR: f64,
    smoothDR: f64,
    smoothER: f64,
    smoothFR: f64,
    smoothGR: f64,
    smoothHR: f64,
    smoothIR: f64,
    smoothJR: f64,
    smoothKR: f64,
    smoothLR: f64,
    iirSampleAR: f64,
    iirSampleBR: f64,
    iirSampleCR: f64,
    iirSampleDR: f64,
    iirSampleER: f64,
    iirSampleFR: f64,
    iirSampleGR: f64,
    iirSampleHR: f64,
    iirSampleIR: f64,
    iirSampleJR: f64,
    iirSampleKR: f64,
    iirSampleLR: f64,
    iirLowpassR: f64,
    iirSpkAR: f64,
    iirSpkBR: f64,
    iirSubR: f64,
    fixA: [f64; 11],
    fixB: [f64; 11],
    fixC: [f64; 11],
    fixD: [f64; 11],
    fixE: [f64; 11],
    fixF: [f64; 11],
    odd: [[f64; 257]; 2],
    even: [[f64; 257]; 2],
    count: i32,
    flip: bool,
    cab: [[f64; 90]; 2],
    smooth_a: [f64; 2],
    smooth_b: [f64; 2],
    last_cab: [f64; 2],
    last_ref: [[f64; 10]; 2],
    cycle: usize,
    fpd: [u32; 2],
}
impl FireAmp {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.5, 0.8, 1.0],
            sample_rate,
            seed,
            lastSampleL: 0.0,
            storeSampleL: 0.0,
            smoothAL: 0.0,
            smoothBL: 0.0,
            smoothCL: 0.0,
            smoothDL: 0.0,
            smoothEL: 0.0,
            smoothFL: 0.0,
            smoothGL: 0.0,
            smoothHL: 0.0,
            smoothIL: 0.0,
            smoothJL: 0.0,
            smoothKL: 0.0,
            smoothLL: 0.0,
            iirSampleAL: 0.0,
            iirSampleBL: 0.0,
            iirSampleCL: 0.0,
            iirSampleDL: 0.0,
            iirSampleEL: 0.0,
            iirSampleFL: 0.0,
            iirSampleGL: 0.0,
            iirSampleHL: 0.0,
            iirSampleIL: 0.0,
            iirSampleJL: 0.0,
            iirSampleKL: 0.0,
            iirSampleLL: 0.0,
            iirLowpassL: 0.0,
            iirSpkAL: 0.0,
            iirSpkBL: 0.0,
            iirSubL: 0.0,
            lastSampleR: 0.0,
            storeSampleR: 0.0,
            smoothAR: 0.0,
            smoothBR: 0.0,
            smoothCR: 0.0,
            smoothDR: 0.0,
            smoothER: 0.0,
            smoothFR: 0.0,
            smoothGR: 0.0,
            smoothHR: 0.0,
            smoothIR: 0.0,
            smoothJR: 0.0,
            smoothKR: 0.0,
            smoothLR: 0.0,
            iirSampleAR: 0.0,
            iirSampleBR: 0.0,
            iirSampleCR: 0.0,
            iirSampleDR: 0.0,
            iirSampleER: 0.0,
            iirSampleFR: 0.0,
            iirSampleGR: 0.0,
            iirSampleHR: 0.0,
            iirSampleIR: 0.0,
            iirSampleJR: 0.0,
            iirSampleKR: 0.0,
            iirSampleLR: 0.0,
            iirLowpassR: 0.0,
            iirSpkAR: 0.0,
            iirSpkBR: 0.0,
            iirSubR: 0.0,
            fixA: [0.0; 11],
            fixB: [0.0; 11],
            fixC: [0.0; 11],
            fixD: [0.0; 11],
            fixE: [0.0; 11],
            fixF: [0.0; 11],
            odd: [[0.0; 257]; 2],
            even: [[0.0; 257]; 2],
            count: 0,
            flip: false,
            cab: [[0.0; 90]; 2],
            smooth_a: [0.0; 2],
            smooth_b: [0.0; 2],
            last_cab: [0.0; 2],
            last_ref: [[0.0; 10]; 2],
            cycle: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    fn setup(&mut self) {
        let cutoff = ((15000.0 + self.params[1] as f64 * 10000.0) / self.sample_rate as f64)
            .clamp(0.001, 0.49);
        for (f, res) in [
            &mut self.fixA,
            &mut self.fixB,
            &mut self.fixC,
            &mut self.fixD,
            &mut self.fixE,
            &mut self.fixF,
        ]
        .into_iter()
        .zip([
            4.46570214, 1.51387132, 0.93979296, 0.70710678, 0.52972649, 0.50316379,
        ]) {
            let k = (std::f64::consts::PI * cutoff).tan();
            let norm = 1.0 / (1.0 + k / res + k * k);
            f[0] = cutoff;
            f[1] = res;
            f[2] = k * k * norm;
            f[3] = 2.0 * f[2];
            f[4] = f[2];
            f[5] = 2.0 * (k * k - 1.0) * norm;
            f[6] = (1.0 - k / res + k * k) * norm;
        }
    }
}
impl AirwindowsEffect for FireAmp {
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
        self.setup();
        let bassfill = self.params[0] as f64;
        let outputlevel = self.params[2] as f64;
        let wet = self.params[3] as f64;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let cycle_end = (overallscale.floor() as usize).clamp(1, 4);
        if self.cycle > cycle_end - 1 {
            self.cycle = cycle_end - 1;
        }
        let samplerate = self.sample_rate as f64;
        let basstrim = bassfill / 16.0;
        let toneEQ = (self.params[1] as f64 / samplerate) * 22050.0;
        let EQ = (basstrim / samplerate) * 22050.0;
        let bleed = outputlevel / 16.0;
        let bassfactor = 1.0 - (basstrim * basstrim);
        let BEQ = (bleed / samplerate) * 22050.0;
        let diagonal = ((0.000861678 * samplerate) as i32).min(127);
        let side = (diagonal as f64 / 1.4142135623730951) as i32;
        let down = (side + diagonal) / 2;
        for (((&left, &right), out_l), out_r) in
            il.iter().zip(ir).zip(ol.iter_mut()).zip(orr.iter_mut())
        {
            let mut l = left as f64;
            let mut r = right as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd[0] as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd[1] as f64 * 1.18e-17;
            }
            let dry = [l, r];
            (l, r) = self.stages(l, r, bassfill, EQ, bassfactor, toneEQ, BEQ);
            if self.count < 0 || self.count > 128 {
                self.count = 128;
            }
            let mut result = [0.0; 2];
            let spk = [self.iirSpkAL, self.iirSpkAR];
            for ch in 0..2 {
                let a = if self.flip {
                    &mut self.odd[ch]
                } else {
                    &mut self.even[ch]
                };
                let n = self.count as usize;
                a[n + 128] = spk[ch];
                a[n] = spk[ch];
                result[ch] = a[(self.count + down) as usize]
                    + a[(self.count + side) as usize]
                    + a[(self.count + diagonal) as usize];
            }
            self.count -= 1;
            self.iirSpkBL = self.iirSpkBL * (1.0 - BEQ) + result[0] * BEQ;
            l += self.iirSpkBL * bleed;
            self.iirSpkBR = self.iirSpkBR * (1.0 - BEQ) + result[1] * BEQ;
            r += self.iirSpkBR * bleed;
            let rect = (l * outputlevel).abs().min(1.57079633).sin();
            l = if l > 0.0 { rect } else { -rect };
            let rect = (r * outputlevel).abs().min(1.57079633).sin();
            r = if r > 0.0 { rect } else { -rect };
            self.iirSubL = self.iirSubL * (1.0 - BEQ) + l * BEQ;
            l += self.iirSubL * bassfill * outputlevel;
            self.iirSubR = self.iirSubR * (1.0 - BEQ) + r * BEQ;
            r += self.iirSubR * bassfill * outputlevel;
            let rand = self.fpd[0] as f64 / u32::MAX as f64 * 0.053;
            l = (l * (1.0 - rand) + self.storeSampleL * rand) * outputlevel;
            self.storeSampleL = l;
            let rand = self.fpd[1] as f64 / u32::MAX as f64 * 0.053;
            r = (r * (1.0 - rand) + self.storeSampleR * rand) * outputlevel;
            self.storeSampleR = r;
            self.flip = !self.flip;
            if wet != 1.0 {
                l = l * wet + dry[0] * (1.0 - wet);
                r = r * wet + dry[1] * (1.0 - wet);
            }
            let input = [l, r];
            self.cycle += 1;
            if self.cycle == cycle_end {
                for ch in 0..2 {
                    let temp = (input[ch] + self.smooth_a[ch]) / 3.0;
                    self.smooth_a[ch] = input[ch];
                    let sample = cabinet::convolve(&mut self.cab[ch], temp);
                    let temp = (sample + self.smooth_b[ch]) / 3.0;
                    self.smooth_b[ch] = sample;
                    let sample = temp / 4.0;
                    let rand = self.fpd[ch] as f64 / u32::MAX as f64 * 0.057;
                    let result = (((sample * (1.0 - rand) + self.last_cab[ch] * rand) * wet)
                        + (dry[ch] * (1.0 - wet)))
                        * outputlevel;
                    self.last_cab[ch] = sample;
                    let refs = &mut self.last_ref[ch];
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
            let mut result = [self.last_ref[0][self.cycle], self.last_ref[1][self.cycle]];
            for stage in (2..=cycle_end).rev() {
                let idx = stage + 3;
                for (ch, sample) in result.iter_mut().enumerate() {
                    self.last_ref[ch][8] = *sample;
                    *sample = (*sample + self.last_ref[ch][idx]) * 0.5;
                    self.last_ref[ch][idx] = self.last_ref[ch][8];
                }
            }
            *out_l = crate::dither::fpd_dither(result[0], &mut self.fpd[0]) as f32;
            *out_r = crate::dither::fpd_dither(result[1], &mut self.fpd[1]) as f32;
        }
    }
}
