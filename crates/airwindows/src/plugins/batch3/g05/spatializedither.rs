//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/SpatializeDither, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
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
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "SpatializeDither",
    upstream_name: "SpatializeDither",
    description: "SpatializeDither is a high-performance clarity and accuracy dither.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(SpatializeDither::new(sample_rate, seed))
}
struct SpatializeDither {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
    err_l: f64,
    err_r: f64,
    flip: bool,
}
impl SpatializeDither {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.0],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            err_l: 0.0,
            err_r: 0.0,
            flip: false,
        }
    }
}
fn quantize(mut input: f64, fpd: &mut u32, err: &mut f64, flip: bool) -> f64 {
    const RANDY: f64 = 1.618_033_988_749_895;
    const OMEGA: f64 = 0.567_143_290_409_783_8;
    const EXP: f64 = 0.065_988_035_845_312_54;
    if input > 0.0 {
        input += 0.383;
    }
    if input < 0.0 {
        input -= 0.383;
    }
    let mut rnd = *fpd as f64 / u32::MAX as f64;
    xorshift(fpd);
    rnd += (*fpd as f64 / u32::MAX as f64) - 1.0;
    rnd *= RANDY;
    rnd -= *err * OMEGA;
    let abs = input.abs();
    *err = abs - abs.floor();
    let mut contingent = *err * 2.0;
    if contingent > 1.0 {
        contingent = ((-contingent + 2.0) * OMEGA) + EXP;
    } else {
        contingent = contingent * OMEGA + EXP;
    }
    if flip {
        rnd = rnd * (1.0 - contingent) + contingent + 0.5;
    } else {
        rnd = rnd * (1.0 - contingent) - contingent + 0.5;
    }
    input += rnd * contingent;
    input.floor()
}
impl AirwindowsEffect for SpatializeDither {
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
                if self.params[0] as f64 * 1.999 < 1.0 {
                    "CD 16".into()
                } else {
                    "HD 24".into()
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
        let out_scale = scale.max(8.0);
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            l *= scale as f64;
            r *= scale as f64;
            l = quantize(l, &mut self.fpd_l, &mut self.err_l, self.flip);
            r = quantize(r, &mut self.fpd_r, &mut self.err_r, self.flip);
            self.flip = !self.flip;
            l /= out_scale as f64;
            r /= out_scale as f64;
            xorshift(&mut self.fpd_l);
            xorshift(&mut self.fpd_r);
            *outl = l as f32;
            *outr = r as f32;
        }
    }
}
