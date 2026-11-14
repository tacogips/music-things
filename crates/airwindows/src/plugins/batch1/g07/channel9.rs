//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Channel9/Channel9.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.0.
use crate::{
    display::{float2string, int2string},
    dither::{fpd_dither, fpd_from_seed},
    params::sanitize_param,
    rand::{replay_fpd_seed_loop, AwRand},
    registry::PluginEntry,
};
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Console Type",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Drive",
        label: "%",
        default: 0.0,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo { name:"Channel9", upstream_name:"Channel9", description:"Channel9 is Channel8 with ultrasonic filtering, and new Teac and Mackie 'low end' settings.", params:PARAMS };
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Channel9::new(rate, seed))
}
struct Channel9 {
    params: [f32; 3],
    rate: f32,
    seed: u32,
    fpd: [u32; 2],
    iir: [[f64; 2]; 2],
    last: [[f64; 3]; 2],
    flip: bool,
    amount: f64,
    threshold: f64,
    biquad: [[f64; 15]; 2],
    cutoff: f64,
}
impl Channel9 {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.0, 1.0],
            rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            iir: [[0.0; 2]; 2],
            last: [[0.0; 3]; 2],
            flip: false,
            amount: 0.005832,
            threshold: 0.33362176,
            biquad: [[0.0; 15]; 2],
            cutoff: 28811.0,
        }
    }
}
fn filter(b: &mut [f64; 15], x: f64, c: usize) -> f64 {
    let i = if c == 0 { 7 } else { 11 };
    let mut y = b[2] * x + b[3] * b[i] + b[4] * b[i + 1] - b[5] * b[i + 2] - b[6] * b[i + 3];
    b[i + 1] = b[i];
    b[i] = x;
    if y.abs() < 1.18e-37 {
        y = 0.0;
    }
    b[i + 3] = b[i + 2];
    b[i + 2] = y;
    y
}

impl AirwindowsEffect for Channel9 {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(slot), Some(v)) = (self.params.get_mut(index), sanitize_param(value)) {
            *slot = v;
            let (amount, threshold, cutoff) = match (self.params[0] as f64 * 4.999) as i32 {
                0 => (0.005832, 0.33362176, 28811.0),
                1 => (0.004096, 0.59969536, 27216.0),
                2 => (0.004913, 0.84934656, 23011.0),
                3 => (0.009216, 0.149, 18544.0),
                4 => (0.011449, 0.092, 19748.0),
                _ => return,
            };
            self.amount = amount;
            self.threshold = threshold;
            self.cutoff = cutoff;
        }
    }
    fn param_display(&self, index: usize) -> String {
        match index {
            0 => ["Neve", "API", "SSL", "Teac", "Mackie"][(self.params[0] as f64 * 4.999) as usize]
                .into(),
            1 => int2string((self.params[1] * 200.0) as i32),
            2 => float2string(self.params[2]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        for (i, p) in params.into_iter().enumerate() {
            self.set_param(i, p);
        }
    }
    // Preserve upstream constants for parity.
    #[allow(clippy::approx_constant, clippy::excessive_precision)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let scale = (1.0 / 44100.0) * self.rate as f64;
        let amount = self.amount / scale;
        let mut density = self.params[1] as f64 * 2.0;
        let mut phattity = density - 1.0;
        if density > 1.0 {
            density = 1.0;
        }
        if phattity < 0.0 {
            phattity = 0.0;
        }
        let nonlinear = 5.0 - density;

        for (idx, q) in [(0, 1.6180339887498948), (1, 0.6180339887498948)] {
            let b = &mut self.biquad[idx];
            b[0] = self.cutoff / self.rate as f64;
            b[1] = q;
            let k = (std::f64::consts::PI * b[0]).tan();
            let norm = 1.0 / (1.0 + k / b[1] + k * k);
            b[2] = k * k * norm;
            b[3] = 2.0 * b[2];
            b[4] = b[2];
            b[5] = 2.0 * (k * k - 1.0) * norm;
            b[6] = (1.0 - k / b[1] + k * k) * norm;
        }

        for (((&l, &r), o_l), o_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut out = [0.0f32; 2];
            for (c, v) in [l, r].into_iter().enumerate() {
                let mut x = v as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[c] as f64 * 1.18e-17;
                }
                if self.biquad[0][0] < 0.49999 {
                    x = filter(&mut self.biquad[0], x, c);
                }
                let dielectric = (2.0 - ((x + nonlinear) / nonlinear)).abs();
                let bank = usize::from(!self.flip);
                if self.iir[bank][c].abs() < 1.18e-37 {
                    self.iir[bank][c] = 0.0;
                }
                self.iir[bank][c] =
                    (self.iir[bank][c] * (1.0 - (amount * dielectric))) + (x * amount * dielectric);
                x -= self.iir[bank][c];
                let dry = x;
                x = x.clamp(-1.0, 1.0);
                let phat = (x * 1.57079633).sin();
                x *= 1.2533141373155;
                let dist = (x * x.abs()).sin() / if x.abs() == 0.0 { 1.0 } else { x.abs() };
                x = dist;
                if density < 1.0 {
                    x = (dry * (1.0 - density)) + (dist * density);
                }
                if phattity > 0.0 {
                    x = (x * (1.0 - phattity)) + (phat * phattity);
                }
                let a = self.last[c][0];
                let b = self.last[c][1];
                let cc = self.last[c][2];
                let mut clamp = (b - cc) * 0.381966011250105;
                clamp -= (a - b) * 0.6180339887498948482045;
                clamp += x - a;
                self.last[c][2] = b;
                self.last[c][1] = a;
                self.last[c][0] = x;
                if clamp > self.threshold {
                    x = a + self.threshold;
                }
                if -clamp > self.threshold {
                    x = a - self.threshold;
                }
                self.last[c][0] =
                    (self.last[c][0] * 0.381966011250105) + (x * 0.6180339887498948482045);
                if self.params[2] < 1.0 {
                    x *= self.params[2] as f64;
                }
                if self.biquad[1][0] < 0.49999 {
                    x = filter(&mut self.biquad[1], x, c);
                }
                out[c] = fpd_dither(x, &mut self.fpd[c]) as f32;
            }
            self.flip = !self.flip;
            *o_l = out[0];
            *o_r = out[1];
        }
    }
}
