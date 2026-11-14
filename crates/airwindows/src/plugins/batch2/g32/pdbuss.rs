//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PDBuss, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Gain",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Drive",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PDBuss",
    upstream_name: "PDBuss",
    description: "PDBuss is Console5 and PurestDrive, sittin’ in a tree… (i.e. both at once, on channels and buss)",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PDBuss::new(sample_rate, seed))
}
struct PDBuss {
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
impl PDBuss {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 1.0],
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
impl AirwindowsEffect for PDBuss {
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
        let inputgain = self.params[0] as f64;
        let intensity = self.params[1] as f64;
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
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            self.chasespeed *= 0.9999;
            self.chasespeed -= 0.01;
            if self.chasespeed < 350.0 {
                self.chasespeed = 350.0;
            }
            self.gainchase =
                ((self.gainchase * self.chasespeed) + inputgain) / (self.chasespeed + 1.0);
            if 1.0 != self.gainchase {
                l *= self.gainchase;
                r *= self.gainchase;
            }
            l = l.clamp(-1.0, 1.0).asin();
            r = r.clamp(-1.0, 1.0).asin();
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
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
