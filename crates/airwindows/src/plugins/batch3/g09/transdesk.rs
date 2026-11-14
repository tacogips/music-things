//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/TransDesk, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
#![allow(clippy::approx_constant)]
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "TransDesk",
    upstream_name: "TransDesk",
    description: "TransDesk is more of a transistory, rock desk analog modeling.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(TransDesk::new(sample_rate, seed))
}
#[derive(Default)]
struct Channel {
    delay: [f64; 20],
    control: f64,
    last_sample: f64,
    last_out: f64,
    last_slew: f64,
}
impl Channel {
    fn process(&mut self, mut input: f64, index: usize, offset: usize, scale: f64) -> f64 {
        let dry = input;
        self.delay[index] = input.abs() * 0.02198359;
        self.delay[index + 9] = self.delay[index];
        self.control += self.delay[index] / offset as f64;
        self.control -= self.delay[index + offset] / offset as f64;
        self.control -= 0.000001;
        let mut clamp = 1.0;
        if self.control < 0.0 {
            self.control = 0.0;
        }
        if self.control > 1.0 {
            clamp -= self.control - 1.0;
            self.control = 1.0;
        }
        if clamp < 0.5 {
            clamp = 0.5;
        }
        let thickness = ((1.0 - self.control) * 2.0) - 1.0;
        let out = thickness.abs();
        let mut bridge = input.abs();
        if bridge > 1.57079633 {
            bridge = 1.57079633;
        }
        bridge = if thickness > 0.0 {
            bridge.sin()
        } else {
            1.0 - bridge.cos()
        };
        input = if input > 0.0 {
            (input * (1.0 - out)) + (bridge * out)
        } else {
            (input * (1.0 - out)) - (bridge * out)
        };
        input *= clamp;
        let mut slew = input - self.last_sample;
        self.last_sample = input;
        let slewgain = 0.197 * scale;
        bridge = (slew * slewgain).abs();
        bridge = if bridge > 1.57079633 {
            1.0
        } else {
            bridge.sin()
        };
        slew = if slew > 0.0 {
            bridge / slewgain
        } else {
            -(bridge / slewgain)
        };
        let balance_b = 0.0001 / scale;
        let balance_a = 1.0 - balance_b;
        input = (self.last_out * balance_a) + (self.last_sample * balance_b) + slew;
        self.last_out = input;
        let mut comb = (dry * self.last_sample).abs();
        if comb > 1.0 {
            comb = 1.0;
        }
        input -= self.last_slew * comb * (0.255 * scale);
        self.last_slew = slew;
        input *= 0.130;
        bridge = input.abs();
        bridge = if bridge > 1.57079633 {
            1.0
        } else {
            bridge.sin()
        };
        input = if input > 0.0 { bridge } else { -bridge };
        input / 0.130
    }
}
struct TransDesk {
    sample_rate: f32,
    seed: u32,
    count: i32,
    left: Channel,
    right: Channel,
    fpd_l: u32,
    fpd_r: u32,
}
impl TransDesk {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            sample_rate,
            seed,
            count: 0,
            left: Channel::default(),
            right: Channel::default(),
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for TransDesk {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let offset = ((3.0 * scale) as i32).clamp(1, 8) as usize;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            if self.count < 0 || self.count > 9 {
                self.count = 9;
            }
            l = self.left.process(l, self.count as usize, offset, scale);
            r = self.right.process(r, self.count as usize, offset, scale);
            self.count -= 1;
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
