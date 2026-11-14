//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Acceleration at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Limit",
        label: "",
        default: 0.32,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(super) const INFO: PluginInfo = PluginInfo {
    name: "Acceleration",
    upstream_name: "Acceleration",
    description: "Acceleration is an acceleration limiter that tames edge, leaves brightness.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Acceleration::new(rate, seed))
}
#[derive(Default)]
struct Channel {
    samples: [f64; 3],
    senses: [f64; 3],
    last: f64,
}
struct Acceleration {
    params: [f32; 2],
    rate: f32,
    seed: u32,
    channels: [Channel; 2],
    fpd: [u32; 2],
}
impl Acceleration {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.32, 1.0],
            rate,
            seed,
            channels: [Channel::default(), Channel::default()],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    fn sample(channel: &mut Channel, fpd: &mut u32, sample: f32, intensity: f64, wet: f64) -> f32 {
        let mut input = sample as f64;
        if input.abs() < 1.18e-23 {
            input = *fpd as f64 * 1.18e-17;
        }
        let dry = input;
        channel.samples[2] = channel.samples[1];
        channel.samples[1] = channel.samples[0];
        channel.samples[0] = input;
        let [s1, s2, s3] = channel.samples;
        let smooth = (s3 + s2 + s1) / 3.0;
        let m1 = (s1 - s2) * ((s1 - s2) / 1.3);
        let m2 = (s2 - s3) * ((s1 - s2) / 1.3);
        let mut sense = intensity * intensity * (m1 - m2).abs();
        channel.senses[2] = channel.senses[1];
        channel.senses[1] = channel.senses[0];
        channel.senses[0] = sense;
        if channel.senses[1] > sense {
            sense = channel.senses[1];
        }
        if channel.senses[2] > sense {
            sense = channel.senses[2];
        }
        if sense > 1.0 {
            sense = 1.0;
        }
        input *= 1.0 - sense;
        input += smooth * sense;
        sense /= 2.0;
        let accumulator = channel.last * sense + input * (1.0 - sense);
        channel.last = input;
        input = accumulator;
        if wet != 1.0 {
            input = input * wet + dry * (1.0 - wet);
        }
        fpd_dither(input, fpd) as f32
    }
}
impl AirwindowsEffect for Acceleration {
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
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.rate as f64;
        let intensity = (self.params[0] as f64).powf(3.0) * (32.0 / scale);
        let wet = self.params[1] as f64;
        for (((l, r), lo), ro) in il.iter().zip(ir).zip(ol).zip(or) {
            *lo = Self::sample(&mut self.channels[0], &mut self.fpd[0], *l, intensity, wet);
            *ro = Self::sample(&mut self.channels[1], &mut self.fpd[1], *r, intensity, wet);
        }
    }
}
