//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PitchNasty, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Note",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Bend",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Grind",
        label: "",
        default: 0.25,
    },
    ParamInfo {
        name: "Feedbck",
        label: "",
        default: 0.0,
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
    name: "PitchNasty",
    upstream_name: "PitchNasty",
    description:
        "PitchNasty is a primitive pitch shifter that also has primitive time-stretch artifacts.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PitchNasty::new(sample_rate, seed))
}
struct PitchNasty {
    params: [f32; 6],
    sample_rate: f32,
    seed: u32,
    l: Box<[f64]>,
    r: Box<[f64]>,
    in_count: i32,
    out_count: f64,
    transition: bool,
    switch_l: f64,
    switch_r: f64,
    feedback_l: f64,
    feedback_r: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl PitchNasty {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 0.0, 0.25, 0.0, 1.0, 1.0],
            sample_rate,
            seed,
            l: vec![0.0; 10002].into_boxed_slice(),
            r: vec![0.0; 10002].into_boxed_slice(),
            in_count: 1,
            out_count: 1.0,
            transition: false,
            switch_l: 0.0,
            switch_r: 0.0,
            feedback_l: 0.0,
            feedback_r: 0.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for PitchNasty {
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
            0 => ((self.params[0] * 24.0) as i32 - 12).to_string(),
            1 => float2string((self.params[1] as f64 * 24.0 - 12.0) as f32),
            2 => ((self.params[2] * 72.0) as i32 - 36).to_string(),
            3..=5 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.l.fill(0.0);
        self.r.fill(0.0);
        self.in_count = 1;
        self.out_count = 1.0;
        self.transition = false;
        self.switch_l = 0.0;
        self.switch_r = 0.0;
        self.feedback_l = 0.0;
        self.feedback_r = 0.0;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d, e, f] = self.params;
        let note = ((a * 24.0) as i32 - 12) as f64;
        let bend = b as f64 * 24.0 - 12.0;
        let speed = 2.0_f64.powf(note / 12.0) * 2.0_f64.powf(bend / 12.0);
        let grind_ref = (1.0 / 261.6) * self.sample_rate as f64;
        let grind_note = 2.0_f64.powf(((c * 72.0) as i32 - 36) as f64 / 12.0);
        let width = (grind_ref / grind_note).min(9990.0);
        let feedback = d as f64;
        let trim = e as f64;
        let wet = f as f64;
        for (((&left, &right), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = left as f64;
            let mut r = right as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            l += self.feedback_l * feedback;
            r += self.feedback_r * feedback;
            if self.feedback_l.abs() > 1.0 {
                l /= self.feedback_l.abs();
            }
            if self.feedback_r.abs() > 1.0 {
                r /= self.feedback_r.abs();
            }
            self.l[self.in_count as usize] = l;
            self.r[self.in_count as usize] = r;
            self.in_count += 1;
            self.out_count += speed;
            if self.out_count > self.in_count as f64 && self.transition {
                self.switch_l = 1.0;
                self.switch_r = 1.0;
                self.transition = false;
            }
            if self.in_count as f64 > width {
                self.in_count = (self.in_count as f64 - width) as i32;
            }
            if self.out_count > width {
                self.out_count -= width;
                self.transition = true;
            }
            let count = self.out_count.floor() as i32;
            let array_width = width.floor() as i32;
            let first = (count
                - if count > array_width {
                    array_width + 1
                } else {
                    0
                }) as usize;
            let next = (count + 1
                - if count + 1 > array_width {
                    array_width + 1
                } else {
                    0
                }) as usize;
            let fraction = self.out_count - self.out_count.floor();
            l = self.l[first] * (1.0 - fraction);
            l += self.l[next] * fraction;
            r = self.r[first] * (1.0 - fraction);
            r += self.r[next] * fraction;
            let idx = self.in_count as usize;
            if self.switch_l > (l - self.l[idx]).abs() * 2.0 {
                self.switch_l = self.switch_l * 0.5 + (l - self.l[idx]).abs();
            }
            if self.switch_r > (r - self.r[idx]).abs() * 2.0 {
                self.switch_r = self.switch_r * 0.5 + (r - self.r[idx]).abs();
            }
            l = self.l[idx] * self.switch_l + l * (1.0 - self.switch_l);
            r = self.r[idx] * self.switch_r + r * (1.0 - self.switch_r);
            self.feedback_l = l;
            self.feedback_r = r;
            if trim != 1.0 {
                l *= trim;
                r *= trim;
            }
            if wet != 1.0 {
                l = l * wet + dry_l * (1.0 - wet);
                r = r * wet + dry_r * (1.0 - wet);
            }
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
