//! Desk4, upstream airwindows/plugins/LinuxVST/src/Desk4 at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Parity: Libm, measured max-abs 0.0.

// Preserve the literal coefficient used by upstream.
#![allow(clippy::approx_constant)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) static INFO: PluginInfo = PluginInfo { name: "Desk4", upstream_name: "Desk4", description: "Desk4 is distinctive analog coloration (a tuneable version of the control-less Desk plugins)", params: &[
    ParamInfo { name: "Overdrive", label: "", default: 0.27 }, ParamInfo { name: "Hi Choke", label: "", default: 0.18 }, ParamInfo { name: "Power Sag", label: "", default: 0.26 }, ParamInfo { name: "Frequency", label: "", default: 0.54 }, ParamInfo { name: "Output Trim", label: "", default: 0.84 }, ParamInfo { name: "Dry/Wet", label: "", default: 1.0 }
] };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Desk4::new(rate, seed))
}
struct Desk4 {
    params: [f32; 6],
    sample_rate: f32,
    seed: u32,
    delay: [Box<[f64]>; 2],
    control: [f64; 2],
    last: [f64; 2],
    last_out: [f64; 2],
    last_slew: [f64; 2],
    count: i32,
    fpd: [u32; 2],
}
impl Desk4 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.27, 0.18, 0.26, 0.54, 0.84, 1.0],
            sample_rate,
            seed,
            delay: [
                vec![0.0; 10000].into_boxed_slice(),
                vec![0.0; 10000].into_boxed_slice(),
            ],
            control: [0.0; 2],
            last: [0.0; 2],
            last_out: [0.0; 2],
            last_slew: [0.0; 2],
            count: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Desk4 {
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
        self.params
            .get(i)
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        for delay in &mut self.delay {
            delay.fill(0.0);
        }
        self.control = [0.0; 2];
        self.last = [0.0; 2];
        self.last_out = [0.0; 2];
        self.last_slew = [0.0; 2];
        self.count = 0;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let p = self.params.map(|x| x as f64);
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let gain = p[0].powf(2.0) * 10.0 + 0.0001;
        let gaintrim = p[0].powf(2.0) * 2.0 + 1.0;
        let mut slewgain = p[1].powf(3.0) * 40.0 + 0.0001;
        let mut prevslew = 0.105;
        let intensity = p[2].powf(6.0) * 15.0 + 0.0001;
        let depth = p[3].powf(4.0) * 940.0 + 0.00001;
        let offset = ((depth * scale) as i32).clamp(1, 4880) as usize;
        let mut balance_b = 0.0001;
        slewgain *= scale;
        prevslew *= scale;
        balance_b /= scale;
        let balance_a = 1.0 - balance_b;
        for (((l, r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [*l as f64, *r as f64];
            if self.count < 0 || self.count > 4900 {
                self.count = 4900;
            }
            let pos = self.count as usize;
            for (ch, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *sample;
                self.delay[ch][pos] = sample.abs() * intensity;
                self.delay[ch][pos + 4900] = self.delay[ch][pos];
                self.control[ch] += self.delay[ch][pos] / offset as f64;
                self.control[ch] -= self.delay[ch][pos + offset] / offset as f64;
                self.control[ch] -= 0.000001;
                let mut clamp = 1.0;
                if self.control[ch] < 0.0 {
                    self.control[ch] = 0.0;
                }
                if self.control[ch] > 1.0 {
                    clamp -= self.control[ch] - 1.0;
                    self.control[ch] = 1.0;
                }
                if clamp < 0.5 {
                    clamp = 0.5;
                }
                let thickness = ((1.0 - self.control[ch]) * 2.0) - 1.0;
                let out = thickness.abs();
                let mut rectifier = sample.abs();
                if rectifier > 1.57079633 {
                    rectifier = 1.57079633;
                }
                if thickness > 0.0 {
                    rectifier = rectifier.sin();
                } else {
                    rectifier = 1.0 - rectifier.cos();
                }
                if *sample > 0.0 {
                    *sample = (*sample * (1.0 - out)) + (rectifier * out);
                } else {
                    *sample = (*sample * (1.0 - out)) - (rectifier * out);
                }
                *sample *= clamp;
                let mut slew = *sample - self.last[ch];
                self.last[ch] = *sample;
                rectifier = (slew * slewgain).abs();
                if rectifier > 1.57079633 {
                    rectifier = 1.0;
                } else {
                    rectifier = rectifier.sin();
                }
                if slew > 0.0 {
                    slew = rectifier / slewgain;
                } else {
                    slew = -(rectifier / slewgain);
                }
                *sample = (self.last_out[ch] * balance_a) + (self.last[ch] * balance_b) + slew;
                self.last_out[ch] = *sample;
                let mut comb = (dry * self.last[ch]).abs();
                if comb > 1.0 {
                    comb = 1.0;
                }
                *sample -= self.last_slew[ch] * comb * prevslew;
                self.last_slew[ch] = slew;
                *sample *= gain;
                rectifier = sample.abs();
                if rectifier > 1.57079633 {
                    rectifier = 1.0;
                } else {
                    rectifier = rectifier.sin();
                }
                if *sample > 0.0 {
                    *sample = rectifier;
                } else {
                    *sample = -rectifier;
                }
                *sample /= gain;
                *sample *= gaintrim;
                if p[4] != 1.0 {
                    *sample *= p[4];
                }
                if p[5] != 1.0 {
                    *sample = (*sample * p[5]) + (dry * (1.0 - p[5]));
                }
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            self.count -= 1;
            *out_l = samples[0] as f32;
            *out_r = samples[1] as f32;
        }
    }
}
