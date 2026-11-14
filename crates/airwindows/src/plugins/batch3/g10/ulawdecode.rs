//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/uLawDecode, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Gain",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "uLawDecode",
    upstream_name: "uLawDecode",
    description: "uLawDecode is a Console-like encode/decode pair, but much more extreme.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ULawDecode::new(rate, seed))
}
struct ULawDecode {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
}
impl ULawDecode {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 1.0],
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for ULawDecode {
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
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let gain = self.params[0] as f64;
        let wet = self.params[1] as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut result = [0.0_f32; 2];
            for (ch, input) in [l, r].into_iter().enumerate() {
                let mut x = input as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = x;
                if gain != 1.0 {
                    x *= gain;
                }
                x = x.clamp(-1.0, 1.0);
                if x > 0.0 {
                    x = (256.0_f64.powf(x.abs()) - 1.0) / 255.0;
                } else if x < 0.0 {
                    x = -((256.0_f64.powf(x.abs()) - 1.0) / 255.0);
                }
                if wet != 1.0 {
                    x = (x * wet) + (dry * (1.0 - wet));
                }
                result[ch] = fpd_dither(x, &mut self.fpd[ch]) as f32;
            }
            *out_l = result[0];
            *out_r = result[1];
        }
    }
}
