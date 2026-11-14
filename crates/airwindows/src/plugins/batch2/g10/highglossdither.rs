//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/HighGlossDither, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::dither::fpd_from_seed;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "HighGlossDither",
    upstream_name: "HighGlossDither",
    description: "HighGlossDither is a hybrid between shiny dither and truncation! Unnatural science experiment.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(HighGlossDither::new(sample_rate, seed))
}
struct HighGlossDither {
    sample_rate: f32,
    seed: u32,
    position: i32,
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl HighGlossDither {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            sample_rate,
            seed,
            position: 99999999,
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for HighGlossDither {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, _: usize) -> f32 {
        0.0
    }
    fn set_param(&mut self, _: usize, _: f32) {}
    fn param_display(&self, _: usize) -> String {
        String::new()
    }
    fn reset(&mut self) {
        *self = Self::new(self.sample_rate, self.seed);
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
            l *= 8388608.0;
            r *= 8388608.0;
            self.position = self.position.wrapping_add(1);
            let mut hot = (self.position.wrapping_mul(self.position) as u32) % 170003;
            hot = hot.wrapping_mul(hot) % 17011;
            hot = hot.wrapping_mul(hot) % 1709;
            hot = hot.wrapping_mul(hot) % 173;
            hot = hot.wrapping_mul(hot) % 17;
            let mut hot = (hot as f64 * 0.0635) as i32;
            if self.flip {
                hot = -hot;
            }
            l = (l + hot as f64).floor();
            r = (r + hot as f64).floor();
            self.flip = !self.flip;
            *outl = (l / 8388608.0) as f32;
            *outr = (r / 8388608.0) as f32;
        }
    }
}
