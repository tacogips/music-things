//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/FromTape, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Louder",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Softer",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Weight",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "FromTape",
    upstream_name: "FromTape",
    description: "FromTape is a minimalist, cleaner analog tape emulation.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(FromTape::new(sample_rate, seed))
}
struct FromTape {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    iir: [[f64; 26]; 2],
    roller: [[f64; 3]; 2],
    flip: usize,
    fpd: [u32; 2],
}
impl FromTape {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.5, 0.5, 1.0],
            sample_rate,
            seed,
            iir: [[0.0; 26]; 2],
            roller: [[0.0; 3]; 2],
            flip: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for FromTape {
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
        match i {
            0 | 3 => float2string(self.params[i] * 2.0),
            1 | 2 | 4 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    #[allow(clippy::approx_constant)] // Preserve the upstream coefficient.
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let scale = self.sample_rate as f64 / 44100.0;
        let inputgain = a as f64 * 2.0;
        let soften = b as f64;
        let roll = (1.0 - soften) / scale;
        let amount = (0.004 * (1.0 - c as f64)) / scale;
        let alt = 1.0 - amount;
        let outputgain = d as f64 * 2.0;
        let wet = e as f64;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut samples = [l as f64, r as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            let dry = samples;
            let randy = (self.fpd[0] as f64 / u32::MAX as f64) * soften;
            let inv = 1.0 - randy;
            let randy = randy / 2.0;
            if self.flip < 1 || self.flip > 3 {
                self.flip = 1;
            }
            let phase = self.flip - 1;
            for (ch, sample) in samples.iter_mut().enumerate() {
                if inputgain != 1.0 {
                    *sample *= inputgain;
                }
                let mut temp = *sample;
                let mut subtract = 0.0;
                for state in &mut self.iir[ch] {
                    *state = (*state * alt) + (temp * amount);
                    temp -= *state;
                    subtract += *state;
                }
                *sample -= subtract;
                let roller = &mut self.roller[ch];
                roller[phase] = (roller[phase] * (1.0 - roll)) + (*sample * roll);
                roller[phase] = match phase {
                    0 => (inv * roller[0]) + (randy * roller[1]) + (randy * roller[2]),
                    1 => (randy * roller[0]) + (inv * roller[1]) + (randy * roller[2]),
                    _ => (randy * roller[0]) + (randy * roller[1]) + (inv * roller[2]),
                };
                let high = *sample - roller[phase];
                let bridge = 1.0 - (high.abs() * 1.57079633).min(1.57079633).cos();
                if high > 0.0 {
                    *sample -= bridge;
                } else if high < 0.0 {
                    *sample += bridge;
                }
                if outputgain != 1.0 {
                    *sample *= outputgain;
                }
                if wet != 1.0 {
                    *sample = (*sample * wet) + (dry[ch] * (1.0 - wet));
                }
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            self.flip += 1;
            *outl = samples[0] as f32;
            *outr = samples[1] as f32;
        }
    }
}
