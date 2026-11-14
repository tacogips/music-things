//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/YNotch, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)] // Preserve upstream coefficients.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Gain",
        label: "",
        default: 0.1,
    },
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Reson8",
        label: "",
        default: 0.1,
    },
    ParamInfo {
        name: "ResEdge",
        label: "",
        default: 0.1,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "YNotch",
    upstream_name: "YNotch",
    description: "YNotch is soft and smooth to nasty, edgy texture-varying filtering.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(YNotch::new(sample_rate, seed))
}
struct YNotch {
    params: [f32; 6],
    sample_rate: f32,
    seed: u32,
    coeff: [f64; 5],
    state: [[f64; 2]; 2],
    fixed: [[[f64; 2]; 2]; 2],
    in_trim: f64,
    out_trim: f64,
    power: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl YNotch {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.1, 0.5, 0.1, 0.1, 1.0, 1.0],
            sample_rate,
            seed,
            coeff: [0.0; 5],
            state: [[0.0; 2]; 2],
            fixed: [[[0.0; 2]; 2]; 2],
            in_trim: 0.1,
            out_trim: 1.0,
            power: 1.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn fixed(s: &mut [f64; 2], x: f64, c: &[f64; 5]) -> f64 {
    let y = x * c[0] + s[0];
    s[0] = x * c[1] - y * c[3] + s[1];
    s[1] = x * c[2] - y * c[4];
    y
}
fn curve(mut x: f64, p: f64) -> f64 {
    if x > 1.0 {
        x = 1.0;
    } else if x > 0.0 {
        x = 1.0 - (1.0 - x).powf(p);
    }
    if x < -1.0 {
        x = -1.0;
    } else if x < 0.0 {
        x = -1.0 + (1.0 + x).powf(p);
    }
    x
}
impl AirwindowsEffect for YNotch {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d, e, f] = self.params;
        let old_coeff = self.coeff;
        let freq = ((b as f64).powf(3.0) * 20000.0).max(15.0) / self.sample_rate as f64;
        let res = (c as f64).powf(2.0) * 15.0 + 0.0001;
        let k = (std::f64::consts::PI * freq).tan();
        let norm = 1.0 / (1.0 + k / res + k * k);
        self.coeff = [
            (1.0 + k * k) * norm,
            2.0 * (k * k - 1.0) * norm,
            (1.0 + k * k) * norm,
            2.0 * (k * k - 1.0) * norm,
            (1.0 - k / res + k * k) * norm,
        ];
        let old_in = self.in_trim;
        self.in_trim = a as f64 * 10.0;
        let old_power = self.power;
        self.power = (d as f64 + 0.9).powf(4.0);
        let old_out = self.out_trim;
        self.out_trim = e as f64;
        let k = (std::f64::consts::PI * 20000.0 / self.sample_rate as f64).tan();
        let norm = 1.0 / (1.0 + k / 0.7071 + k * k);
        let a0 = k * k * norm;
        let fixed_coeff = [
            a0,
            2.0 * a0,
            a0,
            2.0 * (k * k - 1.0) * norm,
            (1.0 - k / 0.7071 + k * k) * norm,
        ];
        let count = il.len().min(ir.len()).min(ol.len()).min(or.len());
        for (index, (((&l, &r), outl), outr)) in il.iter().zip(ir).zip(ol).zip(or).enumerate() {
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
                coeff[i] = old_coeff[i] * t + self.coeff[i] * (1.0 - t);
            }
            let power = old_power * t + self.power * (1.0 - t);
            let input = old_in * t + self.in_trim * (1.0 - t);
            let output = old_out * t + self.out_trim * (1.0 - t);
            for ch in 0..2 {
                x[ch] *= input;
                x[ch] = fixed(&mut self.fixed[0][ch], x[ch], &fixed_coeff);
                x[ch] = curve(x[ch], power);
                let y = x[ch] * coeff[0] + self.state[ch][0];
                self.state[ch][0] = x[ch] * coeff[1] - y * coeff[3] + self.state[ch][1];
                self.state[ch][1] = x[ch] * coeff[2] - y * coeff[4];
                x[ch] = curve(y, 1.0 / power);
                x[ch] *= output;
                x[ch] = fixed(&mut self.fixed[1][ch], x[ch], &fixed_coeff);
                if f < 1.0 {
                    x[ch] = x[ch] * f as f64 + dry[ch] * (1.0 - f as f64);
                }
            }
            *outl = fpd_dither(x[0], &mut self.fpd_l) as f32;
            *outr = fpd_dither(x[1], &mut self.fpd_r) as f32;
        }
    }
}
