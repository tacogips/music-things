//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Air at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
// Preserve upstream constants and operation order.
#![allow(clippy::too_many_arguments)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "22K tap",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "15K tap",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "11K tap",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "filters Q",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Output Level",
        label: " ",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Air",
    upstream_name: "Air",
    description: "Air is a different bright EQ from any other. Requires 44.1K.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Air::new(sample_rate, seed))
}

#[derive(Default)]
struct Channel {
    prev: [f64; 3],
    even: [f64; 3],
    odd: [f64; 3],
    triplet_prev: f64,
    triplet_mid: f64,
    triplet: [f64; 3],
}
impl Channel {
    fn process(
        &mut self,
        input: f64,
        count: usize,
        flop: bool,
        flip: bool,
        q: f64,
        triplet_intensity: f64,
        air_intensity: f64,
        hi_intensity: f64,
    ) -> f64 {
        let mut factor = self.triplet_prev - input;
        self.triplet[count] += factor;
        self.triplet[(count + 2) % 3] -= factor;
        factor = self.triplet[count] * triplet_intensity;
        self.triplet_prev = self.triplet_mid;
        self.triplet_mid = input;
        self.triplet[0] /= q;
        self.triplet[1] /= q;
        self.triplet[2] /= q;
        let mut correction = 0.0 + factor;
        let i = if flop { 0 } else { 1 };
        factor = self.prev[i] - input;
        if flip {
            self.even[i] += factor;
            self.odd[i] -= factor;
            factor = self.even[i] * air_intensity;
        } else {
            self.odd[i] += factor;
            self.even[i] -= factor;
            factor = self.odd[i] * air_intensity;
        }
        self.odd[i] = (self.odd[i] - ((self.odd[i] - self.even[i]) / 256.0)) / q;
        self.even[i] = (self.even[i] - ((self.even[i] - self.odd[i]) / 256.0)) / q;
        self.prev[i] = input;
        correction += factor;
        factor = self.prev[2] - input;
        if flop {
            self.even[2] += factor;
            self.odd[2] -= factor;
            factor = self.even[2] * hi_intensity;
        } else {
            self.odd[2] += factor;
            self.even[2] -= factor;
            factor = self.odd[2] * hi_intensity;
        }
        self.odd[2] = (self.odd[2] - ((self.odd[2] - self.even[2]) / 256.0)) / q;
        self.even[2] = (self.even[2] - ((self.even[2] - self.odd[2]) / 256.0)) / q;
        self.prev[2] = input;
        correction += factor;
        input + correction
    }
}
struct Air {
    params: [f32; 6],
    sample_rate: f32,
    seed: u32,
    channels: [Channel; 2],
    flip: [bool; 2],
    flop: bool,
    count: usize,
    fpd: [u32; 2],
}
impl Air {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.5, 0.0, 1.0, 1.0],
            sample_rate,
            seed,
            channels: [Channel::default(), Channel::default()],
            flip: [false; 2],
            flop: false,
            count: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Air {
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
        match self.params.get(i) {
            Some(&v) if i < 3 => float2string((f64::from(v) * 2.0 - 1.0) as f32),
            Some(&v) => float2string(v),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let hi = -(f64::from(self.params[0]) * 2.0 - 1.0).powf(3.0) * 2.0;
        let tri = -(f64::from(self.params[1]) * 2.0 - 1.0).powf(3.0);
        let air = -(f64::from(self.params[2]) * 2.0 - 1.0).powf(3.0) / 2.0;
        let q = 2.1 - f64::from(self.params[3]);
        let output = f64::from(self.params[4]);
        let wet = f64::from(self.params[5]);
        let dry = 1.0 - wet;
        for (((&l, &r), left), right) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut vals = [f64::from(l), f64::from(r)];
            for (ch, sample) in vals.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = f64::from(self.fpd[ch]) * 1.18e-17;
                }
                let orig = *sample;
                *sample = self.channels[ch].process(
                    *sample,
                    self.count,
                    self.flop,
                    self.flip[if self.flop { 0 } else { 1 }],
                    q,
                    tri,
                    air,
                    hi,
                );
                if output < 1.0 {
                    *sample *= output;
                }
                if wet < 1.0 {
                    *sample = orig * dry + *sample * wet;
                }
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            self.count = (self.count + 1) % 3;
            let i = if self.flop { 0 } else { 1 };
            self.flip[i] = !self.flip[i];
            self.flop = !self.flop;
            *left = vals[0] as f32;
            *right = vals[1] as f32;
        }
    }
}
