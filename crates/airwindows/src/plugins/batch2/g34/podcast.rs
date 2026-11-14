//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Podcast, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Boost",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Podcast",
    upstream_name: "Podcast",
    description: "Podcast is a simpler pile of curve-style compressors with hard clipping.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Podcast::new(sample_rate, seed))
}
struct Podcast {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    comp: [[f64; 5]; 2],
    fpd: [u32; 2],
}
impl Podcast {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 1.0],
            sample_rate,
            seed,
            comp: [[2.0; 5]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Podcast {
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
            .map_or_else(String::new, |&v| float2string(v))
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let compress = 1.0 + (self.params[0] as f64).powf(2.0);
        let wet = self.params[1] as f64;
        let mut speed1 = 64.0 / compress.powf(2.0);
        speed1 *= scale;
        let speed2 = speed1 * 1.4;
        let speed3 = speed2 * 1.5;
        let speed4 = speed3 * 1.6;
        let speed5 = speed4 * 1.7;
        let speeds = [speed5, speed4, speed3, speed2, speed1];
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            for (ch, s) in samples.iter_mut().enumerate() {
                if s.abs() < 1.18e-23 {
                    *s = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *s;
                for (stage, speed) in speeds.iter().enumerate() {
                    *s *= self.comp[ch][stage];
                    let mut trigger = s.abs() * 4.7;
                    if trigger > 4.7 {
                        trigger = 4.7;
                    }
                    trigger = trigger.sin();
                    if trigger < 0.0 {
                        trigger *= 8.0;
                    }
                    if trigger < -4.2 {
                        trigger = -4.2;
                    }
                    self.comp[ch][stage] += trigger / speed;
                    if self.comp[ch][stage] > compress {
                        self.comp[ch][stage] = compress;
                    }
                }
                if compress > 1.0 {
                    *s /= compress;
                }
                if wet != 1.0 {
                    *s = (*s * wet) + (dry * (1.0 - wet));
                }
                *s = s.clamp(-0.999, 0.999);
            }
            *out_l = fpd_dither(samples[0], &mut self.fpd[0]) as f32;
            *out_r = fpd_dither(samples[1], &mut self.fpd[1]) as f32;
        }
    }
}
