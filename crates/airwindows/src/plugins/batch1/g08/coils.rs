//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Coils/CoilsProc.cpp, b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Coils",
    upstream_name: "Coils",
    description: "Coils models the types of distortion you'll find in transformers.",
    params: &[
        ParamInfo {
            name: "Saturat",
            label: "",
            default: 0.0,
        },
        ParamInfo {
            name: "Core DC",
            label: "",
            default: 0.5,
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
    Box::new(Coils::new(rate, seed))
}
struct Coils {
    rate: f32,
    seed: u32,
    params: [f32; 3],
    figure: [[f64; 9]; 2],
    fpd: [u32; 2],
}
impl Coils {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            rate,
            seed,
            params: [0.0, 0.5, 1.0],
            figure: [[0.0; 9]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Coils {
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
        match i {
            0 | 2 => float2string(self.params[i]),
            1 => float2string((self.params[1] as f64 * 2.0 - 1.0) as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let boost = (1.0 - (self.params[0] as f64).powf(2.0)).max(0.001);
        let offset = self.params[1] as f64 * 2.0 - 1.0;
        let sin_offset = offset.sin();
        let wet = self.params[2] as f64;
        let k = (std::f64::consts::PI * (600.0 / self.rate as f64)).tan();
        let norm = 1.0 / (1.0 + k / 0.023 + k * k);
        let b2 = k / 0.023 * norm;
        let b4 = -b2;
        let b5 = 2.0 * (k * k - 1.0) * norm;
        let b6 = (1.0 - k / 0.023 + k * k) * norm;
        for (((&left, &right), out_l), out_r) in l.iter().zip(r).zip(ol).zip(orr) {
            let mut out = [0.0; 2];
            for (ch, raw) in [left, right].into_iter().enumerate() {
                let mut x = raw as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[ch] as f64 * 1.18e-17
                }
                let dry = x;
                let f = &mut self.figure[ch];
                let t = x * b2 + f[7];
                f[7] = -(t * b5) + f[8];
                f[8] = x * b4 - t * b6;
                x = t + ((((dry - t) / boost + offset).sin() - sin_offset) * boost);
                if wet != 1.0 {
                    x = x * wet + dry * (1.0 - wet)
                }
                out[ch] = fpd_dither(x, &mut self.fpd[ch]) as f32;
            }
            *out_l = out[0];
            *out_r = out[1];
        }
    }
}
