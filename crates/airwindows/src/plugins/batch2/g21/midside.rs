//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/MidSide, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Mid/Side",
    label: " ",
    default: 0.5,
}];
pub(super) const INFO: PluginInfo = PluginInfo {
    name: "MidSide",
    upstream_name: "MidSide",
    description: "MidSide is the other side of the EdIsDim mid/side conversion utility plugin.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(MidSide::new(rate, seed))
}
struct MidSide {
    param: f32,
    rate: f32,
    seed: u32,
    fpd: [u32; 2],
}
impl MidSide {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            param: 0.5,
            rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for MidSide {
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
        if let (0, Some(v)) = (i, sanitize_param(v)) {
            self.param = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i == 0 {
            float2string(self.param)
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let param = self.param;
        *self = Self::new(self.rate, self.seed);
        self.param = param;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let sidegain = self.param as f64 * 2.0;
        let midgain = 2.0 - sidegain;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd[0] as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd[1] as f64 * 1.18e-17;
            }
            *out_l = fpd_dither((l + r) * midgain, &mut self.fpd[0]) as f32;
            *out_r = fpd_dither((l - r) * sidegain, &mut self.fpd[1]) as f32;
        }
    }
}
