//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/StereoEnsemble, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Depth",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "FXlevel",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "StereoEnsemble",
    upstream_name: "StereoEnsemble",
    description: "StereoEnsemble is a sort of hyperchorus blast from the past.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(StereoEnsemble::new(sample_rate, seed))
}
const SIZES: [usize; 12] = [
    7491, 7533, 5789, 5747, 4841, 4871, 3119, 3089, 2213, 2223, 1337, 1331,
];
const DELAYS: [f64; 12] = [
    7481.0, 7523.0, 5779.0, 5737.0, 4831.0, 4861.0, 3109.0, 3079.0, 2203.0, 2213.0, 1327.0, 1321.0,
];
const LEVELS: [f64; 12] = [
    0.75, 0.15, 0.15, 0.75, 0.63, 0.27, 0.27, 0.63, 0.48, 0.32, 0.32, 0.48,
];
struct StereoEnsemble {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    buffers: [Box<[f64]>; 12],
    index: [usize; 12],
    refs_l: [f64; 7],
    refs_r: [f64; 7],
    cycle: usize,
    fpd_l: u32,
    fpd_r: u32,
}
impl StereoEnsemble {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 1.0],
            sample_rate,
            seed,
            buffers: std::array::from_fn(|i| vec![0.0; SIZES[i]].into_boxed_slice()),
            index: [1; 12],
            refs_l: [0.0; 7],
            refs_r: [0.0; 7],
            cycle: 0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn interpolate(r: &mut [f64; 7], value: f64, end: usize) {
    match end {
        4 => {
            r[0] = r[4];
            r[2] = (r[0] + value) / 2.0;
            r[1] = (r[0] + r[2]) / 2.0;
            r[3] = (r[2] + value) / 2.0;
            r[4] = value;
        }
        3 => {
            r[0] = r[3];
            r[2] = (r[0] + r[0] + value) / 3.0;
            r[1] = (r[0] + value + value) / 3.0;
            r[3] = value;
        }
        2 => {
            r[0] = r[2];
            r[1] = (r[0] + value) / 2.0;
            r[2] = value;
        }
        _ => r[0] = value,
    }
}
impl AirwindowsEffect for StereoEnsemble {
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
            .map_or_else(String::new, |p| float2string(*p))
    }
    fn reset(&mut self) {
        for b in &mut self.buffers {
            b.fill(0.0);
        }
        self.index = [1; 12];
        self.refs_l = [0.0; 7];
        self.refs_r = [0.0; 7];
        self.cycle = 0;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let end = (scale as usize).clamp(1, 4);
        self.cycle = self.cycle.min(end - 1);
        let factor = 0.66 + (self.params[0] as f64 / 3.0);
        let level = self.params[1] as f64;
        let dry = 1.0 - (level * 0.65);
        let max: [usize; 12] = std::array::from_fn(|i| (DELAYS[i] * factor) as usize);
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            self.cycle += 1;
            if self.cycle == end {
                let mut out_l = 0.0;
                let mut out_r = 0.0;
                for pair in 0..6 {
                    let i = pair * 2;
                    self.buffers[i][self.index[i]] = l;
                    self.index[i] = if self.index[i] == 0 || self.index[i] > max[i] + 1 {
                        max[i]
                    } else {
                        self.index[i] - 1
                    };
                    let mut temp = self.buffers[i][self.index[i]];
                    out_l += temp * LEVELS[i];
                    self.buffers[i + 1][self.index[i + 1]] = r;
                    self.index[i + 1] =
                        if self.index[i + 1] == 0 || self.index[i + 1] > max[i + 1] + 1 {
                            max[i + 1]
                        } else {
                            self.index[i + 1] - 1
                        };
                    temp += self.buffers[i + 1][self.index[i + 1]];
                    out_r += temp * LEVELS[i + 1];
                }
                l = (out_l * level) + (l * dry);
                r = (out_r * level) + (r * dry);
                interpolate(&mut self.refs_l, l, end);
                interpolate(&mut self.refs_r, r, end);
                self.cycle = 0;
            }
            l = self.refs_l[self.cycle];
            r = self.refs_r[self.cycle];
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
