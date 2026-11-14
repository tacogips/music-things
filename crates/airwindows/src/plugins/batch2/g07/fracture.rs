//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Fracture, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::{db2string, float2string};
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Drive",
        label: " ",
        default: 0.25,
    },
    ParamInfo {
        name: "Fracture",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "Out Lvl",
        label: "dB",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Fracture",
    upstream_name: "Fracture",
    description:
        "Fracture is a frequency multiplier/waveshaper with a soft disruption to the sound.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Fracture::new(rate, seed))
}
struct Fracture {
    params: [f32; 4],
    rate: f32,
    seed: u32,
    fpd: [u32; 2],
}
impl Fracture {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.25, 0.5, 1.0, 1.0],
            rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
// Preserve the upstream fracture factor exactly.
#[allow(clippy::approx_constant)]
const FRACTURE_FACTOR: f64 = 3.14159265358979;
impl AirwindowsEffect for Fracture {
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
            0 => float2string(self.params[0] * 4.0),
            1 => (((self.params[1] as f64 * 2.999).floor() as i32) + 1).to_string(),
            2 => db2string(self.params[2]),
            3 => float2string(self.params[3]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let mut density = (a * 4.0) as f64;
        density *= density.abs();
        let fracture = ((b as f64 * 2.999) + 1.0) * FRACTURE_FACTOR;
        let wet = d as f64;
        let dry = 1.0 - wet;
        for (((&l, &r), left), right) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut samples = [l as f64, r as f64];
            for (i, x) in samples.iter_mut().enumerate() {
                if x.abs() < 1.18e-23 {
                    *x = self.fpd[i] as f64 * 1.18e-17;
                }
                let original = *x;
                *x *= density;
                let bridge = (x.abs() * fracture).min(fracture).sin();
                *x = if *x > 0.0 { bridge } else { -bridge };
                *x *= c as f64;
                *x = (original * dry) + (*x * wet);
                *x = fpd_dither(*x, &mut self.fpd[i]);
            }
            *left = samples[0] as f32;
            *right = samples[1] as f32;
        }
    }
}
