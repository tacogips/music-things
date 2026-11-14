//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/MoNoam, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Monotor",
    label: "",
    default: 0.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "MoNoam",
    upstream_name: "MoNoam",
    description: "MoNoam gives you variations on mono, mid, and side in different channels.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(MoNoam::new(rate, seed))
}
struct MoNoam {
    param: f32,
    rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
}
impl MoNoam {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            param: 0.0,
            rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for MoNoam {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        if i == 0 {
            self.param
        } else {
            0.0
        }
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if i == 0 {
            if let Some(v) = sanitize_param(v) {
                self.param = v;
            }
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i != 0 {
            return String::new();
        }
        [
            "Bypass", "Mono", "MonoR", "MonoL", "Side", "SideM", "SideR", "SideL",
        ][(self.param as f64 * 7.999) as usize]
            .into()
    }
    fn reset(&mut self) {
        let param = self.param;
        *self = Self::new(self.rate, self.seed);
        self.param = param;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mode = (self.param as f64 * 7.999) as usize;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut left = l as f64;
            let mut right = r as f64;
            if left.abs() < 1.18e-23 {
                left = self.fpd_l as f64 * 1.18e-17;
            }
            if right.abs() < 1.18e-23 {
                right = self.fpd_r as f64 * 1.18e-17;
            }
            let mut mid = left + right;
            let mut side = left - right;
            if (1..=3).contains(&mode) {
                side = 0.0;
            }
            if (4..=7).contains(&mode) {
                mid = 0.0;
            }
            left = (mid + side) / 2.0;
            right = (mid - side) / 2.0;
            if (5..=7).contains(&mode) {
                left = -left;
            }
            if mode == 2 || mode == 6 {
                left = 0.0;
            }
            if mode == 3 || mode == 7 {
                right = 0.0;
            }
            if mode == 0 {
                left = l as f64;
                right = r as f64;
            }
            *out_l = fpd_dither(left, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(right, &mut self.fpd_r) as f32;
        }
    }
}
