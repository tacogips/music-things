//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Noise, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "HighCut",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "LowCut",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "LShape",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Decay",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Distnc",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Noise",
    upstream_name: "Noise",
    description: "Noise is the Airwindows deep noise oscillator, as a sound reinforcer.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Noise::new(rate, seed))
}
#[derive(Default)]
struct Channel {
    a: f64,
    b: f64,
    c: f64,
    rumble: [f64; 2],
    surge: f64,
    flip: bool,
    history: [f64; 10],
}
struct Noise {
    params: [f32; 6],
    rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    position: i32,
    quadratic: i32,
    filter_flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl Noise {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.5, 1.0, 0.0, 1.0],
            rate,
            seed,
            left: Channel::default(),
            right: Channel::default(),
            position: 99999999,
            quadratic: 0,
            filter_flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn surge(c: &mut Channel, sample: f64, fpd: u32, target: f64, decay: f64) -> f64 {
    let random = fpd as f64 / u32::MAX as f64;
    if c.surge < sample.abs() {
        c.surge += random * (sample.abs() - c.surge);
        if c.surge > 1.0 {
            c.surge = 1.0;
        }
    } else {
        c.surge -= random * (c.surge - sample.abs()) * decay;
        if c.surge < 0.0 {
            c.surge = 0.0;
        }
    }
    (target * c.surge).powf(5.0).min(1.0)
}
#[allow(clippy::too_many_arguments, clippy::needless_range_loop)]
fn channel(
    c: &mut Channel,
    dry: f64,
    cutoff: f64,
    rumblecutoff: f64,
    wet: f64,
    fpd: u32,
    filter_flip: bool,
    weights: &[f64; 10],
) -> f64 {
    let random = fpd as f64 / u32::MAX as f64;
    if c.flip {
        c.a += random;
    } else {
        c.a -= random;
    }
    let mut sample;
    if filter_flip {
        c.b *= 1.0 - cutoff;
        c.b += c.a * cutoff;
        sample = c.b + c.c;
        c.rumble[0] *= 1.0 - rumblecutoff;
        c.rumble[0] += sample * rumblecutoff;
    } else {
        c.c *= 1.0 - cutoff;
        c.c += c.a * cutoff;
        sample = c.b + c.c;
        c.rumble[1] *= 1.0 - rumblecutoff;
        c.rumble[1] += sample * rumblecutoff;
    }
    sample -= c.rumble[0] + c.rumble[1];
    sample *= 1.0 - rumblecutoff;
    sample *= wet;
    sample += dry * (1.0 - wet);
    c.history.copy_within(0..9, 1);
    c.history[0] = sample;
    let mut acc = sample;
    acc *= weights[0];
    for i in 1..10 {
        acc += c.history[i] * weights[i];
    }
    let correction = sample - acc;
    sample -= correction;
    sample
}
impl AirwindowsEffect for Noise {
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
            .map_or_else(String::new, |p| float2string(*p))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let p = self.params.map(f64::from);
        let cutofftarget = p[0] * 3.5;
        let rumblecutoff = cutofftarget * 0.005;
        const DCUT: [i32; 39] = [
            3, 3, 5, 7, 11, 13, 17, 19, 23, 27, 31, 53, 71, 113, 131, 173, 191, 233, 293, 311, 373,
            419, 431, 479, 541, 593, 613, 673, 719, 733, 797, 839, 907, 971, 1013, 1031, 1087,
            1091, 1151,
        ];
        let dcut = DCUT[((p[2] * 38.0).floor() as usize).min(38)];
        let lowcut = (p[1] * 22.0).floor() as i32 + 1;
        let mut decay = 0.001 - ((1.0 - (1.0 - p[3]).powf(3.0)) * 0.001);
        if decay == 0.001 {
            decay = 0.1;
        }
        let wet = p[5] * 0.01;
        let overallscale = p[4] * 9.0 + 1.0;
        let mut gain = overallscale;
        let mut f = [0.0; 10];
        for weight in &mut f {
            if gain > 1.0 {
                *weight = 1.0;
                gain -= 1.0;
            } else {
                *weight = gain;
                gain = 0.0;
            }
            *weight /= overallscale.max(1.0);
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let cl = surge(&mut self.left, l, self.fpd_l, cutofftarget, decay);
            let cr = surge(&mut self.right, r, self.fpd_r, cutofftarget, decay);
            self.left.flip = !self.left.flip;
            self.right.flip = !self.right.flip;
            self.filter_flip = !self.filter_flip;
            self.quadratic -= 1;
            if self.quadratic < 0 {
                // The pinned clang build treats squared operands as nonnegative for remainder.
                self.position = self.position.wrapping_add(1);
                self.quadratic = (self.position.wrapping_mul(self.position) as u32 % 170003) as i32;
                self.quadratic =
                    (self.quadratic.wrapping_mul(self.quadratic) as u32 % 17011) as i32;
                self.quadratic = self.quadratic.wrapping_mul(self.quadratic) % dcut;
                self.quadratic = self.quadratic.wrapping_mul(self.quadratic) % lowcut;
                self.left.flip = self.left.a < 0.0;
                self.right.flip = self.right.a < 0.0;
            }
            xorshift(&mut self.fpd_l);
            xorshift(&mut self.fpd_r);
            l = channel(
                &mut self.left,
                l,
                cl,
                rumblecutoff,
                wet,
                self.fpd_l,
                self.filter_flip,
                &f,
            );
            r = channel(
                &mut self.right,
                r,
                cr,
                rumblecutoff,
                wet,
                self.fpd_r,
                self.filter_flip,
                &f,
            );
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
