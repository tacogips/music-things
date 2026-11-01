//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Ditherbox, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.0.
//! Intentional divergence: upstream leaves flip uninitialized; it starts false here.
#![allow(clippy::approx_constant)] // Preserve upstream constants.
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Type",
    label: "24",
    default: 0.86,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Ditherbox",
    upstream_name: "Ditherbox",
    description:
        "Ditherbox is a switchable selection of dithers in 16 and 24 bit, plus monitoring tools.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Ditherbox::new(sample_rate, seed))
}
#[derive(Clone)]
struct Channel {
    byn: [f64; 13],
    ns: [f64; 16],
    noise: f64,
    err: f64,
    odd: f64,
    prev: f64,
    last: f64,
    out: f64,
    iir: [f64; 26],
    fpd: u32,
}
impl Channel {
    fn new(fpd: u32) -> Self {
        Self {
            byn: [
                1000., 301., 176., 125., 97., 79., 67., 58., 51., 46., 1000., 0., 0.,
            ],
            ns: [0.; 16],
            noise: 0.,
            err: 0.,
            odd: 0.,
            prev: 0.,
            last: 0.,
            out: 0.,
            iir: [0.; 26],
            fpd,
        }
    }
    fn random(&self) -> f64 {
        self.fpd as f64 / u32::MAX as f64
    }
    fn benford(&mut self, x: f64, repetitions: usize) -> f64 {
        fn bin(mut x: f64, n: usize) -> usize {
            while x >= 1. {
                x /= 10.;
            }
            for _ in 0..n {
                if x < 1. {
                    x *= 10.;
                }
            }
            let b = x.floor() as i32;
            if (1..10).contains(&b) {
                b as usize
            } else {
                10
            }
        }
        fn score(byn: &mut [f64; 13], b: usize) -> f64 {
            if b == 10 {
                return 0.;
            }
            byn[b] += 1.;
            let mut total = 0.;
            for (i, target) in [301., 176., 125., 97., 79., 67., 58., 51., 46.]
                .iter()
                .enumerate()
            {
                total += target - byn[i + 1];
            }
            byn[b] -= 1.;
            total
        }
        let a = bin(x.floor(), repetitions);
        let b = bin(x.ceil(), repetitions);
        let ta = score(&mut self.byn, a);
        let tb = score(&mut self.byn, b);
        let result = if ta < tb {
            self.byn[a] += 1.;
            x.floor()
        } else {
            self.byn[b] += 1.;
            x.ceil()
        };
        self.byn[10] /= 2.;
        result
    }
    fn process(&mut self, mut x: f64, mode: i32, flip: bool, iir_amount: f64, alt: f64) -> f64 {
        match mode {
            1 => x = x.floor(),
            2 => {
                x += self.random();
                x -= 0.5;
                x = x.floor();
            }
            3 => {
                x += self.random();
                xorshift(&mut self.fpd);
                x += self.random();
                x -= 1.;
                x = x.floor();
            }
            4 => {
                let d = self.random();
                x += d;
                x -= self.last;
                x = x.floor();
                self.last = d;
            }
            5 => {
                self.ns.copy_within(0..9, 1);
                self.ns[0] = self.random();
                let n = &self.ns;
                let mut d = n[0] * 0.061;
                d -= n[1] * 0.11;
                d += n[8] * 0.126;
                d -= n[7] * 0.23;
                d += n[2] * 0.25;
                d -= n[3] * 0.43;
                d += n[6] * 0.5;
                d -= n[5];
                d += n[4];
                x += d;
                x = x.floor();
            }
            6 => {
                let d = self.random();
                x += d;
                x -= self.ns[4];
                x = x.floor();
                self.ns.copy_within(1..4, 2);
                self.ns[1] = d;
            }
            8 => {
                let mut abs = self.random() - 0.5;
                for i in 0..16 {
                    if i < 15 {
                        xorshift(&mut self.fpd);
                    }
                    self.ns[i] += abs;
                    self.ns[i] /= 2.;
                    abs -= self.ns[i];
                    if i < 15 {
                        abs += self.random() - 0.5;
                    }
                }
                abs += x;
                if self.odd > 0. {
                    self.odd -= 0.97;
                }
                if self.odd < 0. {
                    self.odd += 0.97;
                }
                self.odd -= self.odd * self.odd * self.odd * 0.475;
                self.odd += self.prev;
                abs += self.odd * 0.475;
                self.prev = abs.floor() - x;
                x = abs.floor();
            }
            9 => {
                if x > 0. {
                    x += 0.383;
                }
                if x < 0. {
                    x -= 0.383;
                }
                let mut rnd = self.random();
                xorshift(&mut self.fpd);
                rnd += self.random() - 1.;
                rnd *= 1.618033988749895;
                rnd -= self.err * 0.5671432904097838;
                let abs = x.abs();
                self.err = abs - abs.floor();
                let mut c = self.err * 2.;
                if c > 1. {
                    c = ((-c + 2.) * 0.5671432904097838) + 0.06598803584531254;
                } else {
                    c = (c * 0.5671432904097838) + 0.06598803584531254;
                }
                if flip {
                    rnd = (rnd * (1. - c)) + c + 0.5;
                } else {
                    rnd = (rnd * (1. - c)) - c + 0.5;
                }
                x += rnd * c;
                x = x.floor();
            }
            10 => {
                if x > 0. {
                    x += 0.3333333333;
                }
                if x < 0. {
                    x -= 0.3333333333;
                }
                x += self.random() * 0.6666666666;
                x = self.benford(x, 2);
            }
            11 => x = self.benford(x, 5),
            12 => {
                let out = (x - self.last) * 2.302585092994046;
                self.last = x;
                x = out.clamp(-1., 1.);
            }
            13 => {
                let mut gain = 1.42;
                for i in 0..26 {
                    x *= gain;
                    gain = ((gain - 1.) * 0.75) + 1.;
                    if i > 0 {
                        x = x.clamp(-1., 1.);
                    }
                    self.iir[i] = (self.iir[i] * alt) + (x * iir_amount);
                    x = self.iir[i];
                }
                x = x.clamp(-1., 1.);
            }
            14 => {
                let mut bridge = x.abs() * 1.57079633;
                if bridge > 1.57079633 {
                    bridge = 1.57079633;
                }
                bridge = 1. - bridge.cos();
                if x > 0. {
                    x = bridge;
                } else {
                    x = -bridge;
                }
                let mut s = self.random();
                xorshift(&mut self.fpd);
                s -= 0.5;
                s *= 2.;
                s *= x.abs();
                let mut smooth = self.random();
                smooth -= 0.5;
                smooth *= 2.;
                smooth *= self.last.abs();
                self.last = x;
                s += smooth;
                bridge = s.abs() * 1.57079633;
                if bridge > 1.57079633 {
                    bridge = 1.57079633;
                }
                bridge = bridge.sin();
                if s > 0. {
                    s = bridge;
                } else {
                    s = -bridge;
                }
                x = (s + self.out) / 2.;
                self.out = s;
            }
            _ => (),
        }
        x
    }
}
struct Ditherbox {
    sample_rate: f32,
    seed: u32,
    param: f32,
    ch: [Channel; 2],
    position: i32,
    flip: bool,
}
impl Ditherbox {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            param: 0.86,
            ch: [
                Channel::new(fpd_from_seed(seed, 0)),
                Channel::new(fpd_from_seed(seed, 1)),
            ],
            position: 99999999,
            flip: false,
        }
    }
}
impl AirwindowsEffect for Ditherbox {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        if i == 0 {
            self.param
        } else {
            0.
        }
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if i == 0 {
            if let Some(v) = sanitize_param(v) {
                self.param = v;
            }
        }
    }
    fn param_label(&self, i: usize) -> &'static str {
        if i != 0 {
            return "";
        }
        match (self.param as f64 * 24.999) as i32 {
            0..=10 => "16",
            11..=21 => "24",
            22..=23 => "y",
            _ => "tte",
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i != 0 {
            return String::new();
        }
        let labels = [
            "Trunc", "Flat", "TPDF", "Paul", "DbPaul", "Tape", "HiGloss", "Vinyl", "Spatial",
            "Natural", "NJAD", "Trunc", "Flat", "TPDF", "Paul", "DbPaul", "Tape", "HiGloss",
            "Vinyl", "Spatial", "Natural", "NJAD", "SlewOnl", "SubsOnl", "Silhoue",
        ];
        labels[(self.param as f64 * 24.999) as usize].into()
    }
    fn reset(&mut self) {
        let p = self.param;
        *self = Self::new(self.sample_rate, self.seed);
        self.param = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let mut mode = (self.param as f64 * 24.999) as i32 + 1;
        let mut high = false;
        let mut dithering = true;
        if mode > 11 {
            high = true;
            mode -= 11;
        }
        if mode > 11 {
            dithering = false;
            high = false;
        }
        let mut scale = 1.;
        scale /= 44100.;
        scale *= self.sample_rate as f64;
        let mut amount = 2250. / 44100.;
        amount /= scale;
        let alt = 1. - amount;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut v = [l as f64, r as f64];
            let mut dry = [0f32; 2];
            for (i, x) in v.iter_mut().enumerate() {
                if x.abs() < 1.18e-23 {
                    *x = self.ch[i].fpd as f64 * 1.18e-17;
                }
                dry[i] = *x as f32;
                if mode == 8 {
                    *x -= self.ch[i].noise;
                }
                if dithering {
                    *x *= 32768.;
                }
                if high {
                    *x *= 256.;
                }
            }
            if mode == 7 {
                self.position = self.position.wrapping_add(1);
                let mut h = self.position.wrapping_mul(self.position) % 170003;
                h = h.wrapping_mul(h) % 17011;
                h = h.wrapping_mul(h) % 1709;
                h = h.wrapping_mul(h) % 173;
                h = h.wrapping_mul(h) % 17;
                h = (h as f64 * 0.0635) as i32;
                if self.flip {
                    h = -h;
                }
                for x in &mut v {
                    *x = (*x + h as f64).floor();
                }
            } else {
                for (i, x) in v.iter_mut().enumerate() {
                    *x = self.ch[i].process(*x, mode, self.flip, amount, alt);
                }
            }
            self.flip = !self.flip;
            for (i, x) in v.iter_mut().enumerate() {
                if high {
                    *x /= 256.;
                }
                if dithering {
                    *x /= 32768.;
                }
                if mode == 8 {
                    self.ch[i].noise += *x - dry[i] as f64;
                }
                xorshift(&mut self.ch[i].fpd);
            }
            *outl = v[0] as f32;
            *outr = v[1] as f32;
        }
    }
}
