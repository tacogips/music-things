//! Dirt, upstream airwindows/plugins/LinuxVST/src/Dirt at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Parity: Libm, measured max-abs 0.0.

// Preserve the literal coefficient used by upstream.
#![allow(clippy::approx_constant)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "Dirt",
    upstream_name: "Dirt",
    description: "Dirt is a soft-clip distortion in the spirit of Edge.",
    params: &[
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
    ],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Dirt::new(rate, seed))
}
struct Dirt {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    filter: [[[f64; 2]; 2]; 7],
    iir: [f64; 2],
    fpd: [u32; 2],
}
impl Dirt {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.1, 1.0, 0.0, 1.0, 1.0],
            sample_rate,
            seed,
            filter: [[[0.0; 2]; 2]; 7],
            iir: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Dirt {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        self.params
            .get(i)
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        self.filter = [[[0.0; 2]; 2]; 7];
        self.iir = [0.0; 2];
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let p = self.params.map(|x| x as f64);
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let trim = ((1.0 - (1.0 - p[0]).powf(2.0)) * 1.0) + 1.0;
        let cutoff = (p[1] * 25000.0 / self.sample_rate as f64).clamp(0.001, 0.49);
        let mut iir = (p[2].powf(3.0) * 0.5).max(0.00000001);
        iir /= scale;
        let mut coeff = [[0.0; 5]; 7];
        for (i, reso) in [
            4.46570214, 1.51387132, 0.93979296, 0.70710678, 0.59051105, 0.52972649, 0.50316379,
        ]
        .into_iter()
        .enumerate()
        {
            let k = (std::f64::consts::PI * cutoff).tan();
            let norm = 1.0 / (1.0 + k / reso + k * k);
            let a0 = k * k * norm;
            coeff[i] = [
                a0,
                2.0 * a0,
                a0,
                2.0 * (k * k - 1.0) * norm,
                (1.0 - k / reso + k * k) * norm,
            ];
        }
        for (((l, r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [*l as f64, *r as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                let dry = *sample;
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                if self.iir[ch].abs() < 1.18e-33 {
                    self.iir[ch] = 0.0;
                }
                self.iir[ch] = (self.iir[ch] * (1.0 - iir)) + (*sample * iir);
                *sample -= self.iir[ch];
                *sample = sample.clamp(-1.0, 1.0);
                for (stage, c) in coeff.iter().enumerate() {
                    let state = &mut self.filter[stage][ch];
                    let out = (*sample * c[0]) + state[0];
                    state[0] = (*sample * c[1]) - (out * c[3]) + state[1];
                    state[1] = (*sample * c[2]) - (out * c[4]);
                    *sample = out;
                    if stage < 6 {
                        *sample = (*sample - (*sample * sample.abs() * 0.5)) * trim;
                    }
                }
                *sample = (*sample * p[4] * p[3]) + (dry * (1.0 - p[4]));
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            *out_l = samples[0] as f32;
            *out_r = samples[1] as f32;
        }
    }
}
