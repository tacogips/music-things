//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Facet, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Facet",
    label: "",
    default: 1.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Facet",
    upstream_name: "Facet",
    description: "Facet is a new kind of hard clip that adds two sharp corners into the wave.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Facet::new(rate, seed))
}
struct Facet {
    param: f32,
    rate: f32,
    seed: u32,
    l: u32,
    r: u32,
}
impl Facet {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            param: 1.0,
            rate,
            seed,
            l: fpd_from_seed(seed, 0),
            r: fpd_from_seed(seed, 1),
        }
    }
}
fn process_sample(input: f32, pos: f64, fpd: &mut u32) -> f32 {
    let mut sample = input as f64;
    if sample.abs() < 1.18e-23 {
        sample = *fpd as f64 * 1.18e-17;
    }
    if sample > pos {
        sample = (sample - pos) * pos + pos;
    }
    if sample < -pos {
        sample = (sample + pos) * pos - pos;
    }
    fpd_dither(sample, fpd) as f32
}
impl AirwindowsEffect for Facet {
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
    fn set_param(&mut self, i: usize, value: f32) {
        if i == 0 {
            if let Some(v) = sanitize_param(value) {
                self.param = v;
            }
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
        let p = self.param;
        *self = Self::new(self.rate, self.seed);
        self.param = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let pos = self.param as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(outr) {
            *out_l = process_sample(l, pos, &mut self.l);
            *out_r = process_sample(r, pos, &mut self.r);
        }
    }
}
