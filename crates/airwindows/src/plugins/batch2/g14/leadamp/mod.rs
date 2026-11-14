//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/LeadAmp, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
#![allow(clippy::approx_constant, clippy::excessive_precision)]
mod cabinet;
mod interpolation;
mod stages;
mod state;

use crate::display::float2string;
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
use state::State;

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
    name: "LeadAmp",
    upstream_name: "LeadAmp",
    description: "LeadAmp is an amp sim with a clear, penetrating, vocal tone.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(State::new(sample_rate, seed))
}
impl AirwindowsEffect for State {
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
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let sr = self.sample_rate as f64;
        let bassfill = self.params[0] as f64;
        let basstrim = bassfill / 10.0;
        let tone = (self.params[1] as f64 / sr) * 22050.0;
        let eq = (basstrim / sr) * 22050.0;
        let output = self.params[2] as f64;
        let wet = self.params[3] as f64;
        let bleed = output / 16.0;
        let beq = (bleed / sr) * 22050.0;
        let bassfactor = 1.0 - basstrim * basstrim;
        let diagonal = ((0.000861678 * sr) as usize).min(127);
        let side = (diagonal as f64 / 1.4142135623730951) as usize;
        let down = (side + diagonal) / 2;
        let cycle_end = ((sr / 44100.0).floor() as usize).clamp(1, 4);
        self.cycle = self.cycle.min(cycle_end - 1);
        let cutoff = ((15000.0 + self.params[1] as f64 * 10000.0) / sr).clamp(0.001, 0.49);
        for (f, resonance) in self.fix.iter_mut().zip([
            4.46570214, 1.51387132, 0.93979296, 0.70710678, 0.52972649, 0.50316379,
        ]) {
            f[0] = cutoff;
            f[1] = resonance;
            let k = (std::f64::consts::PI * cutoff).tan();
            let norm = 1.0 / (1.0 + k / resonance + k * k);
            f[2] = k * k * norm;
            f[3] = 2.0 * f[2];
            f[4] = f[2];
            f[5] = 2.0 * (k * k - 1.0) * norm;
            f[6] = (1.0 - k / resonance + k * k) * norm;
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol.iter_mut()).zip(or.iter_mut()) {
            let mut dry = [l as f64, r as f64];
            for (c, x) in dry.iter_mut().enumerate() {
                if x.abs() < 1.18e-23 {
                    *x = self.fpd[c] as f64 * 1.18e-17;
                }
            }
            let mut x = dry;
            for (c, sample) in x.iter_mut().enumerate() {
                *sample = self.gain_stages(c, *sample, bassfill, eq, bassfactor);
            }
            if self.count < 0 || self.count > 128 {
                self.count = 128;
            }
            for c in 0..2 {
                self.lowpass[c] = self.lowpass[c] * (1.0 - tone) + x[c] * tone;
                x[c] = self.lowpass[c];
                self.speaker_a[c] = self.speaker_a[c] * (1.0 - beq) + x[c] * beq;
                let history = if self.flip {
                    &mut self.odd[c]
                } else {
                    &mut self.even[c]
                };
                let count = self.count as usize;
                history[count] = self.speaker_a[c];
                history[count + 128] = self.speaker_a[c];
                let result =
                    history[count + down] + history[count + side] + history[count + diagonal];
                self.speaker_b[c] = self.speaker_b[c] * (1.0 - beq) + result * beq;
                x[c] += self.speaker_b[c] * bleed;
                self.sub[c] = self.sub[c] * (1.0 - beq) + x[c] * beq;
                x[c] += self.sub[c] * bassfill;
                let rect = (x[c] * output).abs().min(1.57079633).sin();
                x[c] = if x[c] > 0.0 { rect } else { -rect };
                let randy = (self.fpd[c] as f64 / u32::MAX as f64) * 0.084;
                x[c] = (x[c] * (1.0 - randy) + self.store_sample[c] * randy) * output;
                self.store_sample[c] = x[c];
                if wet != 1.0 {
                    x[c] = x[c] * wet + dry[c] * (1.0 - wet);
                }
            }
            self.count -= 1;
            self.flip = !self.flip;
            self.cycle += 1;
            if self.cycle == cycle_end {
                for c in 0..2 {
                    let temp = (x[c] + self.smooth_cab_a[c]) / 3.0;
                    self.smooth_cab_a[c] = x[c];
                    let sample = cabinet::convolve(&mut self.cab[c], temp);
                    let temp = (sample + self.smooth_cab_b[c]) / 3.0;
                    self.smooth_cab_b[c] = sample;
                    let sample = temp / 4.0;
                    let randy = (self.fpd[c] as f64 / u32::MAX as f64) * 0.079;
                    let result = ((sample * (1.0 - randy) + self.last_cab[c] * randy) * wet
                        + dry[c] * (1.0 - wet))
                        * output;
                    self.last_cab[c] = sample;
                    interpolation::update(&mut self.last_ref[c], result, cycle_end);
                }
                self.cycle = 0;
            }
            for (c, sample) in x.iter_mut().enumerate() {
                let reference = self.last_ref[c][self.cycle];
                *sample = interpolation::average(&mut self.last_ref[c], reference, cycle_end);
            }
            *out_l = crate::dither::fpd_dither(x[0], &mut self.fpd[0]) as f32;
            *out_r = crate::dither::fpd_dither(x[1], &mut self.fpd[1]) as f32;
        }
    }
}
