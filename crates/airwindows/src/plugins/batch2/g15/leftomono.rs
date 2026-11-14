//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/LeftoMono at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.0 (channel copy).
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

const INFO: PluginInfo = PluginInfo {
    name: "LeftoMono",
    upstream_name: "LeftoMono",
    description: "LeftoMono copies one channel to both, losslessly.",
    params: &[],
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(_: f32, _: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(LeftoMono)
}
struct LeftoMono;
impl AirwindowsEffect for LeftoMono {
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
    fn reset(&mut self) {}
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        for (((&l, _), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            *out_l = l;
            *out_r = l;
        }
    }
}
