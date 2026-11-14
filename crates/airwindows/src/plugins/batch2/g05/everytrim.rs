//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/EveryTrim, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Left",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Right",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Mid",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Side",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Master",
        label: "dB",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "EveryTrim",
    upstream_name: "EveryTrim",
    description: "EveryTrim is Left/Right, Mid/Side, and Master in one plugin.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(EveryTrim::new(sample_rate, seed))
}
struct EveryTrim {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    fpd_l: u32,
    fpd_r: u32,
}
impl EveryTrim {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5; 5],
            sample_rate,
            seed,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for EveryTrim {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(param), Some(value)) = (self.params.get_mut(i), sanitize_param(value)) {
            *param = value;
        }
    }
    fn param_display(&self, i: usize) -> String {
        self.params.get(i).map_or_else(String::new, |p| {
            float2string((*p as f64 * 24.0 - 12.0) as f32)
        })
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let gain = self
            .params
            .map(|p| 10.0_f64.powf((p as f64 * 24.0 - 12.0) / 20.0));
        let leftgain = gain[0] * (gain[4] * 0.5);
        let rightgain = gain[1] * (gain[4] * 0.5);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let mid = (l + r) * gain[2];
            let side = (l - r) * gain[3];
            *out_l = fpd_dither((mid + side) * leftgain, &mut self.fpd_l) as f32;
            *out_r = fpd_dither((mid - side) * rightgain, &mut self.fpd_r) as f32;
        }
    }
}
