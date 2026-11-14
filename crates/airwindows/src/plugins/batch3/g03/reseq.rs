//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ResEQ, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Reso 1",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Reso 2",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Reso 3",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Reso 4",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Reso 5",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Reso 6",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Reso 7",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Reso 8",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "ResEQ",
    upstream_name: "ResEQ",
    description: "ResEQ is a bank of mostly midrange resonances.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ResEQ::new(sample_rate, seed))
}
struct ResEQ {
    params: [f32; 9],
    sample_rate: f32,
    seed: u32,
    history: [[f64; 61]; 2],
    kernel: [[f64; 61]; 2],
    frame: usize,
    fpd: [u32; 2],
}
impl ResEQ {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0; 9],
            sample_rate,
            seed,
            history: [[0.0; 61]; 2],
            kernel: [[0.0; 61]; 2],
            frame: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for ResEQ {
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
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let mut frequencies = [0.0; 8];
        let mut values = [0.0; 8];
        for i in 0..8 {
            let v = self.params[i] as f64;
            frequencies[i] = v.powf(2.0);
            values[i] = (v + 0.2) / overallscale;
        }
        for _ in 0..2 {
            self.frame += 1;
            if self.frame > 59 {
                self.frame = 1;
            }
            let frame = self.frame as f64;
            let falloff = (frame / 19.098992).sin();
            for lane in 0..2 {
                self.kernel[lane][self.frame] = 0.0;
                for i in 0..8 {
                    let f = frame * frequencies[i];
                    let term = if f < 1.57079633 {
                        (f * 2.0).sin()
                    } else {
                        f.cos()
                    };
                    self.kernel[lane][self.frame] += term * falloff * values[i];
                }
            }
        }
        let wet = self.params[8] as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            let mut outputs = [0.0f32; 2];
            for lane in 0..2 {
                if samples[lane].abs() < 1.18e-23 {
                    samples[lane] = self.fpd[lane] as f64 * 1.18e-17;
                }
                let dry = samples[lane];
                self.history[lane].copy_within(0..59, 1);
                self.history[lane][0] = dry;
                let mut sample = self.history[lane][1] * self.kernel[lane][1];
                for i in 2..60 {
                    sample += self.history[lane][i] * self.kernel[lane][i];
                }
                sample /= 12.0;
                if wet != 1.0 {
                    sample = sample * wet + dry * (1.0 - wet);
                }
                outputs[lane] = fpd_dither(sample, &mut self.fpd[lane]) as f32;
            }
            *out_l = outputs[0];
            *out_r = outputs[1];
        }
    }
}
