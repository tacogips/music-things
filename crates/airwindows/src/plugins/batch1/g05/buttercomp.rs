//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ButterComp/ButterComp.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Compress",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "ButterComp",
    upstream_name: "ButterComp",
    description: "ButterComp is my softest, smoothest compressor.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ButterComp::new(sample_rate, seed))
}
struct ButterComp {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    target: [[f64; 2]; 2],
    control: [[f64; 2]; 2],
    fpd: [u32; 2],
}
impl ButterComp {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 1.0],
            sample_rate,
            seed,
            target: [[1.0; 2]; 2],
            control: [[1.0; 2]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn channel(
        &mut self,
        channel: usize,
        input: f32,
        gain: f64,
        output_gain: f64,
        divisor: f64,
        remainder: f64,
        wet: f64,
    ) -> f32 {
        let mut sample = f64::from(input);
        if sample.abs() < 1.18e-23 {
            sample = f64::from(self.fpd[channel]) * 1.18e-17;
        }
        let dry = sample;
        sample *= gain;
        let mut positive = sample + 1.0;
        if positive < 0.0 {
            positive = 0.0;
        }
        let mut output_positive = positive / 2.0;
        if output_positive > 1.0 {
            output_positive = 1.0;
        }
        positive *= positive;
        self.target[channel][0] *= divisor;
        self.target[channel][0] += positive * remainder;
        let calc_positive = (1.0 / self.target[channel][0]).powf(2.0);
        let mut negative = -sample + 1.0;
        if negative < 0.0 {
            negative = 0.0;
        }
        let mut output_negative = negative / 2.0;
        if output_negative > 1.0 {
            output_negative = 1.0;
        }
        negative *= negative;
        self.target[channel][1] *= divisor;
        self.target[channel][1] += negative * remainder;
        let calc_negative = (1.0 / self.target[channel][1]).powf(2.0);
        let active = usize::from(sample <= 0.0);
        self.control[channel][active] *= divisor;
        self.control[channel][active] += if active == 0 {
            calc_positive * remainder
        } else {
            calc_negative * remainder
        };
        let multiplier =
            self.control[channel][0] * output_positive + self.control[channel][1] * output_negative;
        sample *= multiplier;
        sample /= output_gain;
        if wet != 1.0 {
            sample = sample * wet + dry * (1.0 - wet);
        }
        fpd_dither(sample, &mut self.fpd[channel]) as f32
    }
}
impl AirwindowsEffect for ButterComp {
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
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= f64::from(self.sample_rate);
        let gain = 10.0_f64.powf((f64::from(self.params[0]) * 14.0) / 20.0);
        let mut output_gain = gain;
        output_gain -= 1.0;
        output_gain /= 1.5;
        output_gain += 1.0;
        let mut divisor = 0.012 * (f64::from(self.params[0]) / 135.0);
        divisor /= scale;
        let remainder = divisor;
        divisor = 1.0 - divisor;
        let wet = f64::from(self.params[1]);
        for (((l, r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            *ol = self.channel(0, *l, gain, output_gain, divisor, remainder, wet);
            *or = self.channel(1, *r, gain, output_gain, divisor, remainder, wet);
        }
    }
}
