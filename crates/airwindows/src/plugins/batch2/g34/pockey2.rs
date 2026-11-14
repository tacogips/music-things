//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Pockey2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "DeFreq",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "DeRez",
        label: "",
        default: 0.66,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Pockey2",
    upstream_name: "Pockey2",
    description: "Pockey2 is more efficient, more intense lo-fi hiphop in a plugin.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Pockey2::new(sample_rate, seed))
}
struct Pockey2 {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    last: [f64; 2],
    held: [f64; 2],
    previous: [f64; 2],
    position: f64,
    fpd: [u32; 2],
}
impl Pockey2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.66, 1.0],
            sample_rate,
            seed,
            last: [0.0; 2],
            held: [0.0; 2],
            previous: [0.0; 2],
            position: 0.0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Pockey2 {
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
            1 => float2string((4.0 + self.params[1] as f64 * 12.0) as f32),
            _ => self
                .params
                .get(i)
                .map_or_else(String::new, |&v| float2string(v)),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c] = self.params;
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let freq = ((a as f64).powf(3.0) * 32.0 * scale).floor() as i32;
        let rez = 4.0 + b as f64 * 12.0;
        let rez_factor = 2.0_f64.powf(rez) as i32 as f64;
        let wet = c as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            let mut dry = [0.0; 2];
            let mut blur = [0.0; 2];
            for ch in 0..2 {
                let s = &mut samples[ch];
                if s.abs() < 1.18e-23 {
                    *s = self.fpd[ch] as f64 * 1.18e-17;
                }
                dry[ch] = *s;
                *s = s.clamp(-1.0, 1.0);
                if *s > 0.0 {
                    *s = (1.0 + 255.0 * s.abs()).ln() / 255.0_f64.ln();
                }
                if *s < 0.0 {
                    *s = -(1.0 + 255.0 * s.abs()).ln() / 255.0_f64.ln();
                }
                *s *= rez_factor;
                if *s > 0.0 {
                    *s = s.floor();
                }
                if *s < 0.0 {
                    *s = -(-*s).floor();
                }
                *s /= rez_factor;
                *s = s.clamp(-1.0, 1.0);
                if *s > 0.0 {
                    *s = (256.0_f64.powf(s.abs()) - 1.0) / 255.0;
                }
                if *s < 0.0 {
                    *s = -(256.0_f64.powf(s.abs()) - 1.0) / 255.0;
                }
                blur[ch] = (0.618_033_988_749_894_9 - (*s - self.last[ch]).abs() * scale).max(0.0);
            }
            if self.position < 1.0 {
                self.position = freq as f64;
                self.held = samples;
            }
            for ch in 0..2 {
                samples[ch] = self.held[ch];
                self.last[ch] = dry[ch];
                samples[ch] = (samples[ch] * blur[ch]) + (self.previous[ch] * (1.0 - blur[ch]));
                self.previous[ch] = self.held[ch];
                if wet != 1.0 {
                    samples[ch] = (samples[ch] * wet) + (dry[ch] * (1.0 - wet));
                }
            }
            self.position -= 1.0;
            *out_l = fpd_dither(samples[0], &mut self.fpd[0]) as f32;
            *out_r = fpd_dither(samples[1], &mut self.fpd[1]) as f32;
        }
    }
}
