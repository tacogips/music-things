//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Wider, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)] // Preserve upstream coefficients.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Width",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Center",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Wider",
    upstream_name: "Wider",
    description: "Wider is Airwindows stereo space shaping.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Wider::new(sample_rate, seed))
}
struct Wider {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    p: Box<[f64]>,
    count: usize,
    fpd_l: u32,
    fpd_r: u32,
}
impl Wider {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 1.0],
            sample_rate,
            seed,
            p: vec![0.0; 4099].into_boxed_slice(),
            count: 0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn shape(mut sample: f64, density: f64) -> f64 {
    if density != 0.0 {
        let out = density.abs();
        let bridge = (sample.abs() * 1.57079633).min(1.57079633);
        let bridge = if density > 0.0 {
            bridge.sin()
        } else {
            1.0 - bridge.cos()
        };
        if sample > 0.0 {
            sample = sample * (1.0 - out) + bridge * out;
        } else {
            sample = sample * (1.0 - out) - bridge * out;
        }
    }
    sample
}
impl AirwindowsEffect for Wider {
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
            0 | 1 => float2string(self.params[i] * 2.0 - 1.0),
            2 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.p.fill(0.0);
        self.count = 0;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let overallscale = (1.0 / 44100.0) * self.sample_rate as f64;
        let densityside = self.params[0] as f64 * 2.0 - 1.0;
        let densitymid = self.params[1] as f64 * 2.0 - 1.0;
        let wet = self.params[2] as f64 * 0.5;
        let mut offset = (densityside - densitymid) / 2.0;
        if offset > 0.0 {
            offset = offset.sin();
        }
        if offset < 0.0 {
            offset = -(-offset).sin();
        }
        offset = -(offset.powf(4.0) * 20.0 * overallscale);
        let near = offset.abs().floor() as usize;
        let far_level = offset.abs() - near as f64;
        let far = near + 1;
        let near_level = 1.0 - far_level;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let mut mid = shape(l + r, densitymid);
            let mut side = shape(l - r, densityside);
            if self.count < 1 || self.count > 2048 {
                self.count = 2048;
            }
            if offset > 0.0 {
                self.p[self.count] = mid;
                self.p[self.count + 2048] = mid;
                mid = self.p[self.count + near] * near_level;
                mid += self.p[self.count + far] * far_level;
            }
            if offset < 0.0 {
                self.p[self.count] = side;
                self.p[self.count + 2048] = side;
                side = self.p[self.count + near] * near_level;
                side += self.p[self.count + far] * far_level;
            }
            self.count -= 1;
            *outl = fpd_dither(l * (1.0 - wet) + (mid + side) * wet, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r * (1.0 - wet) + (mid - side) * wet, &mut self.fpd_r) as f32;
        }
    }
}
