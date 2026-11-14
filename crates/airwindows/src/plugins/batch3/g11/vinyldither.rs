//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/VinylDither, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
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
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "VinylDither",
    upstream_name: "VinylDither",
    description:
        "VinylDither is a high-performance dither that converts digital noise to ‘groove noise’.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(VinylDither::new(sample_rate, seed))
}
struct VinylDither {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    odd: [f64; 2],
    prev: [f64; 2],
    ns: [[f64; 16]; 2],
    fpd: [u32; 2],
}
impl VinylDither {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 0.0],
            sample_rate,
            seed,
            odd: [0.0; 2],
            prev: [0.0; 2],
            ns: [[0.0; 16]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for VinylDither {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
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
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            for (ch, sample) in input.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                *sample *= scale as f64;
                let mut noise = self.fpd[ch] as f64 / u32::MAX as f64 - 0.5;
                for stage in 0..16 {
                    self.ns[ch][stage] += noise;
                    self.ns[ch][stage] /= 2.0;
                    noise -= self.ns[ch][stage];
                    if stage < 15 {
                        xorshift(&mut self.fpd[ch]);
                        noise += self.fpd[ch] as f64 / u32::MAX as f64 - 0.5;
                    }
                }
                noise += *sample;
                if self.odd[ch] > 0.0 {
                    self.odd[ch] -= 0.97;
                }
                if self.odd[ch] < 0.0 {
                    self.odd[ch] += 0.97;
                }
                self.odd[ch] -= self.odd[ch] * self.odd[ch] * self.odd[ch] * 0.475;
                self.odd[ch] += self.prev[ch];
                noise += self.odd[ch] * 0.475;
                self.prev[ch] = noise.floor() - *sample;
                *sample = noise.floor() / outscale as f64;
                xorshift(&mut self.fpd[ch]);
            }
            *out_l = input[0] as f32;
            *out_r = input[1] as f32;
        }
    }
}
