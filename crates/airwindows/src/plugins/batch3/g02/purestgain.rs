//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PurestGain, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Gain",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Slow Fade",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PurestGain",
    upstream_name: "PurestGain",
    description: "PurestGain is a high-res noise shaped gain, with smoothed fader.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PurestGain::new(sample_rate, seed))
}
struct PurestGain {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    gain: f64,
    setting: f64,
    fade: f64,
    speed: f64,
}
impl PurestGain {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 1.0],
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            gain: -90.0,
            setting: -90.0,
            fade: -90.0,
            speed: 350.0,
        }
    }
}
impl AirwindowsEffect for PurestGain {
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
            0 => float2string(((self.params[0] as f64 * 80.0) - 40.0) as f32),
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let inputgain = (self.params[0] as f64 * 80.0) - 40.0;
        if self.setting != inputgain {
            self.speed *= 2.0;
            self.setting = inputgain;
        }
        if self.speed > 2500.0 {
            self.speed = 2500.0;
        }
        if self.gain < -60.0 {
            self.gain = 10.0_f64.powf(inputgain / 20.0);
        }
        let target_b = self.params[1] as f64;
        if self.fade < 0.0 {
            self.fade = target_b;
        }

        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let target = 10.0_f64.powf(self.setting / 20.0);
            self.speed *= 0.9999;
            self.speed -= 0.01;
            if self.speed < 350.0 {
                self.speed = 350.0;
            }
            self.gain = ((self.gain * self.speed) + target) / (self.speed + 1.0);
            self.fade = ((self.fade * 4000.0) + target_b) / 4001.0;
            let output = self.gain * self.fade;
            if output == 1.0 {
                *out_l = l;
                *out_r = r;
            } else {
                let mut input = [l as f64, r as f64];
                for (i, sample) in input.iter_mut().enumerate() {
                    if sample.abs() < 1.18e-23 {
                        *sample = self.fpd[i] as f64 * 1.18e-17;
                    }
                    *sample *= output;
                    *sample = fpd_dither(*sample, &mut self.fpd[i]);
                }
                *out_l = input[0] as f32;
                *out_r = input[1] as f32;
            }
        }
    }
}
