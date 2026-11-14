//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/TapeDither, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Quant",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "DeRez",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "TapeDither",
    upstream_name: "TapeDither",
    description: "TapeDither is TPDF dither with noise like reel-to-reel tape.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(TapeDither::new(sample_rate, seed))
}
struct TapeDither {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    history: [[f64; 4]; 2],
    fpd_l: u32,
    fpd_r: u32,
}
impl TapeDither {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 0.0],
            sample_rate,
            seed,
            history: [[0.0; 4]; 2],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for TapeDither {
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
            0 => {
                if self.params[0] as f64 * 1.999 < 1.0 {
                    "CD 16".into()
                } else {
                    "HD 24".into()
                }
            }
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let mut scale = if (self.params[0] as f64 * 1.999) as i32 == 1 {
            8388608.0f32
        } else {
            32768.0f32
        };
        if self.params[1] > 0.0 {
            scale = (scale as f64 * (1.0 - self.params[1] as f64).powf(6.0)) as f32;
        }
        if scale < 0.0001 {
            scale = 0.0001;
        }
        let out_scale = scale.max(8.0);
        for (((&l, &r), lo), ro) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            xorshift(&mut self.fpd_l);
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            xorshift(&mut self.fpd_r);
            let dl = self.fpd_l as f64 / u32::MAX as f64;
            let dr = self.fpd_r as f64 / u32::MAX as f64;
            l *= scale as f64;
            r *= scale as f64;
            l += dl;
            r += dr;
            l -= self.history[0][3];
            r -= self.history[1][3];
            l = l.floor();
            r = r.floor();
            self.history[0].copy_within(0..3, 1);
            self.history[0][0] = dl;
            self.history[1].copy_within(0..3, 1);
            self.history[1][0] = dr;
            xorshift(&mut self.fpd_l);
            xorshift(&mut self.fpd_r);
            *lo = (l / out_scale as f64) as f32;
            *ro = (r / out_scale as f64) as f32;
        }
    }
}
