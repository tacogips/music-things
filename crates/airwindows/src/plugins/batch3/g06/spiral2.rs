//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Spiral2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Input",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Highpass",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Presence",
        label: "",
        default: 0.5,
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
    name: "Spiral2",
    upstream_name: "Spiral2",
    description: "Spiral2 is Spiral with controls including Presence.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Spiral2::new(sample_rate, seed))
}
struct Spiral2 {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    iir_a: [f64; 2],
    iir_b: [f64; 2],
    prev: [f64; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl Spiral2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.0, 0.5, 1.0, 1.0],
            sample_rate,
            seed,
            iir_a: [0.0; 2],
            iir_b: [0.0; 2],
            prev: [0.0; 2],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Spiral2 {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let gain = (a as f64 * 2.0).powf(2.0);
        let amount = (b as f64).powf(3.0) / overallscale;
        let presence = c as f64;
        let output = d as f64;
        let wet = e as f64;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            for (ch, sample) in input.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *sample;
                if gain != 1.0 {
                    *sample *= gain;
                    self.prev[ch] *= gain;
                }
                let iir = if self.flip {
                    &mut self.iir_a[ch]
                } else {
                    &mut self.iir_b[ch]
                };
                *iir = (*iir * (1.0 - amount)) + (*sample * amount);
                *sample -= *iir;
                let p = self.prev[ch].abs();
                let mut ps = (*sample * p).sin() / if self.prev[ch] == 0.0 { 1.0 } else { p };
                *sample = (*sample * sample.abs()).sin()
                    / if sample.abs() == 0.0 {
                        1.0
                    } else {
                        sample.abs()
                    };
                if output < 1.0 {
                    *sample *= output;
                    ps *= output;
                }
                if presence > 0.0 {
                    *sample = (*sample * (1.0 - presence)) + (ps * presence);
                }
                if wet < 1.0 {
                    *sample = (dry * (1.0 - wet)) + (*sample * wet);
                }
                self.prev[ch] = dry;
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            self.flip = !self.flip;
            *outl = input[0] as f32;
            *outr = input[1] as f32;
        }
    }
}
