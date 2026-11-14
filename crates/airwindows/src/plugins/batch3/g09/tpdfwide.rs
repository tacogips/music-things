//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/TPDFWide, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
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
pub(crate) const INFO: PluginInfo = PluginInfo { name: "TPDFWide", upstream_name: "TPDFWide", description: "TPDFWide is still TPDF dither but better and wider. With 16/24 bit output and a DeRez control.", params: PARAMS };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(TPDFWide::new(sample_rate, seed))
}
struct TPDFWide {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
}
impl TPDFWide {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for TPDFWide {
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
            if self.params[0] as f64 * 1.999 < 1.0 {
                "CD 16".into()
            } else {
                "HD 24".into()
            }
        } else if i == 1 {
            float2string(self.params[1])
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
        let mut scale = if (self.params[0] as f64 * 1.999) as i32 == 1 {
            8388608.0f32
        } else {
            32768.0f32
        };
        if self.params[1] > 0.0 {
            scale = (scale as f64 * (1.0 - self.params[1] as f64).powf(6.0)) as f32;
        }
        scale = scale.max(0.0001);
        let out_scale = scale.max(8.0);

        for (((&left, &right), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = left as f64;
            let mut r = right as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            l *= scale as f64;
            r *= scale as f64;
            let mut dl = -1.0;
            dl += self.fpd_l as f64 / u32::MAX as f64;
            xorshift(&mut self.fpd_l);
            dl += self.fpd_l as f64 / u32::MAX as f64;
            let mut dr = -1.0;
            dr += self.fpd_r as f64 / u32::MAX as f64;
            xorshift(&mut self.fpd_r);
            dr += self.fpd_r as f64 / u32::MAX as f64;
            if (dl - dr).abs() < 0.5 {
                dl = -1.0;
                dl += self.fpd_l as f64 / u32::MAX as f64;
                xorshift(&mut self.fpd_l);
                dl += self.fpd_l as f64 / u32::MAX as f64;
            }
            if (dl - dr).abs() < 0.5 {
                dr = -1.0;
                dr += self.fpd_r as f64 / u32::MAX as f64;
                xorshift(&mut self.fpd_r);
                dr += self.fpd_r as f64 / u32::MAX as f64;
            }
            if (dl - dr).abs() < 0.5 {
                dl = -1.0;
                dl += self.fpd_l as f64 / u32::MAX as f64;
                xorshift(&mut self.fpd_l);
                dl += self.fpd_l as f64 / u32::MAX as f64;
            }
            l = (l + dl).floor() / out_scale as f64;
            r = (r + dr).floor() / out_scale as f64;
            xorshift(&mut self.fpd_l);
            xorshift(&mut self.fpd_r);
            *out_l = l as f32;
            *out_r = r as f32;
        }
    }
}
