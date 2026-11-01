//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Distance2, b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs pending.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Atmosph",
        label: "",
        default: 0.85,
    },
    ParamInfo {
        name: "Darken",
        label: "",
        default: 0.618,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.618,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Distance2",
    upstream_name: "Distance2",
    description: "Distance2 is a versatile space shaper for creating depth.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Distance2::new(rate, seed))
}
struct Distance2 {
    rate: f32,
    seed: u32,
    params: [f32; 3],
    last: [f64; 2],
    third: [f64; 2],
    history: [[f64; 13]; 2],
    fpd: [u32; 2],
}
impl Distance2 {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            rate,
            seed,
            params: [0.85, 0.618, 0.618],
            last: [0.0; 2],
            third: [0.0; 2],
            history: [[0.0; 13]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn channel(
        &mut self,
        c: usize,
        s: f32,
        scale: f64,
        soft: f64,
        filter: f64,
        offset_scale: f64,
        level: f64,
        wet: f64,
    ) -> f32 {
        let mut x = s as f64;
        if x.abs() < 1.18e-23 {
            x = self.fpd[c] as f64 * 1.18e-17;
        }
        let dry = x;
        let offset = offset_scale - (self.last[c] - x);
        x += offset * offset_scale;
        x *= wet;
        x *= soft;
        let thresholds = [
            0.618033988749894,
            0.679837387624884,
            0.747821126387373,
            0.82260323902611,
            0.904863562928721,
            0.995349919221593,
            1.094884911143752,
            1.204373402258128,
            1.32481074248394,
            1.457291816732335,
            1.603020998405568,
            1.763323098246125,
            1.939655408070737,
        ];
        for (last, t) in self.history[c].iter().zip(thresholds) {
            let threshold = t / scale;
            let clamp = x - *last;
            if clamp > threshold {
                x = *last + threshold;
            }
            if -clamp > threshold {
                x = *last - threshold;
            }
        }
        self.history[c].copy_within(0..12, 1);
        self.history[c][0] = dry;
        x *= level;
        x /= soft;
        x -= offset * offset_scale;
        let thirdfilter = filter / 5.0;
        let secondfilter = filter / 3.0;
        x += self.third[c] * thirdfilter;
        x /= thirdfilter + 1.0;
        x += self.last[c] * secondfilter;
        x /= secondfilter + 1.0;
        self.third[c] = self.last[c];
        self.last[c] = x;
        x *= level;
        if wet != 1.0 {
            x = (x * wet) + (dry * (1.0 - wet));
        }
        fpd_dither(x, &mut self.fpd[c]) as f32
    }
}
impl AirwindowsEffect for Distance2 {
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
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.rate as f64;
        let a = self.params[0] as f64;
        let mut soft = (a.powf(3.0) * 24.0) + 0.6;
        soft *= scale;
        let filter = soft * self.params[1] as f64;
        let offset_scale = a * 0.1618;
        let level = 1.0 + ((filter / 12.0) * a);
        let wet = self.params[2] as f64;
        for (((l, r), o_l), o_r) in il.iter().zip(ir).zip(ol).zip(or) {
            *o_l = self.channel(0, *l, scale, soft, filter, offset_scale, level, wet);
            *o_r = self.channel(1, *r, scale, soft, filter, offset_scale, level, wet);
        }
    }
}
