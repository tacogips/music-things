//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Interstage, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Interstage",
    upstream_name: "Interstage",
    description: "Interstage is a subtle and sophisticated analogifier.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Interstage::new(sample_rate, seed))
}
struct Interstage {
    sample_rate: f32,
    seed: u32,
    stages: [[f64; 6]; 2],
    last: [f64; 2],
    fpd: [u32; 2],
    flip: bool,
}
impl Interstage {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            sample_rate,
            seed,
            stages: [[0.0; 6]; 2],
            last: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            flip: true,
        }
    }
}
impl AirwindowsEffect for Interstage {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let scale = self.sample_rate as f64 / 44100.0;
        let first = 0.381966011250105 / scale;
        let amount = 0.00295 / scale;
        let threshold = 0.381966011250105;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut output = [0.0; 2];
            for (ch, input) in [l, r].into_iter().enumerate() {
                let mut dry = input as f64;
                if dry.abs() < 1.18e-23 {
                    dry = self.fpd[ch] as f64 * 1.18e-17;
                }
                let mut sample = (dry + self.last[ch]) * 0.5;
                let start = if self.flip { 0 } else { 1 };
                for idx in [start, start + 2, start + 4] {
                    let gain = if idx == start { first } else { amount };
                    self.stages[ch][idx] = self.stages[ch][idx] * (1.0 - gain) + sample * gain;
                    sample = self.stages[ch][idx];
                }
                sample = dry - sample;
                let reference = self.stages[ch][start];
                if sample - reference > threshold {
                    sample = reference + threshold;
                }
                if sample - reference < -threshold {
                    sample = reference - threshold;
                }
                self.last[ch] = sample;
                output[ch] = fpd_dither(sample, &mut self.fpd[ch]) as f32;
            }
            self.flip = !self.flip;
            *out_l = output[0];
            *out_r = output[1];
        }
    }
}
