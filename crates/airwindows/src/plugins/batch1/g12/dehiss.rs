// Copyright (c) 2016 airwindows, Airwindows uses the MIT license
// Upstream: airwindows/plugins/LinuxVST/src/DeHiss, b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

static PARAMS: [ParamInfo; 2] = [
    ParamInfo {
        name: "Thresh",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "DeHiss",
    upstream_name: "DeHiss",
    description: "DeHiss tries to suppress background hiss, like a hiss gate.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DeHiss::new(sample_rate, seed))
}

struct Channel {
    stored: [f64; 2],
    diff: [f64; 6],
    gate: f64,
    raw: f64,
    fpd: u32,
}
impl Channel {
    fn new(fpd: u32) -> Self {
        Self {
            stored: [0.0; 2],
            diff: [0.0; 6],
            gate: 1.0,
            raw: 2.0,
            fpd,
        }
    }
    fn process(&mut self, input: f32, threshold: f64, wet: f64, mean_out: &mut f64) -> f32 {
        let mut sample = input as f64;
        if sample.abs() < 1.18e-23 {
            sample = self.fpd as f64 * 1.18e-17;
        }
        let dry = sample;
        self.stored[1] = self.stored[0];
        self.stored[0] = sample;
        self.diff.copy_within(0..5, 1);
        self.diff[0] = self.stored[0] - self.stored[1];
        let d = self.diff;
        let averages = [
            (d[0] + d[1]) / 2.0,
            (d[0] + d[1] + d[2]) / 3.0,
            (d[0] + d[1] + d[2] + d[3]) / 4.0,
            (d[0] + d[1] + d[2] + d[3] + d[4]) / 5.0,
            (d[0] + d[1] + d[2] + d[3] + d[4] + d[5]) / 6.0,
        ];
        let mut a = d[0];
        let mut b = d[0];
        for &v in averages.iter().rev() {
            if v.abs() < b.abs() {
                a = b;
                b = v;
            }
        }
        let last = *mean_out;
        *mean_out = (a + b) / 2.0;
        if self.raw > 0.0 {
            self.raw -= 0.001;
        }
        if sample.abs() > threshold {
            self.gate = 1.0;
            self.raw = 2.0;
        } else {
            self.gate *= 0.999;
            if threshold > 0.0 {
                self.gate += (sample.abs() / threshold) * 0.001;
            }
        }
        if mean_out.abs() <= threshold && last.abs() <= threshold {
            self.stored[0] = self.stored[1] + (*mean_out * self.gate);
        }
        if self.raw < 1.0 {
            sample = (sample * self.raw) + (self.stored[0] * (1.0 - self.raw));
        }
        if wet != 1.0 {
            sample = (sample * wet) + (dry * (1.0 - wet));
        }
        fpd_dither(sample, &mut self.fpd) as f32
    }
}
struct DeHiss {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
}
impl DeHiss {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 1.0],
            sample_rate,
            seed,
            left: Channel::new(fpd_from_seed(seed, 0)),
            right: Channel::new(fpd_from_seed(seed, 1)),
        }
    }
}
impl AirwindowsEffect for DeHiss {
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
            .map_or_else(String::new, |&v| float2string(v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let threshold = (self.params[0] as f64).powf(9.0);
        let wet = self.params[1] as f64;
        let mut mean_l = 0.0;
        let mut mean_r = 0.0;
        for (((&l, &r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            *ol = self.left.process(l, threshold, wet, &mut mean_l);
            *or = self.right.process(r, threshold, wet, &mut mean_r);
        }
    }
}
