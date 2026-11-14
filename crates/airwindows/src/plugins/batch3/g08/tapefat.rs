//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/TapeFat, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Lean/Ft",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Depth",
        label: "taps",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "TapeFat",
    upstream_name: "TapeFat",
    description: "TapeFat is the tone control from TapeDelay.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(TapeFat::new(sample_rate, seed))
}
struct TapeFat {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    p: [[i32; 258]; 2],
    count: i32,
    fpd_l: u32,
    fpd_r: u32,
}
impl TapeFat {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 0.0],
            sample_rate,
            seed,
            p: [[0; 258]; 2],
            count: 0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for TapeFat {
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
            1 => float2string((self.params[1] as f64 * 29.0 + 3.0).floor() as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        const TAPS: [usize; 32] = [
            1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
            83, 89, 97, 101, 103, 107, 109, 113, 127,
        ];
        let lean = self.params[0] as f64 * 2.0 - 1.0;
        let wet = lean.abs();
        let fatness = (self.params[1] as f64 * 29.0 + 3.0).floor() as usize;
        for (((&l, &r), lo), ro) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut input = [l as f64, r as f64];
            if input[0].abs() < 1.18e-23 {
                input[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if input[1].abs() < 1.18e-23 {
                input[1] = self.fpd_r as f64 * 1.18e-17;
            }
            if self.count < 0 || self.count > 128 {
                self.count = 128;
            }
            let count = self.count as usize;
            for (ch, sample) in input.iter_mut().enumerate() {
                let mut sum = (*sample * 8388608.0) as i32;
                self.p[ch][count] = sum;
                self.p[ch][count + 128] = sum;
                for &tap in TAPS[..fatness].iter().rev() {
                    sum = sum.wrapping_add(self.p[ch][count + tap]);
                }
                let mut total = (sum / fatness as i32 + 1) as f64;
                total /= 8388608.0;
                total *= wet;
                *sample = if lean < 0.0 {
                    *sample - total
                } else {
                    *sample * (1.0 - wet) + total
                };
            }
            self.count -= 1;
            *lo = fpd_dither(input[0], &mut self.fpd_l) as f32;
            *ro = fpd_dither(input[1], &mut self.fpd_r) as f32;
        }
    }
}
