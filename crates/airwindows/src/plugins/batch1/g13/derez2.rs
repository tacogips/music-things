// Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
// airwindows/plugins/LinuxVST/src/DeRez2; revision b110fac3991fff7c4655cc2d25d6a3e221f9799c
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
    ParamInfo {
        name: "Hard",
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
    name: "DeRez2",
    upstream_name: "DeRez2",
    description: "DeRez2 brings more authentic retro-digital hardware tones.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DeRez2::new(sample_rate, seed))
}
struct DeRez2 {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    last: [f64; 2],
    held: [f64; 2],
    last_dry: [f64; 2],
    last_output: [f64; 2],
    position: f64,
    increment_a: f64,
    increment_b: f64,
    fpd: [u32; 2],
}
impl DeRez2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0; 4],
            sample_rate,
            seed,
            last: [0.0; 2],
            held: [0.0; 2],
            last_dry: [0.0; 2],
            last_output: [0.0; 2],
            position: 0.0,
            increment_a: 0.0,
            increment_b: 0.0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for DeRez2 {
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
        let mut target_a = (self.params[0] as f64).powf(3.0) + 0.0005;
        if target_a > 1.0 {
            target_a = 1.0;
        }
        let soften = (1.0 + target_a) / 2.0;
        let target_b = (1.0 - self.params[1] as f64).powf(3.0) / 3.0;
        let hard = self.params[2] as f64;
        let wet = self.params[3] as f64;
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        target_a /= scale;
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
            self.increment_a = (self.increment_a * 999.0 + target_a) / 1000.0;
            self.increment_b = (self.increment_b * 999.0 + target_b) / 1000.0;
            self.position += self.increment_a;
            let mut output = self.held;
            if self.position > 1.0 {
                self.position -= 1.0;
                for ch in 0..2 {
                    self.held[ch] =
                        self.last[ch] * self.position + input[ch] * (1.0 - self.position);
                    output[ch] = output[ch] * (1.0 - soften) + self.held[ch] * soften;
                }
            }
            input = output;
            for ch in 0..2 {
                let temp = input[ch];
                if input[ch] != self.last_output[ch] {
                    input[ch] = input[ch] * hard + self.last_dry[ch] * (1.0 - hard);
                }
                self.last_output[ch] = temp;
                self.last_dry[ch] = dry[ch];
                let temp = input[ch];
                input[ch] = input[ch].clamp(-1.0, 1.0);
                if input[ch] > 0.0 {
                    input[ch] = (1.0 + 255.0 * input[ch].abs()).ln() / 256.0_f64.ln();
                }
                if input[ch] < 0.0 {
                    input[ch] = -(1.0 + 255.0 * input[ch].abs()).ln() / 256.0_f64.ln();
                }
                input[ch] = temp * hard + input[ch] * (1.0 - hard);
                if self.increment_b > 0.0005 {
                    if input[ch] > 0.0 {
                        let mut t = input[ch];
                        while t > 0.0 {
                            t -= self.increment_b;
                        }
                        input[ch] -= t;
                    }
                    if input[ch] < 0.0 {
                        let mut t = input[ch];
                        while t < 0.0 {
                            t += self.increment_b;
                        }
                        input[ch] -= t;
                    }
                    input[ch] *= 1.0 - self.increment_b;
                }
                let temp = input[ch];
                input[ch] = input[ch].clamp(-1.0, 1.0);
                if input[ch] > 0.0 {
                    input[ch] = (256.0_f64.powf(input[ch].abs()) - 1.0) / 255.0;
                }
                if input[ch] < 0.0 {
                    input[ch] = -(256.0_f64.powf(input[ch].abs()) - 1.0) / 255.0;
                }
                input[ch] = temp * hard + input[ch] * (1.0 - hard);
                if wet != 1.0 {
                    input[ch] = input[ch] * wet + dry[ch] * (1.0 - wet);
                }
            }
            self.last = dry;
            *ol = fpd_dither(input[0], &mut self.fpd[0]) as f32;
            *or = fpd_dither(input[1], &mut self.fpd[1]) as f32;
        }
    }
}
