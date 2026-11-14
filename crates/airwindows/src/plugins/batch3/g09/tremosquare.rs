//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/TremoSquare, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "TremoSquare",
    upstream_name: "TremoSquare",
    description: "TremoSquare is a squarewave tremolo effect that only switches on zero crossings.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(TremoSquare::new(sample_rate, seed))
}
struct TremoSquare {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
    osc: f64,
    polarity_l: bool,
    polarity_r: bool,
    mute_l: bool,
    mute_r: bool,
}
impl TremoSquare {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 1.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            osc: 0.0,
            polarity_l: false,
            polarity_r: false,
            mute_l: false,
            mute_r: false,
        }
    }
}
impl AirwindowsEffect for TremoSquare {
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
        self.params
            .get(i)
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let overallscale = (1.0 / 44100.0) * self.sample_rate as f64;
        let increment = (self.params[0] as f64).powf(4.0) / (50.0 * overallscale);
        let wet = self.params[1] as f64;
        for (((&left, &right), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = left as f64;
            let mut r = right as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            self.osc += increment;
            if self.osc > 1.0 {
                self.osc = 0.0;
            }
            if l < 0.0 {
                if self.polarity_l {
                    self.mute_l = self.osc < 0.5;
                }
                self.polarity_l = false;
            } else {
                if !self.polarity_l {
                    self.mute_l = self.osc < 0.5;
                }
                self.polarity_l = true;
            }
            if r < 0.0 {
                if self.polarity_r {
                    self.mute_r = self.osc < 0.5;
                }
                self.polarity_r = false;
            } else {
                if !self.polarity_r {
                    self.mute_r = self.osc < 0.5;
                }
                self.polarity_r = true;
            }
            if self.mute_l {
                l = 0.0;
            }
            if self.mute_r {
                r = 0.0;
            }
            if wet != 1.0 {
                l = (l * wet) + (dry_l * (1.0 - wet));
                r = (r * wet) + (dry_r * (1.0 - wet));
            }
            l = fpd_dither(l, &mut self.fpd_l);
            r = fpd_dither(r, &mut self.fpd_r);
            *out_l = l as f32;
            *out_r = r as f32;
        }
    }
}
