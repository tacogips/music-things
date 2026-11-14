//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PurestSquish, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Squish",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "BassBlm",
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
    name: "PurestSquish",
    upstream_name: "PurestSquish",
    description: "PurestSquish is an open-sounding compressor with bass bloom.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PurestSquish::new(sample_rate, seed))
}
#[derive(Clone, Copy)]
struct Channel {
    speed: [f64; 5],
    coefficient: [f64; 5],
    iir: [f64; 5],
    last_b: f64,
    last_d: f64,
    merged: f64,
}
impl Channel {
    fn new() -> Self {
        Self {
            speed: [10000.0; 5],
            coefficient: [1.0; 5],
            iir: [0.0; 5],
            last_b: 1.0,
            last_d: 1.0,
            merged: 1.0,
        }
    }
    fn stage(
        &mut self,
        index: usize,
        input: f64,
        amount: f64,
        threshold: f64,
        release: f64,
        fastest: f64,
    ) {
        self.iir[index] = (self.iir[index] * (1.0 - amount)) + (input * amount);
        let highpass = input - self.iir[index];
        if highpass.abs() > threshold {
            let vary = threshold / highpass.abs();
            let attack = self.speed[index].abs().sqrt();
            self.coefficient[index] *= attack - 1.0;
            self.coefficient[index] += if vary < threshold { threshold } else { vary };
            self.coefficient[index] /= attack;
        } else {
            self.coefficient[index] *= (self.speed[index] * self.speed[index]) - 1.0;
            self.coefficient[index] += 1.0;
            self.coefficient[index] /= self.speed[index] * self.speed[index];
        }
        let mut new_speed = self.speed[index] * (self.speed[index] - 1.0);
        new_speed = new_speed + (highpass * release).abs() + fastest;
        self.speed[index] = new_speed / self.speed[index];
    }
    fn process(
        &mut self,
        input: f64,
        count: usize,
        flip: bool,
        amount: f64,
        threshold: f64,
        scale: f64,
    ) -> f64 {
        let mut gain = 1.0 / threshold;
        let mut output_gain = gain.sqrt();
        gain += output_gain;
        gain *= 0.5;
        output_gain *= 0.5;
        let mut release = self.merged * 32768.0;
        release /= scale;
        let fastest = release.sqrt();
        let correction = self.merged;
        let input = if gain != 1.0 { input * gain } else { input };
        self.stage(count, input, amount, threshold, release, fastest);
        let mut last_a = self.coefficient[count].powf(2.0);
        self.merged = self.last_b;
        self.merged += last_a;
        last_a *= 1.0 - correction;
        last_a += self.coefficient[count] * correction;
        self.last_b = last_a;
        let stage = if flip { 3 } else { 4 };
        self.stage(stage, input, amount, threshold, release, fastest);
        // The upstream flip branch intentionally squares coefficient E in both cases.
        let mut last_c = self.coefficient[4].powf(2.0);
        self.merged += self.last_d;
        self.merged += last_c;
        last_c *= 1.0 - correction;
        last_c += self.coefficient[stage] * correction;
        self.last_d = last_c;
        self.merged *= 0.25;
        let mut output = input * self.merged;
        if output_gain != 1.0 {
            output *= output_gain;
        }
        output
    }
}
struct PurestSquish {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    channels: [Channel; 2],
    count: usize,
    flip: bool,
    fpd: [u32; 2],
}
impl PurestSquish {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.0, 1.0, 1.0],
            sample_rate,
            seed,
            channels: [Channel::new(); 2],
            count: 0,
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for PurestSquish {
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
        let [a, b, c, d] = self.params;
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let threshold = 1.01 - (1.0 - (1.0 - (a as f64 * 0.5)).powf(4.0));
        let amount = (b as f64).powf(4.0) / scale;
        let output = c as f64;
        let wet = d as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut input = [l as f64, r as f64];
            for (i, sample) in input.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[i] as f64 * 1.18e-17;
                }
            }
            let dry = input;
            for (i, sample) in input.iter_mut().enumerate() {
                *sample = self.channels[i]
                    .process(*sample, self.count, self.flip, amount, threshold, scale);
                if output < 1.0 {
                    *sample *= output;
                }
                if wet < 1.0 {
                    *sample = (*sample * wet) + (dry[i] * (1.0 - wet));
                }
                *sample = fpd_dither(*sample, &mut self.fpd[i]);
            }
            self.count = (self.count + 1) % 3;
            self.flip = !self.flip;
            *out_l = input[0] as f32;
            *out_r = input[1] as f32;
        }
    }
}
