//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Tremolo, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Speed",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "Depth",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Tremolo",
    upstream_name: "Tremolo",
    description: "Tremolo is fluctuating saturation curves for a tubey tremolo.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Tremolo::new(sample_rate, seed))
}
struct Tremolo {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
    sweep: f64,
    speed_amount: f64,
    depth_amount: f64,
    last_speed: f64,
    last_depth: f64,
}
impl Tremolo {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 1.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            sweep: std::f64::consts::PI / 2.0,
            speed_amount: 1.0,
            depth_amount: 0.0,
            last_speed: 1000.0,
            last_depth: 1000.0,
        }
    }
}
impl AirwindowsEffect for Tremolo {
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
        let overallscale = (1.0 / 44100.0) * self.sample_rate as f64;
        let speed_chase = (self.params[0] as f64).powf(4.0);
        let depth_chase = self.params[1] as f64;
        let speed_speed = 300.0 / ((self.last_speed - speed_chase).abs() + 1.0);
        let depth_speed = 300.0 / ((self.last_depth - depth_chase).abs() + 1.0);
        self.last_speed = speed_chase;
        self.last_depth = depth_chase;
        for (((&left, &right), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = left as f64;
            let mut r = right as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            self.speed_amount =
                ((self.speed_amount * speed_speed) + speed_chase) / (speed_speed + 1.0);
            self.depth_amount =
                ((self.depth_amount * depth_speed) + depth_chase) / (depth_speed + 1.0);
            let mut speed = 0.0001 + (self.speed_amount / 1000.0);
            speed /= overallscale;
            let depth = 1.0 - (1.0 - self.depth_amount).powf(5.0);
            let skew = 1.0 + self.depth_amount.powf(9.0);
            let density = ((1.0 - self.depth_amount) * 2.0) - 1.0;
            let offset = self.sweep.sin();
            self.sweep += speed;
            if self.sweep > std::f64::consts::PI {
                self.sweep -= std::f64::consts::PI;
            }
            let mut control = offset.abs();
            if density > 0.0 {
                let temp = control.sin();
                control = (control * (1.0 - density)) + (temp * density);
            } else {
                let temp = 1.0 - control.cos();
                control = (control * (1.0 + density)) + (temp * -density);
            }
            let thickness = ((control * 2.0) - 1.0) * skew;
            let out = thickness.abs();
            let apply = |mut input: f64, dry: f64| {
                let mut bridge = input.abs();
                if bridge > 1.57079633 {
                    bridge = 1.57079633;
                }
                if thickness > 0.0 {
                    bridge = bridge.sin();
                } else {
                    bridge = 1.0 - bridge.cos();
                }
                if input > 0.0 {
                    input = (input * (1.0 - out)) + (bridge * out);
                } else {
                    input = (input * (1.0 - out)) - (bridge * out);
                }
                input *= 1.0 - control;
                input *= 2.0;
                (dry * (1.0 - depth)) + (input * depth)
            };
            l = apply(l, dry_l);
            r = apply(r, dry_r);
            l = fpd_dither(l, &mut self.fpd_l);
            r = fpd_dither(r, &mut self.fpd_r);
            *out_l = l as f32;
            *out_r = r as f32;
        }
    }
}
