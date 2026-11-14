//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/DustBunny, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::excessive_precision)]
use crate::display::float2string;
use crate::dither::fpd_from_seed;
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Bunny",
    label: " ",
    default: 0.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "DustBunny",
    upstream_name: "DustBunny",
    description:
        "DustBunny is unpredictable distorty noise, like dust wedged under your turntable needle.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DustBunny::new(sample_rate, seed))
}
#[derive(Default)]
struct Channel {
    last1: f32,
    last2: f32,
    last3: f32,
    a: f32,
    b: f32,
    prev: f32,
    flip: bool,
}
impl Channel {
    fn filter(&mut self, input: f32, dry: f32) -> f32 {
        let mut c = input - dry;
        let decay = 0.915965594177219015_f32;
        if self.flip {
            self.a *= decay;
            self.b *= decay;
            self.a += c;
            self.b -= c;
            c = self.a;
        } else {
            self.b *= decay;
            self.a *= decay;
            self.b += c;
            self.a -= c;
            c = self.b;
        }
        self.flip = !self.flip;
        c * decay
    }
    fn process(&mut self, input: f32, bunny: u32) -> f32 {
        let dry = input;
        let half_dry =
            ((input + self.last1 + ((-self.last2 + self.last3) * 0.0414213562373095048801688_f32))
                as f64
                / 2.0) as f32;
        self.last3 = self.last2;
        self.last2 = self.last1;
        self.last1 = input;
        let half = if half_dry.to_bits() & bunny == 0 {
            0.0
        } else {
            half_dry
        };
        let half_diff = self.filter(half, half_dry);
        let wet = if input.to_bits() & bunny == 0 {
            0.0
        } else {
            input
        };
        let diff = self.filter(wet, dry);
        let result = (dry as f64 + ((diff + half_diff + self.prev) as f64 / 2.5)) as f32;
        self.prev = (diff as f64 / 2.0) as f32;
        result
    }
}
struct DustBunny {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    fpd_l: u32,
    fpd_r: u32,
}
impl DustBunny {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0],
            sample_rate,
            seed,
            left: Channel::default(),
            right: Channel::default(),
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for DustBunny {
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
        if i == 0 {
            float2string(self.params[0])
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let bunny = ((1.255 - self.params[0] as f64).powf(5.0) * 1000.0) as u32;
        let bunny = bunny.wrapping_mul(bunny);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let l = if (l as f64).abs() < 1.18e-23 {
                (self.fpd_l as f64 * 1.18e-17) as f32
            } else {
                l
            };
            let r = if (r as f64).abs() < 1.18e-23 {
                (self.fpd_r as f64 * 1.18e-17) as f32
            } else {
                r
            };
            *out_l = self.left.process(l, bunny);
            *out_r = self.right.process(r, bunny);
        }
    }
}
