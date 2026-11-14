//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ToneSlant, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Voicing",
        label: "taps",
        default: 0.0,
    },
    ParamInfo {
        name: "Highs",
        label: " ",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "ToneSlant",
    upstream_name: "ToneSlant",
    description: "ToneSlant is a super-transparent ‘tilt EQ’ with very low Q.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ToneSlant::new(sample_rate, seed))
}
struct ToneSlant {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    b_l: [f64; 102],
    b_r: [f64; 102],
    f: [f64; 102],
    fpd_l: u32,
    fpd_r: u32,
}
impl ToneSlant {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0; 2],
            sample_rate,
            seed,
            b_l: [0.0; 102],
            b_r: [0.0; 102],
            f: [0.0; 102],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for ToneSlant {
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
            0 => float2string((self.params[0] as f64 * 99.0 + 1.0) as f32),
            1 => float2string((self.params[1] as f64 * 2.0 - 1.0) as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let overallscale = self.params[0] as f64 * 99.0 + 1.0;
        let slant = self.params[1] as f64 * 2.0 - 1.0;
        self.f[0] = 1.0 / overallscale;
        for count in 1..102 {
            if count as f64 <= overallscale {
                self.f[count] = (1.0 - count as f64 / overallscale) / overallscale;
            } else {
                self.b_l[count] = 0.0;
                self.b_r[count] = 0.0;
            }
        }
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            for count in (0..=overallscale as usize).rev() {
                self.b_l[count + 1] = self.b_l[count];
                self.b_r[count + 1] = self.b_r[count];
            }
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            self.b_l[0] = l;
            self.b_r[0] = r;
            let mut acc_l = l * self.f[0];
            let mut acc_r = r * self.f[0];
            let mut count = 1;
            while (count as f64) < overallscale {
                acc_l += self.b_l[count] * self.f[count];
                acc_r += self.b_r[count] * self.f[count];
                count += 1;
            }
            l += (l - acc_l * 2.0) * slant;
            r += (r - acc_r * 2.0) * slant;
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
