//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/UltrasonX, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Q",
    label: "",
    default: 0.5,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "UltrasonX",
    upstream_name: "UltrasonX",
    description: "UltrasonX is a method for rolling your own Console-type systems with total control over your ultrasonic filtering.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(UltrasonX::new(sample_rate, seed))
}
struct UltrasonX {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    state: [[f64; 2]; 2],
    fpd_l: u32,
    fpd_r: u32,
}
impl UltrasonX {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5],
            sample_rate,
            seed,
            state: [[0.0; 2]; 2],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for UltrasonX {
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
        if i != 0 {
            return String::new();
        }
        ["Reso A", "Reso B", "Reso C", "Reso D", "Reso E"][(self.params[0] as f64 * 4.999) as usize]
            .into()
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    #[allow(clippy::approx_constant)] // Preserve the upstream filter coefficient.
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let reso = [3.19622661, 1.10134463, 0.70710678, 0.56116312, 0.50623256]
            [(self.params[0] as f64 * 4.999) as usize];
        let k = (std::f64::consts::PI * (21000.0 / self.sample_rate as f64)).tan();
        let norm = 1.0 / (1.0 + k / reso + k * k);
        let a0 = k * k * norm;
        let a1 = 2.0 * a0;
        let a2 = a0;
        let b1 = 2.0 * (k * k - 1.0) * norm;
        let b2 = (1.0 - k / reso + k * k) * norm;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut left = l as f64;
            let mut right = r as f64;
            if left.abs() < 1.18e-23 {
                left = self.fpd_l as f64 * 1.18e-17;
            }
            if right.abs() < 1.18e-23 {
                right = self.fpd_r as f64 * 1.18e-17;
            }
            let temp = left * a0 + self.state[0][0];
            self.state[0][0] = left * a1 - temp * b1 + self.state[0][1];
            self.state[0][1] = left * a2 - temp * b2;
            left = temp;
            let temp = right * a0 + self.state[1][0];
            self.state[1][0] = right * a1 - temp * b1 + self.state[1][1];
            self.state[1][1] = right * a2 - temp * b2;
            right = temp;
            *out_l = fpd_dither(left, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(right, &mut self.fpd_r) as f32;
        }
    }
}
