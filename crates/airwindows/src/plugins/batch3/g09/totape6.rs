//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ToTape6, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
#![allow(clippy::approx_constant, clippy::excessive_precision)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Input",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Soften",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Head B",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Flutter",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "ToTape6",
    upstream_name: "ToTape6",
    description: "ToTape6 is Airwindows tape emulation for 2020!",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ToTape6::new(sample_rate, seed))
}
#[derive(Clone)]
struct Channel {
    delay: [f64; 502],
    roller: [f64; 2],
    bump: [f64; 2],
    low: [[f64; 9]; 2],
    high: [[f64; 9]; 2],
    last: f64,
}
impl Default for Channel {
    fn default() -> Self {
        Self {
            delay: [0.0; 502],
            roller: [0.0; 2],
            bump: [0.0; 2],
            low: [[0.0; 9]; 2],
            high: [[0.0; 9]; 2],
            last: 0.0,
        }
    }
}
fn configure(q: &mut [f64; 9], freq: f64, res: f64) {
    q[0] = freq;
    q[1] = res;
    let k = (std::f64::consts::PI * q[0]).tan();
    let norm = 1.0 / (1.0 + k / q[1] + k * k);
    q[2] = k / q[1] * norm;
    q[4] = -q[2];
    q[5] = 2.0 * (k * k - 1.0) * norm;
    q[6] = (1.0 - k / q[1] + k * k) * norm;
}
fn filter(x: f64, q: &mut [f64; 9]) -> f64 {
    let temp = (x * q[2]) + q[7];
    q[7] = (x * q[3]) - (temp * q[5]) + q[8];
    q[8] = (x * q[4]) - (temp * q[6]);
    temp.clamp(-1.0, 1.0).asin()
}
impl Channel {
    #[allow(clippy::too_many_arguments)]
    fn process(
        &mut self,
        mut x: f64,
        flip: usize,
        roll: f64,
        head_freq: f64,
        head_control: f64,
        inputgain: f64,
        outputgain: f64,
    ) -> f64 {
        self.roller[flip] = (self.roller[flip] * (1.0 - roll)) + (x * roll);
        let highs = x - self.roller[flip];
        self.bump[flip] += x * 0.05;
        self.bump[flip] -= self.bump[flip] * self.bump[flip] * self.bump[flip] * head_freq;
        self.bump[flip] = filter(self.bump[flip].sin(), &mut self.low[flip]);
        let vib = x;
        x = filter(x.sin(), &mut self.high[flip]);
        let ground = vib - x;
        if inputgain > 1.0 {
            x *= inputgain;
        }
        let mut soften = highs.abs() * 1.57079633;
        if soften > 1.57079633 {
            soften = 1.57079633;
        }
        soften = 1.0 - soften.cos();
        if highs > 0.0 {
            x -= soften;
        }
        if highs < 0.0 {
            x += soften;
        }
        let suppress = (1.0 - x.abs()) * 0.00013;
        for bump in &mut self.bump {
            if *bump > suppress {
                *bump -= suppress;
            }
            if *bump < -suppress {
                *bump += suppress;
            }
        }
        x += (self.bump[0] + self.bump[1]) * head_control;
        x = x.clamp(-1.0, 1.0);
        let mojo = x.abs().powf(0.25);
        if mojo > 0.0 {
            x = (x * mojo * std::f64::consts::PI * 0.5).sin() / mojo;
        }
        x += ground;
        if outputgain != 1.0 {
            x *= outputgain;
        }
        let clip = 0.99;
        let softness = 0.618033988749894848204586;
        if self.last >= clip {
            if x < clip {
                self.last = clip * softness + x * (1.0 - softness);
            } else {
                self.last = clip;
            }
        }
        if self.last <= -clip {
            if x > -clip {
                self.last = -clip * softness + x * (1.0 - softness);
            } else {
                self.last = -clip;
            }
        }
        if x > clip {
            if self.last < clip {
                x = clip * softness + self.last * (1.0 - softness);
            } else {
                x = clip;
            }
        }
        if x < -clip {
            if self.last > -clip {
                x = -clip * softness + self.last * (1.0 - softness);
            } else {
                x = -clip;
            }
        }
        self.last = x;
        x.clamp(-clip, clip)
    }
}
struct ToTape6 {
    params: [f32; 6],
    sample_rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    count: i32,
    rate: f64,
    sweep: f64,
    next: f64,
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl ToTape6 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.5, 0.5, 0.5, 1.0],
            sample_rate,
            seed,
            left: Channel::default(),
            right: Channel::default(),
            count: 0,
            rate: 0.5,
            sweep: std::f64::consts::PI,
            next: 0.5,
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for ToTape6 {
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
            0 | 4 => float2string(((self.params[i] as f64 - 0.5) * 24.0) as f32),
            1..=3 | 5 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e, f] = self.params;
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let inputgain = 10.0f64.powf(((a as f64 - 0.5) * 24.0) / 20.0);
        let roll = (1.0 - ((b as f64).powf(2.0) * 0.45)) / scale;
        let head = c as f64 * 0.25 * inputgain;
        let head_freq = 0.12 / scale;
        for channel in [&mut self.left, &mut self.right] {
            for i in 0..2 {
                configure(&mut channel.low[i], 0.007 / scale, 0.0009);
                configure(&mut channel.high[i], 0.032 / scale, 0.0007);
            }
        }
        let depth = (d as f64).powf(2.0) * scale * 70.0;
        let trim = (0.0024 * (d as f64).powf(2.0)) / scale;
        let outputgain = 10.0f64.powf(((e as f64 - 0.5) * 24.0) / 20.0);
        let wet = f as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            if inputgain < 1.0 {
                l *= inputgain;
                r *= inputgain;
            }
            let random = self.fpd_l as f64 / u32::MAX as f64;
            if self.count < 0 || self.count > 499 {
                self.count = 499;
            }
            self.left.delay[self.count as usize] = l;
            self.right.delay[self.count as usize] = r;
            if depth != 0.0 {
                let offset = depth + (depth * self.rate.powf(2.0) * self.sweep.sin());
                let count = self.count + offset.floor() as i32;
                let i = (count - if count > 499 { 500 } else { 0 }) as usize;
                let j = (count + 1 - if count + 1 > 499 { 500 } else { 0 }) as usize;
                l = self.left.delay[i] * (1.0 - (offset - offset.floor()));
                r = self.right.delay[i] * (1.0 - (offset - offset.floor()));
                l += self.left.delay[j] * (offset - offset.floor());
                r += self.right.delay[j] * (offset - offset.floor());
                self.rate = (self.rate * (1.0 - trim)) + (self.next * trim);
                self.sweep += self.rate * trim;
                if self.sweep >= std::f64::consts::PI * 2.0 {
                    self.sweep -= std::f64::consts::PI;
                    self.next = 0.24 + (random * 0.74);
                }
            }
            self.count -= 1;
            let i = if self.flip { 0 } else { 1 };
            l = self
                .left
                .process(l, i, roll, head_freq, head, inputgain, outputgain);
            r = self
                .right
                .process(r, i, roll, head_freq, head, inputgain, outputgain);
            self.flip = !self.flip;
            if wet != 1.0 {
                l = (l * wet) + (dry_l * (1.0 - wet));
                r = (r * wet) + (dry_r * (1.0 - wet));
            }
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
