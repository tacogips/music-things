//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/SurgeTide, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "SurgeNode",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "SurgeRate",
        label: " ",
        default: 0.3,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "SurgeTide",
    upstream_name: "SurgeTide",
    description: "SurgeTide is a surge and flow dynamics plugin.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(SurgeTide::new(sample_rate, seed))
}
struct SurgeTide {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    chase_a: f64,
    chase_b: f64,
    chase_c: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl SurgeTide {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.3, 1.0],
            sample_rate,
            seed,
            chase_a: 0.0,
            chase_b: 0.0,
            chase_c: 0.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for SurgeTide {
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
    fn param_label(&self, i: usize) -> &'static str {
        if i < 3 {
            " "
        } else {
            ""
        }
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
        let intensity = self.params[0] as f64;
        let attack = ((self.params[1] as f64 + 0.1) * 0.0005) / overallscale;
        let decay = ((self.params[1] as f64 + 0.001) * 0.00005) / overallscale;
        let wet = self.params[2] as f64;
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
            l *= 8.0;
            r *= 8.0;
            l *= intensity;
            r *= intensity;
            let sense = l.abs().max(r.abs());
            if self.chase_c < sense {
                self.chase_a += attack;
            }
            if self.chase_c > sense {
                self.chase_a -= decay;
            }
            if self.chase_a > decay {
                self.chase_a = decay;
            }
            if self.chase_a < -attack {
                self.chase_a = -attack;
            }
            self.chase_b += self.chase_a / overallscale;
            if self.chase_b > decay {
                self.chase_b = decay;
            }
            if self.chase_b < -attack {
                self.chase_b = -attack;
            }
            self.chase_c += self.chase_b / overallscale;
            self.chase_c = self.chase_c.clamp(0.0, 1.0);
            l *= self.chase_c;
            l = dry_l - (l * intensity);
            l = (dry_l * (1.0 - wet)) + (l * wet);
            r *= self.chase_c;
            r = dry_r - (r * intensity);
            r = (dry_r * (1.0 - wet)) + (r * wet);
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
