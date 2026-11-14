//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/NodeDither, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Node",
        label: "samples",
        default: 0.04,
    },
    ParamInfo {
        name: "Phase",
        label: " ",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "NodeDither",
    upstream_name: "NodeDither",
    description:
        "NodeDither is adjusta-TPDF-dither, like a dither flanger. Can do Paul and Tape settings.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(NodeDither::new(rate, seed))
}
struct NodeDither {
    params: [f32; 2],
    rate: f32,
    seed: u32,
    left: Box<[f64]>,
    right: Box<[f64]>,
    count: i32,
    fpd_l: u32,
    fpd_r: u32,
}
impl NodeDither {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.04, 0.0],
            rate,
            seed,
            left: vec![0.0; 5000].into_boxed_slice(),
            right: vec![0.0; 5000].into_boxed_slice(),
            count: 0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for NodeDither {
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
            0 => ((self.params[0] * 100.0).floor() as i32).to_string(),
            1 => {
                if (self.params[1] as f64 * 1.999) as i32 == 0 {
                    "Out".into()
                } else {
                    "In".into()
                }
            }
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.left.fill(0.0);
        self.right.fill(0.0);
        self.count = 0;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let offset = (((self.params[0] * 100.0) as f64 * (self.rate as f64 / 44100.0)) as i32)
            .clamp(1, 2440) as usize;
        let phase = (self.params[1] as f64 * 1.999).floor() as i32;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
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
            if self.count < 0 || self.count > 2450 {
                self.count = 2450;
            }
            let i = self.count as usize;
            let dl = self.fpd_l as f64 / u32::MAX as f64;
            let dr = self.fpd_r as f64 / u32::MAX as f64;
            l += dl;
            r += dr;
            if phase == 1 {
                l -= 1.0;
                l += self.left[i + offset];
                r -= 1.0;
                r += self.right[i + offset];
            } else {
                l -= self.left[i + offset];
                r -= self.right[i + offset];
            }
            l = l.floor();
            r = r.floor();
            self.left[i] = dl;
            self.left[i + 2450] = dl;
            self.right[i] = dr;
            self.right[i + 2450] = dr;
            self.count -= 1;
            xorshift(&mut self.fpd_l);
            xorshift(&mut self.fpd_r);
            *out_l = (l / 8388608.0) as f32;
            *out_r = (r / 8388608.0) as f32;
        }
    }
}
