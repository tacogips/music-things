//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/OneCornerClip, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::{db2string, float2string};
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Input",
        label: "",
        default: 1.0 / 3.0,
    },
    ParamInfo {
        name: "Pos Thr",
        label: "",
        default: 0.966,
    },
    ParamInfo {
        name: "Neg Thr",
        label: "",
        default: 0.966,
    },
    ParamInfo {
        name: "Voicing",
        label: "",
        default: 0.618,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "OneCornerClip",
    upstream_name: "OneCornerClip",
    description: "OneCornerClip is an ultimate full-bandwidth clipper.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(OneCornerClip::new(rate, seed))
}
struct OneCornerClip {
    params: [f32; 5],
    rate: f32,
    seed: u32,
    fpd: [u32; 2],
    last: [f64; 2],
}
impl OneCornerClip {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0 / 3.0, 0.966, 0.966, 0.618, 1.0],
            rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            last: [0.0; 2],
        }
    }
}
impl AirwindowsEffect for OneCornerClip {
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
            0 => float2string((self.params[0] as f64 * 36.0 - 12.0) as f32),
            1 | 2 => db2string(self.params[i]),
            3 | 4 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let gain = 10.0_f64.powf((a as f64 * 36.0 - 12.0) / 20.0);
        let pos = b as f64;
        let neg = -(c as f64);
        let mut voice = d as f64;
        if voice == 0.618 {
            voice = 0.618_033_988_749_894_9;
        }
        let scale = self.rate as f64 / 44100.0;
        if scale > 0.0 {
            voice /= scale;
        }
        voice = voice.clamp(0.0, 1.0);
        let mut pt = [pos; 2];
        let mut nt = [neg; 2];
        let wet = e as f64;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let raw = [l, r];
            let mut input = [l as f64, r as f64];
            for (i, s) in input.iter_mut().enumerate() {
                if s.abs() < 1.18e-23 {
                    *s = self.fpd[i] as f64 * 1.18e-17;
                }
            }
            let dry = input;
            let mut engaged = gain != 1.0;
            if engaged {
                input[0] *= gain;
                input[1] *= gain;
            }
            for i in 0..2 {
                if input[i] > pt[i] {
                    input[i] = self.last[i] * voice + pos * (1.0 - voice);
                    pt[i] = input[i];
                    engaged = true;
                } else {
                    pt[i] = pos;
                }
                if input[i] < nt[i] {
                    input[i] = self.last[i] * voice + neg * (1.0 - voice);
                    nt[i] = input[i];
                    engaged = true;
                } else {
                    nt[i] = neg;
                }
                self.last[i] = input[i];
                if wet != 1.0 {
                    input[i] = input[i] * wet + dry[i] * (1.0 - wet);
                }
                input[i] = fpd_dither(input[i], &mut self.fpd[i]);
            }
            if !engaged {
                input = [raw[0] as f64, raw[1] as f64];
            }
            *outl = input[0] as f32;
            *outr = input[1] as f32;
        }
    }
}
