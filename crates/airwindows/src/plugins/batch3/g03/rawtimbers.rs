//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/RawTimbers, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Quant",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "DeRez",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo { name: "RawTimbers", upstream_name: "RawTimbers", description: "RawTimbers is just the quantization from Dither Me Timbers… and the opposite, as a brightener.", params: PARAMS };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(RawTimbers::new(sample_rate, seed))
}
struct RawTimbers {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    last: [f64; 2],
    last2: [f64; 2],
    fpd: [u32; 2],
}
impl RawTimbers {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 0.0],
            sample_rate,
            seed,
            last: [0.0; 2],
            last2: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for RawTimbers {
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
            0 => {
                if self.params[0] as f64 * 1.999 < 1.0 {
                    "CD 16".into()
                } else {
                    "HD 24".into()
                }
            }
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = if (self.params[0] as f64 * 1.999) as i32 == 1 {
            8388608.0f32
        } else {
            32768.0f32
        };
        if self.params[1] > 0.0 {
            scale = (scale as f64 * (1.0 - self.params[1] as f64).powf(6.0)) as f32;
        }
        if scale < 0.0001 {
            scale = 0.0001;
        }
        let outscale = scale.max(8.0);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut input = [l as f64, r as f64];
            let mut output = [0.0f32; 2];
            for i in 0..2 {
                if input[i].abs() < 1.18e-23 {
                    input[i] = self.fpd[i] as f64 * 1.18e-17;
                }
                xorshift(&mut self.fpd[i]);
                input[i] *= scale as f64;
                input[i] += 0.381966011250105;
                let quant = if self.last[i] + self.last[i] >= input[i] + self.last2[i] {
                    self.last[i].floor()
                } else {
                    (self.last[i] + 1.0).floor()
                };
                self.last2[i] = self.last[i];
                self.last[i] = input[i];
                output[i] = (quant / outscale as f64) as f32;
            }
            *out_l = output[0];
            *out_r = output[1];
        }
    }
}
