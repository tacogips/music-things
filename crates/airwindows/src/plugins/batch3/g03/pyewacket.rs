//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Pyewacket, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)] // Upstream coefficients are intentionally rounded.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Input Gain",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Release",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "Output Gain",
        label: "dB",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo { name: "Pyewacket", upstream_name: "Pyewacket", description: "Pyewacket is an old school compressor for high definition transients. Adds no fatness, just energy.", params: PARAMS };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Pyewacket::new(sample_rate, seed))
}
struct Pyewacket {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
    chase: f64,
    last_l: f64,
    last_r: f64,
}
impl Pyewacket {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.5],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            chase: 1.0,
            last_l: 0.0,
            last_r: 0.0,
        }
    }
}
impl AirwindowsEffect for Pyewacket {
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
        match i {
            0 | 2 => float2string((self.params[i] as f64 * 24.0 - 12.0) as f32),
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }

    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let overallscale = (1.0 / 44100.0) * self.sample_rate as f64;
        let old = 0.618_033_988_749_894_9;
        let new = 1.0 - old;
        let ig = 10.0f64.powf((self.params[0] as f64 * 24.0 - 12.0) / 20.0);
        let attack = ((self.params[1] as f64 + 0.5) * 0.006) / overallscale;
        let decay = ((self.params[1] as f64 + 0.01) * 0.0004) / overallscale;
        let og = 10.0f64.powf((self.params[2] as f64 * 24.0 - 12.0) / 20.0);
        for (((&l, &r), ol), orr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            if ig != 1.0 {
                l *= ig;
                r *= ig;
            }
            let sense = l.abs().max(r.abs());
            if self.chase < sense {
                self.chase += attack;
            }
            if self.chase > 1.0 {
                self.chase = 1.0;
            }
            if self.chase > sense {
                self.chase -= decay;
            }
            if self.chase < 0.0 {
                self.chase = 0.0;
            }
            let mut out = self.chase;
            let mut wet = self.chase;
            if wet > 1.0 {
                wet = 1.0;
            }
            let maxblur = wet * new;
            let blurdry = 1.0 - maxblur;
            if out > old {
                out = old - (out - old);
            }
            if out < 0.0 {
                out = 0.0;
            }
            let dry = 1.0 - wet;
            l = l.clamp(-1.57079633, 1.57079633);
            r = r.clamp(-1.57079633, 1.57079633);
            let temp = 1.0 - l.abs().min(1.57079633).cos();
            let bridge = self.last_l * maxblur + temp * blurdry;
            self.last_l = temp;
            if l > 0.0 {
                l = l * dry + bridge * out;
            } else {
                l = l * dry - bridge * out;
            }
            let temp = 1.0 - r.abs().min(1.57079633).cos();
            let bridge = self.last_r * maxblur + temp * blurdry;
            self.last_r = temp;
            if r > 0.0 {
                r = r * dry + bridge * out;
            } else {
                r = r * dry - bridge * out;
            }
            if og != 1.0 {
                l *= og;
                r *= og;
            }
            *ol = fpd_dither(l, &mut self.fpd_l) as f32;
            *orr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
