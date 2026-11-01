/* Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
 * Upstream: airwindows/plugins/LinuxVST/src/Cabs/Cabs.h
 * Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
 * Parity: Libm; measured max-abs 0.0.
 */
#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]
use super::cabs_coeff::TABLES;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
static PARAMS: [ParamInfo; 6] = [
    ParamInfo {
        name: "CabType",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Intense",
        label: "",
        default: 0.66,
    },
    ParamInfo {
        name: "Rm Loud",
        label: "",
        default: 0.33,
    },
    ParamInfo {
        name: "CabSize",
        label: "",
        default: 0.66,
    },
    ParamInfo {
        name: "OffAxis",
        label: "",
        default: 0.33,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 0.5,
    },
];
static INFO: PluginInfo = PluginInfo {
    name: "Cabs",
    upstream_name: "Cabs",
    description: "Cabs is an Airwindows guitar speaker cabinet simulator.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Cabs::new(rate, seed))
}
#[derive(Clone, Copy)]
struct Side {
    history: [f64; 90],
    sag: [f64; 21],
    control: f64,
    prev: [f64; 3],
    slew: [f64; 5],
    bump: [f64; 4],
    aa: [f64; 2],
    prev_diff: f64,
    refs: [f64; 6],
}
impl Side {
    fn new() -> Self {
        Self {
            history: [0.0; 90],
            sag: [0.0; 21],
            control: 0.0,
            prev: [0.0; 3],
            slew: [0.0; 5],
            bump: [0.0; 4],
            aa: [0.0; 2],
            prev_diff: 0.0,
            refs: [0.0; 6],
        }
    }
}
struct Cabs {
    params: [f32; 6],
    rate: f32,
    seed: u32,
    fpd: [u32; 2],
    side: [Side; 2],
    gcount: i32,
    cycle: usize,
    flip: bool,
}
impl Cabs {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.66, 0.33, 0.66, 0.33, 0.5],
            rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            side: [Side::new(); 2],
            gcount: 0,
            cycle: 0,
            flip: false,
        }
    }
    fn slew(x: f64, last: &mut f64, up: f64, down: f64) -> f64 {
        let delta = x - *last;
        let mut v = x;
        if delta > up {
            v = *last + up;
        }
        if -delta > down {
            v = *last - down;
        }
        *last = v;
        v
    }
    fn bump(s: &mut Side, x: f64, idx: usize, freq: f64, dc: f64) -> f64 {
        s.bump[idx] += x * freq;
        s.bump[idx] -= s.bump[idx] * s.bump[idx] * s.bump[idx] * freq;
        if s.bump[idx] > 0.0 {
            s.bump[idx] -= dc;
        }
        if s.bump[idx] < 0.0 {
            s.bump[idx] += dc;
        }
        if s.bump[idx].abs() > 100.0 {
            s.bump = [0.0; 4];
        }
        s.bump[idx]
    }
    fn anti(s: &mut Side, x: f64, flip: bool) -> f64 {
        if flip {
            s.aa[0] *= 0.94;
            s.aa[1] *= 0.94;
            s.aa[0] += x;
            s.aa[1] -= x;
            s.aa[0] * 0.94
        } else {
            s.aa[1] *= 0.94;
            s.aa[0] *= 0.94;
            s.aa[1] += x;
            s.aa[0] -= x;
            s.aa[1] * 0.94
        }
    }
}
impl AirwindowsEffect for Cabs {
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
        if i == 0 {
            [
                "HPStack", "Vintage", "Boutiqe", "Large C", "Small C", "BassAmp",
            ][(self.params[0] as f64 * 5.999) as usize]
                .to_owned()
        } else {
            self.params
                .get(i)
                .map_or_else(String::new, |v| crate::display::float2string(*v))
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let overall = (1.0 / 44100.0) * self.rate as f64;
        let end = (overall.floor() as usize).clamp(1, 4);
        self.cycle = self.cycle.min(end - 1);
        let speaker = (self.params[0] as f64 * 5.999).floor() as usize;
        let color = (self.params[1] as f64).powf(4.0);
        let boost = 1.0 + color * 4.0;
        let drygain = 1.0 - color;
        let threshold = (1.0 - self.params[2] as f64).powf(5.0) + 0.021;
        let rare = threshold.cbrt();
        let post = rare.sqrt();
        let postrare = post.cbrt();
        let posttrim = postrare.sqrt();
        let freq = 0.0298 + (1.0 - self.params[3] as f64) / 8.0;
        let pad = 0.12 + freq * 12.0;
        let dc = freq.powf(2.0) / 8.0;
        let heavy = (self.params[4] as f64).powf(3.0);
        let output = (self.params[5] as f64).powf(2.0);
        let dynamic = 5.0 * (self.params[1] as f64).powf(2.0) * (self.params[2] as f64).powf(2.0);
        let offset = 4 + (self.params[3] as f64 * 5.0) as usize;
        for (((l, r), a), b) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut x = [*l as f64, *r as f64];
            for ch in 0..2 {
                if x[ch].abs() < 1.18e-23 {
                    x[ch] = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            self.cycle += 1;
            if self.cycle == end {
                if self.gcount < 0 || self.gcount > 10 {
                    self.gcount = 10;
                }
                let g = self.gcount as usize;
                for ch in 0..2 {
                    let s = &mut self.side[ch];
                    let dry = x[ch];
                    let mut half = (x[ch] + s.prev[0] + ((-s.prev[1] + s.prev[2]) * 0.05)) / 2.0;
                    let halfdry = half;
                    s.prev[2] = s.prev[1];
                    s.prev[1] = s.prev[0];
                    s.prev[0] = x[ch];
                    let mut raw = Self::slew(x[ch], &mut s.slew[0], threshold, rare);
                    half = Self::slew(half, &mut s.slew[1], threshold, rare);
                    half -= raw;
                    s.sag[g] = raw.abs();
                    s.sag[g + 10] = s.sag[g];
                    s.control += s.sag[g] / offset as f64;
                    s.control -= s.sag[g + offset] / offset as f64;
                    s.control -= 0.0001;
                    s.control = s.control.clamp(0.0, 13.0);
                    let apply = s.control / offset as f64 * dynamic;
                    s.history.copy_within(0..82, 1);
                    s.history[0] = raw;
                    let mut temp = 0.0;
                    for &(i, c, d, add, plus) in TABLES[speaker] {
                        let coeff = if plus { c + d * apply } else { c - d * apply };
                        let v = s.history[i] * coeff;
                        if add {
                            temp += v
                        } else {
                            temp -= v
                        }
                    }
                    raw *= drygain;
                    raw += temp * color;
                    raw /= boost;
                    half += raw;
                    raw = Self::slew(raw, &mut s.slew[2], threshold, rare);
                    half = Self::slew(half, &mut s.slew[3], threshold, rare);
                    let index = if self.flip { 0 } else { 1 };
                    let h = Self::bump(s, raw, index, freq, dc) / pad;
                    raw = raw * (1.0 - heavy) + h * heavy;
                    let h = Self::bump(s, half, index + 2, freq, dc) / pad;
                    half = half * (1.0 - heavy) + h * heavy;
                    let halfdiff = Self::anti(s, half - halfdry, self.flip);
                    let diff = Self::anti(s, raw - dry, self.flip);
                    let mut out = dry;
                    out += (diff + halfdiff + s.prev_diff) / 1.0;
                    s.prev_diff = diff / 2.0;
                    out = Self::slew(out, &mut s.slew[4], post, postrare);
                    out /= posttrim;
                    out *= output;
                    let refs = &mut s.refs;
                    match end {
                        4 => {
                            refs[0] = refs[4];
                            refs[2] = (refs[0] + out) / 2.0;
                            refs[1] = (refs[0] + refs[2]) / 2.0;
                            refs[3] = (refs[2] + out) / 2.0;
                            refs[4] = out;
                        }
                        3 => {
                            refs[0] = refs[3];
                            refs[2] = (refs[0] + refs[0] + out) / 3.0;
                            refs[1] = (refs[0] + out + out) / 3.0;
                            refs[3] = out;
                        }
                        2 => {
                            refs[0] = refs[2];
                            refs[1] = (refs[0] + out) / 2.0;
                            refs[2] = out;
                        }
                        _ => refs[0] = out,
                    }
                    x[ch] = refs[0];
                }
                self.gcount -= 1;
                self.flip = !self.flip;
                self.cycle = 0;
            } else {
                x = [self.side[0].refs[self.cycle], self.side[1].refs[self.cycle]];
            }
            *a = fpd_dither(x[0], &mut self.fpd[0]) as f32;
            *b = fpd_dither(x[1], &mut self.fpd[1]) as f32;
        }
    }
}
