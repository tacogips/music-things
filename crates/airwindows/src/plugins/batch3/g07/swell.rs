//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Swell, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Thresh",
        label: "",
        default: 0.9,
    },
    ParamInfo {
        name: "Swell",
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
    name: "Swell",
    upstream_name: "Swell",
    description: "Swell is Dial-an-attack, like sidechaining.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Swell::new(sample_rate, seed))
}
struct Swell {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    swell_l: f64,
    swell_r: f64,
    louder_l: bool,
    louder_r: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl Swell {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.9, 0.5, 1.0],
            sample_rate,
            seed,
            swell_l: 0.0,
            swell_r: 0.0,
            louder_l: false,
            louder_r: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Swell {
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
            .map_or_else(String::new, |&v| float2string(v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let [a, b, c] = self.params;
        let threshold_on = (a as f64).powf(2.0) * b as f64;
        let speed_on = ((b as f64).powf(2.0) * 0.001) / overallscale;
        let threshold_off = threshold_on * b as f64;
        let speed_off = (b.sin() as f64 * 0.01) / overallscale;
        let wet = c as f64;
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
            if l.abs() > threshold_on && !self.louder_l {
                self.louder_l = true;
            }
            if l.abs() < threshold_off && self.louder_l {
                self.louder_l = false;
            }
            if self.louder_l {
                self.swell_l = (self.swell_l * (1.0 - speed_on)) + speed_on;
            } else {
                self.swell_l *= 1.0 - speed_off;
            }
            l *= self.swell_l;
            if r.abs() > threshold_on && !self.louder_r {
                self.louder_r = true;
            }
            if r.abs() < threshold_off && self.louder_r {
                self.louder_r = false;
            }
            if self.louder_r {
                self.swell_r = (self.swell_r * (1.0 - speed_on)) + speed_on;
            } else {
                self.swell_r *= 1.0 - speed_off;
            }
            r *= self.swell_r;
            if wet != 1.0 {
                l = (l * wet) + (dry_l * (1.0 - wet));
                r = (r * wet) + (dry_r * (1.0 - wet));
            }
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
