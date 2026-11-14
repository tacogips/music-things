//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Console8SubIn, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Console8SubIn",
    upstream_name: "Console8SubIn",
    description:
        "Console8SubIn moves to a channel/submix/buss topology and adds analog simulation.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Console8SubIn::new(sample_rate, seed))
}

struct Console8SubIn {
    sample_rate: f32,
    seed: u32,
    iir_a: [f64; 2],
    iir_b: [f64; 2],
    soft: [[f64; 11]; 2],
    filter: [[f64; 2]; 2],
    fpd: [u32; 2],
}
impl Console8SubIn {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            iir_a: [0.0; 2],
            iir_b: [0.0; 2],
            soft: [[0.0; 11]; 2],
            filter: [[0.0; 2]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Console8SubIn {
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
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let rate = self.sample_rate as f64;
        let amount = 12.66 / rate;
        for c in 0..2 {
            if self.iir_a[c].abs() < 1.18e-37 {
                self.iir_a[c] = 0.0
            }
            if self.iir_b[c].abs() < 1.18e-37 {
                self.iir_b[c] = 0.0
            }
        }
        let k = (std::f64::consts::PI * (24000.0 / rate)).tan();
        let norm = 1.0 / (1.0 + k / 0.59435114 + k * k);
        let a0 = k * k * norm;
        let a1 = 2.0 * a0;
        let a2 = a0;
        let b1 = 2.0 * (k * k - 1.0) * norm;
        let b2 = (1.0 - k / 0.59435114 + k * k) * norm;
        let mut scale: f64 = 1.0;
        scale /= 44100.0;
        scale *= rate;
        let mut cycle = scale.floor() as i32;
        if cycle < 1 {
            cycle = 1
        }
        if cycle == 3 {
            cycle = 4
        }
        if cycle > 4 {
            cycle = 4
        }
        let n = il.len().min(ir.len()).min(ol.len()).min(or.len());
        for i in 0..n {
            let inp = [il[i], ir[i]];
            let mut out = [0.0f32; 2];
            for c in 0..2 {
                let mut x = inp[c] as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[c] as f64 * 1.18e-17
                }
                self.iir_a[c] = (self.iir_a[c] * (1.0 - amount)) + (x * amount);
                let b = self.iir_a[c].abs() + 0.00001;
                self.iir_b[c] = (self.iir_b[c] * (1.0 - b)) + (self.iir_a[c] * b);
                x -= self.iir_b[c];
                let s = &mut self.soft[c];
                match cycle {
                    4 => {
                        for j in (1..=8).rev() {
                            s[j] = s[j - 1]
                        }
                    }
                    2 => {
                        s[8] = s[6];
                        s[6] = s[4];
                        s[4] = s[2];
                        s[2] = s[0]
                    }
                    _ => {
                        s[8] = s[4];
                        s[4] = s[0]
                    }
                }
                s[0] = x;
                s[9] = (s[0] - s[4]) - (s[4] - s[8]);
                if s[9] > 0.73824539 {
                    x = s[4] + s[4].abs() * (s[9] - 0.73824539).sin() * 0.26175461
                }
                if -s[9] > 0.73824539 {
                    x = s[4] - s[4].abs() * (-s[9] - 0.73824539).sin() * 0.26175461
                }
                x = x.clamp(-1.57079633, 1.57079633);
                if rate > 49000.0 {
                    let y = (x * a0) + self.filter[c][0];
                    self.filter[c][0] = (x * a1) - (y * b1) + self.filter[c][1];
                    self.filter[c][1] = (x * a2) - (y * b2);
                    x = y;
                }
                x = x.clamp(-1.0, 1.0).asin();
                out[c] = fpd_dither(x, &mut self.fpd[c]) as f32;
            }
            ol[i] = out[0];
            or[i] = out[1];
        }
    }
}
