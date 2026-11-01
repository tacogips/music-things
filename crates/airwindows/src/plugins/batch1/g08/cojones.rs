//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Cojones/CojonesProc.cpp, b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Exact; measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Cojones",
    upstream_name: "Cojones",
    description: "Cojones is a new kind of distorty.",
    params: &[
        ParamInfo {
            name: "Breathy",
            label: "",
            default: 0.5,
        },
        ParamInfo {
            name: "Cojones",
            label: "",
            default: 0.5,
        },
        ParamInfo {
            name: "Body",
            label: "",
            default: 0.5,
        },
        ParamInfo {
            name: "Output",
            label: "",
            default: 1.0,
        },
        ParamInfo {
            name: "Dry/Wet",
            label: "",
            default: 1.0,
        },
    ],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Cojones::new(rate, seed))
}
struct Cojones {
    rate: f32,
    seed: u32,
    params: [f32; 5],
    stored: [[f64; 2]; 2],
    diff: [[f64; 6]; 2],
    fpd: [u32; 2],
}
impl Cojones {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            rate,
            seed,
            params: [0.5, 0.5, 0.5, 1.0, 1.0],
            stored: [[0.0; 2]; 2],
            diff: [[0.0; 6]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Cojones {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(v)) {
            *p = v
        }
    }
    fn param_display(&self, i: usize) -> String {
        match self.params.get(i) {
            Some(v) => float2string(if i < 3 { (*v as f64 * 2.0) as f32 } else { *v }),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let breathy = self.params[0] as f64 * 2.0;
        let cojones = self.params[1] as f64 * 2.0;
        let body = self.params[2] as f64 * 2.0;
        let output = self.params[3] as f64;
        let wet = self.params[4] as f64;
        for (((&left, &right), out_l), out_r) in l.iter().zip(r).zip(ol).zip(orr) {
            let mut out = [0.0; 2];
            for (ch, raw) in [left, right].into_iter().enumerate() {
                let mut x = raw as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[ch] as f64 * 1.18e-17
                }
                let dry = x;
                let stored = &mut self.stored[ch];
                let diff = &mut self.diff[ch];
                stored[1] = stored[0];
                stored[0] = x;
                for i in (1..6).rev() {
                    diff[i] = diff[i - 1]
                }
                diff[0] = stored[0] - stored[1];
                let mut avg = [0.0; 5];
                avg[0] = diff[0] + diff[1];
                for i in 1..5 {
                    avg[i] = avg[i - 1] + diff[i + 1]
                }
                for (i, a) in avg.iter_mut().enumerate() {
                    *a /= (i + 2) as f64
                }
                let (mut ma, mut mb) = (diff[0], diff[0]);
                for i in (0..5).rev() {
                    if avg[i].abs() < mb.abs() {
                        ma = mb;
                        mb = avg[i]
                    }
                }
                stored[0] = stored[1] + (ma + mb) / 2.0;
                let mut y = stored[0] * body;
                y += ((x - stored[0]) - avg[1]) * cojones;
                y += avg[1] * breathy;
                if output < 1.0 {
                    y *= output
                }
                if wet < 1.0 {
                    y = y * wet + dry * (1.0 - wet)
                }
                out[ch] = fpd_dither(y, &mut self.fpd[ch]) as f32;
            }
            *out_l = out[0];
            *out_r = out[1];
        }
    }
}
