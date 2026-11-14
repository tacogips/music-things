//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Holt, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Freq",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Reso",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Poles",
        label: "",
        default: 1.0,
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
    name: "Holt",
    upstream_name: "Holt",
    description: "Holt is a synth-like resonant lowpass filter focussed on bass frequencies.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Holt::new(sample_rate, seed))
}
struct Holt {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    previous: [[f64; 4]; 2],
    trend: [[f64; 4]; 2],
    fpd: [u32; 2],
}
impl Holt {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.5, 1.0, 1.0, 1.0],
            sample_rate,
            seed,
            previous: [[0.0; 4]; 2],
            trend: [[0.0; 4]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Holt {
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
        match self.params.get(i) {
            Some(v) if i == 2 => float2string(*v * 4.0),
            Some(v) => float2string(*v),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let a = a as f64;
        let b = b as f64;
        let mut alpha = (a.powf(4.0) + 0.00001).min(1.0);
        let beta = alpha * b.powf(2.0) + 0.00001;
        alpha = (alpha + (1.0 - beta) * a.powf(3.0)).min(1.0);
        let poles = c as f64 * 4.0;
        let stage_wet = [
            poles.clamp(0.0, 1.0),
            (poles - 1.0).clamp(0.0, 1.0),
            (poles - 2.0).clamp(0.0, 1.0),
            (poles - 3.0).clamp(0.0, 1.0),
        ];
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut sample = [l as f64, r as f64];
            for (ch, s) in sample.iter_mut().enumerate() {
                if s.abs() < 1.18e-23 {
                    *s = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *s;
                for (stage, &wet) in stage_wet.iter().enumerate() {
                    if wet > 0.0 {
                        let trend = beta * (*s - self.previous[ch][stage])
                            + (0.999 - beta) * self.trend[ch][stage];
                        let forecast = self.previous[ch][stage] + self.trend[ch][stage];
                        *s = alpha * *s + (0.999 - alpha) * forecast;
                        self.previous[ch][stage] = *s;
                        self.trend[ch][stage] = trend;
                        let bypass = if stage == 0 {
                            dry
                        } else {
                            self.previous[ch][stage - 1]
                        };
                        *s = *s * wet + bypass * (1.0 - wet);
                    }
                }
                if d < 1.0 {
                    *s *= d as f64;
                }
                *s = s.clamp(-1.2533141373155, 1.2533141373155);
                let abs = s.abs();
                *s = (*s * abs).sin() / if abs == 0.0 { 1.0 } else { abs };
                if e < 1.0 {
                    *s = *s * e as f64 + dry * (1.0 - e as f64);
                }
                *s = fpd_dither(*s, &mut self.fpd[ch]);
            }
            *out_l = sample[0] as f32;
            *out_r = sample[1] as f32;
        }
    }
}
