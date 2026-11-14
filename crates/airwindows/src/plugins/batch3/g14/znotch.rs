//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ZNotch, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
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
    name: "ZNotch",
    upstream_name: "ZNotch",
    description: "ZNotch is a notch filter made to sound and act like the Emu e6400 Phaser.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ZNotch::new(sample_rate, seed))
}
struct ZNotch {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    filters: [[f64; 15]; 6],
    iir: [f64; 2],
    fpd_l: u32,
    fpd_r: u32,
}
impl ZNotch {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.1, 0.5, 1.0, 0.5],
            sample_rate,
            seed,
            filters: [[0.0; 15]; 6],
            iir: [0.0; 2],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn stage(b: &mut [f64; 15], x: f64, channel: usize, clip: bool) -> f64 {
    let i = if channel == 0 { 7 } else { 11 };
    let mut y = b[2] * x + b[3] * b[i] + b[4] * b[i + 1] - b[5] * b[i + 2] - b[6] * b[i + 3];
    b[i + 1] = b[i];
    b[i] = x;
    b[i + 3] = b[i + 2];
    if clip {
        y = y.clamp(-1.0, 1.0);
    }
    b[i + 2] = y;
    y
}
impl AirwindowsEffect for ZNotch {
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
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let overallscale = 1.0 / 44100.0 * self.sample_rate as f64;
        let freq = (b as f64).powf(3.0) * 4700.0 / self.sample_rate as f64 + 0.0009963;
        let clip = 0.91 - ((1.0 - b as f64) * 0.15);
        let k = (std::f64::consts::PI * freq).tan();
        let norm = 1.0 / (1.0 + k / 0.6180339887498948 + k * k);
        for filter in &mut self.filters[..4] {
            filter[0] = freq;
            filter[1] = 0.6180339887498948;
            filter[2] = (1.0 + k * k) * norm;
            filter[3] = 2.0 * (k * k - 1.0) * norm;
            filter[4] = filter[2];
            filter[5] = filter[3];
            filter[6] = (1.0 - k / filter[1] + k * k) * norm;
        }
        let mut input = a as f64 * 10.0;
        input *= input;
        input *= input;
        let output = c as f64 * 10.0;
        let iir_amount = 0.00069 / overallscale;
        let k = (std::f64::consts::PI * (15500.0 / self.sample_rate as f64)).tan();
        let norm = 1.0 / (1.0 + k / 0.935 + k * k);
        for filter in &mut self.filters[4..] {
            filter[0] = 15500.0 / self.sample_rate as f64;
            filter[1] = 0.935;
            filter[2] = k * k * norm;
            filter[3] = 2.0 * filter[2];
            filter[4] = filter[2];
            filter[5] = 2.0 * (k * k - 1.0) * norm;
            filter[6] = (1.0 - k / 0.935 + k * k) * norm;
        }
        let trim = 0.1 + 3.712 * freq;
        let wet = (d as f64).powf(2.0) * 4.0;
        let amounts = [
            wet.clamp(0.0, 1.0),
            (wet - 1.0).clamp(0.0, 1.0),
            (wet - 2.0).clamp(0.0, 1.0),
            (wet - 3.0).clamp(0.0, 1.0),
        ];
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut x = [l as f64, r as f64];
            if x[0].abs() < 1.18e-23 {
                x[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if x[1].abs() < 1.18e-23 {
                x[1] = self.fpd_r as f64 * 1.18e-17;
            }
            let dry = x;
            for ch in 0..2 {
                if input != 1.0 {
                    x[ch] *= input;
                }
                x[ch] = x[ch].clamp(-1.0, 1.0);
                x[ch] *= trim;
                x[ch] = stage(&mut self.filters[0], x[ch] / clip, ch, true);
                for (i, amount) in amounts.iter().enumerate().skip(1) {
                    if *amount > 0.0 {
                        let y = stage(&mut self.filters[i], x[ch] / clip, ch, true);
                        x[ch] = y * amount + x[ch] * (1.0 - amount);
                    }
                }
                x[ch] /= clip;
                if self.iir[ch].abs() < 1.18e-37 {
                    self.iir[ch] = 0.0;
                }
                self.iir[ch] = self.iir[ch] * (1.0 - iir_amount) + x[ch] * iir_amount;
                x[ch] -= self.iir[ch];
                x[ch] = stage(&mut self.filters[4], x[ch], ch, false);
                x[ch] = x[ch].clamp(-1.0, 1.0);
                x[ch] -= x[ch] * x[ch] * x[ch] * x[ch] * x[ch] * 0.1768;
                x[ch] = stage(&mut self.filters[5], x[ch], ch, false);
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
