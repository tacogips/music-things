//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/SingleEndedTriode, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)] // Preserve upstream DSP constants and state ordering.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Triode",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Clas AB",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Clas B",
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
    name: "SingleEndedTriode",
    upstream_name: "SingleEndedTriode",
    description: "SingleEndedTriode is unusual analog modeling effects.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(SingleEndedTriode::new(sample_rate, seed))
}
struct SingleEndedTriode {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
}
impl SingleEndedTriode {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0; 4],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for SingleEndedTriode {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let triode = (self.params[0] as f64).powf(2.0) * 8.0;
        let intensity = triode + 0.001;
        let soft = (self.params[1] as f64).powf(3.0) / 8.0;
        let hard = (self.params[2] as f64).powf(7.0) / 8.0;
        let wet = self.params[3] as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut input = [l as f64, r as f64];
            if input[0].abs() < 1.18e-23 {
                input[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if input[1].abs() < 1.18e-23 {
                input[1] = self.fpd_r as f64 * 1.18e-17;
            }
            for x in &mut input {
                let dry = *x;
                if triode > 0.0 {
                    *x *= intensity;
                    *x -= 0.5;
                    let bridge = x.abs().min(1.57079633).sin();
                    *x = if *x > 0.0 { bridge } else { -bridge };
                    *x += 0.5_f64.sin();
                    *x /= intensity;
                }
                if soft > 0.0 {
                    let mut bridge = x.abs();
                    if bridge > 0.0 {
                        bridge -= soft * (bridge + bridge.sqrt());
                    }
                    if bridge < 0.0 {
                        bridge = 0.0;
                    }
                    *x = if *x > 0.0 { bridge } else { -bridge };
                }
                if hard > 0.0 {
                    let mut bridge = x.abs();
                    bridge -= hard;
                    if bridge < 0.0 {
                        bridge = 0.0;
                    }
                    *x = if *x > 0.0 { bridge } else { -bridge };
                }
                if wet != 1.0 {
                    *x = (*x * wet) + (dry * (1.0 - wet));
                }
            }
            *out_l = fpd_dither(input[0], &mut self.fpd_l) as f32;
            *out_r = fpd_dither(input[1], &mut self.fpd_r) as f32;
        }
    }
}
