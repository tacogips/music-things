//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ZRegion2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)] // Preserve upstream coefficients.
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
        name: "First",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Last",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Poles",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "ZRegion2",
    upstream_name: "ZRegion2",
    description: "ZRegion2 is an Emu e6400 style Airwindows Region filter, with control smoothing.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ZRegion2::new(sample_rate, seed))
}
struct ZRegion2 {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    filters: [[f64; 21]; 5],
    fixed: [[f64; 11]; 2],
    gain_prev: f64,
    wet_prev: f64,
    overall_prev: f64,
    iir_l: f64,
    iir_r: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl ZRegion2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.1, 0.5, 0.5, 0.5, 1.0],
            sample_rate,
            seed,
            filters: [[0.0; 21]; 5],
            fixed: [[0.0; 11]; 2],
            gain_prev: 0.1,
            wet_prev: 0.5,
            overall_prev: 1.0,
            iir_l: 0.0,
            iir_r: 0.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn stage(b: &mut [f64], input: f64, right: bool, interpolated: bool) -> f64 {
    let i = if interpolated {
        if right {
            19
        } else {
            17
        }
    } else if right {
        9
    } else {
        7
    };
    let out = input * b[2] + b[i];
    b[i] = input * b[3] - out * b[5] + b[i + 1];
    b[i + 1] = input * b[4] - out * b[6];
    out
}
fn sine(x: f64) -> f64 {
    x.clamp(-1.57079633, 1.57079633).sin()
}
impl AirwindowsEffect for ZRegion2 {
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
        let [a, b, c, d, e] = self.params;
        let mut gain = a as f64 * 10.0;
        gain *= gain;
        gain *= gain;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let iir_amount = 0.00069 / overallscale;
        let high = b as f64;
        let low = c as f64;
        let mid = (high + low) * 0.5;
        let spread = 1.001 - (high - low).abs();
        let tuning = [high, (high + mid) * 0.5, mid, (mid + low) * 0.5, low];
        let mut compensation = [0.0; 5];
        let mut clip = [0.0; 5];
        for (i, filter) in self.filters.iter_mut().enumerate() {
            for j in 0..5 {
                filter[7 + j] = filter[12 + j];
            }
            let freq = (tuning[i].powf(3.0) * 20000.0 / self.sample_rate as f64).max(0.00009);
            compensation[i] = freq.sqrt() * 6.4 * spread;
            clip[i] = 0.75 + (freq * d as f64 * 37.0);
            let k = (std::f64::consts::PI * freq).tan();
            let norm = 1.0 / (1.0 + k / 0.7071 + k * k);
            filter[0] = freq;
            filter[12] = k / 0.7071 * norm;
            filter[14] = -filter[12];
            filter[15] = 2.0 * (k * k - 1.0) * norm;
            filter[16] = (1.0 - k / 0.7071 + k * k) * norm;
        }
        let freq = 15500.0 / self.sample_rate as f64;
        let k = (std::f64::consts::PI * freq).tan();
        let norm = 1.0 / (1.0 + k / 0.935 + k * k);
        let bq = &mut self.fixed[0];
        bq[0] = freq;
        bq[1] = 0.935;
        bq[2] = k * k * norm;
        bq[3] = 2.0 * bq[2];
        bq[4] = bq[2];
        bq[5] = 2.0 * (k * k - 1.0) * norm;
        bq[6] = (1.0 - k / 0.935 + k * k) * norm;
        for i in 0..7 {
            self.fixed[1][i] = self.fixed[0][i];
        }
        let old_gain = self.gain_prev;
        self.gain_prev = gain;
        let old_wet = self.wet_prev;
        self.wet_prev = d as f64;
        let old_overall = self.overall_prev;
        self.overall_prev = e as f64;
        let frames = il.len().min(ir.len()).min(ol.len()).min(or.len());
        for (frame, (((&l, &r), outl), outr)) in il.iter().zip(ir).zip(ol).zip(or).enumerate() {
            let temp = (frames - 1 - frame) as f64 / frames as f64;
            for filter in &mut self.filters {
                for j in 0..5 {
                    filter[2 + j] = filter[7 + j] * temp + filter[12 + j] * (1.0 - temp);
                }
            }
            let gain = old_gain * temp + self.gain_prev * (1.0 - temp);
            let nuke = (old_wet * temp + self.wet_prev * (1.0 - temp)) * 4.0;
            let wets = [
                nuke.clamp(0.0, 1.0),
                (nuke - 1.0).clamp(0.0, 1.0),
                (nuke - 2.0).clamp(0.0, 1.0),
                (nuke - 3.0).clamp(0.0, 1.0),
            ];
            let wet = old_overall * temp + self.overall_prev * (1.0 - temp);
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            if gain != 1.0 {
                l *= gain;
                r *= gain;
            }
            l = stage(&mut self.filters[0], sine(l * clip[0]), false, true) / compensation[0];
            r = stage(&mut self.filters[0], sine(r * clip[0]), true, true) / compensation[0];
            for (i, amount) in wets.into_iter().enumerate() {
                if amount > 0.0 {
                    let nl = stage(&mut self.filters[i + 1], sine(l * clip[i + 1]), false, true)
                        / compensation[i + 1];
                    let nr = stage(&mut self.filters[i + 1], sine(r * clip[i + 1]), true, true)
                        / compensation[i + 1];
                    l = nl * amount + l * (1.0 - amount);
                    r = nr * amount + r * (1.0 - amount);
                }
            }
            if self.iir_l.abs() < 1.18e-37 {
                self.iir_l = 0.0;
            }
            self.iir_l = self.iir_l * (1.0 - iir_amount) + l * iir_amount;
            l -= self.iir_l;
            if self.iir_r.abs() < 1.18e-37 {
                self.iir_r = 0.0;
            }
            self.iir_r = self.iir_r * (1.0 - iir_amount) + r * iir_amount;
            r -= self.iir_r;
            l = stage(&mut self.fixed[0], l, false, false);
            r = stage(&mut self.fixed[0], r, true, false);
            l = l.clamp(-1.0, 1.0);
            l -= l * l * l * l * l * 0.1768;
            r = r.clamp(-1.0, 1.0);
            r -= r * r * r * r * r * 0.1768;
            l = stage(&mut self.fixed[1], l, false, false);
            r = stage(&mut self.fixed[1], r, true, false);
            if wet != 1.0 {
                l = l * wet + dry_l * (1.0 - wet);
                r = r * wet + dry_r * (1.0 - wet);
            }
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
