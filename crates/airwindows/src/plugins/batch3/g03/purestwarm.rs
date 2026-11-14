//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PurestWarm, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Polarity",
    label: "",
    default: 0.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PurestWarm",
    upstream_name: "PurestWarm",
    description: "PurestWarm is a subtle tone shaper and warmth adder.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PurestWarm::new(sample_rate, seed))
}
struct PurestWarm {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
}
impl PurestWarm {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for PurestWarm {
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
            if (self.params[0] as f64 * 1.999) as i32 == 0 {
                "Normal".into()
            } else {
                "Inverse".into()
            }
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
        let polarity = (self.params[0] as f64 * 1.999) as i32;
        for (((&l, &r), ol), orr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            if if polarity == 1 { l < 0.0 } else { l > 0.0 } {
                if polarity == 1 {
                    l = -((-l * 1.57079634).sin() / 1.57079634);
                } else {
                    l = (l * 1.57079634).sin() / 1.57079634;
                }
                l = fpd_dither(l, &mut self.fpd_l);
                r = fpd_dither(r, &mut self.fpd_r);
            }
            if if polarity == 1 { r < 0.0 } else { r > 0.0 } {
                if polarity == 1 {
                    r = -((-r * 1.57079634).sin() / 1.57079634);
                } else {
                    r = (r * 1.57079634).sin() / 1.57079634;
                }
                l = fpd_dither(l, &mut self.fpd_l);
                r = fpd_dither(r, &mut self.fpd_r);
            }
            *ol = l as f32;
            *orr = r as f32;
        }
    }
}
