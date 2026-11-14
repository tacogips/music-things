//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/VoiceTrick, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Tone",
    label: "",
    default: 1.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "VoiceTrick",
    upstream_name: "VoiceTrick",
    description: "VoiceTrick lets you record vocals while monitoring over speakers.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(VoiceTrick::new(sample_rate, seed))
}
struct VoiceTrick {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    lowpass: [f64; 6],
    count: usize,
    amount: f64,
    last: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl VoiceTrick {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0],
            sample_rate,
            seed,
            lowpass: [0.0; 6],
            count: 0,
            amount: 1.0,
            last: 1000.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for VoiceTrick {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let chase = (self.params[0] as f64).powf(2.0);
        let speed = 300.0 / ((self.last - chase).abs() + 1.0);
        self.last = chase;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            self.amount = ((self.amount * speed) + chase) / (speed + 1.0);
            let inv = 1.0 - self.amount;
            let mut s = (l + r) * 0.5;
            self.count = (self.count + 1) % 6;
            let second = if self.count % 2 == 0 { 1 } else { 2 };
            let third = 3 + (self.count % 3);
            for idx in [0, second, third] {
                self.lowpass[idx] = (self.lowpass[idx] * inv) + (s * self.amount);
                s = self.lowpass[idx];
            }
            *outl = fpd_dither(-s, &mut self.fpd_l) as f32;
            *outr = fpd_dither(s, &mut self.fpd_r) as f32;
        }
    }
}
