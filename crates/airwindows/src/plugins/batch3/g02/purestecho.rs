//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PurestEcho, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Time",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Tap 1",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Tap 2",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Tap 3",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Tap 4",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PurestEcho",
    upstream_name: "PurestEcho",
    description:
        "PurestEcho is optimized Airwindows echo with exactly four evenly spaced taps on tap.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PurestEcho::new(sample_rate, seed))
}
struct PurestEcho {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    delay: [Box<[f64]>; 2],
    count: i32,
}
impl PurestEcho {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 1.0, 0.0, 0.0, 0.0],
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            delay: [
                vec![0.0; 65535].into_boxed_slice(),
                vec![0.0; 65535].into_boxed_slice(),
            ],
            count: 0,
        }
    }
}
impl AirwindowsEffect for PurestEcho {
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
        self.delay[0].fill(0.0);
        self.delay[1].fill(0.0);
        self.count = 0;
        let mut rng = AwRand::new(self.seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let loop_limit = (65535.0_f64 * 0.499) as i32;
        let time = (self.params[0] as f64).powf(2.0) * 0.999;
        let taps = [
            self.params[1] as f64,
            self.params[2] as f64,
            self.params[3] as f64,
            self.params[4] as f64,
        ];
        let gain = 1.0 / (1.0 + taps[0] + taps[1] + taps[2] + taps[3]);
        let trim = gain * 0.5;
        let mut pos = [0usize; 4];
        let mut before = [0.0; 4];
        let mut after = [0.0; 4];
        for i in 0..4 {
            let offset = loop_limit as f64 * time * ([0.25, 0.5, 0.75, 1.0][i]);
            pos[i] = offset as usize;
            let fraction = offset - pos[i] as f64;
            before[i] = (1.0 - fraction) * taps[i];
            after[i] = fraction * taps[i];
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            if self.count < 0 || self.count > loop_limit {
                self.count = loop_limit;
            }
            let count = self.count as usize;
            let mut input = [l as f64, r as f64];
            for (lane, sample) in input.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[lane] as f64 * 1.18e-17;
                }
                let d = &mut self.delay[lane];
                d[count] = *sample * trim;
                d[count + loop_limit as usize] = d[count];
                let mut sum = d[count + pos[3].saturating_sub(1)] * before[3];
                sum += d[count + pos[3] + 1] * after[3];
                for i in (0..3).rev() {
                    sum += d[count + pos[i].saturating_sub(1)] * before[i];
                    sum += d[count + pos[i] + 1] * after[i];
                }
                for i in (0..4).rev() {
                    sum += d[count + pos[i]] * taps[i];
                }
                *sample = (*sample * gain) + sum;
                *sample = fpd_dither(*sample, &mut self.fpd[lane]);
            }
            self.count -= 1;
            *out_l = input[0] as f32;
            *out_r = input[1] as f32;
        }
    }
}
