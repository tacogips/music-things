//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Baxandall2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Treble",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Bass",
        label: "dB",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Baxandall2",
    upstream_name: "Baxandall2",
    description: "Baxandall2 is Baxandall, extended and made more powerful.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Baxandall2::new(rate, seed))
}
struct Baxandall2 {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    treble: [[[f64; 9]; 2]; 2],
    bass: [[[f64; 9]; 2]; 2],
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl Baxandall2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5; 2],
            sample_rate,
            seed,
            treble: [[[0.0; 9]; 2]; 2],
            bass: [[[0.0; 9]; 2]; 2],
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn coefficients(f: &mut [[[f64; 9]; 2]; 2], freq: f64, q: f64) {
    let k = (std::f64::consts::PI * freq).tan();
    let norm = 1.0 / (1.0 + k / q + k * k);
    let c2 = k * k * norm;
    let c3 = 2.0 * c2;
    let c5 = 2.0 * (k * k - 1.0) * norm;
    let c6 = (1.0 - k / q + k * k) * norm;
    for channel in f {
        for a in channel {
            a[0] = freq;
            a[1] = q;
            a[2] = c2;
            a[3] = c3;
            a[4] = c2;
            a[5] = c5;
            a[6] = c6;
        }
    }
}
fn lowpass(a: &mut [f64; 9], input: f64) -> f64 {
    let sample = input * a[2] + a[7];
    a[7] = input * a[3] - sample * a[5] + a[8];
    a[8] = input * a[4] - sample * a[6];
    sample
}
impl AirwindowsEffect for Baxandall2 {
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
            Some(v) => float2string((*v as f64 * 48.0 - 24.0) as f32),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let a = self.params[0] as f64;
        let b = self.params[1] as f64;
        let tg = 10.0_f64.powf((a * 48.0 - 24.0) / 20.0);
        let tf = ((4410.0 * tg) / self.sample_rate as f64).min(0.45);
        let bg = 10.0_f64.powf((b * 48.0 - 24.0) / 20.0);
        let bf = ((8820.0 * 10.0_f64.powf(-(b * 48.0 - 24.0) / 20.0)) / self.sample_rate as f64)
            .min(0.45);
        coefficients(&mut self.treble, tf, 0.4);
        coefficients(&mut self.bass, bf, 0.2);
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let slot = usize::from(!self.flip);
            let tl = l - lowpass(&mut self.treble[0][slot], l);
            let bl = lowpass(&mut self.bass[0][slot], l);
            let tr = r - lowpass(&mut self.treble[1][slot], r);
            let br = lowpass(&mut self.bass[1][slot], r);
            self.flip = !self.flip;
            l = bl * bg + tl * tg;
            r = br * bg + tr * tg;
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
