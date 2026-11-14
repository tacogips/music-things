//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Pop, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Intenst",
        label: "",
        default: 0.3,
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
    name: "Pop",
    upstream_name: "Pop",
    description: "Pop is a crazy overcompressor with a very exaggerated sound.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Pop::new(sample_rate, seed))
}
struct Pop {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    speed: [[f64; 2]; 2],
    coefficient: [[f64; 2]; 2],
    thicken: [f64; 2],
    previous: [[f64; 5]; 2],
    delay: [Box<[f64]>; 2],
    index: usize,
    flip: bool,
    fpd: [u32; 2],
}
impl Pop {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.3, 1.0, 1.0],
            sample_rate,
            seed,
            speed: [[10000.0; 2]; 2],
            coefficient: [[1.0; 2]; 2],
            thicken: [1.0; 2],
            previous: [[0.0; 5]; 2],
            delay: [
                vec![0.0; 10001].into_boxed_slice(),
                vec![0.0; 10001].into_boxed_slice(),
            ],
            index: 0,
            flip: false,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Pop {
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
        self.speed = [[10000.0; 2]; 2];
        self.coefficient = [[1.0; 2]; 2];
        self.thicken = [1.0; 2];
        self.previous = [[0.0; 5]; 2];
        self.delay[0].fill(0.0);
        self.delay[1].fill(0.0);
        self.index = 0;
        self.flip = false;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c] = self.params;
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let high = (a as f64).powf(2.0) * 0.023;
        let threshold = 1.001 - (1.0 - (1.0 - a as f64).powf(5.0));
        let makeup = (1.0 / threshold).sqrt();
        let mut release = a as f64 * 100000.0 + 300000.0;
        let maxdelay = ((1450.0 * scale) as usize).min(9999);
        release /= scale;
        let fastest = release.sqrt();
        let output = b as f64;
        let wet = c as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            let mut dry = [0.0; 2];
            for ch in 0..2 {
                if samples[ch].abs() < 1.18e-23 {
                    samples[ch] = self.fpd[ch] as f64 * 1.18e-17;
                }
                dry[ch] = samples[ch];
                self.delay[ch][self.index] = samples[ch];
            }
            self.index = if self.index == 0 || self.index - 1 > maxdelay {
                maxdelay
            } else {
                self.index - 1
            };
            let phase = usize::from(!self.flip);
            for (ch, s) in samples.iter_mut().enumerate() {
                *s = (*s * self.thicken[ch])
                    + (self.delay[ch][self.index] * (1.0 - self.thicken[ch]));
                let mut lowest = *s;
                if s.abs() > self.previous[ch][0].abs() {
                    lowest = self.previous[ch][0];
                }
                for (j, divisor) in [1.99, 1.98, 1.97, 1.96].into_iter().enumerate() {
                    let j = j + 1;
                    if lowest.abs() > self.previous[ch][j].abs() {
                        lowest = (lowest + self.previous[ch][j]) / divisor;
                    }
                }
                self.previous[ch].copy_within(0..4, 1);
                self.previous[ch][0] = *s;
                *s *= makeup;
                let mut punch = 0.95 - (*s * 0.08).abs();
                if punch < 0.65 {
                    punch = 0.65;
                }
                let speed = &mut self.speed[ch][phase];
                let coeff = &mut self.coefficient[ch][phase];
                if lowest.abs() > threshold {
                    let vary = threshold / lowest.abs();
                    let attack = speed.abs().sqrt();
                    *coeff *= attack - 1.0;
                    if vary < threshold {
                        *coeff += threshold;
                    } else {
                        *coeff += vary;
                    }
                    *coeff /= attack;
                } else {
                    *coeff *= (*speed * *speed) - 1.0;
                    *coeff += 1.0;
                    *coeff /= *speed * *speed;
                }
                let mut new_speed = *speed * (*speed - 1.0);
                new_speed += (lowest * release).abs() + fastest;
                *speed = new_speed / *speed;
                let coefficient = high + coeff.powf(2.0);
                *s *= coefficient;
                self.thicken[ch] = (coefficient / 5.0) + punch;
                self.thicken[ch] = (1.0 - wet) + (wet * self.thicken[ch]);
                let mut bridge = s.abs();
                if bridge > 1.2533141373155 {
                    bridge = 1.2533141373155;
                }
                bridge = (bridge * bridge.abs()).sin()
                    / if bridge.abs() == 0.0 {
                        1.0
                    } else {
                        bridge.abs()
                    };
                if *s > 0.0 {
                    *s = (*s * coefficient) + (bridge * (1.0 - coefficient));
                } else {
                    *s = (*s * coefficient) - (bridge * (1.0 - coefficient));
                }
                if output < 1.0 {
                    *s *= output;
                }
                if wet < 1.0 {
                    *s = (dry[ch] * (1.0 - wet)) + (*s * wet);
                }
            }
            self.flip = !self.flip;
            *out_l = fpd_dither(samples[0], &mut self.fpd[0]) as f32;
            *out_r = fpd_dither(samples[1], &mut self.fpd[1]) as f32;
        }
    }
}
