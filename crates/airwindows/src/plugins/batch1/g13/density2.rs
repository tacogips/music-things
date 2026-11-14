// Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
// airwindows/plugins/LinuxVST/src/Density2; revision b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Libm, measured max-abs 0.0.
#![allow(clippy::approx_constant, clippy::excessive_precision)] // Preserve upstream constants.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Density",
        label: "",
        default: 0.2,
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
const INFO: PluginInfo = PluginInfo {
    name: "Density2",
    upstream_name: "Density2",
    description:
        "Density2 is a different color for Density, some old code I had that people wanted.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Density2::new(sample_rate, seed))
}
struct Density2 {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    iir: [[f64; 2]; 2],
    last: [[f64; 3]; 2],
    ata: [[f64; 2]; 2],
    last_diff: [f64; 2],
    fpd: [u32; 2],
}
impl Density2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.2, 0.0, 1.0, 1.0],
            sample_rate,
            seed,
            iir: [[0.0; 2]; 2],
            last: [[0.0; 3]; 2],
            ata: [[0.0; 2]; 2],
            last_diff: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn saturate(mut sample: f64, density: f64, out: f64) -> f64 {
    let mut count = density;
    while count > 1.0 {
        let bridge = (sample.abs() * 1.57079633).min(1.57079633).sin();
        sample = if sample > 0.0 { bridge } else { -bridge };
        count -= 1.0;
    }
    let mut bridge = (sample.abs() * 1.57079633).min(1.57079633);
    bridge = if density > 0.0 {
        bridge.sin()
    } else {
        1.0 - bridge.cos()
    };
    if sample > 0.0 {
        sample * (1.0 - out) + bridge * out
    } else {
        sample * (1.0 - out) - bridge * out
    }
}
impl AirwindowsEffect for Density2 {
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
        let mut out = density.abs();
        while out > 1.0 {
            out -= 1.0;
        }
        density *= density.abs();
        let amount = (self.params[1] as f64).powf(3.0) / overallscale;
        let output = self.params[2] as f64;
        let wet = self.params[3] as f64;
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
            let dry = input;
            for ch in 0..2 {
                let mut half = (input[ch]
                    + self.last[ch][0]
                    + ((-self.last[ch][1] + self.last[ch][2]) * 0.0414213562373095048801688))
                    / 2.0;
                let half_dry = half;
                self.last[ch][2] = self.last[ch][1];
                self.last[ch][1] = self.last[ch][0];
                self.last[ch][0] = input[ch];
                self.iir[1][ch] = self.iir[1][ch] * (1.0 - amount) + half * amount;
                half -= self.iir[1][ch];
                half = saturate(half, density, out);
                let delta = half - half_dry;
                self.ata[ch][0] *= 0.915965594177219015;
                self.ata[ch][1] *= 0.915965594177219015;
                self.ata[ch][1] += delta;
                self.ata[ch][0] -= delta;
                let half_diff = self.ata[ch][1] * 0.915965594177219015;
                self.iir[0][ch] = self.iir[0][ch] * (1.0 - amount) + input[ch] * amount;
                input[ch] -= self.iir[0][ch];
                input[ch] = saturate(input[ch], density, out);
                let delta = input[ch] - dry[ch];
                self.ata[ch][0] *= 0.915965594177219015;
                self.ata[ch][1] *= 0.915965594177219015;
                self.ata[ch][0] += delta;
                self.ata[ch][1] -= delta;
                let diff = self.ata[ch][0] * 0.915965594177219015;
                input[ch] = dry[ch] + ((diff + half_diff + self.last_diff[ch]) / 1.187);
                self.last_diff[ch] = diff / 2.0;
                input[ch] *= output;
                input[ch] = dry[ch] * (1.0 - wet) + input[ch] * wet;
            }
            *ol = fpd_dither(input[0], &mut self.fpd[0]) as f32;
            *or = fpd_dither(input[1], &mut self.fpd[1]) as f32;
        }
    }
}
