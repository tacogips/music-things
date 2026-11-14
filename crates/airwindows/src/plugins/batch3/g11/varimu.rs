//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/VariMu, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 5.3941392e-8.
//! Upstream function-static noise counters persist across fixture renders; this port resets them per instance.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Intensty",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Speed",
        label: "",
        default: 0.5,
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
    name: "VariMu",
    upstream_name: "VariMu",
    description: "VariMu is a more organic variation on Pressure (a compressor)",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(VariMu::new(sample_rate, seed))
}
struct VariMu {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    speed: [[f64; 2]; 2],
    coef: [[f64; 2]; 2],
    previous: [f64; 2],
    noise: [u32; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl VariMu {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.5, 1.0, 1.0],
            sample_rate,
            seed,
            speed: [[10000.0; 2]; 2],
            coef: [[1.0; 2]; 2],
            previous: [0.0; 2],
            noise: [0, 850010],
            flip: false,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for VariMu {
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
        let [a, b, c, d] = self.params;
        let mut overall = 2.0;
        overall /= 44100.0;
        overall *= self.sample_rate as f64;
        let threshold = 1.001 - (1.0 - (1.0 - a as f64).powf(3.0));
        let mut makeup = (1.0 / threshold).sqrt();
        makeup = (makeup + makeup.sqrt()) / 2.0;
        makeup = makeup.sqrt();
        let output = makeup.sqrt() * c as f64;
        let mut release = (1.15 - b as f64).powf(5.0) * 32768.0;
        release /= overall;
        let fastest = release.sqrt();
        let wet = d as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut samples = [l as f64, r as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                self.noise[ch] = self.noise[ch] % 1700021 + 1;
                let mut residue = self.noise[ch].wrapping_mul(self.noise[ch]);
                for divisor in [170003, 17011, 1709, 173] {
                    residue %= divisor;
                    residue = residue.wrapping_mul(residue);
                }
                residue %= 17;
                let mut air = residue as f64;
                air *= 0.00000001;
                air *= 0.00000001;
                *sample += air;
                if *sample < 1.2e-38 && -*sample < 1.2e-38 {
                    *sample -= air;
                }
                let dry = *sample;
                let squared = if sample.abs() > self.previous[ch].abs() {
                    self.previous[ch] * self.previous[ch]
                } else {
                    *sample * *sample
                };
                self.previous[ch] = *sample;
                *sample *= makeup;
                let lane = if self.flip { 0 } else { 1 };
                let speed = &mut self.speed[ch][lane];
                let coef = &mut self.coef[ch][lane];
                if squared.abs() > threshold {
                    let vary = threshold / squared.abs();
                    let attack = speed.abs().sqrt();
                    *coef *= attack - 1.0;
                    *coef += if vary < threshold { threshold } else { vary };
                    *coef /= attack;
                } else {
                    *coef *= *speed * *speed - 1.0;
                    *coef += 1.0;
                    *coef /= *speed * *speed;
                }
                let mut new_speed = *speed * (*speed - 1.0);
                new_speed += (squared * release).abs() + fastest;
                *speed = new_speed / *speed;
                let coeff = (*coef + coef.powf(2.0)) / 2.0;
                *sample *= coeff;
                if output < 1.0 {
                    *sample *= output;
                }
                if wet < 1.0 {
                    *sample = dry * (1.0 - wet) + *sample * wet;
                }
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            self.flip = !self.flip;
            *out_l = samples[0] as f32;
            *out_r = samples[1] as f32;
        }
    }
}
