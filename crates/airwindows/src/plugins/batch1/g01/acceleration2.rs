//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Acceleration2 at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
// Preserve upstream constants and operation order.
#![allow(clippy::approx_constant)]
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
pub(super) const INFO: PluginInfo = PluginInfo { name: "Acceleration2", upstream_name: "Acceleration2", description: "Acceleration2 is Acceleration, better! Updated for high sample rate and more extreme effect.", params: PARAMS };
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Acceleration2::new(rate, seed))
}
struct Acceleration2 {
    params: [f32; 2],
    rate: f32,
    seed: u32,
    samples: [[f64; 34]; 2],
    a: [f64; 11],
    b: [f64; 11],
    fpd: [u32; 2],
}
impl Acceleration2 {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.32, 1.0],
            rate,
            seed,
            samples: [[0.0; 34]; 2],
            a: [0.0; 11],
            b: [0.0; 11],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    fn coefficients(b: &mut [f64; 11], frequency: f64) {
        b[0] = frequency;
        b[1] = 0.7071;
        let k = (std::f64::consts::PI * b[0]).tan();
        let norm = 1.0 / (1.0 + k / b[1] + k * k);
        b[2] = k * k * norm;
        b[3] = 2.0 * b[2];
        b[4] = b[2];
        b[5] = 2.0 * (k * k - 1.0) * norm;
        b[6] = (1.0 - k / b[1] + k * k) * norm;
    }
    fn filter(b: &mut [f64; 11], channel: usize, input: f64) -> f64 {
        let i = 7 + 2 * channel;
        let temp = input * b[2] + b[i];
        b[i] = input * b[3] - temp * b[5] + b[i + 1];
        b[i + 1] = input * b[4] - temp * b[6];
        temp
    }
    fn sample(
        &mut self,
        channel: usize,
        sample: f32,
        spacing: usize,
        intensity: f64,
        wet: f64,
    ) -> f32 {
        let mut input = sample as f64;
        if input.abs() < 1.18e-23 {
            input = self.fpd[channel] as f64 * 1.18e-17;
        }
        let dry = input;
        let smooth = Self::filter(&mut self.a, channel, input);
        let history = &mut self.samples[channel];
        for count in (0..=spacing * 2).rev() {
            history[count + 1] = history[count];
        }
        history[0] = input;
        let d1 = history[0] - history[spacing];
        let d2 = history[spacing] - history[spacing * 2];
        let m1 = d1 * d1.abs();
        let m2 = d2 * d2.abs();
        let mut sense = intensity * intensity * (m1 - m2).abs();
        if sense > 1.0 {
            sense = 1.0;
        }
        input = input * (1.0 - sense) + smooth * sense;
        input = Self::filter(&mut self.b, channel, input);
        if wet != 1.0 {
            input = input * wet + dry * (1.0 - wet);
        }
        fpd_dither(input, &mut self.fpd[channel]) as f32
    }
}
impl AirwindowsEffect for Acceleration2 {
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
        let intensity = (self.params[0] as f64).powf(3.0) * 32.0;
        let wet = self.params[1] as f64;
        let spacing = (((1.73 * scale) as i32) + 1).min(16) as usize;
        Self::coefficients(
            &mut self.a,
            (20000.0 * (1.0 - (self.params[0] as f64 * 0.618_033_988_749_894_9)))
                / self.rate as f64,
        );
        Self::coefficients(&mut self.b, 20000.0 / self.rate as f64);
        for (((l, r), lo), ro) in il.iter().zip(ir).zip(ol).zip(or) {
            *lo = self.sample(0, *l, spacing, intensity, wet);
            *ro = self.sample(1, *r, spacing, intensity, wet);
        }
    }
}
