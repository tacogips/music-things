//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/HardVacuum, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::approx_constant)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Drive",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Warmth",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Aura",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "HardVacuum",
    upstream_name: "HardVacuum",
    description: "HardVacuum is tube style saturation effects.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(HardVacuum::new(sample_rate, seed))
}
struct HardVacuum {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    last_l: f64,
    last_r: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl HardVacuum {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.0, 0.0, 1.0, 1.0],
            sample_rate,
            seed,
            last_l: 0.0,
            last_r: 0.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn distort(
    mut input: f64,
    last: &mut f64,
    multistage: f64,
    warmth: f64,
    invwarmth: f64,
    aura: f64,
) -> f64 {
    let mut skew = input - *last;
    *last = input;
    let bridge = skew.abs().min(3.1415926).sin();
    skew = if skew > 0.0 {
        bridge * aura
    } else {
        -bridge * aura
    };
    skew *= input;
    skew *= 1.557079633;
    let mut countdown = multistage;
    while countdown > 0.0 {
        let drive = if countdown > 1.0 {
            1.557079633
        } else {
            countdown * (1.0 + (0.557079633 * invwarmth))
        };
        let positive = drive - warmth;
        let negative = drive + warmth;
        let mut bridge = input.abs();
        bridge += skew;
        if bridge > 1.57079633 {
            bridge = 1.57079633;
        }
        bridge = bridge.sin();
        bridge *= drive;
        bridge += skew;
        if bridge > 1.57079633 {
            bridge = 1.57079633;
        }
        bridge = bridge.sin();
        input = if input > 0.0 {
            (input * (1.0 - positive + skew)) + (bridge * (positive + skew))
        } else {
            (input * (1.0 - negative + skew)) - (bridge * (negative + skew))
        };
        countdown -= 1.0;
    }
    input
}
impl AirwindowsEffect for HardVacuum {
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
            0 => float2string((self.params[0] as f64 * 2.0) as f32),
            1..=4 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let mut multistage = a as f64 * 2.0;
        if multistage > 1.0 {
            multistage *= multistage;
        }
        let mut warmth = b as f64;
        let invwarmth = 1.0 - warmth;
        warmth /= 1.57079633;
        let aura = c as f64 * 3.1415926;
        let out = d as f64;
        let wet = e as f64;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            l = distort(l, &mut self.last_l, multistage, warmth, invwarmth, aura);
            r = distort(r, &mut self.last_r, multistage, warmth, invwarmth, aura);
            if out != 1.0 {
                l *= out;
                r *= out;
            }
            if wet != 1.0 {
                l = (l * wet) + (dry_l * (1.0 - wet));
                r = (r * wet) + (dry_r * (1.0 - wet));
            }
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
