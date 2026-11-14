//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Tape, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Slam",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Bump",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Tape",
    upstream_name: "Tape",
    description: "Tape is simplified, all-purpose tape mojo: my personal jam.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Tape::new(sample_rate, seed))
}
struct Channel {
    mid: [f64; 2],
    bump: [f64; 2],
    head: [[f64; 9]; 2],
    body: [[f64; 9]; 2],
    last: f64,
    fpd: u32,
}
impl Channel {
    fn new(seed: u32, lane: u32) -> Self {
        Self {
            mid: [0.0; 2],
            bump: [0.0; 2],
            head: [[0.0; 9]; 2],
            body: [[0.0; 9]; 2],
            last: 0.0,
            fpd: fpd_from_seed(seed, lane),
        }
    }
    fn configure(f: &mut [f64; 9], freq: f64, res: f64) {
        f[0] = freq;
        f[1] = res;
        let k = (std::f64::consts::PI * f[0]).tan();
        let norm = 1.0 / (1.0 + k / f[1] + k * k);
        f[2] = k / f[1] * norm;
        f[4] = -f[2];
        f[5] = 2.0 * (k * k - 1.0) * norm;
        f[6] = (1.0 - k / f[1] + k * k) * norm;
    }
    fn filter(x: f64, f: &mut [f64; 9]) -> f64 {
        let t = x * f[2] + f[7];
        f[7] = x * f[3] - t * f[5] + f[8];
        f[8] = x * f[4] - t * f[6];
        t.clamp(-1.0, 1.0).asin()
    }
    #[allow(clippy::approx_constant, clippy::excessive_precision)]
    fn process(
        &mut self,
        mut x: f64,
        idx: usize,
        roll: f64,
        bumpfreq: f64,
        gain: f64,
        bumpgain: f64,
    ) -> f64 {
        if x.abs() < 1.18e-23 {
            x = self.fpd as f64 * 1.18e-17;
        }
        let dry = x;
        self.mid[idx] = self.mid[idx] * (1.0 - roll) + x * roll;
        let high = x - self.mid[idx];
        self.bump[idx] += x * 0.05;
        self.bump[idx] -= self.bump[idx] * self.bump[idx] * self.bump[idx] * bumpfreq;
        self.bump[idx] = self.bump[idx].sin();
        self.bump[idx] = Self::filter(self.bump[idx], &mut self.head[idx]);
        x = Self::filter(x.sin(), &mut self.body[idx]);
        let ground = dry - x;
        if gain != 1.0 {
            x *= gain;
        }
        let mut soften = high.abs() * 1.57079633;
        if soften > 1.57079633 {
            soften = 1.57079633;
        }
        soften = 1.0 - soften.cos();
        if high > 0.0 {
            x -= soften;
        }
        if high < 0.0 {
            x += soften;
        }
        x = x.clamp(-1.2533141373155, 1.2533141373155);
        x = (x * x.abs()).sin() / if x.abs() == 0.0 { 1.0 } else { x.abs() };
        let suppress = (1.0 - x.abs()) * 0.00013;
        for b in &mut self.bump {
            if *b > suppress {
                *b -= suppress;
            }
            if *b < -suppress {
                *b += suppress;
            }
        }
        x += ground;
        x += (self.bump[0] + self.bump[1]) * bumpgain;
        let softness = 0.618033988749894848204586;
        if self.last >= 0.99 {
            if x < 0.99 {
                self.last = 0.99 * softness + x * (1.0 - softness);
            } else {
                self.last = 0.99;
            }
        }
        if self.last <= -0.99 {
            if x > -0.99 {
                self.last = -0.99 * softness + x * (1.0 - softness);
            } else {
                self.last = -0.99;
            }
        }
        if x > 0.99 {
            if self.last < 0.99 {
                x = 0.99 * softness + self.last * (1.0 - softness);
            } else {
                x = 0.99;
            }
        }
        if x < -0.99 {
            if self.last > -0.99 {
                x = -0.99 * softness + self.last * (1.0 - softness);
            } else {
                x = -0.99;
            }
        }
        self.last = x;
        x = x.clamp(-0.99, 0.99);
        fpd_dither(x, &mut self.fpd)
    }
}
struct Tape {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    l: Channel,
    r: Channel,
    flip: bool,
}
impl Tape {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5],
            sample_rate,
            seed,
            l: Channel::new(seed, 0),
            r: Channel::new(seed, 1),
            flip: false,
        }
    }
}
impl AirwindowsEffect for Tape {
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
            0 => float2string(((self.params[0] as f64 - 0.5) * 24.0) as f32),
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.l = Channel::new(self.seed, 0);
        self.r = Channel::new(self.seed, 1);
        self.flip = false;
    }
    #[allow(clippy::excessive_precision)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let gain = 10.0_f64.powf(((self.params[0] as f64 - 0.5) * 24.0) / 20.0);
        let bumpgain = self.params[1] as f64 * 0.1;
        let bumpfreq = 0.12 / scale;
        let roll = (1.0 - 0.618033988749894848204586) / scale;
        for ch in [&mut self.l, &mut self.r] {
            for i in 0..2 {
                Channel::configure(&mut ch.head[i], 0.0072 / scale, 0.0009);
                Channel::configure(&mut ch.body[i], 0.032 / scale, 0.0007);
            }
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let idx = if self.flip { 0 } else { 1 };
            *out_l = self
                .l
                .process(l as f64, idx, roll, bumpfreq, gain, bumpgain) as f32;
            *out_r = self
                .r
                .process(r as f64, idx, roll, bumpfreq, gain, bumpgain) as f32;
            self.flip = !self.flip;
        }
    }
}
