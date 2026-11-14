//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/GlitchShifter, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::{float2string, int2string};
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Note",
        label: "semi",
        default: 0.5,
    },
    ParamInfo {
        name: "Trim",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Tighten",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Feedbck",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "GlitchShifter",
    upstream_name: "GlitchShifter",
    description:
        "GlitchShifter is a really gnarly, raw-sounding pitch shifter with a dose of insanity!",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(GlitchShifter::new(sample_rate, seed))
}
struct Channel {
    p: Box<[i32]>,
    offset: [i32; 258],
    past: [i32; 258],
    previous: [i32; 258],
    third: [i32; 258],
    fourth: [i32; 258],
    temp: i32,
    last: i32,
    third_temp: i32,
    fourth_temp: i32,
    since: i32,
    crosses: i32,
    zeroes: i32,
    position: f64,
    splicing: bool,
    air_prev: f64,
    air_even: f64,
    air_odd: f64,
}
impl Channel {
    fn new() -> Self {
        Self {
            p: vec![0; 131076].into_boxed_slice(),
            offset: [0; 258],
            past: [0; 258],
            previous: [0; 258],
            third: [0; 258],
            fourth: [0; 258],
            temp: 0,
            last: 0,
            third_temp: 0,
            fourth_temp: 0,
            since: 0,
            crosses: 0,
            zeroes: 0,
            position: 0.0,
            splicing: false,
            air_prev: 0.0,
            air_even: 0.0,
            air_odd: 0.0,
        }
    }
    fn reset(&mut self) {
        self.p.fill(0);
        self.offset.fill(0);
        self.past.fill(0);
        self.previous.fill(0);
        self.third.fill(0);
        self.fourth.fill(0);
        self.temp = 0;
        self.last = 0;
        self.third_temp = 0;
        self.fourth_temp = 0;
        self.since = 0;
        self.crosses = 0;
        self.zeroes = 0;
        self.position = 0.0;
        self.splicing = false;
        self.air_prev = 0.0;
        self.air_even = 0.0;
        self.air_odd = 0.0;
    }
    fn process(
        &mut self,
        mut input: f64,
        flip: bool,
        bounds: (i32, i32),
        speed: f64,
        bias: f64,
        feedback: f64,
    ) -> i32 {
        let (count, width) = bounds;
        let mut air = self.air_prev - input;
        if flip {
            self.air_even += air;
            self.air_odd -= air;
            air = self.air_even;
        } else {
            self.air_odd += air;
            self.air_even -= air;
            air = self.air_odd;
        }
        self.air_odd = (self.air_odd - ((self.air_odd - self.air_even) / 256.0)) / 1.0001;
        self.air_even = (self.air_even - ((self.air_even - self.air_odd) / 256.0)) / 1.0001;
        self.air_prev = input;
        input += air;
        let countone = wrap(count - 1, width);
        let counttwo = wrap(count - 2, width);
        let value = (input * 8388352.0 + self.last as f64 * feedback) as i32;
        self.p[count as usize] = value;
        self.p[(count + width) as usize] = value;
        let prev = self.p[countone as usize];
        if (prev > 0 && value < 0) || (prev < 0 && value > 0) {
            self.crosses += 1;
            self.zeroes += 1;
            if self.crosses > 256 {
                self.crosses = 0;
            }
            if self.zeroes > 256 {
                self.zeroes = 256;
            }
            let index = self.crosses as usize;
            self.offset[index] = count;
            self.past[index] = value;
            self.previous[index] = prev;
            self.third[index] = self.p[counttwo as usize];
        }
        self.position -= speed;
        if self.position > width as f64 {
            self.splice(width, bias, true);
        }
        if self.position < 0.0 {
            self.splice(width, bias, false);
        }
        let count = wrap(count - self.position.floor() as i32, width) as usize;
        let frac = self.position - self.position.floor();
        self.temp = (self.p[count] as f64 * (1.0 - frac)) as i32;
        self.temp = self.temp.wrapping_add(self.p[count + 1]);
        self.temp = self
            .temp
            .wrapping_add((self.p[count + 2] as f64 * frac) as i32);
        self.temp = self.temp.wrapping_sub(
            self.p[count]
                .wrapping_sub(self.p[count + 1])
                .wrapping_sub(self.p[count + 1].wrapping_sub(self.p[count + 2]))
                / 50,
        );
        self.temp /= 2;
        if self.temp.wrapping_abs() > 8388352 {
            self.temp = self
                .last
                .wrapping_add(self.last.wrapping_sub(self.third_temp));
        }
        self.since += 1;
        if self.since < 0 || self.since > width {
            self.since = 0;
        }
        if self.splicing {
            self.temp = self.temp.wrapping_add(
                self.last
                    .wrapping_add(self.last.wrapping_sub(self.third_temp)),
            ) / 2;
            self.splicing = false;
        }
        if (self.last > 0 && self.temp < 0) || (self.last < 0 && self.temp > 0) {
            self.since = 0;
        }
        self.fourth_temp = self.third_temp;
        self.third_temp = self.last;
        self.last = self.temp;
        self.temp
    }
    fn splice(&mut self, width: i32, bias: f64, upper: bool) {
        if self.zeroes > 0 {
            self.position = 0.0;
            let mut diff = 99999999.0;
            let mut best = 0;
            for scan in (0..self.zeroes).rev() {
                let mut index = scan + self.crosses;
                if index > 256 {
                    index -= 256;
                }
                let index = index as usize;
                let how = self
                    .temp
                    .wrapping_sub(self.past[index])
                    .wrapping_add(self.last.wrapping_sub(self.previous[index]))
                    .wrapping_add(self.third_temp.wrapping_sub(self.third[index]))
                    .wrapping_add(self.fourth_temp.wrapping_sub(self.fourth[index]))
                    as f64
                    - scan as f64 * bias;
                if how < diff {
                    diff = how;
                    best = index;
                }
            }
            self.position = (self.offset[best] - self.since) as f64;
        } else if upper {
            self.position -= width as f64;
        } else {
            self.position += width as f64;
        }
        self.crosses = 0;
        self.zeroes = 0;
        self.splicing = true;
    }
}
fn wrap(mut n: i32, width: i32) -> i32 {
    while n < 0 {
        n += width;
    }
    while n > width {
        n -= width;
    }
    n
}
struct GlitchShifter {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    channels: [Channel; 2],
    count: i32,
    last_width: i32,
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl GlitchShifter {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.5, 0.5, 0.0, 0.5],
            sample_rate,
            seed,
            channels: [Channel::new(), Channel::new()],
            count: 0,
            last_width: 16386,
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for GlitchShifter {
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
            0 => int2string((self.params[0] as f64 * 24.999) as i32 - 12),
            1 => float2string((self.params[1] as f64 * 2.0 - 1.0) as f32),
            2..=4 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        for c in &mut self.channels {
            c.reset();
        }
        self.count = 0;
        self.last_width = 16386;
        self.flip = false;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
        let _ = self.sample_rate;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let note = (a as f64 * 24.999) as i32 - 12;
        let trim = b as f64 * 2.0 - 1.0;
        let mut speed = (1.0 / 12.0) * note as f64 + trim;
        if speed < 0.0 {
            speed *= 0.5;
        }
        let width = (65536.0 - (1.0 - (1.0 - c as f64).powf(2.0)) * 65530.0) as i32;
        let bias = (c as f64).powf(3.0);
        let feedback = d as f64 / 1.5;
        let wet = e as f64;
        if self.last_width != width {
            for ch in &mut self.channels {
                ch.crosses = 0;
                ch.zeroes = 0;
            }
            self.last_width = width;
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            if input[0].abs() < 1.18e-23 {
                input[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if input[1].abs() < 1.18e-23 {
                input[1] = self.fpd_r as f64 * 1.18e-17;
            }
            self.count += 1;
            if self.count < 0 || self.count > width {
                self.count = 0;
            }
            let l = self.channels[0].process(
                input[0],
                self.flip,
                (self.count, width),
                speed,
                bias,
                feedback,
            );
            let r = self.channels[1].process(
                input[1],
                self.flip,
                (self.count, width),
                speed,
                bias,
                feedback,
            );
            self.flip = !self.flip;
            let l = (input[0] * (1.0 - wet) + (l as f64 / 8388352.0) * wet).clamp(-4.0, 4.0);
            let r = (input[1] * (1.0 - wet) + (r as f64 / 8388352.0) * wet).clamp(-4.0, 4.0);
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
