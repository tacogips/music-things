//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Logical4, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::excessive_precision)]
mod stage;
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
use stage::Stage;

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Threshold",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Ratio",
        label: "/1",
        default: 0.2,
    },
    ParamInfo {
        name: "Speed",
        label: "ms",
        default: 0.19202020202020202,
    },
    ParamInfo {
        name: "MakeupGn",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Logical4",
    upstream_name: "Logical4",
    description: "Logical4 is a classic 2-buss compressor.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Logical4::new(rate, seed))
}
struct Logical4 {
    params: [f32; 5],
    rate: f32,
    seed: u32,
    stages: [Stage; 3],
    count: i32,
    flip: bool,
    fpd: [u32; 2],
}
impl Logical4 {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.2, 0.19202020202020202, 0.5, 1.0],
            rate,
            seed,
            stages: [Stage::new(), Stage::new(), Stage::new()],
            count: 0,
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Logical4 {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        let [a, b, c, d, e] = self.params;
        let value = match i {
            0 => (a as f64 * 40.0 - 20.0) as f32,
            1 => ((b * b) as f64 * 15.0 + 1.0) as f32,
            2 => ((c * c) as f64 * 99.0 + 1.0) as f32,
            3 => (d as f64 * 40.0 - 20.0) as f32,
            4 => e,
            _ => return String::new(),
        };
        float2string(value)
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let scale = (1.0 / 44100.0) * self.rate as f64;
        let gain = 10.0_f64.powf(-((a as f64 * 40.0) - 20.0) / 20.0);
        let speed = 10.0 / (((c * c) as f64 * 99.0) + 1.0).sqrt();
        let rem = [
            0.000782 * speed / scale,
            0.000819 * speed / scale,
            0.000857 / scale * speed,
        ];
        let div = [1.0 - rem[0], 1.0 - rem[1], 1.0 - 0.000857 / scale];
        let offset = ((2.42 * scale) as i32).clamp(1, 498) as usize;
        let mut ratio = (((b * b) as f64 * 15.0) + 1.0).sqrt() - 1.0;
        ratio = ratio.clamp(0.0, 2.99999);
        let selector = ratio.floor() as usize;
        ratio -= selector as f64;
        let inv = 1.0 - ratio;
        let outputgain = 10.0_f64.powf(((d as f64 * 40.0) - 20.0) / 20.0);
        let wet = e as f64;
        for (((&left, &right), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = left as f64;
            let mut r = right as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd[0] as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd[1] as f64 * 1.18e-17;
            }
            let dry_l = l as f32 as f64;
            let dry_r = r as f32 as f64;
            self.count -= 1;
            if self.count < 0 || self.count > 499 {
                self.count = 499;
            }
            let mut samples = [(dry_l, dry_r); 4];
            self.stages[0].first(
                &mut l,
                &mut r,
                self.count as usize,
                offset,
                div[0],
                rem[0],
                self.flip,
                gain,
            );
            samples[1] = (l / gain, r / gain);
            if selector > 0 {
                self.stages[1].second(
                    &mut l,
                    &mut r,
                    self.count as usize,
                    offset,
                    div[1],
                    rem[1],
                    self.flip,
                    gain,
                );
                samples[2] = (l / gain, r / gain);
            }
            if selector > 1 {
                self.stages[2].third(
                    &mut l,
                    &mut r,
                    self.count as usize,
                    offset,
                    div[2],
                    rem[2],
                    self.flip,
                    gain,
                );
                samples[3] = (l / gain, r / gain);
            }
            l = samples[selector].0 * inv + samples[selector + 1].0 * ratio;
            r = samples[selector].1 * inv + samples[selector + 1].1 * ratio;
            if outputgain != 1.0 {
                l *= outputgain;
                r *= outputgain;
            }
            if wet != 1.0 {
                l = l * wet + dry_l * (1.0 - wet);
                r = r * wet + dry_r * (1.0 - wet);
            }
            self.flip = !self.flip;
            *out_l = fpd_dither(l, &mut self.fpd[0]) as f32;
            *out_r = fpd_dither(r, &mut self.fpd[1]) as f32;
        }
    }
}
