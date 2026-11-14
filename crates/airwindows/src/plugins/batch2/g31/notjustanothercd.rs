//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/NotJustAnotherCD, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::dither::fpd_from_seed;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};
const INFO: PluginInfo = PluginInfo {
    name: "NotJustAnotherCD",
    upstream_name: "NotJustAnotherCD",
    description: "NotJustAnotherCD is a next-gen wordlength reducers for 16 bit with DeRez.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(NotJustAnotherCD::new(rate, seed))
}
struct NotJustAnotherCD {
    rate: f32,
    seed: u32,
    fpd: [u32; 2],
    bins: [[f64; 11]; 2],
    shaping: [f64; 2],
}
impl NotJustAnotherCD {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            bins: [[
                1000.0, 301.0, 176.0, 125.0, 97.0, 79.0, 67.0, 58.0, 51.0, 46.0, 1000.0,
            ]; 2],
            shaping: [0.0; 2],
        }
    }
}
fn score(mut value: f64, bins: &mut [f64; 11]) -> (usize, f64) {
    while value >= 1.0 {
        value /= 10.0;
    }
    for _ in 0..5 {
        if value < 1.0 {
            value *= 10.0;
        }
    }
    let bin = value.floor() as i32;
    if !(1..10).contains(&bin) {
        return (10, 0.0);
    }
    let bin = bin as usize;
    bins[bin] += 1.0;
    let target = [301.0, 176.0, 125.0, 97.0, 79.0, 67.0, 58.0, 51.0, 46.0];
    let mut total = 0.0;
    for i in 0..9 {
        total += target[i] - bins[i + 1];
    }
    bins[bin] -= 1.0;
    (bin, total)
}
impl AirwindowsEffect for NotJustAnotherCD {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            for (i, x) in input.iter_mut().enumerate() {
                if x.abs() < 1.18e-23 {
                    *x = self.fpd[i] as f64 * 1.18e-17;
                }
                let dry = *x as f32;
                *x -= self.shaping[i];
                *x *= 32768.0;
                let (a, ta) = score(x.floor(), &mut self.bins[i]);
                let (b, tb) = score(x.ceil(), &mut self.bins[i]);
                if ta < tb {
                    self.bins[i][a] += 1.0;
                    *x = x.floor();
                } else {
                    self.bins[i][b] += 1.0;
                    *x = x.ceil();
                }
                self.bins[i][10] /= 2.0;
                *x /= 32768.0;
                self.shaping[i] += *x - dry as f64;
            }
            *outl = input[0] as f32;
            *outr = input[1] as f32;
        }
    }
}
