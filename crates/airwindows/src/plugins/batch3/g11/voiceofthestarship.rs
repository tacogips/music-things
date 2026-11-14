//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/VoiceOfTheStarship, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Filter",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Algrthm",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "VoiceOfTheStarship",
    upstream_name: "VoiceOfTheStarship",
    description: "VoiceOfTheStarship is a deep noise tone source.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(VoiceOfTheStarship::new(sample_rate, seed))
}
struct VoiceOfTheStarship {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    noise: [[f64; 3]; 2],
    position: i32,
    quadratic: i32,
    flip: [bool; 2],
    filterflip: bool,
    history: [[f64; 10]; 2],
    last: i32,
    fpd: [u32; 2],
}
impl VoiceOfTheStarship {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.0],
            sample_rate,
            seed,
            noise: [[0.0; 3]; 2],
            position: 99999999,
            quadratic: 0,
            flip: [false; 2],
            filterflip: false,
            history: [[0.0; 10]; 2],
            last: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for VoiceOfTheStarship {
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
            0 => float2string(self.params[0]),
            1 => float2string((self.params[1] as f64 * 16.9).floor() as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let cutoff = ((self.params[0] as f64 * 0.89) + 0.1).powf(3.0).min(1.0);
        let inv = 1.0 - cutoff;
        let scale = ((1.0 - self.params[0] as f64) * 9.0) + 1.0;
        let mut gain = scale;
        let mut weights = [0.0; 10];
        for w in &mut weights {
            if gain > 1.0 {
                *w = 1.0;
                gain -= 1.0;
            } else {
                *w = gain;
                gain = 0.0;
            }
            *w /= scale.max(1.0);
        }
        let algo = (self.params[1] as f64 * 16.9).floor() as i32;
        if self.last != algo {
            self.noise = [[0.0; 3]; 2];
            self.history = [[0.0; 10]; 2];
            self.last = algo;
        }
        let pairs = [
            (1, 11),
            (2, 23),
            (3, 37),
            (4, 43),
            (5, 59),
            (6, 67),
            (7, 79),
            (11, 113),
            (13, 131),
            (17, 173),
            (19, 191),
            (23, 233),
            (31, 311),
            (53, 541),
            (71, 719),
            (113, 1151),
            (1151, 11517),
        ];
        let (low, dcut) = pairs[algo as usize];
        let trim = (low as f64).sqrt();
        for (((_, _), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            self.quadratic -= 1;
            if self.quadratic < 0 {
                self.position = self.position.wrapping_add(1);
                // The pinned clang fixture build treats the overflowing signed squares
                // as nonnegative for these reductions. Preserve its 32-bit behavior.
                let p = self.position as u32;
                let mut q = p.wrapping_mul(p) % 170003;
                q = q.wrapping_mul(q) % 17011;
                q = q.wrapping_mul(q) % 1709;
                q = q.wrapping_mul(q) % dcut as u32;
                q = q.wrapping_mul(q) % low as u32;
                self.quadratic = q as i32;
                for ch in 0..2 {
                    self.flip[ch] = self.noise[ch][0] < 0.0;
                }
            }
            let mut output = [0.0; 2];
            for (ch, value) in output.iter_mut().enumerate() {
                let n = self.fpd[ch] as f64 / u32::MAX as f64;
                if self.flip[ch] {
                    self.noise[ch][0] += n;
                } else {
                    self.noise[ch][0] -= n;
                }
                let idx = if self.filterflip { 1 } else { 2 };
                self.noise[ch][idx] *= inv;
                self.noise[ch][idx] += self.noise[ch][0] * cutoff;
                let sample = self.noise[ch][idx];
                self.history[ch].copy_within(0..9, 1);
                self.history[ch][0] = sample;
                *value = sample * weights[0];
                for (i, weight) in weights.iter().enumerate().skip(1) {
                    *value += self.history[ch][i] * weight;
                }
                *value *= 0.1;
                *value *= inv;
                *value /= trim;
                self.flip[ch] = !self.flip[ch];
                *value = fpd_dither(*value, &mut self.fpd[ch]);
            }
            self.filterflip = !self.filterflip;
            *out_l = output[0] as f32;
            *out_r = output[1] as f32;
        }
    }
}
