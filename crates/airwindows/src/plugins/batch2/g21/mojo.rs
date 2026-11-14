//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Mojo, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Input",
    label: "dB",
    default: 0.5,
}];
pub(super) const INFO: PluginInfo = PluginInfo {
    name: "Mojo",
    upstream_name: "Mojo",
    description: "Mojo is a biggenator that also works as a loudenator.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Mojo::new(rate, seed))
}
struct Mojo {
    param: f32,
    rate: f32,
    seed: u32,
    fpd: [u32; 2],
}
impl Mojo {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            param: 0.5,
            rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Mojo {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        if i == 0 {
            self.param
        } else {
            0.0
        }
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if let (0, Some(v)) = (i, sanitize_param(v)) {
            self.param = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i == 0 {
            float2string((self.param as f64 * 24.0 - 12.0) as f32)
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let param = self.param;
        *self = Self::new(self.rate, self.seed);
        self.param = param;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let gain = 10.0_f64.powf((self.param as f64 * 24.0 - 12.0) / 20.0);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            for (input, out, fpd) in [(l, out_l), (r, out_r)]
                .into_iter()
                .zip(self.fpd.iter_mut())
                .map(|((i, o), f)| (i, o, f))
            {
                let mut sample = input as f64;
                if sample.abs() < 1.18e-23 {
                    sample = *fpd as f64 * 1.18e-17;
                }
                if gain != 1.0 {
                    sample *= gain;
                }
                let mojo = sample.abs().powf(0.25);
                if mojo > 0.0 {
                    sample =
                        (sample * mojo * std::f64::consts::PI * 0.5).sin() / mojo * 0.987654321;
                }
                *out = fpd_dither(sample, fpd) as f32;
            }
        }
    }
}
