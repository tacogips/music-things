//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/UnBox, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Input",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "UnBox",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "UnBox",
    upstream_name: "UnBox",
    description:
        "UnBox is a distortion where only the harmonics that don't alias are allowed to distort.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(UnBox::new(sample_rate, seed))
}
struct UnBox {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    a: [[f64; 5]; 2],
    b: [[f64; 5]; 2],
    c: [[f64; 10]; 2],
    iir: [[f64; 2]; 2],
    fpd: [u32; 2],
}
impl UnBox {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.0, 0.5],
            sample_rate,
            seed,
            a: [[0.0; 5]; 2],
            b: [[0.0; 5]; 2],
            c: [[0.0; 10]; 2],
            iir: [[0.0; 2]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn coefficients<const N: usize>(treble: f64) -> [f64; N] {
    let mut result = [0.0; N];
    let mut gain = treble;
    for coefficient in &mut result {
        if gain > 1.0 {
            *coefficient = 1.0;
            gain -= 1.0;
        } else {
            *coefficient = gain;
            gain = 0.0;
        }
        *coefficient /= treble.max(1.0);
    }
    result
}
fn average<const N: usize>(sample: f64, history: &mut [f64; N], weights: &[f64; N]) -> f64 {
    history.copy_within(0..N - 1, 1);
    history[0] = sample;
    let mut result = sample * weights[0];
    for i in 1..N {
        result += history[i] * weights[i];
    }
    result
}
impl AirwindowsEffect for UnBox {
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
        match i {
            0 | 2 => float2string((self.params[i] as f64 * 2.0) as f32),
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let input_gain = self.params[0] as f64 * 2.0;
        let mut unbox = self.params[1] as f64 + 1.0;
        unbox *= unbox;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let amount = (unbox * 0.00052) / overallscale;
        let output_gain = self.params[2] as f64 * 2.0;
        let e = coefficients::<5>(unbox);
        let f = coefficients::<10>(unbox * 2.0);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut values = [l as f64, r as f64];
            for (ch, value) in values.iter_mut().enumerate() {
                if input_gain != 1.0 {
                    *value *= input_gain;
                }
                if value.abs() < 1.18e-23 {
                    *value = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *value;
                *value = average(*value, &mut self.a[ch], &e);
                *value = average(*value, &mut self.b[ch], &e);
                *value *= unbox;
                *value = value.clamp(-1.2533141373155, 1.2533141373155);
                let abs = value.abs();
                *value = (*value * abs).sin() / if abs == 0.0 { 1.0 } else { abs };
                *value /= unbox;
                let mut accumulator = average(dry - *value, &mut self.c[ch], &f);
                for pole in 0..2 {
                    self.iir[ch][pole] = self.iir[ch][pole] * (1.0 - amount) + accumulator * amount;
                    accumulator -= self.iir[ch][pole];
                }
                *value = dry - accumulator;
                if output_gain != 1.0 {
                    *value *= output_gain;
                }
                *value = fpd_dither(*value, &mut self.fpd[ch]);
            }
            *out_l = values[0] as f32;
            *out_r = values[1] as f32;
        }
    }
}
