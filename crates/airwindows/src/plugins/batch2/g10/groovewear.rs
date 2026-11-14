//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/GrooveWear, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Wear",
        label: "",
        default: 0.064,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "GrooveWear",
    upstream_name: "GrooveWear",
    description: "GrooveWear is for scrubbing highs off mechanically like a stylus would.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(GrooveWear::new(sample_rate, seed))
}
struct GrooveWear {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    mid: [[[f64; 21]; 2]; 4],
    previous: [[f64; 2]; 4],
    fpd: [u32; 2],
}
impl GrooveWear {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.064, 1.0],
            sample_rate,
            seed,
            mid: [[[0.0; 21]; 2]; 4],
            previous: [[0.0; 2]; 4],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for GrooveWear {
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
        let overallscale = (self.params[0] as f64).powf(2.0) * 19.0 + 1.0;
        let mut gain = overallscale;
        let mut weights = [0.0; 20];
        for weight in &mut weights {
            if gain > 1.0 {
                *weight = 1.0;
                gain -= 1.0;
            } else {
                *weight = gain;
                gain = 0.0;
            }
            *weight /= overallscale.max(1.0);
        }
        let mut wet = [1.0, 1.0, 1.0, self.params[1] as f64 * 4.0];
        if wet[3] < 1.0 {
            wet = [wet[3], 0.0, 0.0, 0.0];
        } else if wet[3] < 2.0 {
            wet = [1.0, wet[3] - 1.0, 0.0, 0.0];
        } else if wet[3] < 3.0 {
            wet = [1.0, 1.0, wet[3] - 2.0, 0.0];
        } else {
            wet[3] -= 3.0;
        }
        for w in &mut wet {
            *w *= 0.5;
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut sample = [l as f64, r as f64];
            for (channel, value) in sample.iter_mut().enumerate() {
                if value.abs() < 1.18e-23 {
                    *value = self.fpd[channel] as f64 * 1.18e-17;
                }
            }
            for (stage, &w) in wet.iter().enumerate() {
                if w > 0.0 {
                    for (channel, value) in sample.iter_mut().enumerate() {
                        let dry = *value;
                        let history = &mut self.mid[stage][channel];
                        history.copy_within(0..19, 1);
                        history[0] = *value - self.previous[stage][channel];
                        let mut accumulator = history[0] * weights[0];
                        for i in 1..20 {
                            accumulator += history[i] * weights[i];
                        }
                        let correction = (*value - self.previous[stage][channel]) - accumulator;
                        self.previous[stage][channel] = *value;
                        *value -= correction;
                        *value = (*value * w) + (dry * (1.0 - w));
                    }
                }
            }
            *out_l = fpd_dither(sample[0], &mut self.fpd[0]) as f32;
            *out_r = fpd_dither(sample[1], &mut self.fpd[1]) as f32;
        }
    }
}
