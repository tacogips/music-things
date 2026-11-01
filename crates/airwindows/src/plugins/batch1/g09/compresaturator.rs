//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: plugins/LinuxVST/src/Compresaturator, b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.

use crate::display::float2string;
use crate::dither::{fpd_from_seed, frexpf_exponent, pow2, xorshift};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Drive",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Clamp",
        label: "%",
        default: 0.5,
    },
    ParamInfo {
        name: "Expand",
        label: "sa",
        default: 0.5,
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
    name: "Compresaturator",
    upstream_name: "Compresaturator",
    description: "Compresaturator fades between compressing and soft clipping.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Compresaturator::new(rate, seed))
}

struct Compresaturator {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    d_l: Box<[f32]>,
    d_r: Box<[f32]>,
    count: usize,
    width_l: i32,
    width_r: i32,
    pad_l: f64,
    pad_r: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl Compresaturator {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.5, 0.5, 1.0, 1.0],
            sample_rate,
            seed,
            d_l: vec![0.0; 11000].into_boxed_slice(),
            d_r: vec![0.0; 11000].into_boxed_slice(),
            count: 0,
            width_l: 500,
            width_r: 500,
            pad_l: 0.0,
            pad_r: 0.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn drive(
    mut s: f64,
    pad: &mut f64,
    width: i32,
    sat: f64,
    gain: f64,
    range: i32,
) -> (f64, f64, i32) {
    let mut temp = s;
    let speed = (1.0 + ((*pad / width as f64) * sat)).max(1.0);
    let total = gain / speed;
    if total != 1.0 {
        s *= total;
        if total < 1.0 {
            temp *= total;
        }
    }
    let mut bridge = s.abs();
    let mut target = range;
    if bridge < 0.01 {
        *pad *= 0.9999;
    }
    #[allow(clippy::approx_constant)]
    if bridge > 1.57079633 {
        bridge = 1.57079633;
        target = 8;
    }
    bridge = bridge.sin();
    let mut over = 0.0;
    if s > 0.0 {
        s = bridge;
        over = temp - bridge;
    }
    if s < 0.0 {
        s = -bridge;
        over = -temp - bridge;
    }
    (s, over, target)
}
fn update_pad(d: &[f32], count: usize, pad: &mut f64, width: &mut i32, target: i32, fpd: u32) {
    *pad += d[count] as f64;
    let randy = fpd as f64 / u32::MAX as f64;
    if target as f64 * randy > *width as f64 {
        *width += 1;
    } else {
        *pad -= d[count + *width as usize] as f64;
        if target < *width {
            *width -= 1;
            if *width < 2 {
                *width = 2;
            }
            *pad -= d[count + *width as usize] as f64;
        }
    }
    if *pad < 0.0 {
        *pad = 0.0;
    }
}
fn dither(s: f64, fpd: &mut u32) -> f32 {
    let exponent = frexpf_exponent(s as f32);
    xorshift(fpd);
    (s + (*fpd as i32 as f64) * 5.960464655174751e-36 * pow2(exponent + 62)) as f32
}
impl AirwindowsEffect for Compresaturator {
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
        let p = self.params;
        match i {
            0 => float2string((p[0] as f64 * 24.0 - 12.0) as f32),
            1 => float2string(p[1] * 100.0),
            2 => float2string(p[2] * p[2] * 5000.0),
            3 | 4 => float2string(p[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.d_l.fill(0.0);
        self.d_r.fill(0.0);
        self.count = 0;
        self.width_l = 500;
        self.width_r = 500;
        self.pad_l = 0.0;
        self.pad_r = 0.0;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let _ = self.sample_rate;
        let p = self.params;
        let gain = 10.0_f64.powf(((p[0] as f64 * 24.0) - 12.0) / 20.0);
        let mut sat = p[1] as f64 * 2.0;
        let range = ((p[2] * p[2] * p[2] * 5000.0) as i32).max(50);
        sat += (range as f64 / 3000.0) * sat;
        let output = p[3] as f64;
        let wet = p[4] as f64;
        for (((&l, &r), ol), or) in l.iter().zip(r).zip(ol).zip(or) {
            let mut sl = l as f64;
            let mut sr = r as f64;
            if sl.abs() < 1.18e-23 {
                sl = self.fpd_l as f64 * 1.18e-17;
            }
            if sr.abs() < 1.18e-23 {
                sr = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = sl;
            let dry_r = sr;
            if self.count < 1 || self.count > 5000 {
                self.count = 5000;
            }
            (sl, _, _) = drive(sl, &mut self.pad_l, self.width_l, sat, gain, range);
            let over;
            let target;
            (sr, over, target) = drive(sr, &mut self.pad_r, self.width_r, sat, gain, range);
            let v = (over * sat) as f32;
            self.d_l[self.count] = v;
            self.d_l[self.count + 5000] = v;
            self.d_r[self.count] = v;
            self.d_r[self.count + 5000] = v;
            self.count -= 1;
            update_pad(
                &self.d_l,
                self.count,
                &mut self.pad_l,
                &mut self.width_l,
                target,
                self.fpd_l,
            );
            update_pad(
                &self.d_r,
                self.count,
                &mut self.pad_r,
                &mut self.width_r,
                target,
                self.fpd_r,
            );
            if output < 1.0 {
                sl *= output;
                sr *= output;
            }
            if wet < 1.0 {
                sl = sl * wet + dry_l * (1.0 - wet);
                sr = sr * wet + dry_r * (1.0 - wet);
            }
            *ol = dither(sl, &mut self.fpd_l);
            *or = dither(sr, &mut self.fpd_r);
        }
    }
}
