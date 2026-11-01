// Copyright (c) 2016 airwindows, Airwindows uses the MIT license
// Upstream: airwindows/plugins/LinuxVST/src/DeEss, b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Libm; measured max-abs 5.1315766e-8.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
static PARAMS: [ParamInfo; 3] = [
    ParamInfo {
        name: "Intense",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Max DS",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.5,
    },
];
pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "DeEss",
    upstream_name: "DeEss",
    description: "DeEss is the best de-essing solution there is. A go-to utility plugin.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DeEss::new(rate, seed))
}
struct Channel {
    s: [f64; 3],
    ratio: [f64; 2],
    iir: [f64; 2],
    noise: i32,
    fpd: u32,
}
impl Channel {
    fn new(noise: i32, fpd: u32) -> Self {
        Self {
            s: [0.0; 3],
            ratio: [1.0; 2],
            iir: [0.0; 2],
            noise,
            fpd,
        }
    }
    fn process(
        &mut self,
        input: f32,
        intensity: f64,
        maxdess: f64,
        amount: f64,
        flip: bool,
    ) -> f32 {
        let mut x = input as f64;
        self.noise = self.noise % 1700021 + 1;
        let mut res = self.noise.wrapping_mul(self.noise);
        for modulus in [170003, 17011, 1709, 173] {
            res %= modulus;
            res = res.wrapping_mul(res);
        }
        res %= 17;
        let mut add = res as f64;
        add *= 0.00000001;
        add *= 0.00000001;
        x += add;
        if x < 1.2e-38 && -x < 1.2e-38 {
            x -= add;
        }
        self.s[2] = self.s[1];
        self.s[1] = self.s[0];
        self.s[0] = x;
        let m1 = (self.s[0] - self.s[1]) * ((self.s[0] - self.s[1]) / 1.3);
        let m2 = (self.s[1] - self.s[2]) * ((self.s[0] - self.s[1]) / 1.3);
        let mut sense = ((m1 - m2) * ((m1 - m2) / 1.3)).abs();
        let attack = 7.0 + (sense * 1024.0);
        sense = 1.0 + (intensity * intensity * sense);
        if sense > intensity {
            sense = intensity;
        }
        let recovery = 1.0 + (0.01 / sense);
        let offset = 1.0 - x.abs();
        let i = if flip { 0 } else { 1 };
        self.iir[i] = (self.iir[i] * (1.0 - (offset * amount))) + (x * (offset * amount));
        if self.ratio[i] < sense {
            self.ratio[i] = ((self.ratio[i] * attack) + sense) / (attack + 1.0);
        } else {
            self.ratio[i] = 1.0 + ((self.ratio[i] - 1.0) / recovery);
        }
        if self.ratio[i] > maxdess {
            self.ratio[i] = maxdess;
        }
        x = self.iir[i] + ((x - self.iir[i]) / self.ratio[i]);
        fpd_dither(x, &mut self.fpd) as f32
    }
}
struct DeEss {
    params: [f32; 3],
    rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    flip: bool,
}
impl DeEss {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.5, 0.5],
            rate,
            seed,
            left: Channel::new(0, fpd_from_seed(seed, 0)),
            right: Channel::new(850010, fpd_from_seed(seed, 1)),
            flip: false,
        }
    }
}
impl AirwindowsEffect for DeEss {
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
            1 => float2string(((self.params[1] as f64 - 1.0) * 48.0) as f32),
            0 | 2 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let overall = (1.0 / 44100.0) * self.rate as f64;
        let intensity = (self.params[0] as f64).powf(5.0) * (8192.0 / overall);
        let maxdess = 1.0 / 10.0_f64.powf(((self.params[1] as f64 - 1.0) * 48.0) / 20.0);
        let amount = (self.params[2] as f64).powf(2.0) / overall;
        for (((&l, &r), o1), o2) in il.iter().zip(ir).zip(ol).zip(or) {
            *o1 = self.left.process(l, intensity, maxdess, amount, self.flip);
            *o2 = self.right.process(r, intensity, maxdess, amount, self.flip);
            self.flip = !self.flip;
        }
    }
}
