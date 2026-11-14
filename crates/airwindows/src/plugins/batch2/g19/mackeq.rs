//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/MackEQ, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 5.9604645e-8.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Trim",
        label: "",
        default: 0.1,
    },
    ParamInfo {
        name: "Hi",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Lo",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Gain",
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
    name: "MackEQ",
    upstream_name: "MackEQ",
    description: "MackEQ is the Mackie distortion but with treble and bass controls added.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(MackEq::new(sample_rate, seed))
}
struct MackEq {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    iir: [[f64; 2]; 6],
    biquad: [[f64; 15]; 4],
    fpd: [u32; 2],
}
impl MackEq {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.1, 0.5, 0.5, 1.0, 1.0],
            sample_rate,
            seed,
            iir: [[0.0; 2]; 6],
            biquad: [[0.0; 15]; 4],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn coefficients(b: &mut [f64; 15], rate: f64, q: f64) {
    b[0] = 19160.0 / rate;
    b[1] = q;
    let k = (std::f64::consts::PI * b[0]).tan();
    let norm = 1.0 / (1.0 + k / b[1] + k * k);
    b[2] = k * k * norm;
    b[3] = 2.0 * b[2];
    b[4] = b[2];
    b[5] = 2.0 * (k * k - 1.0) * norm;
    b[6] = (1.0 - k / b[1] + k * k) * norm;
}
fn filter(b: &mut [f64; 15], input: f64, lane: usize) -> f64 {
    let i = 7 + lane * 4;
    let output = b[2] * input + b[3] * b[i] + b[4] * b[i + 1] - b[5] * b[i + 2] - b[6] * b[i + 3];
    b[i + 1] = b[i];
    b[i] = input;
    b[i + 3] = b[i + 2];
    b[i + 2] = output;
    output
}
fn highpass(input: f64, state: &mut f64, amount: f64) -> f64 {
    if state.abs() < 1.18e-37 {
        *state = 0.0;
    }
    *state = *state * (1.0 - amount) + input * amount;
    input - *state
}
fn lowpass(input: f64, state: &mut f64, amount: f64) -> f64 {
    if state.abs() < 1.18e-37 {
        *state = 0.0;
    }
    *state = *state * (1.0 - amount) + input * amount;
    *state
}
fn clip(x: f64) -> f64 {
    let x = x.clamp(-1.0, 1.0);
    x - x.powf(5.0) * 0.1768
}
impl AirwindowsEffect for MackEq {
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
        let scale = self.sample_rate as f64 / 44100.0;
        let trim = self.params[0] as f64 * 10.0;
        let trim = trim * trim;
        let high = self.params[1] as f64;
        let bass = self.params[2] as f64;
        let gain_high = high.powf(2.0) * 4.0;
        let out_high = high.sqrt();
        let gain_bass = bass.powf(2.0) * 4.0;
        let out_bass = bass.sqrt();
        let pad = self.params[3] as f64;
        let wet = self.params[4] as f64;
        for (b, q) in self.biquad.iter_mut().zip([
            0.431684981684982,
            1.1582298,
            0.657027382751269,
            1.076210852946577,
        ]) {
            coefficients(b, self.sample_rate as f64, q);
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            for (lane, channel) in samples.iter_mut().enumerate() {
                let mut x = *channel;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[lane] as f64 * 1.18e-17;
                }
                let dry = x;
                x = highpass(x, &mut self.iir[0][lane], 0.001860867 / scale);
                if trim != 1.0 {
                    x *= trim;
                }
                x = filter(&mut self.biquad[0], x, lane);
                x = clip(x);
                x = filter(&mut self.biquad[1], x, lane);
                x = highpass(x, &mut self.iir[1][lane], 0.000287496 / scale);
                let mut lo = lowpass(x, &mut self.iir[2][lane], 0.159 / scale);
                let mut mid = x - lo;
                if gain_bass != 1.0 {
                    lo *= gain_bass;
                }
                lo = clip(lo);
                lo = lowpass(lo, &mut self.iir[3][lane], 0.159 / scale);
                let low_mid = lowpass(mid, &mut self.iir[4][lane], 0.236 / scale);
                let mut hi = mid - low_mid;
                mid = low_mid;
                hi = highpass(hi, &mut self.iir[5][lane], 0.236 / scale);
                if gain_high != 1.0 {
                    hi *= gain_high;
                }
                hi = clip(hi);
                x = (lo * out_bass + mid + hi * out_high) * 4.0;
                x = filter(&mut self.biquad[2], x, lane);
                x = clip(x);
                x = filter(&mut self.biquad[3], x, lane);
                if pad != 1.0 {
                    x *= pad;
                }
                if wet != 1.0 {
                    x = x * wet + dry * (1.0 - wet);
                }
                *channel = fpd_dither(x, &mut self.fpd[lane]);
            }
            *out_l = samples[0] as f32;
            *out_r = samples[1] as f32;
        }
    }
}
