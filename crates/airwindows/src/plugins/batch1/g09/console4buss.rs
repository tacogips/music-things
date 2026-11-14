//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Console4Buss/, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, max-abs 0.0.
use crate::{
    display::float2string,
    dither::{fpd_dither, fpd_from_seed},
    params::sanitize_param,
    rand::{replay_fpd_seed_loop, AwRand},
    registry::PluginEntry,
};
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo { name: "Console4Buss", upstream_name: "Console4Buss", description: "Console4Buss is the original Airwindows ITB mixing system, a two-plugin mix buss replacement for DAWs.", params: &[ParamInfo { name: "Trim", label: " ", default: 1.0 }] };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Console4Buss::new(sample_rate, seed))
}
struct Console4Buss {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    gainchase: f64,
    settingchase: f64,
    chasespeed: f64,
    last: [f64; 2],
    fx: [f64; 2],
    correct: [f64; 2],
    fpd: [u32; 2],
}
impl Console4Buss {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0],
            sample_rate,
            seed,
            gainchase: -90.0,
            settingchase: -90.0,
            chasespeed: 350.0,
            last: [0.0; 2],
            fx: [0.0; 2],
            correct: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Console4Buss {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i == 0 {
            float2string(self.params[0])
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let scale = (1.0 / 44100.0) * self.sample_rate as f64;
        let gain = self.params[0] as f64;
        if self.settingchase != gain {
            self.chasespeed *= 2.0;
            self.settingchase = gain;
        }
        if self.chasespeed > 2500.0 {
            self.chasespeed = 2500.0;
        }
        if self.gainchase < 0.0 {
            self.gainchase = gain;
        }
        for (((&l, &r), ol), or) in in_l
            .iter()
            .zip(in_r)
            .zip(out_l.iter_mut())
            .zip(out_r.iter_mut())
        {
            self.chasespeed *= 0.9999;
            self.chasespeed -= 0.01;
            if self.chasespeed < 350.0 {
                self.chasespeed = 350.0;
            }
            self.gainchase = ((self.gainchase * self.chasespeed) + gain) / (self.chasespeed + 1.0);
            let mut result = [0.0f32; 2];
            for (c, input) in [l, r].into_iter().enumerate() {
                let mut x = input as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[c] as f64 * 1.18e-17;
                }
                if self.gainchase != 1.0 {
                    x *= self.gainchase;
                }
                let mut half = x;
                let falf = half.abs();
                half *= falf;
                half *= falf;
                let mut slew = (x - self.last[c]).abs() * scale;
                if slew > 1.0 {
                    slew = 1.0;
                }
                half *= 1.0 - slew;
                self.last[c] = x;
                x += half;
                result[c] = fpd_dither(x, &mut self.fpd[c]) as f32;
            }
            *ol = result[0];
            *or = result[1];
        }
    }
}
