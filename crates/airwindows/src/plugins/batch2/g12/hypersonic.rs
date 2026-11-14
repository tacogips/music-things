//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Hypersonic, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

#[allow(clippy::approx_constant)] // Preserve the upstream literals.
const QS: [f64; 7] = [
    4.46570214, 1.51387132, 0.93979296, 0.70710678, 0.59051105, 0.52972649, 0.50316379,
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Hypersonic",
    upstream_name: "Hypersonic",
    description: "Hypersonic is Ultrasonic, but steeper and higher cutoff.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Effect::new(sample_rate, seed))
}
struct Effect {
    sample_rate: f32,
    seed: u32,
    state: [[[f64; 2]; 2]; 7],
    fpd: [u32; 2],
}
impl Effect {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            sample_rate,
            seed,
            state: [[[0.0; 2]; 2]; 7],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Effect {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        let _ = i;
        0.0
    }
    fn set_param(&mut self, i: usize, v: f32) {
        let _ = (i, v);
    }
    fn param_display(&self, i: usize) -> String {
        let _ = i;
        String::new()
    }
    fn reset(&mut self) {
        *self = Self::new(self.sample_rate, self.seed);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let cutoff = (25000.0 / self.sample_rate as f64).min(0.49);
        let k = (std::f64::consts::PI * cutoff).tan();
        let qs = QS;
        let mut coeff = [[0.0; 5]; 7];
        for (c, reso) in coeff.iter_mut().zip(qs) {
            let norm = 1.0 / (1.0 + k / reso + k * k);
            c[0] = k * k * norm;
            c[1] = 2.0 * c[0];
            c[2] = c[0];
            c[3] = 2.0 * (k * k - 1.0) * norm;
            c[4] = (1.0 - k / reso + k * k) * norm;
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut out = [0.0; 2];
            for (ch, input) in [l, r].into_iter().enumerate() {
                let mut sample = input as f64;
                if sample.abs() < 1.18e-23 {
                    sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                for (state, c) in self.state.iter_mut().zip(coeff) {
                    let s = &mut state[ch];
                    let output = sample * c[0] + s[0];
                    s[0] = sample * c[1] - output * c[3] + s[1];
                    s[1] = sample * c[2] - output * c[4];
                    sample = output;
                }
                out[ch] = fpd_dither(sample, &mut self.fpd[ch]) as f32;
            }
            *out_l = out[0];
            *out_r = out[1];
        }
    }
}
