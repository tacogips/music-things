//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Hombre, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Voicing",
        label: "",
        default: 0.421,
    },
    ParamInfo {
        name: "Intensity",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Hombre",
    upstream_name: "Hombre",
    description: "Hombre is atmosphere and texture (through very short delays).",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Hombre::new(sample_rate, seed))
}
struct Hombre {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    p: [[f64; 4001]; 2],
    gcount: i32,
    slide: f64,
    fpd: [u32; 2],
}
impl Hombre {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.421, 0.5],
            sample_rate,
            seed,
            p: [[0.0; 4001]; 2],
            gcount: 0,
            slide: 0.421,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Hombre {
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
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let scale = self.sample_rate as f64 / 44100.0;
        let width_a = scale as usize;
        let width_b = (7.0 * scale) as usize;
        let wet = self.params[1] as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut sample = [l as f64, r as f64];
            for (s, fpd) in sample.iter_mut().zip(self.fpd) {
                if s.abs() < 1.18e-23 {
                    *s = fpd as f64 * 1.18e-17;
                }
            }
            let dry = sample;
            self.slide = self.slide * 0.9997 + self.params[0] as f64 * 0.0003;
            let offset_a = (self.slide.powf(2.0) * 77.0 + 3.2) * scale;
            let offset_b = (3.85 * (self.slide.powf(2.0) * 77.0 + 3.2) + 41.0) * scale;
            if self.gcount < 1 || self.gcount > 2000 {
                self.gcount = 2000;
            }
            let count = self.gcount as usize;
            for (ch, s) in sample.iter_mut().enumerate() {
                self.p[ch][count] = *s;
                self.p[ch][count + 2000] = *s;
                let a = count + offset_a.floor() as usize;
                let total_a = self.p[ch][a] * 0.391
                    + self.p[ch][a + width_a]
                    + self.p[ch][a + width_a * 2] * 0.391;
                *s += total_a * 0.274;
                let b = count + offset_b.floor() as usize;
                let total_b = self.p[ch][b] * 0.918
                    + self.p[ch][b + width_b]
                    + self.p[ch][b + width_b * 2] * 0.918;
                *s -= total_b * 0.629;
                *s /= 4.0;
                if wet != 1.0 {
                    *s = *s * wet + dry[ch] * (1.0 - wet);
                }
                *s = fpd_dither(*s, &mut self.fpd[ch]);
            }
            self.gcount -= 1;
            *out_l = sample[0] as f32;
            *out_r = sample[1] as f32;
        }
    }
}
