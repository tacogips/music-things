//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PhaseNudge, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Phase Nudge",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PhaseNudge",
    upstream_name: "PhaseNudge",
    description: "PhaseNudge is a phase rotator/allpass filter.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PhaseNudge::new(sample_rate, seed))
}
struct PhaseNudge {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    l: [f64; 1503],
    r: [f64; 1503],
    one: i32,
    maxdelay: i32,
    fpd_l: u32,
    fpd_r: u32,
}
impl PhaseNudge {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 1.0],
            sample_rate,
            seed,
            l: [0.0; 1503],
            r: [0.0; 1503],
            one: 1,
            maxdelay: 9001,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for PhaseNudge {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let target = ((self.params[0] as f64).powf(3.0) * 1501.0) as i32;
        let wet = self.params[1] as f64;
        let golden = 0.618_033_988_749_894_9_f64;
        for (((&left, &right), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = left as f64;
            let mut r = right as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            l /= 4.0;
            r /= 4.0;
            l = if l > 0.0 {
                l.abs().sin()
            } else {
                -l.abs().sin()
            };
            r = if r > 0.0 {
                r.abs().sin()
            } else {
                -r.abs().sin()
            };
            if (self.maxdelay - target).abs() > 1500 {
                self.maxdelay = target;
            }
            if self.maxdelay < target {
                self.maxdelay += 1;
                let n = self.maxdelay as usize;
                self.l[n] = (self.l[0] + self.l[n - 1]) / 2.0;
                self.r[n] = (self.r[0] + self.r[n - 1]) / 2.0;
            }
            if self.maxdelay > target {
                self.maxdelay -= 1;
                let n = self.maxdelay as usize;
                self.l[n] = (self.l[0] + self.l[n]) / 2.0;
                self.r[n] = (self.r[0] + self.r[n]) / 2.0;
            }
            let mut idx = self.one - 1;
            if idx < 0 || idx > self.maxdelay {
                idx = self.maxdelay;
            }
            l -= self.l[idx as usize] * golden;
            r -= self.r[idx as usize] * golden;
            self.l[self.one as usize] = l;
            self.r[self.one as usize] = r;
            l *= golden;
            r *= golden;
            self.one -= 1;
            if self.one < 0 || self.one > self.maxdelay {
                self.one = self.maxdelay;
            }
            l += self.l[self.one as usize];
            r += self.r[self.one as usize];
            let bridge_l = 1.0 - l.abs().cos();
            let bridge_r = 1.0 - r.abs().cos();
            if l > 0.0 {
                l -= bridge_l;
            } else {
                l += bridge_l;
            }
            if r > 0.0 {
                r -= bridge_r;
            } else {
                r += bridge_r;
            }
            l *= 4.0;
            r *= 4.0;
            if wet < 1.0 {
                l = dry_l * (1.0 - wet) + l * wet;
                r = dry_r * (1.0 - wet) + r * wet;
            }
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
