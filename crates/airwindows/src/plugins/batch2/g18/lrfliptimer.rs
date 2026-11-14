//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/LRFlipTimer at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact; measured max-abs 0.0.
use crate::display::float2string;
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const INFO: PluginInfo = PluginInfo {
    name: "LRFlipTimer",
    upstream_name: "LRFlipTimer",
    description: "LRFlipTimer is a utility that swaps Left with Right every few (1-10) minutes.",
    params: &[ParamInfo {
        name: "Minutes",
        label: "",
        default: 0.0,
    }],
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, _: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(LRFlipTimer {
        rate,
        param: 0.0,
        tick: 0,
    })
}
struct LRFlipTimer {
    rate: f32,
    param: f32,
    tick: u32,
}
impl AirwindowsEffect for LRFlipTimer {
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
            float2string((self.param as f64 * 9.0 + 1.0) as f32)
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        self.tick = 0;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let time = ((self.param as f64 * 9.0 + 1.0) * 60.0 * self.rate as f64) as u32;
        let transition = (self.rate as f64 * 0.1) as u32;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            self.tick = self.tick.wrapping_add(1);
            if self.tick > time.wrapping_mul(2) {
                self.tick = 0;
            }
            let (a, b, offset) = if self.tick < time {
                (r, l, self.tick)
            } else {
                (l, r, self.tick - time)
            };
            if offset < transition {
                let fade = offset as f64 / transition as f64;
                let (a, b, fade) = if fade < 0.5 {
                    (a, b, fade)
                } else {
                    (b, a, (fade - 1.0).abs())
                };
                *out_l = (a as f64 * (1.0 - fade) + b as f64 * fade) as f32;
                *out_r = (b as f64 * (1.0 - fade) + a as f64 * fade) as f32;
            } else {
                *out_l = b;
                *out_r = a;
            }
        }
    }
}
