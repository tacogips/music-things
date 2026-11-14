//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/DoublePaul, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::dither::{fpd_from_seed, xorshift};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "DoublePaul",
    upstream_name: "DoublePaul",
    description: "DoublePaul is like PaulDither but more so.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(_: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DoublePaul::new(seed))
}
struct DoublePaul {
    seed: u32,
    fpd: [u32; 2],
    history: [[f64; 11]; 2],
}
impl DoublePaul {
    fn new(seed: u32) -> Self {
        Self {
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            history: [[0.0; 11]; 2],
        }
    }
    fn channel(&mut self, sample: f32, ch: usize) -> f32 {
        let mut sample = sample as f64;
        if sample.abs() < 1.18e-23 {
            sample = self.fpd[ch] as f64 * 1.18e-17;
        }
        sample *= 8388608.0;
        let b = &mut self.history[ch];
        b.copy_within(0..9, 1);
        b[0] = self.fpd[ch] as f64 / u32::MAX as f64;
        let mut d = b[0] * 0.061;
        d -= b[1] * 0.11;
        d += b[8] * 0.126;
        d -= b[7] * 0.23;
        d += b[2] * 0.25;
        d -= b[3] * 0.43;
        d += b[6] * 0.5;
        d -= b[5];
        d += b[4];
        sample += d;
        sample = sample.floor() / 8388608.0;
        xorshift(&mut self.fpd[ch]);
        sample as f32
    }
}
impl AirwindowsEffect for DoublePaul {
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
        *self = Self::new(self.seed);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        for (((&l, &r), ol), or) in il.iter().zip(ir).zip(ol).zip(or) {
            *ol = self.channel(l, 0);
            *or = self.channel(r, 1);
        }
    }
}
