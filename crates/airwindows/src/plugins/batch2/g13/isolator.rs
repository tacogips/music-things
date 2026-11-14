//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Isolator, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Freq",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "High",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Low",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Isolator",
    upstream_name: "Isolator",
    description: "Isolator is a steep highpass or lowpass filter, like you might find in a speaker crossover.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Isolator::new(sample_rate, seed))
}
struct Isolator {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    biquads: [[f64; 15]; 3],
    fpd: [u32; 2],
}
impl Isolator {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.0, 1.0],
            sample_rate,
            seed,
            biquads: [[0.0; 15]; 3],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Isolator {
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
        let [a, b, c] = self.params;
        let bypass = a == 1.0 && b <= 0.0 && c >= 1.0;
        let scale = self.sample_rate as f64 / 44100.0;
        let frequency = ((a as f64).powf(2.0 * scale.sqrt()) * 0.4999).max(0.001);
        for (biquad, q) in self
            .biquads
            .iter_mut()
            .zip([0.5, 0.6180339887498948, 1.618033988749895])
        {
            let k = (std::f64::consts::PI * frequency).tan();
            let norm = 1.0 / (1.0 + k / q + k * k);
            biquad[2] = k * k * norm;
            biquad[3] = 2.0 * biquad[2];
            biquad[4] = biquad[2];
            biquad[5] = 2.0 * (k * k - 1.0) * norm;
            biquad[6] = (1.0 - k / q + k * k) * norm;
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut output = [0.0; 2];
            for (ch, input) in [l, r].into_iter().enumerate() {
                let dry = input as f64;
                let mut sample = dry;
                if sample.abs() < 1.18e-23 {
                    sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                sample = sample.sin();
                let i = 7 + ch * 4;
                for biquad in &mut self.biquads {
                    let out =
                        biquad[2] * sample + biquad[3] * biquad[i] + biquad[4] * biquad[i + 1]
                            - biquad[5] * biquad[i + 2]
                            - biquad[6] * biquad[i + 3];
                    biquad[i + 1] = biquad[i];
                    biquad[i] = sample;
                    sample = out;
                    biquad[i + 3] = biquad[i + 2];
                    biquad[i + 2] = sample;
                }
                sample = sample.clamp(-1.0, 1.0).asin();
                sample = if bypass {
                    dry
                } else {
                    sample * c as f64 + (dry - sample) * b as f64
                };
                output[ch] = fpd_dither(sample, &mut self.fpd[ch]) as f32;
            }
            *out_l = output[0];
            *out_r = output[1];
        }
    }
}
