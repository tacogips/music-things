// Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
// Upstream: airwindows/plugins/LinuxVST/src/Console5DarkCh, b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Libm; measured max-abs 0.0.
use crate::{
    display::float2string,
    dither::{fpd_dither, fpd_from_seed},
    params::sanitize_param,
    rand::{replay_fpd_seed_loop, AwRand},
    registry::PluginEntry,
};
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: [ParamInfo; 1] = [ParamInfo {
    name: "Input",
    label: "",
    default: 1.0,
}];
static INFO: PluginInfo = PluginInfo {
    name: "Console5DarkCh",
    upstream_name: "Console5DarkCh",
    description: "Console5DarkCh is a richer, warmer Console system.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Console5DarkCh::new(sample_rate, seed))
}
struct Console5DarkCh {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    last: [f64; 2],
    fx: [f64; 2],
    correct: [f64; 2],
    gain: f64,
    setting: f64,
    speed: f64,
}
impl Console5DarkCh {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0],
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            last: [0.0; 2],
            fx: [0.0; 2],
            correct: [0.0; 2],
            gain: -90.0,
            setting: -90.0,
            speed: 350.0,
        }
    }
}
impl AirwindowsEffect for Console5DarkCh {
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
    #[allow(clippy::approx_constant)] // Preserve the upstream literal.
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let servo = 0.0000001 / scale;
        let bass = 0.005 / scale;
        let inputgain = self.params[0] as f64;
        if self.setting != inputgain {
            self.speed *= 2.0;
            self.setting = inputgain;
        }
        if self.speed > 2500.0 {
            self.speed = 2500.0;
        }
        if self.gain < 0.0 {
            self.gain = inputgain;
        }
        let n = l.len().min(r.len()).min(ol.len()).min(orr.len());
        for i in 0..n {
            self.speed *= 0.9999;
            self.speed -= 0.01;
            if self.speed < 350.0 {
                self.speed = 350.0;
            }
            self.gain = ((self.gain * self.speed) + inputgain) / (self.speed + 1.0);
            for (ch, input, output) in [(0, l[i], &mut ol[i]), (1, r[i], &mut orr[i])] {
                let mut x = input as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[ch] as f64 * 1.18e-17;
                }
                if self.gain != 1.0 {
                    x *= self.gain;
                }
                let mut diff = self.last[ch] - x;
                self.last[ch] = x;
                diff = diff.clamp(-1.0, 1.0);
                diff = self.fx[ch] + diff.sin();
                self.correct[ch] += x - diff;
                x = diff;
                self.fx[ch] = x;
                self.fx[ch] = self.fx[ch].clamp(-1.0, 1.0);
                let near = (self.fx[ch].abs() - 1.0).abs().powf(2.0);
                if self.correct[ch] > 0.0 {
                    self.correct[ch] -= servo;
                }
                if self.correct[ch] < 0.0 {
                    self.correct[ch] += servo;
                }
                self.fx[ch] += self.correct[ch] * 0.0000005;
                self.fx[ch] *= 1.0 - (near * bass);
                x = x.clamp(-1.57079633, 1.57079633);
                x = x.sin();
                *output = fpd_dither(x, &mut self.fpd[ch]) as f32;
            }
        }
    }
}
