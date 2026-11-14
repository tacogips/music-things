//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/XHighpass, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Gain",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Freq",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Nuke",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "XHighpass",
    upstream_name: "XHighpass",
    description: "XHighpass is a distorted digital EQ, inspired by retro sampler DSP.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(XHighpass::new(sample_rate, seed))
}
struct XHighpass {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    stages: [[f64; 15]; 5],
    fpd_l: u32,
    fpd_r: u32,
}
impl XHighpass {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 1.0, 0.0, 1.0],
            sample_rate,
            seed,
            stages: [[0.0; 15]; 5],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
#[allow(clippy::approx_constant)]
fn stage(q: &mut [f64; 15], input: f64, i: usize, clip_threshold: f64) -> f64 {
    let mut out = q[2] * input + q[3] * q[i] + q[4] * q[i + 1] - q[5] * q[i + 2] - q[6] * q[i + 3];
    q[i + 1] = q[i];
    q[i] = input;
    q[i + 3] = q[i + 2];
    if out > clip_threshold {
        out = clip_threshold;
    }
    if out < -clip_threshold {
        out = -clip_threshold;
    }
    q[i + 2] = out;
    out
}
impl AirwindowsEffect for XHighpass {
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
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    // Preserve the upstream truncated filter coefficient.
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let gain = (a as f64 + 0.5).powf(4.0);
        let frequency = ((b as f64).powf(5.0) * 20000.0 / self.sample_rate as f64).max(0.001);
        let clip_threshold = 1.0 - (c * b.sqrt()) as f64;
        let k = (std::f64::consts::PI * frequency).tan();
        let norm = 1.0 / (1.0 + k / 0.7071 + k * k);
        let mut q = [0.0; 7];
        q[0] = frequency;
        q[2] = norm;
        q[3] = -2.0 * q[2];
        q[4] = q[2];
        q[5] = 2.0 * (k * k - 1.0) * norm;
        q[6] = (1.0 - k / 0.7071 + k * k) * norm;
        for stage in &mut self.stages {
            stage[..7].copy_from_slice(&q);
        }
        let mut wet = [1.0, 1.0, 1.0, c as f64 * 4.0];
        if wet[3] < 1.0 {
            wet = [wet[3], 0.0, 0.0, 0.0];
        } else if wet[3] < 2.0 {
            wet = [1.0, wet[3] - 1.0, 0.0, 0.0];
        } else if wet[3] < 3.0 {
            wet = [1.0, 1.0, wet[3] - 2.0, 0.0];
        } else {
            wet[3] -= 3.0;
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            if gain != 1.0 {
                l *= gain;
                r *= gain;
            }
            l = stage(&mut self.stages[0], l, 7, clip_threshold);
            r = stage(&mut self.stages[0], r, 11, clip_threshold);
            for (idx, amount) in wet.into_iter().enumerate() {
                if amount > 0.0 {
                    let next_l = stage(&mut self.stages[idx + 1], l, 7, clip_threshold);
                    l = next_l * amount + l * (1.0 - amount);
                    let next_r = stage(&mut self.stages[idx + 1], r, 11, clip_threshold);
                    r = next_r * amount + r * (1.0 - amount);
                }
            }
            if d < 1.0 {
                l = dry_l * (1.0 - d as f64) + l * d as f64;
                r = dry_r * (1.0 - d as f64) + r * d as f64;
            }
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
