//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Point, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Input Trim",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Point",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "Reaction Speed",
        label: " ",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Point",
    upstream_name: "Point",
    description: "Point is an explosive transient designer.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Point::new(sample_rate, seed))
}
struct Point {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    nib: [[f64; 2]; 2],
    nob: [[f64; 2]; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl Point {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5; 3],
            sample_rate,
            seed,
            nib: [[0.0; 2]; 2],
            nob: [[0.0; 2]; 2],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Point {
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
            0 => float2string((self.params[0] as f64 * 24.0 - 12.0) as f32),
            1 => float2string((self.params[1] as f64 * 2.0 - 1.0) as f32),
            2 => float2string(self.params[2]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c] = self.params;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let gaintrim = 10.0_f64.powf((a as f64 * 24.0 - 12.0) / 20.0);
        let mut nib_div = 1.0 / (c as f64 + 0.2).powf(7.0);
        nib_div /= overallscale;
        let point = b as f64 * 2.0 - 1.0;
        let nob_div = if point > 0.0 {
            nib_div / (1.001 - point)
        } else {
            nib_div * (1.001 - (point * 0.75).powf(2.0))
        };
        let mut factor = 0.0;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                *sample *= gaintrim;
                let absolute = sample.abs();
                let phase = usize::from(!self.flip);
                self.nib[ch][phase] += absolute / nib_div;
                self.nib[ch][phase] /= 1.0 + 1.0 / nib_div;
                self.nob[ch][phase] += absolute / nob_div;
                self.nob[ch][phase] /= 1.0 + 1.0 / nob_div;
                if self.nob[ch][phase] > 0.0 {
                    factor = self.nib[ch][phase] / self.nob[ch][phase];
                }
                *sample *= factor;
            }
            self.flip = !self.flip;
            *out_l = fpd_dither(samples[0], &mut self.fpd[0]) as f32;
            *out_r = fpd_dither(samples[1], &mut self.fpd[1]) as f32;
        }
    }
}
