//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Golem, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Balance",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Offset",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Phase",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Golem",
    upstream_name: "Golem",
    description: "Golem lets you blend a stereo track of two mics on an amp.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Golem::new(sample_rate, seed))
}
struct Golem {
    params: [f32; 3],
    seed: u32,
    delay: Box<[f64]>,
    count: usize,
    fpd_l: u32,
    fpd_r: u32,
}
impl Golem {
    fn new(_sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.0],
            seed,
            delay: vec![0.0; 4098].into_boxed_slice(),
            count: 0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Golem {
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
            0 | 1 => float2string((self.params[i] as f64 * 2.0 - 1.0) as f32),
            2 => ["In/Mic", "Out/Mic", "In/DI", "Out/DI", "DefTime", "DefGain"]
                [(self.params[2] as f64 * 5.999) as usize]
                .into(),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.delay.fill(0.0);
        self.count = 0;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c] = self.params;
        let phase = (c as f64 * 5.999 + 1.0) as i32;
        let balance = (a as f64 * 2.0 - 1.0) / 2.0;
        let mut gain_l = 0.5 - balance;
        let mut gain_r = 0.5 + balance;
        let range = if phase == 3 || phase == 4 {
            700.0
        } else {
            30.0
        };
        let mut offset = (b as f64 * 2.0 - 1.0).powi(5) * range;
        if phase > 4 {
            offset = 0.0;
        }
        if phase > 5 {
            gain_l = 0.5;
            gain_r = 0.5;
        }
        let near = offset.abs().floor() as usize;
        let far_level = offset.abs() - near as f64;
        let near_level = 1.0 - far_level;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            if phase == 2 || phase == 4 {
                l = -l;
            }
            l *= gain_l;
            r *= gain_r;
            if self.count < 1 || self.count > 2048 {
                self.count = 2048;
            }
            if offset > 0.0 {
                self.delay[self.count] = l;
                self.delay[self.count + 2048] = l;
                l = self.delay[self.count + near] * near_level
                    + self.delay[self.count + near + 1] * far_level;
            }
            if offset < 0.0 {
                self.delay[self.count] = r;
                self.delay[self.count + 2048] = r;
                r = self.delay[self.count + near] * near_level
                    + self.delay[self.count + near + 1] * far_level;
            }
            self.count -= 1;
            let mono = l + r;
            *out_l = fpd_dither(mono, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(mono, &mut self.fpd_r) as f32;
        }
    }
}
