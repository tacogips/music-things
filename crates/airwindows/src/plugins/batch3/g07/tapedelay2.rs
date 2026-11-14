//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/TapeDelay2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Time",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Regen",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Reso",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Flutter",
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
    name: "TapeDelay2",
    upstream_name: "TapeDelay2",
    description: "TapeDelay2 is a new implementation of a flexible tape echo.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(TapeDelay2::new(sample_rate, seed))
}
struct Channel {
    delay_line: Box<[f64]>,
    prev: f64,
    delay: f64,
    sweep: f64,
    regen: [f64; 9],
    out: [f64; 9],
    refs: [f64; 10],
    fpd: u32,
}
impl Channel {
    fn new(seed: u32, lane: u32) -> Self {
        Self {
            delay_line: vec![0.0; 88211].into_boxed_slice(),
            prev: 0.0,
            delay: 0.0,
            sweep: 0.0,
            regen: [0.0; 9],
            out: [0.0; 9],
            refs: [0.0; 10],
            fpd: fpd_from_seed(seed, lane),
        }
    }
    fn reset(&mut self, seed: u32, lane: u32) {
        self.delay_line.fill(0.0);
        self.prev = 0.0;
        self.delay = 0.0;
        self.sweep = 0.0;
        self.regen.fill(0.0);
        self.out.fill(0.0);
        self.refs.fill(0.0);
        self.fpd = fpd_from_seed(seed, lane);
    }
    fn configure(filter: &mut [f64; 9], freq: f64, res: f64) {
        filter[0] = freq;
        filter[1] = res;
        let k = (std::f64::consts::PI * filter[0]).tan();
        let norm = 1.0 / (1.0 + k / filter[1] + k * k);
        filter[2] = k / filter[1] * norm;
        filter[4] = -filter[2];
        filter[5] = 2.0 * (k * k - 1.0) * norm;
        filter[6] = (1.0 - k / filter[1] + k * k) * norm;
    }
    fn filter(input: f64, f: &mut [f64; 9]) -> f64 {
        let t = input * f[2] + f[7];
        f[7] = -(t * f[5]) + f[8];
        f[8] = input * f[4] - t * f[6];
        t
    }
    #[allow(clippy::approx_constant)]
    fn tick(&mut self, input: f64, base: f64, vib: f64, feedback: f64) -> f64 {
        let speed = base + vib * (self.sweep.sin() + 1.0);
        self.sweep += 0.05 * input * input;
        if self.sweep > 6.283185307179586 {
            self.sweep -= 6.283185307179586;
        }
        let mut pos = self.delay.floor() as i32;
        let sample = input + self.delay_line[pos as usize] * feedback;
        let sample = Self::filter(sample, &mut self.regen);
        self.delay -= speed;
        if self.delay < 0.0 {
            self.delay += 88200.0;
        }
        let increment = (sample - self.prev) / speed;
        self.delay_line[pos as usize] = self.prev;
        while pos != self.delay.floor() as i32 {
            self.delay_line[pos as usize] = self.prev;
            self.prev += increment;
            pos -= 1;
            if pos < 0 {
                pos += 88200;
            }
        }
        self.prev = sample;
        let input = self.delay_line[self.delay.floor() as usize];
        Self::filter(input, &mut self.out)
    }
    fn interpolate(&mut self, input: f64, end: usize) {
        match end {
            4 => {
                self.refs[0] = self.refs[4];
                self.refs[2] = (self.refs[0] + input) / 2.0;
                self.refs[1] = (self.refs[0] + self.refs[2]) / 2.0;
                self.refs[3] = (self.refs[2] + input) / 2.0;
                self.refs[4] = input;
            }
            3 => {
                self.refs[0] = self.refs[3];
                self.refs[2] = (self.refs[0] + self.refs[0] + input) / 3.0;
                self.refs[1] = (self.refs[0] + input + input) / 3.0;
                self.refs[3] = input;
            }
            2 => {
                self.refs[0] = self.refs[2];
                self.refs[1] = (self.refs[0] + input) / 2.0;
                self.refs[2] = input;
            }
            _ => self.refs[0] = input,
        }
    }
    fn smooth(&mut self, mut x: f64, end: usize) -> f64 {
        for i in (2..=end).rev() {
            self.refs[8] = x;
            x = (x + self.refs[i + 3]) * 0.5;
            self.refs[i + 3] = self.refs[8];
        }
        x
    }
}
struct TapeDelay2 {
    params: [f32; 6],
    sample_rate: f32,
    seed: u32,
    l: Channel,
    r: Channel,
    cycle: usize,
}
impl TapeDelay2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.0, 0.5, 0.0, 0.0, 1.0],
            sample_rate,
            seed,
            l: Channel::new(seed, 0),
            r: Channel::new(seed, 1),
            cycle: 0,
        }
    }
}
impl AirwindowsEffect for TapeDelay2 {
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
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        self.l.reset(self.seed, 0);
        self.r.reset(self.seed, 1);
        self.cycle = 0;
    }
    #[allow(clippy::excessive_precision)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d, e, f] = self.params;
        let mut overall = 1.0;
        overall /= 44100.0;
        overall *= self.sample_rate as f64;
        let end = (overall as usize).clamp(1, 4);
        if self.cycle > end - 1 {
            self.cycle = end - 1;
        }
        let base = (a as f64).powf(4.0) * 25.0 + 1.0;
        let feedback = (b as f64).powf(2.0);
        let freq = (c as f64).powf(3.0) * 0.4 + 0.0001;
        let res = (d as f64).powf(2.0) + 0.01;
        for ch in [&mut self.l, &mut self.r] {
            Channel::configure(&mut ch.regen, freq, res);
            Channel::configure(&mut ch.out, freq, res * 1.618033988749894848204586);
        }
        let vib = (e as f64).powf(5.0) * base * (freq * 0.09 + 0.025);
        let wet = (f as f64 * 2.0).clamp(0.0, 1.0);
        let dry = (2.0 - f as f64 * 2.0).clamp(0.0, 1.0);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.l.fpd as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.r.fpd as f64 * 1.18e-17;
            }
            let mut dl = l;
            let mut dr = r;
            self.cycle += 1;
            if self.cycle == end {
                let x = self.l.tick(l, base, vib, feedback);
                let y = self.r.tick(r, base, vib, feedback);
                self.l.interpolate(x, end);
                self.r.interpolate(y, end);
                self.cycle = 0;
            }
            l = self.l.refs[self.cycle];
            r = self.r.refs[self.cycle];
            l = self.l.smooth(l, end);
            r = self.r.smooth(r, end);
            if wet < 1.0 {
                l *= wet;
                r *= wet;
            }
            if dry < 1.0 {
                dl *= dry;
                dr *= dry;
            }
            l += dl;
            r += dr;
            *out_l = fpd_dither(l, &mut self.l.fpd) as f32;
            *out_r = fpd_dither(r, &mut self.r.fpd) as f32;
        }
    }
}
