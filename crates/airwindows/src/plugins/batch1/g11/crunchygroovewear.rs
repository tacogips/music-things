//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/CrunchyGrooveWear, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.064,
    },
    ParamInfo {
        name: "Apply",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "CrunchyGrooveWear",
    upstream_name: "CrunchyGrooveWear",
    description: "CrunchyGrooveWear is a version of GrooveWear for more edge and distortion.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(CrunchyGrooveWear::new(rate, seed))
}
struct CrunchyGrooveWear {
    params: [f32; 2],
    rate: f32,
    seed: u32,
    mid: [[[f64; 20]; 2]; 4],
    prev: [[f64; 2]; 4],
    fpd: [u32; 2],
}
impl CrunchyGrooveWear {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.064, 1.0],
            rate,
            seed,
            mid: [[[0.0; 20]; 2]; 4],
            prev: [[0.0; 2]; 4],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for CrunchyGrooveWear {
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
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let scale = (self.params[0] as f64).powf(2.0) * 19.0 + 1.0;
        let mut gain = scale;
        let mut weights = [0.0; 20];
        for weight in &mut weights {
            if gain > 1.0 {
                *weight = 1.0;
                gain -= 1.0;
            } else {
                *weight = gain;
                gain = 0.0;
            }
        }
        for weight in &mut weights {
            *weight /= scale.max(1.0);
        }
        let d = self.params[1] as f64 * 4.0;
        let wet = if d < 1.0 {
            [d, 0.0, 0.0, 0.0]
        } else if d < 2.0 {
            [1.0, d - 1.0, 0.0, 0.0]
        } else if d < 3.0 {
            [1.0, 1.0, d - 2.0, 0.0]
        } else {
            [1.0, 1.0, 1.0, d - 3.0]
        };
        for i in 0..il.len().min(ir.len()).min(ol.len()).min(or.len()) {
            let input = [il[i], ir[i]];
            let mut out = [0.0; 2];
            for c in 0..2 {
                let mut x = input[c] as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[c] as f64 * 1.18e-17;
                }
                let mut dry = x;
                for (stage, &mix) in wet.iter().enumerate() {
                    if mix > 0.0 {
                        let mid = &mut self.mid[stage][c];
                        for j in (1..20).rev() {
                            mid[j] = mid[j - 1];
                        }
                        mid[0] = x - self.prev[stage][c];
                        let mut accumulator = mid[0] * weights[0];
                        for j in 1..20 {
                            accumulator += mid[j] * weights[j];
                        }
                        let correction = (x - self.prev[stage][c]) - accumulator;
                        self.prev[stage][c] = x;
                        x -= correction;
                        x = x * mix + dry * (1.0 - mix);
                        dry = x;
                    }
                }
                out[c] = fpd_dither(x, &mut self.fpd[c]) as f32;
            }
            ol[i] = out[0];
            or[i] = out[1];
        }
    }
}
