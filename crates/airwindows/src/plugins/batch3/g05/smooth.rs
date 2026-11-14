//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Smooth, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Smooth",
        label: "",
        default: 0.0,
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
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Smooth",
    upstream_name: "Smooth",
    description: "Smooth can tame pointy sounds or make drums explode.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Smooth::new(sample_rate, seed))
}
struct Smooth {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
    last_l: [f64; 5],
    last_r: [f64; 5],
    gain_l: [f64; 5],
    gain_r: [f64; 5],
}
impl Smooth {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 1.0, 1.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            last_l: [0.0; 5],
            last_r: [0.0; 5],
            gain_l: [1.0; 5],
            gain_r: [1.0; 5],
        }
    }
}
impl AirwindowsEffect for Smooth {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c] = self.params;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let mut chase = (a as f64).powf(2.0);
        let makeup = (1.0 + chase * 1.6) * b as f64;
        let ratio = chase * 24.0;
        chase /= overallscale;
        chase *= 0.083;
        let wet = c as f64;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            for i in 0..5 {
                let clamp = ((l - self.last_l[i]).abs() * ratio).sin();
                self.last_l[i] = l;
                self.gain_l[i] *= 1.0 - chase;
                self.gain_l[i] += (1.0 - clamp) * chase;
                self.gain_l[i] = self.gain_l[i].clamp(0.0, 1.0);
                if self.gain_l[i] != 1.0 {
                    l *= self.gain_l[i];
                }
            }
            for i in 0..5 {
                let clamp = ((r - self.last_r[i]).abs() * ratio).sin();
                self.last_r[i] = r;
                self.gain_r[i] *= 1.0 - chase;
                self.gain_r[i] += (1.0 - clamp) * chase;
                self.gain_r[i] = self.gain_r[i].clamp(0.0, 1.0);
                if self.gain_r[i] != 1.0 {
                    r *= self.gain_r[i];
                }
            }
            if makeup != 1.0 {
                l *= makeup;
                r *= makeup;
            }
            if wet != 1.0 {
                l = l * wet + dry_l * (1.0 - wet);
                r = r * wet + dry_r * (1.0 - wet);
            }
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
