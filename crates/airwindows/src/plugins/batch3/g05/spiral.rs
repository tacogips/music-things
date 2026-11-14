//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Spiral, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Spiral",
    upstream_name: "Spiral",
    description: "Spiral is the new best smoothest distortion algorithm.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Spiral::new(sample_rate, seed))
}
struct Spiral {
    params: [f32; 0],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
}
impl Spiral {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Spiral {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, _: usize, _: f32) {}
    fn param_display(&self, _: usize) -> String {
        String::new()
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            l = (l * l.abs()).sin() / if l.abs() == 0.0 { 1.0 } else { l.abs() };
            r = (r * r.abs()).sin() / if r.abs() == 0.0 { 1.0 } else { r.abs() };
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
