//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Remap, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Remap",
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
    name: "Remap",
    upstream_name: "Remap",
    description: "Remap puts the guts back into overloudenated audio!",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Remap::new(sample_rate, seed))
}
struct Remap {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
}
impl Remap {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 1.0, 1.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
#[allow(clippy::approx_constant)]
fn map(mut sample: f64, drive: f64, gain: f64, out: f64, wet: f64) -> f64 {
    let dry = sample;
    let mut trim = sample.abs();
    let mut rect = trim * gain;
    if trim > 1.0 {
        trim = 1.0;
    } else {
        trim *= trim;
    }
    if sample > 1.57079633 {
        rect = 1.0 - (1.0 - rect.sin());
    } else {
        rect = rect.sin();
    }
    if sample > 0.0 {
        sample -= rect * trim * drive;
    } else {
        sample += rect * trim * drive;
    }
    if out < 1.0 {
        sample *= out;
    }
    if wet < 1.0 {
        sample = sample * wet + dry * (1.0 - wet);
    }
    sample
}
impl AirwindowsEffect for Remap {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let drive = self.params[0] as f64 * 0.5;
        let gain = (drive + 0.2) * 8.0;
        let out = self.params[1] as f64;
        let wet = self.params[2] as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            *out_l = fpd_dither(map(l, drive, gain, out, wet), &mut self.fpd_l) as f32;
            *out_r = fpd_dither(map(r, drive, gain, out, wet), &mut self.fpd_r) as f32;
        }
    }
}
