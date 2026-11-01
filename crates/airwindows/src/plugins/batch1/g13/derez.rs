// Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
// airwindows/plugins/LinuxVST/src/DeRez; revision b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Rate",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Rez",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "DeRez",
    upstream_name: "DeRez",
    description:
        "DeRez is an analog-style bit and sample rate crusher with continuous adjustments.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DeRez::new(sample_rate, seed))
}
struct DeRez {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    last: [f64; 2],
    held: [f64; 2],
    position: f64,
    increment_a: f64,
    increment_b: f64,
    fpd: [u32; 2],
}
impl DeRez {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0; 2],
            sample_rate,
            seed,
            last: [0.0; 2],
            held: [0.0; 2],
            position: 0.0,
            increment_a: 0.0,
            increment_b: 0.0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for DeRez {
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
        self.params
            .get(index)
            .map_or_else(String::new, |v| float2string(*v))
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
        let mut target_a = (self.params[0] as f64).powf(3.0) + 0.0005;
        if target_a > 1.0 {
            target_a = 1.0;
        }
        let soften = (1.0 + target_a) / 2.0;
        let target_b = (1.0 - self.params[1] as f64).powf(3.0) / 3.0;
        target_a /= overallscale;
        for (((&l, &r), ol), or) in in_l
            .iter()
            .zip(in_r)
            .zip(out_l.iter_mut())
            .zip(out_r.iter_mut())
        {
            let mut input = [l as f64, r as f64];
            for (ch, sample) in input.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 && target_b == 0.0 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            let dry = input;
            self.increment_a = ((self.increment_a * 999.0) + target_a) / 1000.0;
            self.increment_b = ((self.increment_b * 999.0) + target_b) / 1000.0;
            self.position += self.increment_a;
            let mut output = self.held;
            if self.position > 1.0 {
                self.position -= 1.0;
                for ch in 0..2 {
                    self.held[ch] =
                        (self.last[ch] * self.position) + (input[ch] * (1.0 - self.position));
                    output[ch] = (output[ch] * (1.0 - soften)) + (self.held[ch] * soften);
                }
            }
            input = output;
            if self.increment_b > 0.0005 {
                for sample in &mut input {
                    if *sample > 0.0 {
                        let mut offset = *sample;
                        while offset > 0.0 {
                            offset -= self.increment_b;
                        }
                        *sample -= offset;
                    }
                    if *sample < 0.0 {
                        let mut offset = *sample;
                        while offset < 0.0 {
                            offset += self.increment_b;
                        }
                        *sample -= offset;
                    }
                    *sample *= 1.0 - self.increment_b;
                }
            }
            self.last = dry;
            *ol = fpd_dither(input[0], &mut self.fpd[0]) as f32;
            *or = fpd_dither(input[1], &mut self.fpd[1]) as f32;
        }
    }
}
