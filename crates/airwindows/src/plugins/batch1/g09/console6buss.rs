// Copyright (c) Airwindows, Airwindows uses the MIT license
// Upstream: airwindows/plugins/LinuxVST/src/Console6Buss, b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Exact; measured max-abs 0.0.
use crate::{
    display::float2string,
    dither::{fpd_dither, fpd_from_seed},
    params::sanitize_param,
    rand::{replay_fpd_seed_loop, AwRand},
    registry::PluginEntry,
};
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: [ParamInfo; 1] = [ParamInfo {
    name: "Input",
    label: "",
    default: 1.0,
}];
static INFO: PluginInfo = PluginInfo {
    name: "Console6Buss",
    upstream_name: "Console6Buss",
    description: "Console6Buss is a new more intense Console that peaks at lower dB.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Console6Buss::new(sample_rate, seed))
}
struct Console6Buss {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
}
impl Console6Buss {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Console6Buss {
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
        if i == 0 {
            float2string(self.params[0])
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let n = l.len().min(r.len()).min(ol.len()).min(orr.len());

        for i in 0..n {
            let gain = self.params[0] as f64;
            for (input, output, fpd) in [
                (l[i], &mut ol[i], &mut self.fpd_l),
                (r[i], &mut orr[i], &mut self.fpd_r),
            ] {
                let mut x = input as f64;
                if x.abs() < 1.18e-23 {
                    x = *fpd as f64 * 1.18e-17;
                }
                if gain != 1.0 {
                    x *= gain;
                }
                if x > 1.0 {
                    x = 1.0;
                } else if x > 0.0 {
                    x = x / (1.0 + (1.0 - x).sqrt());
                }
                if x < -1.0 {
                    x = -1.0;
                } else if x < 0.0 {
                    x = x / ((x + 1.0).sqrt() + 1.0);
                }
                *output = fpd_dither(x, fpd) as f32;
            }
        }
    }
}
