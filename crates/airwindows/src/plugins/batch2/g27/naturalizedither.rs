//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/NaturalizeDither, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::dither::{fpd_from_seed, xorshift};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo { name: "NaturalizeDither", upstream_name: "NaturalizeDither", description: "NaturalizeDither is deterministic dither that uses Benford Realness calculations for each sample.", params: &[] };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(NaturalizeDither::new(rate, seed))
}
const INITIAL: [f64; 11] = [
    1000.0, 301.0, 176.0, 125.0, 97.0, 79.0, 67.0, 58.0, 51.0, 46.0, 1000.0,
];
struct NaturalizeDither {
    rate: f32,
    seed: u32,
    left: [f64; 11],
    right: [f64; 11],
    fpd_l: u32,
    fpd_r: u32,
}
impl NaturalizeDither {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            rate,
            seed,
            left: INITIAL,
            right: INITIAL,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn bin(mut value: f64) -> usize {
    while value >= 1.0 {
        value /= 10.0;
    }
    if value < 1.0 {
        value *= 10.0;
    }
    if value < 1.0 {
        value *= 10.0;
    }
    let value = value.floor() as i32;
    if (1..10).contains(&value) {
        value as usize
    } else {
        10
    }
}
fn score(bins: &mut [f64; 11], index: usize) -> f64 {
    if index == 10 {
        return 0.0;
    }
    bins[index] += 1.0;
    let mut total = 0.0;
    for i in 1..10 {
        total += INITIAL[i] - bins[i];
    }
    bins[index] -= 1.0;
    total
}
fn quantize(mut sample: f64, bins: &mut [f64; 11], fpd: &mut u32) -> f32 {
    if sample.abs() < 1.18e-23 {
        sample = *fpd as f64 * 1.18e-17;
    }
    sample *= 8388608.0;
    if sample > 0.0 {
        sample += 0.3333333333;
    }
    if sample < 0.0 {
        sample -= 0.3333333333;
    }
    sample += (*fpd as f64 / u32::MAX as f64) * 0.6666666666;
    let a = bin(sample.floor());
    let b = bin(sample.ceil());
    let sa = score(bins, a);
    let sb = score(bins, b);
    if sa < sb {
        bins[a] += 1.0;
        sample = sample.floor();
    } else {
        bins[b] += 1.0;
        sample = sample.ceil();
    }
    bins[10] /= 2.0;
    xorshift(fpd);
    (sample / 8388608.0) as f32
}
impl AirwindowsEffect for NaturalizeDither {
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
        *self = Self::new(self.rate, self.seed);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            *out_l = quantize(l as f64, &mut self.left, &mut self.fpd_l);
            *out_r = quantize(r as f64, &mut self.right, &mut self.fpd_r);
        }
    }
}
