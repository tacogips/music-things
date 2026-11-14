//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Edge, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant, clippy::needless_range_loop)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Gain",
        label: "",
        default: 0.1,
    },
    ParamInfo {
        name: "Lowpass",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Highpass",
        label: "",
        default: 0.0,
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
    name: "Edge",
    upstream_name: "Edge",
    description:
        "Edge is a seven-stage distortion with Hypersonic filtering and tone shaping controls.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Edge::new(sample_rate, seed))
}
struct Edge {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    state: [[[f64; 2]; 2]; 7],
    iir: [f64; 2],
    fpd_l: u32,
    fpd_r: u32,
}
impl Edge {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.1, 1.0, 0.0, 1.0, 1.0],
            sample_rate,
            seed,
            state: [[[0.0; 2]; 2]; 7],
            iir: [0.0; 2],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Edge {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let scale = 1.0 / 44100.0 * self.sample_rate as f64;
        let trim = a as f64 * 4.0 + 0.5;
        let cutoff = (b as f64 * 25000.0 / self.sample_rate as f64).clamp(0.001, 0.49);
        let amount = (c as f64).powf(3.0) * 0.5;
        let amount = amount.max(0.00000001) / scale;
        let k = (std::f64::consts::PI * cutoff).tan();
        let resonances = [
            4.46570214, 1.51387132, 0.93979296, 0.70710678, 0.59051105, 0.52972649, 0.50316379,
        ];
        let mut coeff = [[0.0; 5]; 7];
        for (co, q) in coeff.iter_mut().zip(resonances) {
            let norm = 1.0 / (1.0 + k / q + k * k);
            co[0] = k * k * norm;
            co[1] = 2.0 * co[0];
            co[2] = co[0];
            co[3] = 2.0 * (k * k - 1.0) * norm;
            co[4] = (1.0 - k / q + k * k) * norm;
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let dry = [l as f64, r as f64];
            let mut v = dry;
            if v[0].abs() < 1.18e-23 {
                v[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if v[1].abs() < 1.18e-23 {
                v[1] = self.fpd_r as f64 * 1.18e-17;
            }
            for channel in 0..2 {
                if self.iir[channel].abs() < 1.18e-33 {
                    self.iir[channel] = 0.0;
                }
                self.iir[channel] = self.iir[channel] * (1.0 - amount) + v[channel] * amount;
                v[channel] -= self.iir[channel];
            }
            for (stage, co) in coeff.iter().enumerate() {
                for channel in 0..2 {
                    let state = &mut self.state[stage][channel];
                    let output = v[channel] * co[0] + state[0];
                    state[0] = v[channel] * co[1] - output * co[3] + state[1];
                    state[1] = v[channel] * co[2] - output * co[4];
                    v[channel] = output;
                    if stage < 6 {
                        v[channel] = (v[channel] * trim).clamp(-1.0, 1.0);
                    }
                }
            }
            let wet = e as f64;
            let pad = d as f64;
            v[0] = v[0] * wet * pad + dry[0] * (1.0 - wet);
            v[1] = v[1] * wet * pad + dry[1] * (1.0 - wet);
            *out_l = fpd_dither(v[0], &mut self.fpd_l) as f32;
            *out_r = fpd_dither(v[1], &mut self.fpd_r) as f32;
        }
    }
}
