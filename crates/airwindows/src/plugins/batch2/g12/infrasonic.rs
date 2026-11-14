//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Infrasonic, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Infrasonic",
    upstream_name: "Infrasonic",
    description: "Infrasonic is a very steep subsonic filter, built like Ultrasonic.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Infrasonic::new(sample_rate, seed))
}
struct Infrasonic {
    sample_rate: f32,
    seed: u32,
    filters: [[f64; 15]; 5],
    fpd_l: u32,
    fpd_r: u32,
}
impl Infrasonic {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            sample_rate,
            seed,
            filters: [[0.0; 15]; 5],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Infrasonic {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, _: usize) -> f32 {
        0.0
    }
    fn set_param(&mut self, _: usize, _: f32) {}
    fn param_display(&self, _: usize) -> String {
        String::new()
    }
    fn reset(&mut self) {
        *self = Self::new(self.sample_rate, self.seed);
    }
    #[allow(clippy::approx_constant)] // Preserve the upstream literals.
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        for (b, resonance) in self
            .filters
            .iter_mut()
            .zip([0.50623256, 0.56116312, 0.70710678, 1.10134463, 3.19622661])
        {
            b[0] = 20.0 / self.sample_rate as f64;
            b[1] = resonance;
            let k = (std::f64::consts::PI * b[0]).tan();
            let norm = 1.0 / (1.0 + k / b[1] + k * k);
            b[2] = norm;
            b[3] = -2.0 * b[2];
            b[4] = b[2];
            b[5] = 2.0 * (k * k - 1.0) * norm;
            b[6] = (1.0 - k / b[1] + k * k) * norm;
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            for b in &mut self.filters {
                let next = b[2] * l + b[3] * b[7] + b[4] * b[8] - b[5] * b[9] - b[6] * b[10];
                b[8] = b[7];
                b[7] = l;
                l = next;
                b[10] = b[9];
                b[9] = l;
            }
            for b in &mut self.filters {
                let next = b[2] * r + b[3] * b[11] + b[4] * b[12] - b[5] * b[13] - b[6] * b[14];
                b[12] = b[11];
                b[11] = r;
                r = next;
                b[14] = b[13];
                b[13] = r;
            }
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
