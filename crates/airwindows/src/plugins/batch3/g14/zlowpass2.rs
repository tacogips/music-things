//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ZLowpass2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Input",
        label: "",
        default: 0.1,
    },
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Poles",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "ZLowpass2",
    upstream_name: "ZLowpass2",
    description:
        "ZLowpass2 acts more like the Emu e6400 Ultra lowpass in motion, with control smoothing.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ZLowpass2::new(rate, seed))
}
struct ZLowpass2 {
    params: [f32; 4],
    rate: f32,
    seed: u32,
    coeff: [f64; 5],
    states: [[[f64; 2]; 2]; 4],
    fixed: [[[f64; 2]; 2]; 2],
    iir: [f64; 2],
    input: f64,
    output: f64,
    wet: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl ZLowpass2 {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.1, 0.5, 1.0, 0.5],
            rate,
            seed,
            coeff: [0.0; 5],
            states: [[[0.0; 2]; 2]; 4],
            fixed: [[[0.0; 2]; 2]; 2],
            iir: [0.0; 2],
            input: 0.1,
            output: 1.0,
            wet: 0.5,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn stage(s: &mut [f64; 2], x: f64, c: &[f64; 5], clip: bool) -> f64 {
    let mut y = x * c[0] + s[0];
    if clip {
        y = y.clamp(-1.0, 1.0);
    }
    s[0] = x * c[1] - y * c[3] + s[1];
    s[1] = x * c[2] - y * c[4];
    y
}
impl AirwindowsEffect for ZLowpass2 {
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
            .map_or_else(String::new, |&p| float2string(p))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let scale = 1.0 / 44100.0 * self.rate as f64;
        let freq = (b as f64).powf(3.0) * 18930.0 / self.rate as f64 + 0.00162;
        let clip = 1.212 - (1.0 - b as f64) * 0.496;
        let k = (std::f64::consts::PI * freq).tan();
        let norm = 1.0 / (1.0 + k / 0.7071 + k * k);
        let old = self.coeff;
        let a0 = k * k * norm;
        let a1 = 2.0 * a0;
        self.coeff = [
            a0,
            a1,
            a0,
            2.0 * (k * k - 1.0) * norm,
            (1.0 - k / 0.7071 + k * k) * norm,
        ];
        let old_input = self.input;
        self.input = a as f64 * 10.0;
        self.input *= self.input;
        self.input *= self.input;
        let old_output = self.output;
        self.output = c as f64 * 10.0;
        let old_wet = self.wet;
        self.wet = (d as f64).powf(2.0);
        let iir = 0.00069 / scale;
        let k = (std::f64::consts::PI * (15500.0 / self.rate as f64)).tan();
        let norm = 1.0 / (1.0 + k / 0.935 + k * k);
        let a0 = k * k * norm;
        let fixed = [
            a0,
            2.0 * a0,
            a0,
            2.0 * (k * k - 1.0) * norm,
            (1.0 - k / 0.935 + k * k) * norm,
        ];
        let trim = 0.1 + 3.712 * freq;
        let count = il.len().min(ir.len()).min(ol.len()).min(orr.len());
        for (index, (((&l, &r), outl), outr)) in il.iter().zip(ir).zip(ol).zip(orr).enumerate() {
            let mut x = [l as f64, r as f64];
            if x[0].abs() < 1.18e-23 {
                x[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if x[1].abs() < 1.18e-23 {
                x[1] = self.fpd_r as f64 * 1.18e-17;
            }
            let dry = x;
            let t = (count - 1 - index) as f64 / count as f64;
            let mut coeff = [0.0; 5];
            for i in 0..5 {
                coeff[i] = old[i] * t + self.coeff[i] * (1.0 - t);
            }
            let input = old_input * t + self.input * (1.0 - t);
            let output = old_output * t + self.output * (1.0 - t);
            let wet = (old_wet * t + self.wet * (1.0 - t)) * 4.0;
            let amounts = [
                wet.clamp(0.0, 1.0),
                (wet - 1.0).clamp(0.0, 1.0),
                (wet - 2.0).clamp(0.0, 1.0),
                (wet - 3.0).clamp(0.0, 1.0),
            ];
            for ch in 0..2 {
                if input != 1.0 {
                    x[ch] *= input;
                }
                x[ch] = x[ch].clamp(-1.0, 1.0);
                x[ch] *= trim;
                x[ch] /= clip;
                x[ch] = stage(&mut self.states[0][ch], x[ch], &coeff, true);
                for (i, amount) in amounts.iter().enumerate().skip(1) {
                    if *amount > 0.0 {
                        let y = stage(&mut self.states[i][ch], x[ch] / clip, &coeff, true);
                        x[ch] = y * amount + x[ch] * (1.0 - amount);
                    }
                }
                x[ch] /= clip;
                if self.iir[ch].abs() < 1.18e-37 {
                    self.iir[ch] = 0.0;
                }
                self.iir[ch] = self.iir[ch] * (1.0 - iir) + x[ch] * iir;
                x[ch] -= self.iir[ch];
                x[ch] = stage(&mut self.fixed[0][ch], x[ch], &fixed, false);
                x[ch] = x[ch].clamp(-1.0, 1.0);
                x[ch] -= x[ch] * x[ch] * x[ch] * x[ch] * x[ch] * 0.1768;
                x[ch] = stage(&mut self.fixed[1][ch], x[ch], &fixed, false);
                if output != 1.0 {
                    x[ch] *= output;
                }
                if amounts[0] != 1.0 {
                    x[ch] = x[ch] * amounts[0] + dry[ch] * (1.0 - amounts[0]);
                }
            }
            *outl = fpd_dither(x[0], &mut self.fpd_l) as f32;
            *outr = fpd_dither(x[1], &mut self.fpd_r) as f32;
        }
    }
}
