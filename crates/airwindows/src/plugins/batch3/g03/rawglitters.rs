//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/RawGlitters, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Quant",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "DeRez",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo { name: "RawGlitters", upstream_name: "RawGlitters", description: "RawGlitters is just the quantization from Dither Me Timbers… and the opposite, as a brightener.", params: PARAMS };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(RawGlitters::new(sample_rate, seed))
}
struct RawGlitters {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
    last_l: f64,
    last2_l: f64,
    last_r: f64,
    last2_r: f64,
}
impl RawGlitters {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            last_l: 0.0,
            last2_l: 0.0,
            last_r: 0.0,
            last2_r: 0.0,
        }
    }
}
impl AirwindowsEffect for RawGlitters {
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
            0 => {
                if (self.params[0] as f64 * 1.999) as i32 == 1 {
                    "HD 24".into()
                } else {
                    "CD 16".into()
                }
            }
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
        let mut scale = if (self.params[0] as f64 * 1.999) as i32 == 1 {
            8388608.0f32
        } else {
            32768.0f32
        };
        let derez = self.params[1];
        if derez > 0.0 {
            scale = (scale as f64 * (1.0 - derez as f64).powf(6.0)) as f32;
        }
        if scale < 0.0001 {
            scale = 0.0001;
        }
        let outscale = scale.max(8.0);
        for (((&l, &r), ol), orr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            xorshift(&mut self.fpd_l);
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            xorshift(&mut self.fpd_r);
            l *= scale as f64;
            r *= scale as f64;
            l += 0.381966011250105;
            r += 0.381966011250105;
            let outl = if self.last_l + self.last_l <= l + self.last2_l {
                self.last_l.floor()
            } else {
                (self.last_l + 1.0).floor()
            };
            let outr = if self.last_r + self.last_r <= r + self.last2_r {
                self.last_r.floor()
            } else {
                (self.last_r + 1.0).floor()
            };
            self.last2_l = self.last_l;
            self.last_l = l;
            self.last2_r = self.last_r;
            self.last_r = r;
            *ol = (outl / outscale as f64) as f32;
            *orr = (outr / outscale as f64) as f32;
        }
    }
}
