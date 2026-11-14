//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Monitoring, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Parity: Libm, measured max-abs 0.0 across all three renders.
mod modes;
use crate::dither::fpd_from_seed;
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Monitor",
    label: "",
    default: 0.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Monitoring",
    upstream_name: "Monitoring",
    description: "Monitoring is your one-stop shop for final 2-buss work!",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Monitoring::new(sample_rate, seed))
}
struct Monitoring {
    param: f32,
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    bins: [[f64; 13]; 2],
    shape: [f64; 2],
    delay: [[Box<[f64]>; 2]; 4],
    index: [usize; 4],
    last: [f64; 2],
    iir: [[f64; 26]; 2],
    biquad: [[f64; 11]; 2],
}
impl Monitoring {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let bins = [
            1000.0, 301.0, 176.0, 125.0, 97.0, 79.0, 67.0, 58.0, 51.0, 46.0, 1000.0, 0.0, 0.0,
        ];
        Self {
            param: 0.0,
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            bins: [bins; 2],
            shape: [0.0; 2],
            delay: std::array::from_fn(|_| {
                std::array::from_fn(|_| vec![0.0; 1503].into_boxed_slice())
            }),
            index: [1; 4],
            last: [0.0; 2],
            iir: [[0.0; 26]; 2],
            biquad: [[0.0; 11]; 2],
        }
    }
}
impl AirwindowsEffect for Monitoring {
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
    fn set_param(&mut self, i: usize, value: f32) {
        if i == 0 {
            if let Some(v) = sanitize_param(value) {
                self.param = v;
            }
        }
    }
    fn param_display(&self, i: usize) -> String {
        const NAMES: [&str; 17] = [
            "Out24", "Out16", "Peaks", "Slew", "Subs", "Mono", "Side", "Vinyl", "Aurat", "MonoRat",
            "MonoLat", "Phone", "Cans A", "Cans B", "Cans C", "Cans D", "V Trick",
        ];
        if i == 0 {
            NAMES[(self.param as f64 * 16.999) as usize].into()
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
        let bins = [
            1000.0, 301.0, 176.0, 125.0, 97.0, 79.0, 67.0, 58.0, 51.0, 46.0, 1000.0, 0.0, 0.0,
        ];
        self.bins = [bins; 2];
        self.shape = [0.0; 2];
        for pair in &mut self.delay {
            for channel in pair {
                channel.fill(0.0);
            }
        }
        self.index = [1; 4];
        self.last = [0.0; 2];
        self.iir = [[0.0; 26]; 2];
        self.biquad = [[0.0; 11]; 2];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mode = (self.param as f64 * 16.999) as usize;
        let scale = (1.0 / 44100.0) * self.sample_rate as f64;
        let (freq, q) = match mode {
            7 => (0.0385, 0.0825),
            11 => (0.1245, 0.46),
            _ => (0.0375, 0.1575),
        };
        let k = (std::f64::consts::PI * (freq / scale)).tan();
        let norm = 1.0 / (1.0 + k / q + k * k);
        for b in &mut self.biquad {
            b[0] = freq / scale;
            b[1] = q;
            b[2] = k / q * norm;
            b[4] = -b[2];
            b[5] = 2.0 * (k * k - 1.0) * norm;
            b[6] = (1.0 - k / q + k * k) * norm;
        }
        let quant_scale = if mode == 1 { 32768.0 } else { 8388608.0 };
        for (((&left, &right), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = left as f64;
            let mut r = right as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd[0] as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd[1] as f64 * 1.18e-17;
            }
            self.mode(mode, scale, &mut l, &mut r);
            *out_l = modes::quantize(l, &mut self.bins[0], &mut self.shape[0], quant_scale);
            *out_r = modes::quantize(r, &mut self.bins[1], &mut self.shape[1], quant_scale);
        }
    }
}
