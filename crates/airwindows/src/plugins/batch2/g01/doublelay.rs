//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Doublelay, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Detune",
        label: "",
        default: 0.2,
    },
    ParamInfo {
        name: "Delay L",
        label: "sec",
        default: 0.1,
    },
    ParamInfo {
        name: "Delay R",
        label: "sec",
        default: 0.2,
    },
    ParamInfo {
        name: "Feedbk",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.6,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Doublelay",
    upstream_name: "Doublelay",
    description: "Doublelay is StereoDoubler with independent delays and feedback.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Doublelay::new(rate, seed))
}
struct Channel {
    delay: Box<[f64]>,
    pitch: Box<[f64]>,
    count: i32,
    last_count: i32,
    position: [f64; 8],
    last_position: [f64; 8],
    tracking: [f64; 8],
    active: usize,
    best: usize,
    feedback: f64,
    air_prev: f64,
    air_even: f64,
    air_odd: f64,
    reference: [f64; 7],
    fpd: u32,
}
impl Channel {
    fn new(seed: u32, lane: u32) -> Self {
        Self {
            delay: vec![0.0; 48010].into_boxed_slice(),
            pitch: vec![0.0; 5010].into_boxed_slice(),
            count: 0,
            last_count: 0,
            position: [0.0; 8],
            last_position: [0.0; 8],
            tracking: [0.0; 8],
            active: 0,
            best: 4,
            feedback: 0.0,
            air_prev: 0.0,
            air_even: 0.0,
            air_odd: 0.0,
            reference: [0.0; 7],
            fpd: fpd_from_seed(seed, lane),
        }
    }
    fn reset(&mut self, seed: u32, lane: u32) {
        self.delay.fill(0.0);
        self.pitch.fill(0.0);
        self.count = 0;
        self.last_count = 0;
        self.position = [0.0; 8];
        self.last_position = [0.0; 8];
        self.tracking = [0.0; 8];
        self.active = 0;
        self.best = 4;
        self.feedback = 0.0;
        self.air_prev = 0.0;
        self.air_even = 0.0;
        self.air_odd = 0.0;
        self.reference = [0.0; 7];
        self.fpd = fpd_from_seed(seed, lane);
    }
    fn pitch_shift(&mut self, input: f64, speed: f64) -> f64 {
        const WIDTH: i32 = 2300;
        const ADJUST: f64 = 1100.0;
        self.count += 1;
        for position in &mut self.position {
            *position += speed;
        }
        let mut gplus = self.count + 1100;
        let mut lastplus = self.last_count + 1100;
        if gplus > WIDTH {
            gplus -= WIDTH;
        }
        if lastplus > WIDTH {
            lastplus -= WIDTH;
        }
        if self.tracking[self.active] == 0.0 {
            let mut posplus = self.position[self.active] + ADJUST;
            let mut lastposplus = self.last_position[self.active] + ADJUST;
            if posplus > WIDTH as f64 {
                posplus -= WIDTH as f64;
            }
            if lastposplus > WIDTH as f64 {
                lastposplus -= WIDTH as f64;
            }
            if (gplus as f64 > self.position[self.active])
                && ((lastplus as f64) < self.last_position[self.active])
            {
                self.tracking[self.active] = 1.0;
            }
            if (posplus > self.count as f64) && (lastposplus < self.last_count as f64) {
                self.tracking[self.active] = 1.0;
            }
        }
        for i in 0..8 {
            if self.position[i] > WIDTH as f64 {
                self.position[i] -= WIDTH as f64;
            }
            self.last_position[i] = self.position[i];
        }
        if self.count < 0 || self.count > WIDTH {
            self.count -= WIDTH;
        }
        self.last_count = self.count;
        let count = self.count as usize;
        self.pitch[count] = input;
        self.pitch[count + WIDTH as usize] = input;
        let mut temp = [0.0; 8];
        for (i, value) in temp.iter_mut().enumerate() {
            let base = self.position[i].floor() as usize;
            let frac = self.position[i] - base as f64;
            *value = self.pitch[base] * (1.0 - frac);
            *value += self.pitch[base + 1];
            *value += self.pitch[base + 2] * frac;
            *value -= ((self.pitch[base] - self.pitch[base + 1])
                - (self.pitch[base + 1] - self.pitch[base + 2]))
                / 50.0;
            *value /= 2.0;
        }
        if self.tracking[self.active] > 0.0 {
            let crossfade = (self.tracking[self.best] * 1.57).sin();
            let output = temp[self.active] * crossfade + temp[self.best] * (1.0 - crossfade);
            for i in 0..8 {
                let depth = 0.5 - (temp[self.active] - temp[i]).abs();
                if depth > 0.0 && i != self.active {
                    self.tracking[i] -= depth / ADJUST;
                    self.best = i;
                }
            }
            let mut best_yet = 1.0;
            for i in 0..8 {
                if self.tracking[i] < best_yet && i != self.active {
                    self.best = i;
                    best_yet = self.tracking[i];
                }
            }
            if self.tracking[self.best] < 0.0 {
                self.tracking.fill(1.0);
                self.active = self.best;
                self.tracking[self.active] = 0.0;
            }
            output
        } else {
            temp[self.active]
        }
    }
    fn interpolate(&mut self, sample: f64, end: usize) {
        match end {
            4 => {
                self.reference[0] = self.reference[4];
                self.reference[2] = (self.reference[0] + sample) / 2.0;
                self.reference[1] = (self.reference[0] + self.reference[2]) / 2.0;
                self.reference[3] = (self.reference[2] + sample) / 2.0;
                self.reference[4] = sample;
            }
            3 => {
                self.reference[0] = self.reference[3];
                self.reference[2] = (self.reference[0] + self.reference[0] + sample) / 3.0;
                self.reference[1] = (self.reference[0] + sample + sample) / 3.0;
                self.reference[3] = sample;
            }
            2 => {
                self.reference[0] = self.reference[2];
                self.reference[1] = (self.reference[0] + sample) / 2.0;
                self.reference[2] = sample;
            }
            _ => self.reference[0] = sample,
        }
    }
}
struct Doublelay {
    params: [f32; 5],
    rate: f32,
    seed: u32,
    channels: [Channel; 2],
    dcount: i32,
    cycle: usize,
    flip: bool,
    initialized: bool,
}
impl Doublelay {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.2, 0.1, 0.2, 0.0, 0.6],
            rate,
            seed,
            channels: [Channel::new(seed, 0), Channel::new(seed, 1)],
            dcount: 0,
            cycle: 0,
            flip: false,
            initialized: false,
        }
    }
}
impl AirwindowsEffect for Doublelay {
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
            Some(v) => float2string(if i == 0 {
                (*v as f64 * 2.0 - 1.0) as f32
            } else {
                *v
            }),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        for (i, c) in self.channels.iter_mut().enumerate() {
            c.reset(self.seed, i as u32);
        }
        self.dcount = 0;
        self.cycle = 0;
        self.flip = false;
        self.initialized = false;
    }
    #[allow(clippy::excessive_precision)] // Preserve upstream literal.
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let p = self.params.map(f64::from);
        let end = (((1.0 / 44100.0) * self.rate as f64).floor() as usize).clamp(1, 4);
        self.cycle = self.cycle.min(end - 1);
        let trim_delay = ((self.rate / end as f32) as f64 / 48001.0).clamp(0.0, 0.99999);
        let mut trim = p[0] * 2.0 - 1.0;
        trim *= trim.abs();
        trim /= 40.0;
        let speeds = [(trim + 1.0).max(0.0), (1.0 - trim).max(0.0)];
        let delay_max = (48000.0 * trim_delay) as i32;
        let delays = [
            (p[1] * delay_max as f64) as i32,
            (p[2] * delay_max as f64) as i32,
        ];
        if !self.initialized {
            for c in &mut self.channels {
                c.position = [0.0, 766.0, 1532.0, 460.0, 920.0, 1380.0, 1840.0, 1150.0];
            }
            self.initialized = true;
        }
        let direct = p[3] * 0.618033988749894848204586;
        let cross = p[3] * (1.0 - 0.618033988749894848204586);
        for (((&l, &r), ol), or) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut x = [l as f64, r as f64];
            for (ch, s) in x.iter_mut().enumerate() {
                if s.abs() < 1.18e-23 {
                    *s = self.channels[ch].fpd as f64 * 1.18e-17;
                }
            }
            self.cycle += 1;
            if self.cycle == end {
                let dry = x;
                for (ch, s) in x.iter_mut().enumerate() {
                    let c = &mut self.channels[ch];
                    let mut factor = c.air_prev - *s;
                    if self.flip {
                        c.air_even += factor;
                        c.air_odd -= factor;
                        factor = c.air_even;
                    } else {
                        c.air_odd += factor;
                        c.air_even -= factor;
                        factor = c.air_odd;
                    }
                    c.air_odd = (c.air_odd - ((c.air_odd - c.air_even) / 256.0)) / 1.0001;
                    c.air_even = (c.air_even - ((c.air_even - c.air_odd) / 256.0)) / 1.0001;
                    c.air_prev = *s;
                    *s += factor;
                }
                self.flip = !self.flip;
                let fb = [self.channels[0].feedback, self.channels[1].feedback];
                x[0] += fb[0] * direct;
                x[1] += fb[1] * direct;
                x[0] += fb[1] * cross;
                x[1] += fb[0] * cross;
                if self.dcount < 1 || self.dcount > 48005 {
                    self.dcount = 48005;
                }
                for ch in 0..2 {
                    let c = &mut self.channels[ch];
                    c.delay[self.dcount as usize] = x[ch];
                    let mut index = self.dcount + delays[ch];
                    if index > 48005 {
                        index -= 48005;
                    }
                    x[ch] = c.delay[index as usize];
                }
                self.dcount -= 1;
                for ch in 0..2 {
                    x[ch] = self.channels[ch].pitch_shift(x[ch], speeds[ch]);
                    self.channels[ch].feedback = x[ch];
                    x[ch] = x[ch] * p[4] + dry[ch] * (1.0 - p[4]);
                    self.channels[ch].interpolate(x[ch], end);
                }
                self.cycle = 0;
            }
            *ol = fpd_dither(
                self.channels[0].reference[self.cycle],
                &mut self.channels[0].fpd,
            ) as f32;
            *or = fpd_dither(
                self.channels[1].reference[self.cycle],
                &mut self.channels[1].fpd,
            ) as f32;
        }
    }
}
