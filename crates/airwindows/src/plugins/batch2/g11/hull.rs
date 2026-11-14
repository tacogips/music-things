//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Hull, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
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
        default: 0.0,
    },
    ParamInfo {
        name: "Bright",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Hull",
    upstream_name: "Hull",
    description: "Hull is an alternate form of highpass/lowpass filter.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Hull::new(sample_rate, seed))
}
struct Hull {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    b: [Box<[f64]>; 2],
    c: [[f64; 102]; 2],
    bp: usize,
    cp: usize,
    fpd: [u32; 2],
}
impl Hull {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 0.5],
            sample_rate,
            seed,
            b: [
                vec![0.0; 4002].into_boxed_slice(),
                vec![0.0; 4002].into_boxed_slice(),
            ],
            c: [[0.0; 102]; 2],
            bp: 1,
            cp: 1,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Hull {
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
            0 => float2string(self.params[0]),
            1 => float2string(self.params[1] * 2.0 - 1.0),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.b[0].fill(0.0);
        self.b[1].fill(0.0);
        self.c = [[0.0; 102]; 2];
        self.bp = 1;
        self.cp = 1;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let setting =
            (self.params[0] as f64).powf(3.0) * (self.sample_rate as f64 / 96000.0).min(1.0);
        let la = (setting * 2000.0 + 1.0) as usize;
        let lb = (setting * 1000.0 + 1.0) as usize;
        let lc = ((setting * 2000.0).sqrt() + 1.0) as usize;
        let wet = -(self.params[1] as f64 * 2.0 - 1.0);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut sample = [l as f64, r as f64];
            for (s, fpd) in sample.iter_mut().zip(self.fpd) {
                if s.abs() < 1.18e-23 {
                    *s = fpd as f64 * 1.18e-17;
                }
            }
            self.bp = if self.bp == 0 { 1999 } else { self.bp - 1 };
            self.cp = if self.cp == 0 { 49 } else { self.cp - 1 };
            for (ch, s) in sample.iter_mut().enumerate() {
                self.b[ch][self.bp] = *s;
                self.b[ch][self.bp + 2000] = *s;
                let mut long = 0.0;
                let mut short = 0.0;
                for x in self.bp..self.bp + lb {
                    short += self.b[ch][x];
                    long += self.b[ch][x];
                }
                for x in self.bp + lb..self.bp + la {
                    long += self.b[ch][x];
                }
                long *= 1.0 / la as f64;
                short *= 1.0 / lb as f64;
                self.c[ch][self.cp] = short + (short - long);
                self.c[ch][self.cp + 50] = short + (short - long);
                let mut shortest = 0.0;
                for x in self.cp..self.cp + lc {
                    shortest += self.c[ch][x];
                }
                shortest *= 1.0 / lc as f64;
                *s = if wet > 0.0 {
                    shortest * wet + *s * (1.0 - wet)
                } else {
                    (*s - shortest) * wet.abs() + *s * (1.0 + wet)
                };
                *s = fpd_dither(*s, &mut self.fpd[ch]);
            }
            *out_l = sample[0] as f32;
            *out_r = sample[1] as f32;
        }
    }
}
