//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Pop2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Compres",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Attack",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Release",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Drive",
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
    name: "Pop2",
    upstream_name: "Pop2",
    description: "Pop2 adds control and punch to Pop.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Pop2::new(sample_rate, seed))
}
struct Pop2 {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    speed: [[f64; 2]; 2],
    coefficient: [[f64; 2]; 2],
    flip: bool,
    last: [f64; 2],
    intermediate: [[f64; 17]; 2],
    pos: [bool; 2],
    neg: [bool; 2],
    fpd: [u32; 2],
}
impl Pop2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.5, 0.5, 0.5, 1.0],
            sample_rate,
            seed,
            speed: [[1000.0; 2]; 2],
            coefficient: [[1.0; 2]; 2],
            flip: false,
            last: [0.0; 2],
            intermediate: [[0.0; 17]; 2],
            pos: [false; 2],
            neg: [false; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Pop2 {
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
        let [a, b, c, d, e] = self.params;
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let spacing = (scale.floor() as usize).clamp(1, 16);
        let threshold = 1.0 - ((1.0 - (1.0 - a as f64).powf(2.0)) * 0.9);
        let attack = ((b as f64).powf(4.0) * 100000.0 + 10.0) * scale;
        let release = ((c as f64).powf(5.0) * 2000000.0 + 20.0) * scale;
        let max_release = release * 4.0;
        let pre = 1.0 / threshold;
        let makeup = (1.0 / threshold).sqrt() * d as f64;
        let wet = e as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            let phase = usize::from(!self.flip);
            for (ch, s) in samples.iter_mut().enumerate() {
                if s.abs() < 1.18e-23 {
                    *s = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *s;
                *s *= pre;
                let speed = &mut self.speed[ch][phase];
                let coefficient = &mut self.coefficient[ch][phase];
                if s.abs() > threshold {
                    let vary = threshold / s.abs();
                    let attack_s = speed.abs().sqrt();
                    *coefficient *= attack_s - 1.0;
                    if vary < threshold {
                        *coefficient += threshold;
                    } else {
                        *coefficient += vary;
                    }
                    *coefficient /= attack_s;
                    let mut new_speed = *speed * (*speed - 1.0);
                    new_speed += release;
                    *speed = new_speed / *speed;
                    if *speed > max_release {
                        *speed = max_release;
                    }
                } else {
                    *coefficient *= (*speed * *speed) - 1.0;
                    *coefficient += 1.0;
                    *coefficient /= *speed * *speed;
                    let mut new_speed = *speed * (*speed - 1.0);
                    new_speed += attack;
                    *speed = new_speed / *speed;
                }
                *s *= coefficient.powf(2.0);
                *s *= makeup;
                *s = s.clamp(-4.0, 4.0);
                if self.pos[ch] {
                    if *s < self.last[ch] {
                        self.last[ch] = 0.7058208 + (*s * 0.2609148);
                    } else {
                        self.last[ch] = 0.2491717 + (self.last[ch] * 0.7390851);
                    }
                }
                self.pos[ch] = false;
                if *s > 0.9549925859 {
                    self.pos[ch] = true;
                    *s = 0.7058208 + (self.last[ch] * 0.2609148);
                }
                if self.neg[ch] {
                    if *s > self.last[ch] {
                        self.last[ch] = -0.7058208 + (*s * 0.2609148);
                    } else {
                        self.last[ch] = -0.2491717 + (self.last[ch] * 0.7390851);
                    }
                }
                self.neg[ch] = false;
                if *s < -0.9549925859 {
                    self.neg[ch] = true;
                    *s = -0.7058208 + (self.last[ch] * 0.2609148);
                }
                self.intermediate[ch][spacing] = *s;
                *s = self.last[ch];
                for x in (1..=spacing).rev() {
                    self.intermediate[ch][x - 1] = self.intermediate[ch][x];
                }
                self.last[ch] = self.intermediate[ch][0];
                if wet < 1.0 {
                    *s = (dry * (1.0 - wet)) + (*s * wet);
                }
            }
            self.flip = !self.flip;
            *out_l = fpd_dither(samples[0], &mut self.fpd[0]) as f32;
            *out_r = fpd_dither(samples[1], &mut self.fpd[1]) as f32;
        }
    }
}
