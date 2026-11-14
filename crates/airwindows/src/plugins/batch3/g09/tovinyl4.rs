//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ToVinyl4, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Mid HiP",
        label: "hz",
        default: 0.203419,
    },
    ParamInfo {
        name: "SideHiP",
        label: "hz",
        default: 0.3424051,
    },
    ParamInfo {
        name: "H Limit",
        label: "",
        default: 0.32,
    },
    ParamInfo {
        name: "Gv Wear",
        label: "",
        default: 0.064,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "ToVinyl4",
    upstream_name: "ToVinyl4",
    description: "ToVinyl4 is a vinyl-mastering simulator bringing several vinyl-type colors.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ToVinyl4::new(sample_rate, seed))
}
#[derive(Default)]
struct Channel {
    s: [f64; 3],
    o: [f64; 3],
    last: f64,
}
impl Channel {
    #[allow(clippy::assign_op_pattern)]
    fn deess(&mut self, sample: f64, intensity: f64) -> (f64, f64) {
        self.s[2] = self.s[1];
        self.s[1] = self.s[0];
        self.s[0] = sample;
        let smooth = (self.s[2] + self.s[1] + self.s[0]) / 3.0;
        let m1 = (self.s[0] - self.s[1]) * ((self.s[0] - self.s[1]) / 1.3);
        let m2 = (self.s[1] - self.s[2]) * ((self.s[0] - self.s[1]) / 1.3);
        let mut sense = (m1 - m2).abs();
        sense = intensity * intensity * sense;
        self.o[2] = self.o[1];
        self.o[1] = self.o[0];
        self.o[0] = sense;
        if self.o[1] > sense {
            sense = self.o[1];
        }
        if self.o[2] > sense {
            sense = self.o[2];
        }
        if sense > 1.0 {
            sense = 1.0;
        }
        let mut sample = sample;
        sample *= 1.0 - sense;
        sample += smooth * sense;
        (sample, sense)
    }
    fn antialias(&mut self, sample: f64, sense: f64) -> f64 {
        let result = (self.last * sense) + (sample * (1.0 - sense));
        self.last = sample;
        result
    }
}
#[derive(Default)]
struct Groove {
    a: [f64; 11],
    b: [f64; 11],
    prev_a: f64,
    prev_b: f64,
    filters: [f64; 26],
}
fn weights(mut gain: f64) -> [f64; 10] {
    let scale = gain.max(1.0);
    let mut f = [0.0; 10];
    for item in &mut f {
        if gain > 1.0 {
            *item = 1.0;
            gain -= 1.0;
        } else {
            *item = gain;
            gain = 0.0;
        }
    }
    for item in &mut f {
        *item /= scale;
    }
    f
}
fn wear_step(input: f64, history: &mut [f64; 11], previous: &mut f64, weight: &[f64; 10]) -> f64 {
    history.copy_within(0..9, 1);
    history[0] = input - *previous;
    let mut acc = history[0];
    acc *= weight[0];
    for i in 1..10 {
        acc += history[i] * weight[i];
    }
    let correction = (input - *previous) - acc;
    *previous = input;
    correction
}
impl Groove {
    fn process(&mut self, input: f64, initial_amount: f64, weight: &[f64; 10]) -> f64 {
        let mut temp = input;
        let mut amount = initial_amount;
        for state in &mut self.filters {
            amount *= 0.992;
            *state = (*state * (1.0 - amount)) + (temp * amount);
            temp -= *state;
        }
        let mut correction = input - temp;
        let mut sample = input;
        sample -= correction;
        let first = wear_step(sample, &mut self.a, &mut self.prev_a, weight);
        correction += first;
        sample -= first;
        let second = wear_step(sample, &mut self.b, &mut self.prev_b, weight);
        correction += second;
        input - correction
    }
}
struct ToVinyl4 {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    mid: Groove,
    side: Groove,
    fpd_l: u32,
    fpd_r: u32,
}
impl ToVinyl4 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.203419, 0.3424051, 0.32, 0.064],
            sample_rate,
            seed,
            left: Channel::default(),
            right: Channel::default(),
            mid: Groove::default(),
            side: Groove::default(),
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for ToVinyl4 {
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
            0 | 1 => float2string(((self.params[i] * self.params[i]) as f64 * 290.0 + 10.0) as f32),
            2 | 3 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let mid_amount = (((a * a) as f64 * 290.0) + 10.0) / 50000.0 / overallscale;
        let side_amount = (((b * b) as f64 * 290.0) + 10.0) / 50000.0 / overallscale;
        let intensity = (c as f64).powf(3.0) * (32.0 / overallscale);
        let mw = weights(d as f64 * 9.0 + 1.0);
        let sw = weights(d as f64 * 4.5 + 1.0);
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let (l, mut sense_l) = self.left.deess(l, intensity);
            let (r, mut sense_r) = self.right.deess(r, intensity);
            let mid = self.mid.process(l + r, mid_amount, &mw);
            let side = self.side.process(l - r, side_amount, &sw);
            let l = (mid + side) / 2.0;
            let r = (mid - side) / 2.0;
            sense_l /= 2.0;
            sense_r /= 2.0;
            let l = self.left.antialias(l, sense_l);
            let r = self.right.antialias(r, sense_r);
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
