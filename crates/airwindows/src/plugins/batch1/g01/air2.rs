//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Air2 at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
// Preserve upstream constants and operation order.
#![allow(clippy::needless_range_loop, clippy::approx_constant)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Hiss",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Glitter",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Air",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Silk",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Air2",
    upstream_name: "Air2",
    description: "Air2 is a different bright EQ with silk tone and high sample rate support.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Air2::new(rate, seed))
}
#[derive(Default)]
struct Channel {
    prev: [f64; 3],
    even: [f64; 3],
    odd: [f64; 3],
    tp: f64,
    tm: f64,
    trip: [f64; 3],
    refs: [f64; 10],
}
impl Channel {
    fn band(&mut self, idx: usize, input: f64, flip: bool, intensity: f64, q: f64) -> f64 {
        let mut factor = self.prev[idx] - input;
        if flip {
            self.even[idx] += factor;
            self.odd[idx] -= factor;
            factor = self.even[idx] * intensity;
        } else {
            self.odd[idx] += factor;
            self.even[idx] -= factor;
            factor = self.odd[idx] * intensity;
        }
        self.odd[idx] = (self.odd[idx] - ((self.odd[idx] - self.even[idx]) / 256.0)) / q;
        self.even[idx] = (self.even[idx] - ((self.even[idx] - self.odd[idx]) / 256.0)) / q;
        self.prev[idx] = input;
        factor
    }
    fn triplet(&mut self, input: f64, count: usize, intensity: f64, q: f64) -> f64 {
        let mut factor = self.tp - input;
        self.trip[count] += factor;
        self.trip[(count + 2) % 3] -= factor;
        factor = self.trip[count] * intensity;
        self.tp = self.tm;
        self.tm = input;
        self.trip[0] /= q;
        self.trip[1] /= q;
        self.trip[2] /= q;
        factor
    }
    fn interpolate(&mut self, end: usize, value: f64) {
        match end {
            4 => {
                self.refs[0] = self.refs[4];
                self.refs[2] = (self.refs[0] + value) / 2.0;
                self.refs[1] = (self.refs[0] + self.refs[2]) / 2.0;
                self.refs[3] = (self.refs[2] + value) / 2.0;
                self.refs[4] = value;
            }
            3 => {
                self.refs[0] = self.refs[3];
                self.refs[2] = (self.refs[0] + self.refs[0] + value) / 3.0;
                self.refs[1] = (self.refs[0] + value + value) / 3.0;
                self.refs[3] = value;
            }
            2 => {
                self.refs[0] = self.refs[2];
                self.refs[1] = (self.refs[0] + value) / 2.0;
                self.refs[2] = value;
            }
            _ => self.refs[0] = value,
        }
    }
    fn average(&mut self, cycle: usize, end: usize) -> f64 {
        let mut value = self.refs[cycle];
        for i in (5..end + 4).rev() {
            self.refs[8] = value;
            value = (value + self.refs[i]) * 0.5;
            self.refs[i] = self.refs[8];
        }
        value
    }
}
struct Air2 {
    params: [f32; 5],
    rate: f32,
    seed: u32,
    ch: [Channel; 2],
    cycle: usize,
    count: usize,
    flip: [bool; 2],
    flop: bool,
    fpd: [u32; 2],
}
impl Air2 {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.5, 0.0, 1.0],
            rate,
            seed,
            ch: [Channel::default(), Channel::default()],
            cycle: 0,
            count: 0,
            flip: [false; 2],
            flop: false,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Air2 {
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
        match self.params.get(i) {
            Some(&v) if i < 3 => float2string((f64::from(v) * 2.0 - 1.0) as f32),
            Some(&v) => float2string(v),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let end = ((1.0 / 44100.0 * f64::from(self.rate)).floor() as usize).clamp(1, 4);
        self.cycle = self.cycle.min(end - 1);
        let mut a = [0.0; 3];
        for i in 0..3 {
            a[i] = f64::from(self.params[i]) * 2.0 - 1.0;
            if a[i] < 0.0 {
                a[i] *= [0.57525, 0.71325, 0.5712][i];
            }
            a[i] = -a[i].powf(3.0);
        }
        let q = [
            1.5 + (a[0] * 0.5).abs(),
            1.5 + (a[1] * 0.5).abs(),
            1.5 + (a[2] * 0.5).abs(),
        ];
        let intensity = f64::from(self.params[3]).powf(3.0) * 4.0 + 0.0001;
        let mix = f64::from(self.params[4]);
        let drymix = ((1.0 - mix) * 4.0).min(1.0);
        for (((&l, &r), left), right) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut input = [f64::from(l), f64::from(r)];
            for (i, x) in input.iter_mut().enumerate() {
                if x.abs() < 1.18e-23 {
                    *x = f64::from(self.fpd[i]) * 1.18e-17;
                }
            }
            let dry = input;
            self.cycle += 1;
            if self.cycle == end {
                for (i, &x) in input.iter().enumerate() {
                    let mut correction = 0.0;
                    if a[0].abs() > 0.0001 {
                        correction += self.ch[i].band(2, x, self.flop, a[0], q[0]);
                    }
                    if a[1].abs() > 0.0001 {
                        correction += self.ch[i].triplet(x, self.count, a[1], q[1]);
                    }
                    if a[2].abs() > 0.0001 {
                        let j = if if a[0].abs() > 0.0001 {
                            !self.flop
                        } else {
                            self.flop
                        } {
                            0
                        } else {
                            1
                        };
                        correction += self.ch[i].band(j, x, self.flip[j], a[2], q[2]);
                    }
                    correction *= intensity;
                    correction -= 1.0;
                    let bridge = correction.abs().min(1.57079633).sin();
                    correction = if correction > 0.0 { bridge } else { -bridge };
                    correction += 1.0_f64.sin();
                    correction /= intensity;
                    self.ch[i].interpolate(end, correction * 4.0 * mix);
                }
                if a[0].abs() > 0.0001 {
                    self.flop = !self.flop;
                }
                if a[1].abs() > 0.0001 {
                    self.count = (self.count + 1) % 3;
                }
                if a[2].abs() > 0.0001 {
                    let j = if self.flop { 0 } else { 1 };
                    self.flip[j] = !self.flip[j];
                }
                self.cycle = 0;
            }
            let mut out = [0.0; 2];
            for i in 0..2 {
                out[i] = self.ch[i].average(self.cycle, end);
                let mut d = dry[i];
                if drymix < 1.0 {
                    d *= drymix;
                }
                out[i] += d;
                out[i] = fpd_dither(out[i], &mut self.fpd[i]);
            }
            *left = out[0] as f32;
            *right = out[1] as f32;
        }
    }
}
