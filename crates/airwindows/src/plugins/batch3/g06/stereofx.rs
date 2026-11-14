//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/StereoFX, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Wide",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "MonoBs",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "CSquish",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "StereoFX",
    upstream_name: "StereoFX",
    description: "StereoFX is an aggressive stereo widener.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(StereoFX::new(sample_rate, seed))
}
struct StereoFX {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    iir_a: f64,
    iir_b: f64,
    fpd_l: u32,
    fpd_r: u32,
    flip: bool,
}
impl StereoFX {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0; 3],
            sample_rate,
            seed,
            iir_a: 0.0,
            iir_b: 0.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            flip: false,
        }
    }
}
impl AirwindowsEffect for StereoFX {
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
            .map_or_else(String::new, |p| float2string(*p))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c] = self.params;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let density = a as f64 * 2.4;
        let sustain = 1.0 - (1.0 / (1.0 + (density / 7.0)));
        let iir_amount = (b as f64).powf(3.0) / overallscale;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let mut mid = l + r;
            let mut side = l - r;
            let mut count = density;
            while count > 1.0 {
                let bridge = (side.abs() * 1.57079633).min(1.57079633).sin();
                side = if side > 0.0 { bridge } else { -bridge };
                count -= 1.0;
            }
            let mut bridge = (side.abs() * 1.57079633).min(1.57079633).sin();
            side = if side > 0.0 {
                (side * (1.0 - count)) + (bridge * count)
            } else {
                (side * (1.0 - count)) - (bridge * count)
            };
            bridge = (1.0 - (side.abs() * 1.57079633).min(1.57079633).cos()) * 3.141592653589793;
            side = if side > 0.0 {
                (side * (1.0 - sustain)) + (bridge * sustain)
            } else {
                (side * (1.0 - sustain)) - (bridge * sustain)
            };
            let offset =
                (0.666666666666666 + ((1.0 - side.abs()) * -0.33333333333333)).clamp(0.0, 1.0);
            let iir = if self.flip {
                &mut self.iir_a
            } else {
                &mut self.iir_b
            };
            *iir = (*iir * (1.0 - (offset * iir_amount))) + (side * (offset * iir_amount));
            side -= *iir;
            bridge = (mid.abs() / 1.273239544735162).min(1.57079633).sin() * 1.273239544735162;
            mid = if mid > 0.0 {
                (mid * (1.0 - c as f64)) + (bridge * c as f64)
            } else {
                (mid * (1.0 - c as f64)) - (bridge * c as f64)
            };
            l = (mid + side) / 2.0;
            r = (mid - side) / 2.0;
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
