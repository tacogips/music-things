//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Beam, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Quant",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Focus",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "DeRez",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Beam",
    upstream_name: "Beam",
    description: "Beam is a wordlength reducer that tries to heighten sonority.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Beam::new(sample_rate, seed))
}
struct Beam {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    l: [f32; 100],
    r: [f32; 100],
    fpd_l: u32,
    fpd_r: u32,
}
impl Beam {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 0.5, 0.0],
            sample_rate,
            seed,
            l: [0.0; 100],
            r: [0.0; 100],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn quantize(input: f64, history: &mut [f32; 100], depth: usize, sonority: f32) -> f64 {
    let qa = input.floor() as i32;
    let qb = (input + 1.0).floor() as i32;
    let mut a = 0.0f32;
    for x in 0..depth {
        a += history[x + 1] - history[x];
    }
    let mut b = a;
    a += history[0] - qa as f32;
    b += history[0] - qb as f32;
    let mut clamp = sonority;
    if input.abs() < sonority as f64 {
        clamp = input.abs() as f32;
    }
    let ta = (a.abs() - clamp).abs();
    let tb = (b.abs() - clamp).abs();
    let result = if ta < tb { qa } else { qb };
    history.copy_within(0..=depth, 1);
    history[0] = result as f32;
    result as f64
}
impl AirwindowsEffect for Beam {
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
            1 | 2 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c] = self.params;
        let sonority = (b as f64 * 1.618_033_988_749_895) as f32;
        let depth =
            ((17.0 * (1.0 / 44100.0 * self.sample_rate as f64)) as i32).clamp(3, 98) as usize;
        let mut scale = if (a as f64 * 1.999) as i32 == 1 {
            8388608.0f32
        } else {
            32768.0f32
        };
        if c > 0.0 {
            scale = (scale as f64 * (1.0 - c as f64).powf(6.0)) as f32;
        }
        if scale < 0.0001 {
            scale = 0.0001;
        }
        let outscale = scale.max(8.0);
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
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
            *outl = (quantize(l, &mut self.l, depth, sonority) / outscale as f64) as f32;
            *outr = (quantize(r, &mut self.r, depth, sonority) / outscale as f64) as f32;
        }
    }
}
