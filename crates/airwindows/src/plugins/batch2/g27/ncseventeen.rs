//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/NCSeventeen, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "LOUDER",
        label: "dB",
        default: 0.0,
    },
    ParamInfo {
        name: "Output",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "NCSeventeen",
    upstream_name: "NCSeventeen",
    description: "NCSeventeen is Dirty Loud!",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(NCSeventeen::new(rate, seed))
}
#[derive(Default)]
struct Channel {
    iir: [f64; 2],
    bass: f64,
    treble: f64,
    cheb: f64,
}
struct NCSeventeen {
    params: [f32; 2],
    rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl NCSeventeen {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 1.0],
            rate,
            seed,
            left: Channel::default(),
            right: Channel::default(),
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
#[allow(clippy::approx_constant, clippy::manual_clamp)]
fn stage(sample: f64, level: &mut f64, decay: f64, feed: f64, maxfeed: f64, limiting: f64) -> f64 {
    let mut in_p2 = sample * sample;
    if in_p2 > 1.0 {
        in_p2 = 1.0;
    }
    if in_p2 < -1.0 {
        in_p2 = -1.0;
    }
    let mut cheb = 2.0 * in_p2;
    cheb *= *level;
    if *level > 0.0 {
        *level -= decay;
    }
    if *level < 0.0 {
        *level += decay;
    }
    let bridge = sample.abs().min(1.57079633).sin();
    let dist = if sample > 0.0 { bridge } else { -bridge };
    let minus = (sample - dist).clamp(-maxfeed, maxfeed);
    let plus = (sample + dist).clamp(-maxfeed, maxfeed);
    if sample > dist {
        *level += minus * feed;
    }
    if sample < -dist {
        *level -= plus * feed;
    }
    *level = level.clamp(-1.0, 1.0);
    let bridge = sample.abs().min(1.57079633).sin();
    let mut out = if sample > 0.0 { bridge } else { -bridge };
    out /= 1.0 + (*level * limiting).abs();
    out += cheb;
    out
}
fn channel(input: f64, c: &mut Channel, flip: bool, scale: f64) -> f64 {
    let idx = if flip { 0 } else { 1 };
    c.iir[idx] = c.iir[idx] * 0.9 + input * 0.1;
    let low = c.iir[idx];
    let high = input - low;
    let low = stage(
        low,
        &mut c.bass,
        0.0002364 / scale,
        0.000243 / scale,
        0.972,
        0.39,
    );
    let high = stage(
        high,
        &mut c.treble,
        0.0005484 / scale,
        0.000407 / scale,
        0.972,
        0.6,
    );
    stage(
        low + high,
        &mut c.cheb,
        0.0004716 / scale,
        0.000326 / scale,
        0.975,
        0.36,
    )
}
impl AirwindowsEffect for NCSeventeen {
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
            0 => float2string((self.params[0] as f64 * 24.0) as f32),
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let scale = self.rate as f64 / 44100.0;
        let gain = 10.0_f64.powf(self.params[0] as f64 * 24.0 / 20.0);
        let out = self.params[1] as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            l = channel(l * gain, &mut self.left, self.flip, scale);
            r = channel(r * gain, &mut self.right, self.flip, scale);
            self.flip = !self.flip;
            if out < 1.0 {
                l *= out;
                r *= out;
            }
            l = l.clamp(-0.95, 0.95);
            r = r.clamp(-0.95, 0.95);
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
