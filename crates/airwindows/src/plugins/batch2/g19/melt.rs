//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Melt, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Depth",
        label: " ",
        default: 0.25,
    },
    ParamInfo {
        name: "Range",
        label: " ",
        default: 0.75,
    },
    ParamInfo {
        name: "Output",
        label: " ",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Melt",
    upstream_name: "Melt",
    description: "Melt is a wobbly chorusy weird diffuse effect.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Melt::new(sample_rate, seed))
}
struct Melt {
    params: [f32; 4],
    seed: u32,
    delay: [Box<[f32]>; 2],
    combine: [f32; 2],
    scale: [f32; 2],
    position: [i32; 32],
    step: [i32; 32],
    step_count: i32,
    slow_count: i32,
    count: i32,
    fpd: [u32; 2],
}
impl Melt {
    fn new(_sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.25, 0.75, 1.0, 1.0],
            seed,
            delay: [
                vec![0.0; 32002].into_boxed_slice(),
                vec![0.0; 32002].into_boxed_slice(),
            ],
            combine: [0.0; 2],
            scale: [0.999; 2],
            position: [1; 32],
            step: [1; 32],
            step_count: 0,
            slow_count: 0,
            count: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Melt {
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
        for d in &mut self.delay {
            d.fill(0.0);
        }
        self.combine = [0.0; 2];
        self.scale = [0.999; 2];
        self.position = [1; 32];
        self.step = [1; 32];
        self.step_count = 0;
        self.slow_count = 0;
        self.count = 0;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let rate = 1.0 / ((a as f64).powf(2.0) + 0.001);
        let depth_b = b as f64 * 139.5 + 2.0;
        let depth_a = depth_b * (1.0 - a as f64);
        let output = c as f64 * 0.05;
        let wet = d as f64;
        let dry = 1.0 - wet;
        const FACTORS: [i32; 31] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 117,
        ];
        let min = FACTORS.map(|v| (v as f64 * depth_a).floor() as i32);
        let max = FACTORS.map(|v| (v as f64 * depth_b).floor() as i32);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut input = [l as f64, r as f64];
            for (v, fpd) in input.iter_mut().zip(self.fpd) {
                if v.abs() < 1.18e-23 {
                    *v = fpd as f64 * 1.18e-17;
                }
            }
            let original = input;
            if self.count < 0 || self.count > 16000 {
                self.count = 16000;
            }
            let count = self.count as usize;
            for (delay, v) in self.delay.iter_mut().zip(input) {
                delay[count] = v as f32;
                delay[count + 16000] = v as f32;
            }
            if self.slow_count as f64 > rate || self.slow_count < 0 {
                self.slow_count = 0;
                self.step_count += 1;
                if self.step_count > 29 || self.step_count < 0 {
                    self.step_count = 0;
                }
                let n = self.step_count as usize;
                self.position[n] += self.step[n];
                if self.position[n] < min[n] {
                    self.position[n] = min[n];
                    self.step[n] = 1;
                }
                if self.position[n] > max[n] {
                    self.position[n] = max[n];
                    self.step[n] = -1;
                }
            }
            for (ch, sample) in input.iter_mut().enumerate() {
                self.scale[ch] = (self.scale[ch] as f64 * 0.9999) as f32;
                self.scale[ch] = (self.scale[ch] as f64
                    + (100.0 - self.combine[ch].abs() as f64) * 0.000001)
                    as f32;
                for n in (0..15).rev() {
                    self.combine[ch] *= self.scale[ch];
                    self.combine[ch] -= self.delay[ch][count + self.position[n * 2 + 1] as usize];
                    self.combine[ch] += self.delay[ch][count + self.position[n * 2] as usize];
                }
                *sample = self.combine[ch] as f64;
                if output < 1.0 {
                    *sample *= output;
                }
                if wet < 1.0 {
                    *sample = original[ch] * dry + *sample * wet;
                }
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            self.count -= 1;
            self.slow_count += 1;
            *out_l = input[0] as f32;
            *out_r = input[1] as f32;
        }
    }
}
