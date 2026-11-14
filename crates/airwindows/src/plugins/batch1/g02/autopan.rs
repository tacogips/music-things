//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/AutoPan/AutoPan.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Rate",
        label: "",
        default: 0.1,
    },
    ParamInfo {
        name: "Phase",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Wide",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "AutoPan",
    upstream_name: "AutoPan",
    description: "AutoPan is for getting some organic stereo movement into your mix.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(AutoPan::new(sample_rate, seed))
}
struct AutoPan {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    rate: f64,
    oldfpd: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl AutoPan {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.1, 0.5, 0.0, 1.0],
            sample_rate,
            seed,
            rate: 3.0,
            oldfpd: 429496.7295,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for AutoPan {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(index), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, index: usize) -> String {
        self.params
            .get(index)
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let drift = ((self.params[0] as f64).powf(4.0) * 0.01) / overallscale;
        let offset = std::f64::consts::PI + (self.params[1] as f64 * std::f64::consts::TAU);
        let panlaw = 1.0 - (self.params[2] as f64).powf(2.0);
        let wet = self.params[3] as f64;
        let len = in_l.len().min(in_r.len()).min(out_l.len()).min(out_r.len());
        for i in 0..len {
            let mut l = in_l[i] as f64;
            let mut r = in_r[i] as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            self.rate += self.oldfpd * drift;
            if self.rate > std::f64::consts::TAU {
                self.rate = 0.0;
                self.oldfpd = 0.4294967295 + self.fpd_l as f64 * 0.0000000000618;
            } else if self.rate < 0.0 {
                self.rate = std::f64::consts::TAU;
                self.oldfpd = 0.4294967295 + self.fpd_l as f64 * 0.0000000000618;
            }
            l *= self.rate.sin() + 1.0;
            r *= (self.rate + offset).sin() + 1.0;
            let mid = (l + r) * panlaw;
            let side = l - r;
            l = (mid + side) / 4.0;
            r = (mid - side) / 4.0;
            if wet != 1.0 {
                l = l * wet + dry_l * (1.0 - wet);
                r = r * wet + dry_r * (1.0 - wet);
            }
            out_l[i] = fpd_dither(l, &mut self.fpd_l) as f32;
            out_r[i] = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
