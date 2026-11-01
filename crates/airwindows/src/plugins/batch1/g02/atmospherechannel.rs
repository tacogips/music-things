//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/AtmosphereChannel/AtmosphereChannel.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Input",
    label: "",
    default: 1.0,
}];
const INFO: PluginInfo = PluginInfo {
    name: "AtmosphereChannel",
    upstream_name: "AtmosphereChannel",
    description:
        "AtmosphereChannel is Console5 processing with powerful new acoustic distance effects.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(AtmosphereChannel::new(sample_rate, seed))
}
struct AtmosphereChannel {
    param: f32,
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    last: [[f64; 13]; 2],
    gainchase: f64,
    settingchase: f64,
    chasespeed: f64,
}
impl AtmosphereChannel {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            param: 1.0,
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            last: [[0.0; 13]; 2],
            gainchase: -90.0,
            settingchase: -90.0,
            chasespeed: 350.0,
        }
    }
}
impl AirwindowsEffect for AtmosphereChannel {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        if index == 0 {
            self.param
        } else {
            0.0
        }
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if index == 0 {
            if let Some(value) = sanitize_param(value) {
                self.param = value;
            }
        }
    }
    fn param_display(&self, index: usize) -> String {
        if index == 0 {
            float2string(self.param)
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let param = self.param;
        *self = Self::new(self.sample_rate, self.seed);
        self.param = param;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let inputgain = self.param as f64;
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
        let mut thresholds = [
            0.618033988749894,
            0.679837387624884,
            0.747821126387373,
            0.82260323902611,
            0.904863562928721,
            0.995349919221593,
            1.094884911143752,
            1.204373402258128,
            1.32481074248394,
            1.457291816732335,
            1.603020998405568,
            1.763323098246125,
            1.939655408070737,
        ];
        for t in &mut thresholds {
            *t /= overallscale;
        }
        let len = in_l.len().min(in_r.len()).min(out_l.len()).min(out_r.len());
        for i in 0..len {
            let mut values = [in_l[i] as f64, in_r[i] as f64];
            for (ch, v) in values.iter_mut().enumerate() {
                if v.abs() < 1.18e-23 {
                    *v = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            self.chasespeed *= 0.9999;
            self.chasespeed -= 0.01;
            if self.chasespeed < 350.0 {
                self.chasespeed = 350.0;
            }
            self.gainchase =
                ((self.gainchase * self.chasespeed) + inputgain) / (self.chasespeed + 1.0);
            if self.gainchase != 1.0 {
                values[0] *= self.gainchase;
                values[1] *= self.gainchase;
            }
            for (ch, v) in values.iter_mut().enumerate() {
                let dry = *v;
                *v = v.sin();
                for (j, threshold) in thresholds.iter().enumerate() {
                    let clamp = *v - self.last[ch][j];
                    if clamp > *threshold {
                        *v = self.last[ch][j] + *threshold;
                    }
                    if -clamp > *threshold {
                        *v = self.last[ch][j] - *threshold;
                    }
                }
                self.last[ch].copy_within(0..12, 1);
                self.last[ch][0] = dry;

                *v = fpd_dither(*v, &mut self.fpd[ch]);
            }
            out_l[i] = values[0] as f32;
            out_r[i] = values[1] as f32;
        }
    }
}
