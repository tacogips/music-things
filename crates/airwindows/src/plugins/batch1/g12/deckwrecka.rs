// Copyright (c) 2016 airwindows, Airwindows uses the MIT license
// Upstream: airwindows/plugins/LinuxVST/src/Deckwrecka, b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
static PARAMS: [ParamInfo; 1] = [ParamInfo {
    name: "Wreck",
    label: "",
    default: 0.0,
}];
pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "Deckwrecka",
    upstream_name: "Deckwrecka",
    description: "Deckwrecka fattens and dirties up beats.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Deckwrecka::new(rate, seed))
}
struct Channel {
    head: [f64; 3],
    iir: [f64; 26],
    fpd: u32,
}
impl Channel {
    fn new(fpd: u32) -> Self {
        Self {
            head: [0.0; 3],
            iir: [0.0; 26],
            fpd,
        }
    }
    fn process(&mut self, input: f32, wreck: f64, freq: f64, hp: f64, slot: usize) -> f32 {
        let mut sample = input as f64;
        if sample.abs() < 1.18e-23 {
            sample = self.fpd as f64 * 1.18e-17;
        }
        let mut random = self.fpd as f64 / u32::MAX as f64;
        for _ in 0..5 {
            xorshift(&mut self.fpd);
            random += self.fpd as f64 / u32::MAX as f64;
        }
        random /= 6.0;
        random *= wreck;
        self.head[slot] += sample * random;
        self.head[slot] -= self.head[slot] * self.head[slot] * self.head[slot] * freq;
        let mut head = self.head[0] + self.head[1] + self.head[2];
        let mut temp = head;
        let mut correction = 0.0;
        for stage in &mut self.iir {
            *stage = (*stage * (1.0 - hp)) + (temp * hp);
            temp -= *stage;
            correction += *stage;
        }
        head -= correction;
        head *= 1.0 - (wreck / 2.0);
        #[allow(clippy::approx_constant)] // Preserve the upstream clamp.
        let bridge = head.abs().min(1.57079633).sin();
        head = if head > 0.0 { bridge } else { -bridge };
        sample *= 1.0 - wreck;
        sample += head * wreck;
        fpd_dither(sample, &mut self.fpd) as f32
    }
}
struct Deckwrecka {
    params: [f32; 1],
    rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    bflip: usize,
}
impl Deckwrecka {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0],
            rate,
            seed,
            left: Channel::new(fpd_from_seed(seed, 0)),
            right: Channel::new(fpd_from_seed(seed, 1)),
            bflip: 1,
        }
    }
}
impl AirwindowsEffect for Deckwrecka {
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
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let overall = (1.0 / 44100.0) * self.rate as f64;
        let wreck = (self.params[0] as f64).powf(2.0);
        let freq = 0.01 / overall;
        let hp = 0.0000014 / overall;
        for (((&l, &r), o1), o2) in il.iter().zip(ir).zip(ol).zip(or) {
            self.bflip += 1;
            if self.bflip > 3 {
                self.bflip = 1;
            }
            *o1 = self.left.process(l, wreck, freq, hp, self.bflip - 1);
            *o2 = self.right.process(r, wreck, freq, hp, self.bflip - 1);
        }
    }
}
