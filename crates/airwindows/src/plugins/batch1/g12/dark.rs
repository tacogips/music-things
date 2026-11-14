//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: plugins/LinuxVST/src/Dark, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Dark",
    upstream_name: "Dark",
    description: "Dark is a wordlength reducer that gives your music a blacker backdrop.",
    params: &[
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
    ],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Dark::new(sample_rate, seed))
}
struct Dark {
    sample_rate: f32,
    seed: u32,
    params: [f32; 2],
    last_l: [f32; 100],
    last_r: [f32; 100],
    fpd_l: u32,
    fpd_r: u32,
}
impl Dark {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rand = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rand);
        replay_fpd_seed_loop(&mut rand);
        Self {
            sample_rate,
            seed,
            params: [1.0, 0.0],
            last_l: [0.0; 100],
            last_r: [0.0; 100],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
    fn quantize(input: f64, history: &mut [f32; 100], depth: usize) -> f64 {
        let a = input.floor() as i32;
        let b = (input + 1.0).floor() as i32;
        let mut slew = 0.0_f32;
        for x in 0..depth {
            slew += history[x + 1] - history[x];
        }
        slew /= depth as f32;
        let test_a = ((history[0] - a as f32) - slew).abs();
        let test_b = ((history[0] - b as f32) - slew).abs();
        let value = if test_a < test_b { a } else { b };
        for x in (0..=depth).rev() {
            history[x + 1] = history[x];
        }
        history[0] = value as f32;
        f64::from(value)
    }
}
impl AirwindowsEffect for Dark {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(index), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, index: usize) -> String {
        match index {
            0 => {
                if (f64::from(self.params[0]) * 1.999) as i32 == 1 {
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
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let depth =
            ((17.0 * (1.0 / 44100.0 * f64::from(self.sample_rate))) as i32).clamp(3, 98) as usize;
        let mut scale: f32 = if (f64::from(self.params[0]) * 1.999) as i32 == 1 {
            8388608.0
        } else {
            32768.0
        };
        if self.params[1] > 0.0 {
            scale = (f64::from(scale) * (1.0 - f64::from(self.params[1])).powf(6.0)) as f32;
        }
        if scale < 0.0001 {
            scale = 0.0001;
        }
        let output_scale = scale.max(8.0);
        for (((l, r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            let mut l = f64::from(*l);
            let mut r = f64::from(*r);
            if l.abs() < 1.18e-23 {
                l = f64::from(self.fpd_l) * 1.18e-17;
            }
            xorshift(&mut self.fpd_l);
            if r.abs() < 1.18e-23 {
                r = f64::from(self.fpd_r) * 1.18e-17;
            }
            xorshift(&mut self.fpd_r);
            l *= f64::from(scale);
            r *= f64::from(scale);
            *ol = (Self::quantize(l, &mut self.last_l, depth) / f64::from(output_scale)) as f32;
            *or = (Self::quantize(r, &mut self.last_r, depth) / f64::from(output_scale)) as f32;
        }
    }
}
