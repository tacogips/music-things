// Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
// Upstream: plugins/LinuxVST/src/Console8ChannelOut/ at b110fac3991fff7c4655cc2d25d6a3e221f9799c
// Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Fader",
    label: "",
    default: 0.5,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Console8ChannelOut",
    upstream_name: "Console8ChannelOut",
    description:
        "Console8ChannelOut moves to a channel/submix/buss topology and adds analog simulation.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Console8ChannelOut::new(sample_rate, seed))
}
struct Console8ChannelOut {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    trim_a: f64,
    trim_b: f64,
    fix: [f64; 11],
    fpd_l: u32,
    fpd_r: u32,
}
impl Console8ChannelOut {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5],
            sample_rate,
            seed,
            trim_a: 0.5,
            trim_b: 0.5,
            fix: [0.0; 11],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Console8ChannelOut {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(index), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, index: usize) -> String {
        if index == 0 {
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
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let n = il.len().min(ir.len()).min(ol.len()).min(orr.len());
        let freq = 24000.0 / self.sample_rate as f64;
        let k = (std::f64::consts::PI * freq).tan();
        let norm = 1.0 / (1.0 + k / 3.51333709 + k * k);
        let a0 = k * k * norm;
        let a1 = 2.0 * a0;
        let a2 = a0;
        let b1 = 2.0 * (k * k - 1.0) * norm;
        let b2 = (1.0 - k / 3.51333709 + k * k) * norm;
        self.trim_a = self.trim_b;
        self.trim_b = self.params[0] as f64 * 2.0;
        for i in 0..n {
            let mut l = il[i] as f64;
            let mut r = ir[i] as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let position = (n - 1 - i) as f64 / n as f64;
            let trim = self.trim_a * position + self.trim_b * (1.0 - position);
            l = (l * trim).clamp(-1.57079633, 1.57079633).sin();
            r = (r * trim).clamp(-1.57079633, 1.57079633).sin();
            if self.sample_rate > 49000.0 {
                let v = l * a0 + self.fix[7];
                self.fix[7] = l * a1 - v * b1 + self.fix[8];
                self.fix[8] = l * a2 - v * b2;
                l = v;
                let v = r * a0 + self.fix[9];
                self.fix[9] = r * a1 - v * b1 + self.fix[10];
                self.fix[10] = r * a2 - v * b2;
                r = v;
            }
            l = (l * trim).clamp(-1.57079633, 1.57079633).sin();
            r = (r * trim).clamp(-1.57079633, 1.57079633).sin();
            ol[i] = fpd_dither(l, &mut self.fpd_l) as f32;
            orr[i] = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
