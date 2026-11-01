// Copyright (c) 2016 airwindows, Airwindows uses the MIT license
// Upstream: airwindows/plugins/LinuxVST/src/DeBess, b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
static PARAMS: [ParamInfo; 5] = [
    ParamInfo {
        name: "Intense",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Sharp",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Depth",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Filter",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Sense",
        label: "Ess",
        default: 0.0,
    },
];
pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "DeBess",
    upstream_name: "DeBess",
    description: "DeBess is an improved DeEss, with perfect rejection of non-ess audio.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DeBess::new(rate, seed))
}
struct Channel {
    s: [f64; 41],
    m: [f64; 41],
    ratio: [f64; 2],
    iir: [f64; 2],
    fpd: u32,
}
impl Channel {
    fn new(fpd: u32) -> Self {
        Self {
            s: [0.0; 41],
            m: [0.0; 41],
            ratio: [1.0; 2],
            iir: [0.0; 2],
            fpd,
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn process(
        &mut self,
        input: f32,
        sharp: f64,
        intensity: f64,
        speed: f64,
        depth: f64,
        amount: f64,
        flip: bool,
        monitoring: bool,
    ) -> f32 {
        let mut sample = input as f64;
        if sample.abs() < 1.18e-23 {
            sample = self.fpd as f64 * 1.18e-17;
        }
        self.s[0] = sample;
        for x in (1..=sharp as usize).rev() {
            self.s[x] = self.s[x - 1];
        }
        self.m[1] = (self.s[1] - self.s[2]) * ((self.s[1] - self.s[2]) / 1.3);
        for x in (2..=((sharp - 1.0) as usize)).rev() {
            self.m[x] = (self.s[x] - self.s[x + 1]) * ((self.s[x - 1] - self.s[x]) / 1.3);
        }
        let mut sense = (self.m[1] - self.m[2]).abs() * sharp * sharp;
        for x in (1..=((sharp - 1.0) as usize)).rev() {
            let mult = (self.m[x] - self.m[x + 1]).abs() * sharp * sharp;
            if mult < 1.0 {
                sense *= mult;
            }
        }
        sense = 1.0 + (intensity * intensity * sense);
        if sense > intensity {
            sense = intensity;
        }
        let i = if flip { 0 } else { 1 };
        self.iir[i] = (self.iir[i] * (1.0 - amount)) + (sample * amount);
        self.ratio[i] = (self.ratio[i] * (1.0 - speed)) + (sense * speed);
        if self.ratio[i] > depth {
            self.ratio[i] = depth;
        }
        if self.ratio[0] > 1.0 {
            sample = self.iir[i] + ((sample - self.iir[i]) / self.ratio[i]);
        }
        if monitoring {
            sample = input as f64 - sample;
        }
        fpd_dither(sample, &mut self.fpd) as f32
    }
}
struct DeBess {
    params: [f32; 5],
    rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    flip: bool,
}
impl DeBess {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.5, 0.5, 0.5, 0.0],
            rate,
            seed,
            left: Channel::new(fpd_from_seed(seed, 0)),
            right: Channel::new(fpd_from_seed(seed, 1)),
            flip: false,
        }
    }
}
impl AirwindowsEffect for DeBess {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(index), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, index: usize) -> String {
        self.params
            .get(index)
            .map_or_else(String::new, |&p| float2string(p))
    }
    fn param_label(&self, index: usize) -> &'static str {
        if index == 4 {
            if self.params[4] > 0.499999 {
                "Vox"
            } else {
                "Ess"
            }
        } else {
            ""
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let overall = (1.0 / 44100.0) * self.rate as f64;
        let intensity = (self.params[0] as f64).powf(5.0) * (8192.0 / overall);
        let sharp = (self.params[1] as f64 * 40.0).max(2.0);
        let speed = 0.1 / sharp;
        let depth = 1.0 / (self.params[2] as f64 + 0.0001);
        let amount = self.params[3] as f64;
        let monitoring = self.params[4] > 0.49999;
        for (((&l, &r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            *ol = self.left.process(
                l, sharp, intensity, speed, depth, amount, self.flip, monitoring,
            );
            *or = self.right.process(
                r, sharp, intensity, speed, depth, amount, self.flip, monitoring,
            );
            self.flip = !self.flip;
        }
    }
}
