//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/TapeDelay, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Dry",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Wet",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Delay",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Feedbak",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Lean/Ft",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Depth",
        label: "taps",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "TapeDelay",
    upstream_name: "TapeDelay",
    description: "TapeDelay is an old school tape echo with pitch swerve effects.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(TapeDelay::new(sample_rate, seed))
}
struct TapeDelay {
    params: [f32; 6],
    sample_rate: f32,
    seed: u32,
    p_l: [i32; 258],
    p_r: [i32; 258],
    d_l: Box<[f64]>,
    d_r: Box<[f64]>,
    gcount: i32,
    delay: i32,
    maxdelay: i32,
    chase: i32,
    fpd_l: u32,
    fpd_r: u32,
}
impl TapeDelay {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [1.0, 0.0, 0.5, 0.0, 1.0, 0.0],
            sample_rate,
            seed,
            p_l: [0; 258],
            p_r: [0; 258],
            d_l: vec![0.0; 44100].into_boxed_slice(),
            d_r: vec![0.0; 44100].into_boxed_slice(),
            gcount: 0,
            delay: 0,
            maxdelay: 0,
            chase: 0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for TapeDelay {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        match i {
            0..=3 => float2string(self.params[i]),
            4 => float2string((self.params[4] as f64 * 2.0 - 1.0) as f32),
            5 => float2string((self.params[5] as f64 * 29.0 + 3.0).floor() as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.p_l.fill(0);
        self.p_r.fill(0);
        self.d_l.fill(0.0);
        self.d_r.fill(0.0);
        self.gcount = 0;
        self.delay = 0;
        self.maxdelay = 0;
        self.chase = 0;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let _ = self.sample_rate;
        let [a, b, c, d, e, f] = self.params;
        let dry = (a as f64).powf(2.0);
        let wet = (b as f64).powf(2.0);
        let target = (44000.0 * c) as i32;
        let feedback = d as f64 * 1.3;
        let lean = e as f64 * 2.0 - 1.0;
        let fatwet = lean.abs();
        let fatness = (f as f64 * 29.0 + 3.0).floor() as usize;
        const TAPS: [usize; 32] = [
            1, 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
            83, 89, 97, 101, 103, 107, 109, 113, 127,
        ];
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            if self.gcount < 0 || self.gcount > 128 {
                self.gcount = 128;
            }
            let count = self.gcount as usize;
            if self.delay < 0 || self.delay > self.maxdelay {
                self.delay = self.maxdelay;
            }
            let idx = self.delay as usize;
            let sum_l = l + self.d_l[idx] * feedback;
            let sum_r = r + self.d_r[idx] * feedback;
            let mut total_l = (sum_l * 8388608.0) as i32;
            let mut total_r = (sum_r * 8388608.0) as i32;
            self.p_l[count] = total_l;
            self.p_l[count + 128] = total_l;
            self.p_r[count] = total_r;
            self.p_r[count + 128] = total_r;
            for tap in TAPS[..fatness].iter().rev() {
                total_l = total_l.wrapping_add(self.p_l[count + tap]);
                total_r = total_r.wrapping_add(self.p_r[count + tap]);
            }
            let mut fl = (total_l / fatness as i32 + 1) as f64;
            let mut fr = (total_r / fatness as i32 + 1) as f64;
            fl /= 8388608.0;
            fr /= 8388608.0;
            fl *= fatwet;
            fr *= fatwet;
            let (store_l, store_r) = if lean < 0.0 {
                (sum_l - fl, sum_r - fr)
            } else {
                (sum_l * (1.0 - fatwet) + fl, sum_r * (1.0 - fatwet) + fr)
            };
            self.chase += (self.maxdelay - target).abs();
            if self.chase > 9000 {
                if self.maxdelay > target {
                    self.d_l[self.delay as usize] = store_l;
                    self.d_r[self.delay as usize] = store_r;
                    self.maxdelay -= 1;
                    self.delay -= 1;
                    if self.delay < 0 {
                        self.delay = self.maxdelay;
                    }
                    self.d_l[self.delay as usize] = store_l;
                    self.d_r[self.delay as usize] = store_r;
                }
                if self.maxdelay < target {
                    self.maxdelay += 1;
                    self.delay += 1;
                    if self.delay > self.maxdelay {
                        self.delay = 0;
                    }
                    self.d_l[self.delay as usize] = store_l;
                    self.d_r[self.delay as usize] = store_r;
                }
                self.chase = 0;
            } else {
                self.d_l[self.delay as usize] = store_l;
                self.d_r[self.delay as usize] = store_r;
            }
            self.gcount -= 1;
            self.delay -= 1;
            if self.delay < 0 || self.delay > self.maxdelay {
                self.delay = self.maxdelay;
            }
            l = l * dry + self.d_l[self.delay as usize] * wet;
            r = r * dry + self.d_r[self.delay as usize] * wet;
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
