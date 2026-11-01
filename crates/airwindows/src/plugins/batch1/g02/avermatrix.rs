//! AverMatrix, ported from airwindows/plugins/LinuxVST/src/AverMatrix at
//! b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Parity: Exact; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) static INFO: PluginInfo = PluginInfo {
    name: "AverMatrix",
    upstream_name: "AverMatrix",
    description: "AverMatrix turns averaging into a full-featured EQ.",
    params: &[
        ParamInfo {
            name: "Average",
            label: "taps",
            default: 0.0,
        },
        ParamInfo {
            name: "Depth",
            label: "poles",
            default: 0.0,
        },
        ParamInfo {
            name: "Inv/Wet",
            label: "",
            default: 1.0,
        },
    ],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(AverMatrix::new(rate, seed))
}
struct AverMatrix {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    left: [[f64; 11]; 11],
    right: [[f64; 11]; 11],
    fpd_l: u32,
    fpd_r: u32,
}
impl AverMatrix {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.0, 1.0],
            sample_rate,
            seed,
            left: [[0.0; 11]; 11],
            right: [[0.0; 11]; 11],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for AverMatrix {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(index), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, index: usize) -> String {
        match index {
            0 | 1 => float2string((self.params[index] as f64 * 9.0 + 1.0) as f32),
            2 => float2string((self.params[2] as f64 * 2.0 - 1.0) as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let overall = self.params[0] as f64 * 9.0 + 1.0;
        let mut taps = overall;
        let poles = self.params[1] as f64 * 9.0 + 1.0;
        let ylim = poles.floor() as usize + 1;
        let partial = poles - poles.floor();
        let wet = self.params[2] as f64 * 2.0 - 1.0;
        let mut f = [0.0; 11];
        let mut xlim = 1;
        for x in &mut f {
            if taps > 1.0 {
                *x = 1.0;
                taps -= 1.0;
                xlim += 1;
            } else {
                *x = taps;
                taps = 0.0;
            }
        }
        xlim = xlim.min(9);
        for x in &mut f[..xlim] {
            *x /= overall;
        }
        for (((&l, &r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dl = l;
            let dr = r;
            let mut pl = 0.0;
            let mut pr = 0.0;
            for y in 0..ylim {
                for x in (0..=xlim).rev() {
                    self.left[x + 1][y] = self.left[x][y];
                    self.right[x + 1][y] = self.right[x][y];
                }
                self.left[0][y] = l;
                self.right[0][y] = r;
                pl = l;
                pr = r;
                l = 0.0;
                r = 0.0;
                for (x, &weight) in f[..xlim].iter().enumerate() {
                    l += self.left[x][y] * weight;
                    r += self.right[x][y] * weight;
                }
            }
            l = pl * (1.0 - partial) + l * partial;
            r = pr * (1.0 - partial) + r * partial;
            l = l * wet + dl * (1.0 - wet);
            r = r * wet + dr * (1.0 - wet);
            *ol = fpd_dither(l, &mut self.fpd_l) as f32;
            *or = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
