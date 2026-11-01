//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ADClip7 at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
// Preserve upstream constants and operation order.
#![allow(clippy::excessive_precision)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Boost",
        label: "dB",
        default: 0.0,
    },
    ParamInfo {
        name: "Soften",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Enhance",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Mode",
        label: "",
        default: 0.0,
    },
];
pub(super) const INFO: PluginInfo = PluginInfo {
    name: "ADClip7",
    upstream_name: "ADClip7",
    description: "ADClip7 is the original Airwindows loudness maximizer/ peak control.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ADClip7::new(rate, seed))
}

struct Channel {
    buffer: Box<[f32]>,
    last: f64,
    lows: f64,
    iir_a: f64,
    iir_b: f64,
    refclip: f64,
    fpd: u32,
}
impl Channel {
    fn new(seed: u32, lane: u32) -> Self {
        Self {
            buffer: vec![0.0; 22200].into_boxed_slice(),
            last: 0.0,
            lows: 0.0,
            iir_a: 0.0,
            iir_b: 0.0,
            refclip: 0.99,
            fpd: fpd_from_seed(seed, lane),
        }
    }
    fn reset(&mut self, seed: u32, lane: u32) {
        self.buffer.fill(0.0);
        self.last = 0.0;
        self.lows = 0.0;
        self.iir_a = 0.0;
        self.iir_b = 0.0;
        self.refclip = 0.99;
        self.fpd = fpd_from_seed(seed, lane);
    }
    fn sample(&mut self, sample: f32, count: usize, cfg: &Config) -> f32 {
        let mut input = sample as f64;
        if input.abs() < 1.18e-23 {
            input = self.fpd as f64 * 1.18e-17;
        }
        if cfg.gain != 1.0 {
            input *= cfg.gain;
        }
        let mut overshoot = input.abs() - self.refclip;
        if overshoot < 0.0 {
            overshoot = 0.0;
        }
        self.buffer[count] = overshoot as f32;
        self.buffer[count + 11020] = self.buffer[count];
        let b = &self.buffer;
        let mut highs = 0.0;
        if cfg.highslift > 0.0 {
            for (stage, sign) in [(2, -1.0), (1, 1.0), (0, -1.0)] {
                let t = count + cfg.refs_h[stage];
                let frac = cfg.fracs_h[stage];
                highs += sign * (b[t] as f64 * (1.0 - frac));
                highs += sign * b[t + 1] as f64;
                highs += sign * (b[t + 2] as f64 * frac);
                // All arithmetic in this correction is float in the upstream code.
                let correction = ((b[t] - b[t + 1]) - (b[t + 1] - b[t + 2])) / 50.0;
                highs -= sign * correction as f64;
                highs *= cfg.adjust;
            }
        }
        let bridge = (highs.abs() * cfg.hardness).sin();
        highs = if highs > 0.0 { bridge } else { -bridge };
        if cfg.subslift > 0.0 {
            const OFFSETS: [usize; 32] = [
                127, 113, 109, 107, 103, 101, 97, 89, 83, 79, 73, 71, 67, 61, 59, 53, 47, 43, 41,
                37, 31, 29, 23, 19, 17, 13, 11, 7, 5, 3, 2, 1,
            ];
            self.lows *= cfg.subs;
            for off in OFFSETS {
                self.lows -= b[count + cfg.ref_l1 + off] as f64;
            }
            self.lows *= cfg.subs;
            self.lows *= cfg.subs;
            for off in OFFSETS {
                self.lows += b[count + cfg.ref_l2 + off] as f64;
            }
            self.lows *= cfg.subs;
        }
        let bridge = (self.lows.abs() * cfg.softness).sin();
        self.lows = if self.lows > 0.0 { bridge } else { -bridge };
        self.iir_a = (self.iir_a * cfg.invcalibsubs) + (self.lows * cfg.calibsubs);
        self.lows = self.iir_a;
        let bridge = self.lows.abs().sin();
        self.lows = if self.lows > 0.0 { bridge } else { -bridge };
        self.iir_b = (self.iir_b * cfg.invcalibsubs) + (self.lows * cfg.calibsubs);
        self.lows = self.iir_b;
        let bridge = self.lows.abs().sin() * 2.0;
        self.lows = if self.lows > 0.0 { bridge } else { -bridge };
        if cfg.highslift > 0.0 {
            input += highs * (1.0 - (input * cfg.hardness).abs());
        }
        if cfg.subslift > 0.0 {
            input += self.lows * (1.0 - (input * cfg.softness).abs());
        }
        if input > self.refclip && self.refclip > 0.9 {
            self.refclip -= 0.01;
        }
        if input < -self.refclip && self.refclip > 0.9 {
            self.refclip -= 0.01;
        }
        if self.refclip < 0.99 {
            self.refclip += 0.00001;
        }
        if self.last >= self.refclip {
            self.last = if input < self.refclip {
                self.refclip * cfg.hardness + input * cfg.softness
            } else {
                self.refclip
            };
        }
        if self.last <= -self.refclip {
            self.last = if input > -self.refclip {
                -self.refclip * cfg.hardness + input * cfg.softness
            } else {
                -self.refclip
            };
        }
        if input > self.refclip {
            input = if self.last < self.refclip {
                self.refclip * cfg.hardness + self.last * cfg.softness
            } else {
                self.refclip
            };
        }
        if input < -self.refclip {
            input = if self.last > -self.refclip {
                -self.refclip * cfg.hardness + self.last * cfg.softness
            } else {
                -self.refclip
            };
        }
        self.last = input;
        match cfg.mode {
            2 => input /= cfg.gain,
            3 => input = overshoot + highs + self.lows,
            _ => {}
        }
        if input > self.refclip {
            input = self.refclip;
        }
        if input < -self.refclip {
            input = -self.refclip;
        }
        fpd_dither(input, &mut self.fpd) as f32
    }
}
struct Config {
    gain: f64,
    softness: f64,
    hardness: f64,
    highslift: f64,
    adjust: f64,
    subslift: f64,
    calibsubs: f64,
    invcalibsubs: f64,
    subs: f64,
    mode: i32,
    refs_h: [usize; 3],
    fracs_h: [f64; 3],
    ref_l1: usize,
    ref_l2: usize,
}
struct ADClip7 {
    params: [f32; 4],
    rate: f32,
    seed: u32,
    channels: [Channel; 2],
    count: i32,
}
impl ADClip7 {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.5, 0.5, 0.0],
            rate,
            seed,
            channels: [Channel::new(seed, 0), Channel::new(seed, 1)],
            count: 0,
        }
    }
}
impl AirwindowsEffect for ADClip7 {
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
        match i {
            0 => float2string((self.params[0] as f64 * 18.0) as f32),
            1 | 2 => float2string(self.params[i]),
            3 => match (self.params[3] as f64 * 2.999) as i32 {
                0 => "Normal",
                1 => "Atten",
                _ => "Clips",
            }
            .into(),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.count = 0;
        self.channels[0].reset(self.seed, 0);
        self.channels[1].reset(self.seed, 1);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.rate as f64;
        let fp_new = 1.0 - 0.618033988749894848204586_f64;
        let softness = self.params[1] as f64 * fp_new;
        let highslift = 0.307 * self.params[2] as f64;
        let subslift = 0.796 * self.params[2] as f64;
        let calibsubs = subslift / 53.0;
        let h1 = 1.84 * scale;
        let hs = [h1, h1 * 1.9, h1 * 2.7];
        let l1 = 612.0 * scale;
        let cfg = Config {
            gain: 10.0_f64.powf((self.params[0] as f64 * 18.0) / 20.0),
            softness,
            hardness: 1.0 - softness,
            highslift,
            adjust: highslift.powf(3.0) * 0.416,
            subslift,
            calibsubs,
            invcalibsubs: 1.0 - calibsubs,
            subs: 0.81 + (calibsubs * 2.0),
            mode: (self.params[3] as f64 * 2.999).floor() as i32 + 1,
            refs_h: hs.map(|v| v.floor() as usize),
            fracs_h: hs.map(|v| v - v.floor()),
            ref_l1: l1.floor() as usize,
            ref_l2: (l1 * 2.0).floor() as usize,
        };
        for (((l, r), lo), ro) in il.iter().zip(ir).zip(ol).zip(or) {
            if self.count < 0 || self.count > 11020 {
                self.count = 11020;
            }
            let count = self.count as usize;
            self.count -= 1;
            *lo = self.channels[0].sample(*l, count, &cfg);
            *ro = self.channels[1].sample(*r, count, &cfg);
        }
    }
}
