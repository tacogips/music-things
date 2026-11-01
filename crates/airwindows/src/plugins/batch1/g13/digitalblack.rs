//! DigitalBlack, upstream airwindows/plugins/LinuxVST/src/DigitalBlack at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "DigitalBlack",
    upstream_name: "DigitalBlack",
    description: "DigitalBlack is a quick, staccato gate.",
    params: &[
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
    ],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DigitalBlack::new(rate, seed))
}
struct DigitalBlack {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    negative: [bool; 2],
    crossing: [i32; 2],
    roller: [f64; 2],
    fpd: [u32; 2],
}
impl DigitalBlack {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 1.0],
            sample_rate,
            seed,
            negative: [false; 2],
            crossing: [0; 2],
            roller: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for DigitalBlack {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(value)) {
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
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let threshold = (self.params[0] as f64).powf(4.0) / 3.0 + 0.00018;
        let release = 0.0064 / overallscale;
        let wet = self.params[1] as f64;
        for (((l, r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [*l as f64, *r as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *sample;
                if *sample > 0.0 {
                    if self.negative[ch] {
                        self.crossing[ch] = 0;
                    }
                    self.negative[ch] = false;
                } else {
                    self.crossing[ch] += 1;
                    self.negative[ch] = true;
                }
                if self.crossing[ch] > 6000 {
                    self.crossing[ch] = 6000;
                }
                if sample.abs() > threshold {
                    if self.roller[ch] < self.crossing[ch] as f64 {
                        self.roller[ch] = self.crossing[ch] as f64;
                    }
                } else {
                    self.roller[ch] -= release;
                }
                if self.roller[ch] < 0.0 {
                    self.roller[ch] = 0.0;
                }
                let gate = if self.roller[ch] < 1.0 {
                    self.roller[ch]
                } else {
                    1.0
                };
                let rectifier = 1.0 - sample.abs().cos();
                if *sample > 0.0 {
                    *sample = (*sample * gate) + (rectifier * (1.0 - gate));
                } else {
                    *sample = (*sample * gate) - (rectifier * (1.0 - gate));
                }
                if gate == 0.0 && (*sample * 3.0).abs() < threshold {
                    *sample = 0.0;
                }
                if wet != 1.0 {
                    *sample = (*sample * wet) + (dry * (1.0 - wet));
                }
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            *out_l = samples[0] as f32;
            *out_r = samples[1] as f32;
        }
    }
}
