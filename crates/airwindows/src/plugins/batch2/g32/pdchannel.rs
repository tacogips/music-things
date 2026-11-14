//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PDChannel, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.

use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PDChannel",
    upstream_name: "PDChannel",
    description: "PDChannel is Console5 and PurestDrive, sittin’ in a tree… (i.e. both at once, on channels and buss)",
    params: &[
        ParamInfo { name: "Gain", label: "", default: 1.0 },
        ParamInfo { name: "Drive", label: "", default: 1.0 },
    ],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PDChannel::new(sample_rate, seed))
}
struct PDChannel {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    gainchase: f64,
    settingchase: f64,
    chasespeed: f64,
    previous_l: f64,
    previous_r: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl PDChannel {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0; 2],
            sample_rate,
            seed,
            gainchase: -90.0,
            settingchase: -90.0,
            chasespeed: 350.0,
            previous_l: 0.0,
            previous_r: 0.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for PDChannel {
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
            .map_or_else(String::new, |v| crate::display::float2string(*v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let n = il.len().min(ir.len()).min(ol.len()).min(or.len());
        let inputgain = f64::from(self.params[0]);
        let intensity = f64::from(self.params[1]);
        if self.settingchase != inputgain {
            self.chasespeed *= 2.0;
            self.settingchase = inputgain;
        }
        if self.chasespeed > 2500.0 {
            self.chasespeed = 2500.0;
        }
        if self.gainchase < 0.0 {
            self.gainchase = inputgain;
        }
        for i in 0..n {
            let mut l = f64::from(il[i]);
            let mut r = f64::from(ir[i]);
            if l.abs() < 1.18e-23 {
                l = f64::from(self.fpd_l) * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = f64::from(self.fpd_r) * 1.18e-17;
            }
            self.chasespeed *= 0.9999;
            self.chasespeed -= 0.01;
            if self.chasespeed < 350.0 {
                self.chasespeed = 350.0;
            }
            self.gainchase =
                ((self.gainchase * self.chasespeed) + inputgain) / (self.chasespeed + 1.0);
            if self.gainchase != 1.0 {
                l *= self.gainchase;
                r *= self.gainchase;
            }
            l = l.sin();
            r = r.sin();
            let dry_l = l;
            let dry_r = r;
            l = l.sin();
            r = r.sin();
            let apply_l = ((self.previous_l + l).abs() / 2.0) * intensity;
            let apply_r = ((self.previous_r + r).abs() / 2.0) * intensity;
            l = (dry_l * (1.0 - apply_l)) + (l * apply_l);
            r = (dry_r * (1.0 - apply_r)) + (r * apply_r);
            self.previous_l = dry_l.sin();
            self.previous_r = dry_r.sin();
            ol[i] = fpd_dither(l, &mut self.fpd_l) as f32;
            or[i] = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
