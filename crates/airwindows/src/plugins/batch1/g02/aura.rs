//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Aura/Aura.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Voicing",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Aura",
    upstream_name: "Aura",
    description: "Aura is a new kind of resonant lowpass EQ.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Aura::new(sample_rate, seed))
}
struct Aura {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    last: [f64; 2],
    previous_velocity: [f64; 2],
    buffer: [[f64; 21]; 2],
    weights: [f64; 21],
}
impl Aura {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 1.0],
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            last: [0.0; 2],
            previous_velocity: [0.0; 2],
            buffer: [[0.0; 21]; 2],
            weights: [0.0; 21],
        }
    }
}
impl AirwindowsEffect for Aura {
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
        let mut trim = self.params[0] as f64;
        let wet = self.params[1] as f64;
        let mut overallscale = trim * 10.0;
        let mut gain = overallscale + wet.powf(3.0) * 0.187859642462067;
        trim *= 1.0 - wet.powf(3.0) * 0.187859642462067;
        if gain < 1.0 {
            gain = 1.0;
        }
        for w in &mut self.weights[..20] {
            if gain > 1.0 {
                *w = 1.0;
                gain -= 1.0;
            } else {
                *w = gain;
                gain = 0.0;
            }
        }
        if overallscale < 1.0 {
            overallscale = 1.0;
        }
        for w in &mut self.weights[..20] {
            *w /= overallscale;
        }
        let len = in_l.len().min(in_r.len()).min(out_l.len()).min(out_r.len());
        for i in 0..len {
            let mut values = [in_l[i] as f64, in_r[i] as f64];
            for (ch, value) in values.iter_mut().enumerate() {
                if value.abs() < 1.18e-23 {
                    *value = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *value;
                let mut velocity = self.last[ch] - *value;
                let correction = self.previous_velocity[ch] - velocity;
                self.buffer[ch].copy_within(0..19, 1);
                self.buffer[ch][0] = correction;
                let mut accumulator = correction * self.weights[0];
                for j in 1..20 {
                    accumulator += self.buffer[ch][j] * self.weights[j];
                }
                velocity = self.previous_velocity[ch] + accumulator;
                *value = self.last[ch] + velocity;
                self.last[ch] = *value;
                self.previous_velocity[ch] = -velocity * trim.powf(2.0);
                if wet != 1.0 {
                    *value = *value * wet + dry * (1.0 - wet);
                }
                *value = fpd_dither(*value, &mut self.fpd[ch]);
            }
            out_l[i] = values[0] as f32;
            out_r[i] = values[1] as f32;
        }
    }
}
