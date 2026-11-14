//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Mackity, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "In Trim",
        label: "",
        default: 0.1,
    },
    ParamInfo {
        name: "Out Pad",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Mackity",
    upstream_name: "Mackity",
    description: "Mackity is an emulation of the input stage of a vintage Mackie 1202!",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Mackity::new(sample_rate, seed))
}
struct Mackity {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    iir_a: [f64; 2],
    iir_b: [f64; 2],
    biquad_a: [f64; 15],
    biquad_b: [f64; 15],
    fpd: [u32; 2],
}
impl Mackity {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.1, 1.0],
            sample_rate,
            seed,
            iir_a: [0.0; 2],
            iir_b: [0.0; 2],
            biquad_a: [0.0; 15],
            biquad_b: [0.0; 15],
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
impl AirwindowsEffect for Mackity {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let scale = self.sample_rate as f64 / 44100.0;
        let trim = self.params[0] as f64 * 10.0;
        let trim = trim * trim;
        let pad = self.params[1] as f64;
        coefficients(
            &mut self.biquad_a,
            self.sample_rate as f64,
            0.431684981684982,
        );
        coefficients(&mut self.biquad_b, self.sample_rate as f64, 1.1582298);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut sample = [l as f64, r as f64];
            for (lane, channel) in sample.iter_mut().enumerate() {
                if channel.abs() < 1.18e-23 {
                    *channel = self.fpd[lane] as f64 * 1.18e-17;
                }
                let mut x = highpass(*channel, &mut self.iir_a[lane], 0.001860867 / scale);
                if trim != 1.0 {
                    x *= trim;
                }
                x = filter(&mut self.biquad_a, x, lane);
                x = x.clamp(-1.0, 1.0);
                x -= x.powf(5.0) * 0.1768;
                x = filter(&mut self.biquad_b, x, lane);
                x = highpass(x, &mut self.iir_b[lane], 0.000287496 / scale);
                if pad != 1.0 {
                    x *= pad;
                }
                *channel = fpd_dither(x, &mut self.fpd[lane]);
            }
            *out_l = sample[0] as f32;
            *out_r = sample[1] as f32;
        }
    }
}
