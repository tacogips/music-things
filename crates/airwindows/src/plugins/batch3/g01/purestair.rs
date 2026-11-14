//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PurestAir, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Boost",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Limit",
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
    name: "PurestAir",
    upstream_name: "PurestAir",
    description: "PurestAir is a brightness experiment.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PurestAir::new(sample_rate, seed))
}
#[derive(Default)]
struct Channel {
    last1: f64,
    last: f64,
    s1: f64,
    s2: f64,
    s3: f64,
    apply: f64,
}
impl Channel {
    fn process(
        &mut self,
        raw: f64,
        input: f64,
        target: f64,
        threshold: f64,
        intensity: f64,
    ) -> f64 {
        let half_dry = (input + self.last1) / 2.0;
        let mut halfway = half_dry;
        self.last1 = input;
        self.s3 = self.s2;
        self.s2 = self.s1;
        self.s1 = input;
        let m1 = (self.s1 - self.s2) * ((self.s1 - self.s2) / 1.3);
        let m2 = (self.s2 - self.s3) * ((self.s1 - self.s2) / 1.3);
        let sense = ((m1 - m2) * ((m1 - m2) / 1.3)).abs() * intensity;
        self.apply += target - sense;
        self.apply *= 0.5;
        if self.apply < -1.0 {
            self.apply = -1.0;
        }
        let clamp = halfway - half_dry;
        if clamp > threshold {
            halfway = self.last + threshold;
        }
        if -clamp > threshold {
            halfway = self.last - threshold;
        }
        self.last = halfway;
        let mut sample = input;
        let clamp = sample - self.last;
        if clamp > threshold {
            sample = self.last + threshold;
        }
        if -clamp > threshold {
            sample = self.last - threshold;
        }
        self.last = sample;
        let diff = raw - sample;
        let half_diff = half_dry - halfway;
        raw + ((diff + half_diff) * self.apply)
    }
}
struct PurestAir {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    fpd_l: u32,
    fpd_r: u32,
}
impl PurestAir {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.0, 1.0],
            sample_rate,
            seed,
            left: Channel::default(),
            right: Channel::default(),
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for PurestAir {
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
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut target = self.params[0] as f64 * 2.0 - 1.0;
        let threshold = (1.0 - target.abs()).powf(3.0);
        if target > 0.0 {
            target *= 3.0;
        }
        let intensity = (self.params[1] as f64).powf(2.0) * 5.0;
        let wet = self.params[2] as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut dry_l = l as f64;
            let mut dry_r = r as f64;
            if dry_l.abs() < 1.18e-23 {
                dry_l = self.fpd_l as f64 * 1.18e-17;
            }
            if dry_r.abs() < 1.18e-23 {
                dry_r = self.fpd_r as f64 * 1.18e-17;
            }
            let mut l = self
                .left
                .process(l as f64, dry_l, target, threshold, intensity);
            let mut r = self
                .right
                .process(r as f64, dry_r, target, threshold, intensity);
            if wet != 1.0 {
                l = l * wet + dry_l * (1.0 - wet);
                r = r * wet + dry_r * (1.0 - wet);
            }
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
