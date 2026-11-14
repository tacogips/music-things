//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/TapeDust, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand, RAND_MAX};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Dust",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "TapeDust",
    upstream_name: "TapeDust",
    description: "TapeDust is just a special treble-erode noise, a ‘slew noise’ plugin.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(TapeDust::new(sample_rate, seed))
}
struct TapeDust {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    b: [[f64; 11]; 2],
    fpd_l: u32,
    fpd_r: u32,
    flip: bool,
    rng: AwRand,
}
impl TapeDust {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 1.0],
            sample_rate,
            seed,
            b: [[0.0; 11]; 2],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
            flip: true,
            rng,
        }
    }
}
impl AirwindowsEffect for TapeDust {
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
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let range = (self.params[0] as f64).powf(2.0) * 5.0;
        let fuzz = range * 0.002;
        let offset = range * 0.4 + 1.0;
        let wet = self.params[1] as f64;
        for (((&l, &r), lo), ro) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut dry = [l as f64, r as f64];
            if dry[0].abs() < 1.18e-23 {
                dry[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if dry[1].abs() < 1.18e-23 {
                dry[1] = self.fpd_r as f64 * 1.18e-17;
            }
            self.b[0][0] = dry[0];
            self.b[1][0] = dry[1];
            let random = [
                self.rng.next() as f64 / RAND_MAX as f64,
                self.rng.next() as f64 / RAND_MAX as f64,
            ];
            let mut out = [0.0; 2];
            for ch in 0..2 {
                let depth = random[ch] * range + offset;
                let mut gain = depth;
                let mut sample =
                    random[ch] * ((1.0 - (self.b[ch][0] - self.b[ch][1]).abs()) * fuzz);
                if self.flip {
                    sample = -sample;
                }
                for count in 0..9 {
                    let mut f;
                    if gain > 1.0 {
                        f = 1.0;
                        gain -= 1.0;
                    } else {
                        f = gain;
                        gain = 0.0;
                    }
                    f /= depth;
                    sample += self.b[ch][count] * f;
                }
                if wet < 1.0 {
                    sample = sample * wet + dry[ch] * (1.0 - wet);
                }
                out[ch] = sample;
            }
            self.flip = !self.flip;
            *lo = fpd_dither(out[0], &mut self.fpd_l) as f32;
            *ro = fpd_dither(out[1], &mut self.fpd_r) as f32;
        }
    }
}
