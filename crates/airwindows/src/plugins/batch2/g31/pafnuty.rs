//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Pafnuty, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Second",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Third",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Fourth",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Fifth",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Sixth",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Seventh",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Eighth",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Ninth",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Tenth",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Elevnth",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Twelvth",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Thirtnh",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Inv/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Pafnuty",
    upstream_name: "Pafnuty",
    description: "Pafnuty is a Chebyshev filter, that adds harmonics.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Pafnuty::new(rate, seed))
}
struct Pafnuty {
    params: [f32; 13],
    rate: f32,
    seed: u32,
    fpd: [u32; 2],
}
impl Pafnuty {
    fn new(rate: f32, seed: u32) -> Self {
        let mut params = [0.5; 13];
        params[12] = 1.0;
        Self {
            params,
            rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn transform(x: f64, weights: &[f64; 12], amount: f64) -> f64 {
    let mut p = [0.0; 14];
    p[1] = x;
    for i in 2..14 {
        p[i] = p[i - 1] * x;
    }
    let mut effect = 0.0;
    if weights[0] != 0.0 {
        effect += (2.0 * p[2]) * weights[0];
    }
    if weights[1] != 0.0 {
        effect += (4.0 * p[3] - 3.0 * p[1]) * weights[1];
    }
    if weights[2] != 0.0 {
        effect += (8.0 * p[4] - 8.0 * p[2]) * weights[2];
    }
    if weights[3] != 0.0 {
        effect += (16.0 * p[5] - 20.0 * p[3] + 5.0 * p[1]) * weights[3];
    }
    if weights[4] != 0.0 {
        effect += (32.0 * p[6] - 48.0 * p[4] + 18.0 * p[2]) * weights[4];
    }
    if weights[5] != 0.0 {
        effect += (64.0 * p[7] - 112.0 * p[5] + 56.0 * p[3] - 7.0 * p[1]) * weights[5];
    }
    if weights[6] != 0.0 {
        effect += (128.0 * p[8] - 256.0 * p[6] + 160.0 * p[4] - 32.0 * p[2]) * weights[6];
    }
    if weights[7] != 0.0 {
        effect +=
            (256.0 * p[9] - 576.0 * p[7] + 432.0 * p[5] - 120.0 * p[3] + 9.0 * p[1]) * weights[7];
    }
    if weights[8] != 0.0 {
        effect += (512.0 * p[10] - 1280.0 * p[8] + 1120.0 * p[6] - 400.0 * p[4] + 50.0 * p[2])
            * weights[8];
    }
    if weights[9] != 0.0 {
        effect += (1024.0 * p[11] - 2816.0 * p[9] + 2816.0 * p[7] - 1232.0 * p[5] + 220.0 * p[3]
            - 11.0 * p[1])
            * weights[9];
    }
    if weights[10] != 0.0 {
        effect += (2048.0 * p[12] - 6144.0 * p[10] + 6912.0 * p[8] - 3584.0 * p[6] + 840.0 * p[4]
            - 72.0 * p[2])
            * weights[10];
    }
    if weights[11] != 0.0 {
        effect += (4096.0 * p[13] - 13312.0 * p[11] + 16640.0 * p[9] - 9984.0 * p[7]
            + 2912.0 * p[5]
            - 364.0 * p[3]
            + 13.0 * p[1])
            * weights[11];
    }
    x + effect * amount
}
impl AirwindowsEffect for Pafnuty {
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
        self.params.get(i).map_or_else(String::new, |p| {
            float2string((*p as f64 * 2.0 - 1.0) as f32)
        })
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let scales = [
            1.0, -0.60, -0.60, 0.45, 0.45, -0.38, -0.38, 0.35, 0.35, -0.32, -0.32, 0.30,
        ];
        let mut weights = [0.0; 12];
        for i in 0..12 {
            let x = (self.params[i] as f64 * 2.0 - 1.0) * scales[i];
            weights[i] = x * x.abs();
        }
        let amount = self.params[12] as f64 * 2.0 - 1.0;
        let amount = amount * amount.abs();
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            for (i, x) in input.iter_mut().enumerate() {
                if x.abs() < 1.18e-23 {
                    *x = self.fpd[i] as f64 * 1.18e-17;
                }
                *x = fpd_dither(transform(*x, &weights, amount), &mut self.fpd[i]);
            }
            *outl = input[0] as f32;
            *outr = input[1] as f32;
        }
    }
}
