//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/StarChild, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Sustain",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Grain",
        label: "",
        default: 0.7,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.2,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "StarChild",
    upstream_name: "StarChild",
    description: "StarChild is a weird digital ambience/echo plugin.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(StarChild::new(sample_rate, seed))
}
const PRIMES: [i32; 167] = [
    1, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101,
    103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197,
    199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311,
    313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431,
    433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557,
    563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661,
    673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809,
    811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937,
    941, 947, 953, 967, 971, 977, 983, 991, 997, 998, 999,
];
struct StarChild {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    d: Box<[f64]>,
    d_count: i32,
    t: [i32; 171],
    pitch: usize,
    increment: i32,
    duty: i32,
    wear: [[f64; 11]; 2],
    prev: [f64; 2],
    fpd: [u32; 2],
}
impl StarChild {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        let mut t = [0; 171];
        t[..165].copy_from_slice(&PRIMES[..165]);
        Self {
            params: [1.0, 0.7, 0.2],
            sample_rate,
            seed,
            d: vec![0.0; 45102].into_boxed_slice(),
            d_count: 0,
            t,
            pitch: 2,
            increment: 1,
            duty: 1,
            wear: [[0.0; 11]; 2],
            prev: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for StarChild {
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
        self.d.fill(0.0);
        self.d_count = 0;
        self.t.fill(0);
        self.t[..165].copy_from_slice(&PRIMES[..165]);
        self.pitch = 2;
        self.increment = 1;
        self.duty = 1;
        self.wear = [[0.0; 11]; 2];
        self.prev = [0.0; 2];
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c] = self.params;
        let range = ((b as f64).powf(2.0) * 156.0 + 7.0) as i32;
        let scale = ((a as f64).powf(2.0) * (3280.0 / range as f64) + 2.0) as f32;
        let pad = (4.0 * range as f64 * (range as f64).sqrt()) as f32;
        let mut overall = ((1.0 - b as f64) * 9.0 + 1.0) as f32;
        let mut gain = overall;
        let mut factor = [0.0f64; 10];
        for f in &mut factor {
            if gain > 1.0 {
                *f = 1.0;
                gain -= 1.0;
            } else {
                *f = gain as f64;
                gain = 0.0;
            }
        }
        if overall < 1.0 {
            overall = 1.0;
        }
        for f in &mut factor {
            *f /= overall as f64;
        }
        let dryness = 1.0f32 - c;
        for (i, p) in PRIMES.iter().enumerate().take(165).skip(1) {
            self.t[i] = (*p as f32 * scale) as i32;
        }
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            for (ch, x) in input.iter_mut().enumerate() {
                if x.abs() < 1.18e-23 {
                    *x = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            let dry = input;
            if self.d_count < 0 || self.d_count > 22050 {
                self.d_count = 22050;
            }
            let idx = self.d_count as usize;
            self.d[idx] = input[0] + input[1];
            self.d[idx + 22050] = self.d[idx];
            self.d_count -= 1;
            self.duty += 1;
            if self.duty as f32 > scale {
                self.duty = 1;
                self.t[self.pitch] += self.increment;
                self.pitch += 1;
                if self.pitch > range as usize {
                    self.pitch = 1;
                    if self.increment == 1 {
                        if self.t[1] as f32 > (11.0 * scale) + 1000.0 {
                            self.increment = -1;
                        }
                    } else if (self.t[1] as f32) < 11.0 * scale {
                        self.increment = 1;
                        for (i, p) in PRIMES.iter().enumerate().take(165).skip(1) {
                            self.t[i] = (*p as f32 * scale) as i32;
                        }
                    }
                }
            }
            let mut acc = [0i32; 2];
            for i in (1..=range as usize).rev() {
                let p = PRIMES[i] as f64;
                let weights = match PRIMES[i] % 10 {
                    1 => [0.0, p],
                    3 => [p * 0.25, p * 0.75],
                    7 => [p * 0.75, p * 0.25],
                    9 => [p, 0.0],
                    _ => [0.0, 0.0],
                };
                let value = self.d[(self.d_count + self.t[i]) as usize];
                for ch in 0..2 {
                    acc[ch] += (value * weights[ch]) as i32;
                }
            }
            for ch in 0..2 {
                let mut x = acc[ch] as f64;
                self.wear[ch].copy_within(0..9, 1);
                let mut accum = (x - self.prev[ch]) as f32;
                self.wear[ch][0] = accum as f64;
                accum = (accum as f64 * factor[0]) as f32;
                for (i, f) in factor.iter().enumerate().skip(1) {
                    accum = (accum as f64 + self.wear[ch][i] * f) as f32;
                }
                let correction = ((x - self.prev[ch]) + accum as f64) as f32;
                self.prev[ch] = x;
                x += correction as f64;
                x /= pad as f64;
                x *= c as f64;
                x += dry[ch] * dryness as f64;
                input[ch] = fpd_dither(x, &mut self.fpd[ch]);
            }
            *outl = input[0] as f32;
            *outr = input[1] as f32;
        }
    }
}
