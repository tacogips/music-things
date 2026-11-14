//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Flutter, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Flutter",
    label: "",
    default: 0.0,
}];
const INFO: PluginInfo = PluginInfo {
    name: "Flutter",
    upstream_name: "Flutter",
    description: "Flutter is the flutter from ToTape6, standalone.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Flutter::new(rate, seed))
}
struct Flutter {
    value: f32,
    rate: f32,
    seed: u32,
    buf: [[f64; 1001]; 2],
    count: i32,
    sweep: f64,
    rateof: f64,
    nextmax: f64,
    fpd: [u32; 2],
}
impl Flutter {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            value: 0.0,
            rate,
            seed,
            buf: [[0.0; 1001]; 2],
            count: 0,
            sweep: std::f64::consts::PI,
            rateof: 0.5,
            nextmax: 0.5,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Flutter {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        if i == 0 {
            self.value
        } else {
            0.0
        }
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if i == 0 {
            if let Some(v) = sanitize_param(v) {
                self.value = v;
            }
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i == 0 {
            float2string(self.value)
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let v = self.value;
        *self = Self::new(self.rate, self.seed);
        self.value = v;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let scale = 1.0 / 44100.0 * self.rate as f64;
        let depth = (self.value as f64).powf(2.0) * scale * 70.0;
        let trim = (0.0024 * (self.value as f64).powf(2.0)) / scale;
        for (((&l, &r), left), right) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            for (x, fpd) in input.iter_mut().zip(self.fpd) {
                if x.abs() < 1.18e-23 {
                    *x = fpd as f64 * 1.18e-17;
                }
            }
            if self.count < 0 || self.count > 999 {
                self.count = 999;
            }
            self.buf[0][self.count as usize] = input[0];
            self.buf[1][self.count as usize] = input[1];
            let offset = depth + (depth * self.rateof.powf(2.0) * self.sweep.sin());
            let count = self.count as usize + offset.floor() as usize;
            let fraction = offset - offset.floor();
            for (lane, x) in input.iter_mut().enumerate() {
                *x = self.buf[lane][if count > 999 { count - 1000 } else { count }]
                    * (1.0 - fraction)
                    + self.buf[lane][if count + 1 > 999 {
                        count + 1 - 1000
                    } else {
                        count + 1
                    }] * fraction;
            }
            self.rateof = (self.rateof * (1.0 - trim)) + (self.nextmax * trim);
            self.sweep += self.rateof * trim;
            if self.sweep >= std::f64::consts::PI * 2.0 {
                self.sweep -= std::f64::consts::PI;
                self.nextmax = 0.24 + (self.fpd[0] as f64 / u32::MAX as f64 * 0.74);
            }
            self.count -= 1;
            *left = fpd_dither(input[0], &mut self.fpd[0]) as f32;
            *right = fpd_dither(input[1], &mut self.fpd[1]) as f32;
        }
    }
}
