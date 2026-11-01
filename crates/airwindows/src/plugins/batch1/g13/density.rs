// Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
// airwindows/plugins/LinuxVST/src/Density; revision b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Libm, measured max-abs 0.0.
#![allow(clippy::approx_constant)] // Preserve the upstream saturation constant.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Density",
        label: " ",
        default: 0.2,
    },
    ParamInfo {
        name: "Highpass",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Out Level",
        label: " ",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Density",
    upstream_name: "Density",
    description: "Density is a Swiss Army Knife of saturation/antisaturation.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Density::new(sample_rate, seed))
}
struct Density {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    iir: [[f64; 2]; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl Density {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.2, 0.0, 1.0, 1.0],
            sample_rate,
            seed,
            iir: [[0.0; 2]; 2],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Density {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(index), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, index: usize) -> String {
        match self.params.get(index) {
            Some(v) if index == 0 => float2string(((*v as f64 * 5.0) - 1.0) as f32),
            Some(v) => float2string(*v),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let mut density = self.params[0] as f64 * 5.0 - 1.0;
        let iir_amount = (self.params[1] as f64).powf(3.0) / overallscale;
        let output = self.params[2] as f64;
        let wet = self.params[3] as f64;
        let dry = 1.0 - wet;
        let mut out = density.abs();
        density *= density.abs();
        for (((&l, &r), ol), or) in in_l
            .iter()
            .zip(in_r)
            .zip(out_l.iter_mut())
            .zip(out_r.iter_mut())
        {
            let mut input = [l as f64, r as f64];
            for (ch, sample) in input.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            let original = input;
            let phase = if self.flip { 0 } else { 1 };
            for (ch, sample) in input.iter_mut().enumerate() {
                self.iir[phase][ch] =
                    (self.iir[phase][ch] * (1.0 - iir_amount)) + (*sample * iir_amount);
                *sample -= self.iir[phase][ch];
            }
            self.flip = !self.flip;
            let mut count = density;
            while count > 1.0 {
                for sample in &mut input {
                    let bridge = (sample.abs() * 1.57079633).min(1.57079633).sin();
                    *sample = if *sample > 0.0 { bridge } else { -bridge };
                }
                count -= 1.0;
            }
            while out > 1.0 {
                out -= 1.0;
            }
            for ch in 0..2 {
                let mut bridge = (input[ch].abs() * 1.57079633).min(1.57079633);
                bridge = if density > 0.0 {
                    bridge.sin()
                } else {
                    1.0 - bridge.cos()
                };
                input[ch] = if input[ch] > 0.0 {
                    input[ch] * (1.0 - out) + bridge * out
                } else {
                    input[ch] * (1.0 - out) - bridge * out
                };
                if output < 1.0 {
                    input[ch] *= output;
                }
                if wet < 1.0 {
                    input[ch] = original[ch] * dry + input[ch] * wet;
                }
            }
            *ol = fpd_dither(input[0], &mut self.fpd[0]) as f32;
            *or = fpd_dither(input[1], &mut self.fpd[1]) as f32;
        }
    }
}
