//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/BrassRider/BrassRider.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: [ParamInfo; 2] = [
    ParamInfo {
        name: "Thresh",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "BrassRider",
    upstream_name: "BrassRider",
    description: "BrassRider is One Weird Trick for drum overheads in metal!",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(BrassRider::new(rate, seed))
}
#[derive(Default)]
struct Filter {
    high: f64,
    high2: f64,
    slew: f64,
    slew2: f64,
    last: f64,
}
impl Filter {
    fn process(&mut self, mut sample: f64) -> f64 {
        self.high *= 0.5;
        self.high += sample * 0.5;
        sample -= self.high;
        self.high2 *= 0.5;
        self.high2 += sample * 0.5;
        sample -= self.high2;
        let mut slew = (sample - self.last).abs();
        self.last = sample;
        slew /= (sample * self.last).abs() + 0.2;
        self.slew *= 0.5;
        self.slew += slew * 0.5;
        slew = (slew - self.slew).abs();
        self.slew2 *= 0.5;
        self.slew2 += slew * 0.5;
        slew = (slew - self.slew2).abs();
        if slew > 3.1415 {
            slew = 0.0;
        }
        slew.sin()
    }
}
struct BrassRider {
    params: [f32; 2],
    rate: f32,
    seed: u32,
    d: Box<[f64]>,
    e: Box<[f64]>,
    control: f64,
    clamp: f64,
    count: i32,
    left: Filter,
    right: Filter,
    fpd_l: u32,
    fpd_r: u32,
}
impl BrassRider {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0; 2],
            rate,
            seed,
            d: vec![0.0; 80002].into_boxed_slice(),
            e: vec![0.0; 80002].into_boxed_slice(),
            control: 0.0,
            clamp: 0.0,
            count: 0,
            left: Filter::default(),
            right: Filter::default(),
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
    fn update(&mut self, value: f64, wet: f64) {
        if self.count < 0 || self.count > 40000 {
            self.count = 40000;
        }
        let index = self.count as usize;
        self.d[index] = value;
        self.d[index + 40000] = value;
        self.control += self.d[index] / 13501.0;
        self.control -= self.d[index + 13500] / 13500.0;
        let ramp = (self.control * self.control) * 16.0;
        self.e[index] = ramp;
        self.e[index + 40000] = ramp;
        self.clamp += self.e[index] / 16701.0;
        self.clamp -= self.e[index + 16700] / 16700.0;
        if self.clamp > wet * 8.0 {
            self.clamp = wet * 8.0;
        }
        self.count -= 1;
    }
}
impl AirwindowsEffect for BrassRider {
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
            .map_or_else(String::new, |p| float2string(*p))
    }
    fn reset(&mut self) {
        self.d.fill(0.0);
        self.e.fill(0.0);
        self.control = 0.0;
        self.clamp = 0.0;
        self.count = 0;
        self.left = Filter::default();
        self.right = Filter::default();
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, l: &[f32], r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let limit = (self.params[0] * 16.0) as f64;
        let wet = self.params[1] as f64;
        let n = l.len().min(r.len()).min(out_l.len()).min(out_r.len());
        for i in 0..n {
            let mut dry_l = l[i] as f64;
            let mut dry_r = r[i] as f64;
            if dry_l.abs() < 1.18e-23 {
                dry_l = self.fpd_l as f64 * 1.18e-17;
            }
            if dry_r.abs() < 1.18e-23 {
                dry_r = self.fpd_r as f64 * 1.18e-17;
            }
            let filtered_l = self.left.process(dry_l * limit);
            self.update(filtered_l, wet);
            let filtered_r = self.right.process(dry_r * limit);
            self.update(filtered_r, wet);
            let output_l = dry_l * (1.0 - wet) + dry_l * self.clamp * wet * 16.0;
            let output_r = dry_r * (1.0 - wet) + dry_r * self.clamp * wet * 16.0;
            out_l[i] = fpd_dither(output_l, &mut self.fpd_l) as f32;
            out_r[i] = fpd_dither(output_r, &mut self.fpd_r) as f32;
        }
    }
}
