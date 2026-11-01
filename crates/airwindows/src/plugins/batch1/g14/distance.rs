//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Distance, b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs pending.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Distance",
        label: "miles",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Distance",
    upstream_name: "Distance",
    description: "Distance is a sound design or reverb far-away-izer.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Distance::new(sample_rate, seed))
}
struct Distance {
    sample_rate: f32,
    seed: u32,
    params: [f32; 2],
    last: [f64; 2],
    clamp: [f64; 2],
    previous: [f64; 2],
    third: [f64; 2],
    fpd: [u32; 2],
}
impl Distance {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            params: [0.0, 1.0],
            last: [0.0; 2],
            clamp: [0.0; 2],
            previous: [0.0; 2],
            third: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    #[allow(clippy::approx_constant)]
    fn channel(&mut self, c: usize, s: f32, soft: f64, wet: f64) -> f32 {
        let mut x = s as f64;
        if x.abs() < 1.18e-23 {
            x = self.fpd[c] as f64 * 1.18e-17;
        }
        let dry = x;
        x *= soft;
        let lastclamp = self.clamp[c];
        self.clamp[c] = x - self.last[c];
        let mut change = (self.clamp[c] - lastclamp).abs();
        let post = change + soft / 2.0;
        if change > 1.5707963267949 {
            change = 1.5707963267949;
        }
        let mut bridge = 1.0 - change.sin();
        if bridge < 0.0 {
            bridge = 0.0;
        }
        x = self.last[c] + self.clamp[c] * bridge;
        self.last[c] = x;
        x /= soft;
        let thirdfilter = soft / 3.0;
        x += self.third[c] * thirdfilter;
        x /= thirdfilter + 1.0;
        x += self.previous[c] * post;
        x /= post + 1.0;
        self.third[c] = self.previous[c];
        self.previous[c] = x;
        x *= 1.0 + soft / 6.0;
        if wet < 1.0 {
            x = (dry * (1.0 - wet)) + (x * wet);
        }
        fpd_dither(x, &mut self.fpd[c]) as f32
    }
}
impl AirwindowsEffect for Distance {
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
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut overall = 1.0;
        overall /= 44100.0;
        overall *= self.sample_rate as f64;
        let mut soft = ((self.params[0] as f64 * 2.0).powf(3.0) * 12.0) + 0.6;
        soft *= overall;
        let wet = self.params[1] as f64;
        for (((l, r), o_l), o_r) in il.iter().zip(ir).zip(ol).zip(or) {
            *o_l = self.channel(0, *l, soft, wet);
            *o_r = self.channel(1, *r, soft, wet);
        }
    }
}
