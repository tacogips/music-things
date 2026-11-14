//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/TripleSpread, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Spread",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Tighten",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "TripleSpread",
    upstream_name: "TripleSpread",
    description:
        "TripleSpread is a stereo tripler with extra wideness and GlitchShifter processing.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(TripleSpread::new(sample_rate, seed))
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
    fn air(&mut self, mut x: f64, flip: bool) -> f64 {
        let mut factor = self.air_prev - x;
        if flip {
            self.air_even += factor;
            self.air_odd -= factor;
            factor = self.air_even;
        } else {
            self.air_odd += factor;
            self.air_even -= factor;
            factor = self.air_odd;
        }
        self.air_odd = (self.air_odd - ((self.air_odd - self.air_even) / 256.0)) / 1.0001;
        self.air_even = (self.air_even - ((self.air_even - self.air_odd) / 256.0)) / 1.0001;
        self.air_prev = x;
        x += factor;
        x
    }
    fn feed(&mut self, x: f64, count: i32, one: i32, two: i32, width: i32) {
        self.p[count as usize] = (x * 8388352.0) as i32;
        self.p[(count + width) as usize] = self.p[count as usize];
        let a = self.p[one as usize];
        let b = self.p[count as usize];
        if (a > 0 && b < 0) || (a < 0 && b > 0) {
            self.crosses += 1;
            self.zeroes += 1;
            if self.crosses > 256 {
                self.crosses = 0;
            }
            if self.zeroes > 256 {
                self.zeroes = 256;
            }
            let i = self.crosses as usize;
            self.offset[i] = count;
            self.past[i] = b;
            self.previous[i] = a;
            self.third[i] = self.p[two as usize];
        }
    }
    fn shift(&mut self, delta: f64, width: i32, bias: f64, gcount: i32) -> f64 {
        self.position += delta;
        if self.position > width as f64 || self.position < 0.0 {
            if self.zeroes > 0 {
                self.position = 0.0;
                let mut diff = 99999999.0;
                let mut best = 0;
                for scan in (0..self.zeroes).rev() {
                    let mut one = scan + self.crosses;
                    if one > 256 {
                        one -= 256;
                    }
                    let i = one as usize;
                    let mut howdiff = ((self.temp - self.past[i])
                        + (self.last - self.previous[i])
                        + (self.third_temp - self.third[i])
                        + (self.fourth_temp - self.fourth[i]))
                        as f64;
                    howdiff -= scan as f64 * bias;
                    if howdiff < diff {
                        diff = howdiff;
                        best = i;
                    }
                }
                self.position = (self.offset[best] - self.since) as f64;
            } else if self.position > width as f64 {
                self.position -= width as f64;
            } else {
                self.position += width as f64;
            }
            self.crosses = 0;
            self.zeroes = 0;
            self.splicing = true;
        }
        let mut count = gcount - self.position.floor() as i32;
        while count < 0 {
            count += width;
        }
        while count > width {
            count -= width;
        }
        let i = count as usize;
        let fraction = self.position - self.position.floor();
        self.temp = 0;
        self.temp += (self.p[i] as f64 * (1.0 - fraction)) as i32;
        self.temp += self.p[i + 1];
        self.temp += (self.p[i + 2] as f64 * fraction) as i32;
        self.temp -= ((self.p[i] - self.p[i + 1]) - (self.p[i + 1] - self.p[i + 2])) / 50;
        self.temp /= 2;
        if self.temp.abs() > 8388352 {
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
        self.temp as f64 / 8388352.0
    }
}
struct TripleSpread {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    count: i32,
    last_width: i32,
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl TripleSpread {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5; 3],
            sample_rate,
            seed,
            left: Channel::new(),
            right: Channel::new(),
            count: 0,
            last_width: 16386,
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for TripleSpread {
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
            1 | 2 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.left.reset();
        self.right.reset();
        self.count = 0;
        self.last_width = 16386;
        self.flip = false;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c] = self.params;
        let speed = ((a as f64 * 2.0) - 1.0) * 0.041666666666667;
        let width = (65536.0 - ((1.0 - ((1.0_f32 - b) as f64).powf(2.0)) * 65530.0)) as i32;
        let bias = (b as f64).powf(3.0);
        let wet = c as f64;
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
            l = self.left.air(l, self.flip);
            r = self.right.air(r, self.flip);
            self.flip = !self.flip;
            if self.last_width != width {
                self.left.crosses = 0;
                self.left.zeroes = 0;
                self.right.crosses = 0;
                self.right.zeroes = 0;
                self.last_width = width;
            }
            self.count += 1;
            if self.count < 0 || self.count > width {
                self.count = 0;
            }
            let count = self.count;
            let mut one = count - 1;
            let mut two = count - 2;
            while one < 0 {
                one += width;
            }
            while two < 0 {
                two += width;
            }
            self.left.feed(l, count, one, two, width);
            self.right.feed(r, count, one, two, width);
            let l = self.left.shift(-speed, width, bias, count);
            let r = self.right.shift(speed, width, bias, count);
            let l = (dry_l * (1.0 - wet) + l * wet).clamp(-4.0, 4.0);
            let r = (dry_r * (1.0 - wet) + r * wet).clamp(-4.0, 4.0);
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
