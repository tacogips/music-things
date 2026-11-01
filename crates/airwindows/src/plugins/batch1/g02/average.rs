//! Average, ported from airwindows/plugins/LinuxVST/src/Average at
//! b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Parity: Exact; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "Average",
    upstream_name: "Average",
    description: "Average is a distinctive sort of lowpass filter.",
    params: &[
        ParamInfo {
            name: "Average",
            label: "taps",
            default: 0.0,
        },
        ParamInfo {
            name: "Dry/Wet",
            label: " ",
            default: 1.0,
        },
    ],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Average::new(sample_rate, seed))
}
struct Average {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    left: [f64; 11],
    right: [f64; 11],
    fpd_l: u32,
    fpd_r: u32,
}
impl Average {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 1.0],
            sample_rate,
            seed,
            left: [0.0; 11],
            right: [0.0; 11],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Average {
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
        match index {
            0 => float2string((self.params[0] as f64 * 9.0 + 1.0) as f32),
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let scale = self.params[0] as f64 * 9.0 + 1.0;
        let wet = self.params[1] as f64;
        let mut gain = scale;
        let mut f = [0.0; 10];
        for x in &mut f {
            if gain > 1.0 {
                *x = 1.0;
                gain -= 1.0;
            } else {
                *x = gain;
                gain = 0.0;
            }
            *x /= scale;
        }
        for (((&l, &r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
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
            self.left.copy_within(0..9, 1);
            self.right.copy_within(0..9, 1);
            self.left[0] = l;
            self.right[0] = r;
            let mut acc_l = l * f[0];
            let mut acc_r = r * f[0];
            for (i, &weight) in f.iter().enumerate().skip(1) {
                acc_l += self.left[i] * weight;
                acc_r += self.right[i] * weight;
            }
            let correction_l = l - acc_l;
            l -= correction_l;
            let correction_r = r - acc_r;
            r -= correction_r;
            if wet < 1.0 {
                l = l * wet + dry_l * (1.0 - wet);
                r = r * wet + dry_r * (1.0 - wet);
            }
            *ol = fpd_dither(l, &mut self.fpd_l) as f32;
            *or = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
