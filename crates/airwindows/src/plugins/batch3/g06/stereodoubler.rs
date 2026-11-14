//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/StereoDoubler, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Detune",
        label: "",
        default: 0.6,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.6,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "StereoDoubler",
    upstream_name: "StereoDoubler",
    description:
        "StereoDoubler is like GlitchShifter optimized for pitch shift doubling and tripling.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(StereoDoubler::new(sample_rate, seed))
}
struct Channel {
    buffer: Box<[f64]>,
    count: i32,
    last_count: i32,
    tracking: [f64; 9],
    temp: [f64; 9],
    position: [f64; 9],
    last_position: [f64; 9],
    active: usize,
    best: usize,
    prev: f64,
    even: f64,
    odd: f64,
    refs: [f64; 7],
}
impl Channel {
    fn new() -> Self {
        Self {
            buffer: vec![0.0; 5001].into_boxed_slice(),
            count: 0,
            last_count: 0,
            tracking: [0.0; 9],
            temp: [0.0; 9],
            position: [0.0; 9],
            last_position: [0.0; 9],
            active: 0,
            best: 4,
            prev: 0.0,
            even: 0.0,
            odd: 0.0,
            refs: [0.0; 7],
        }
    }
    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.count = 0;
        self.last_count = 0;
        self.tracking = [0.0; 9];
        self.temp = [0.0; 9];
        self.position = [0.0; 9];
        self.last_position = [0.0; 9];
        self.active = 0;
        self.best = 4;
        self.prev = 0.0;
        self.even = 0.0;
        self.odd = 0.0;
        self.refs = [0.0; 7];
    }
    fn process(&mut self, mut input: f64, speed: f64, flip: bool, wet: f64) -> f64 {
        let dry = input;
        let mut factor = self.prev - input;
        if flip {
            self.even += factor;
            self.odd -= factor;
            factor = self.even;
        } else {
            self.odd += factor;
            self.even -= factor;
            factor = self.odd;
        }
        self.odd = (self.odd - ((self.odd - self.even) / 256.0)) / 1.0001;
        self.even = (self.even - ((self.even - self.odd) / 256.0)) / 1.0001;
        self.prev = input;
        input += factor;
        self.count += 1;
        for p in &mut self.position[..8] {
            *p += speed;
        }
        let mut gp = self.count + 1100;
        let mut lp = self.last_count + 1100;
        if gp > 2300 {
            gp -= 2300;
        }
        if lp > 2300 {
            lp -= 2300;
        }
        if self.tracking[self.active] == 0.0 {
            let mut pp = self.position[self.active] + 1100.0;
            let mut lpp = self.last_position[self.active] + 1100.0;
            if pp > 2300.0 {
                pp -= 2300.0;
            }
            if lpp > 2300.0 {
                lpp -= 2300.0;
            }
            if (gp as f64 > self.position[self.active])
                && ((lp as f64) < self.last_position[self.active])
            {
                self.tracking[self.active] = 1.0;
            }
            if (pp > self.count as f64) && (lpp < self.last_count as f64) {
                self.tracking[self.active] = 1.0;
            }
        }
        for i in 0..8 {
            if self.position[i] > 2300.0 {
                self.position[i] -= 2300.0;
            }
            self.last_position[i] = self.position[i];
        }
        if self.count < 0 || self.count > 2300 {
            self.count -= 2300;
        }
        self.last_count = self.count;
        let idx = self.count as usize;
        self.buffer[idx] = input;
        self.buffer[idx + 2300] = input;
        for i in 0..8 {
            let base = self.position[i].floor() as usize;
            let fraction = self.position[i] - base as f64;
            self.temp[i] = self.buffer[base] * (1.0 - fraction);
            self.temp[i] += self.buffer[base + 1];
            self.temp[i] += self.buffer[base + 2] * fraction;
            self.temp[i] -= ((self.buffer[base] - self.buffer[base + 1])
                - (self.buffer[base + 1] - self.buffer[base + 2]))
                / 50.0;
            self.temp[i] /= 2.0;
        }
        input = if self.tracking[self.active] > 0.0 {
            let cross = (self.tracking[self.best] * 1.57).sin();
            let result = (self.temp[self.active] * cross) + (self.temp[self.best] * (1.0 - cross));
            for i in 0..8 {
                let depth = 0.5 - (self.temp[self.active] - self.temp[i]).abs();
                if depth > 0.0 && i != self.active {
                    self.tracking[i] -= depth / 1100.0;
                    self.best = i;
                }
            }
            let mut bestyet = 1.0;
            for i in 0..8 {
                if self.tracking[i] < bestyet && i != self.active {
                    self.best = i;
                    bestyet = self.tracking[i];
                }
            }
            if self.tracking[self.best] < 0.0 {
                for i in 0..8 {
                    self.tracking[i] = 1.0;
                }
                self.active = self.best;
                self.tracking[self.active] = 0.0;
            }
            result
        } else {
            self.temp[self.active]
        };
        (input * wet) + (dry * (1.0 - wet))
    }
    fn interpolate(&mut self, v: f64, end: usize) {
        let r = &mut self.refs;
        match end {
            4 => {
                r[0] = r[4];
                r[2] = (r[0] + v) / 2.0;
                r[1] = (r[0] + r[2]) / 2.0;
                r[3] = (r[2] + v) / 2.0;
                r[4] = v;
            }
            3 => {
                r[0] = r[3];
                r[2] = (r[0] + r[0] + v) / 3.0;
                r[1] = (r[0] + v + v) / 3.0;
                r[3] = v;
            }
            2 => {
                r[0] = r[2];
                r[1] = (r[0] + v) / 2.0;
                r[2] = v;
            }
            _ => r[0] = v,
        }
    }
}
struct StereoDoubler {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    ch: [Channel; 2],
    prevwidth: i32,
    flip: bool,
    cycle: usize,
    fpd: [u32; 2],
}
impl StereoDoubler {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.6; 2],
            sample_rate,
            seed,
            ch: [Channel::new(), Channel::new()],
            prevwidth: 643,
            flip: false,
            cycle: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for StereoDoubler {
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
            0 => float2string((self.params[0] as f64 * 2.0 - 1.0) as f32),
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        for c in &mut self.ch {
            c.reset();
        }
        self.prevwidth = 643;
        self.flip = false;
        self.cycle = 0;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let end = (scale as usize).clamp(1, 4);
        self.cycle = self.cycle.min(end - 1);
        let mut trim = (self.params[0] as f64 * 2.0) - 1.0;
        trim *= trim.abs();
        trim /= 40.0;
        let speed = [(trim + 1.0).max(0.0), (-trim + 1.0).max(0.0)];
        if self.prevwidth != 2300 {
            for c in &mut self.ch {
                c.position[..8]
                    .copy_from_slice(&[0.0, 766.0, 1532.0, 460.0, 920.0, 1380.0, 1840.0, 1150.0]);
            }
            self.prevwidth = 2300;
        }
        let wet = self.params[1] as f64;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            for (i, v) in input.iter_mut().enumerate() {
                if v.abs() < 1.18e-23 {
                    *v = self.fpd[i] as f64 * 1.18e-17;
                }
            }
            self.cycle += 1;
            if self.cycle == end {
                for i in 0..2 {
                    let v = self.ch[i].process(input[i], speed[i], self.flip, wet);
                    self.ch[i].interpolate(v, end);
                }
                self.flip = !self.flip;
                self.cycle = 0;
            }
            *outl = fpd_dither(self.ch[0].refs[self.cycle], &mut self.fpd[0]) as f32;
            *outr = fpd_dither(self.ch[1].refs[self.cycle], &mut self.fpd[1]) as f32;
        }
    }
}
