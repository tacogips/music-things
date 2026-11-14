//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Monitoring2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Monitor",
    label: "",
    default: 0.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Monitoring2",
    upstream_name: "Monitoring2",
    description: "Monitoring2 is Monitoring, but with Dark as the wordlength reducer.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Monitoring2::new(rate, seed))
}
struct Monitoring2 {
    param: f32,
    rate: f32,
    seed: u32,
    delay: [Box<[f64]>; 8],
    index: [usize; 4],
    dark: [[f32; 100]; 2],
    last: [f64; 2],
    iir: [[f64; 26]; 2],
    biquad: [[f64; 2]; 2],
    fpd: [u32; 2],
}
impl Monitoring2 {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            param: 0.0,
            rate,
            seed,
            delay: std::array::from_fn(|_| vec![0.0; 1503].into_boxed_slice()),
            index: [1; 4],
            dark: [[0.0; 100]; 2],
            last: [0.0; 2],
            iir: [[0.0; 26]; 2],
            biquad: [[0.0; 2]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    fn allpass(&mut self, input: &mut [f64; 2], stage: usize, max: usize, darkened: bool) {
        let pos = self.index[stage];
        let prev = if pos == 0 || pos - 1 > max {
            max
        } else {
            pos - 1
        };
        for (ch, sample) in input.iter_mut().enumerate() {
            *sample -= self.delay[stage * 2 + ch][prev] * 0.5;
            self.delay[stage * 2 + ch][pos] = *sample;
            *sample *= 0.5;
        }
        let pos = if pos == 0 || pos - 1 > max {
            max
        } else {
            pos - 1
        };
        self.index[stage] = pos;
        for (ch, sample) in input.iter_mut().enumerate() {
            if darkened {
                *sample += self.delay[stage * 2 + ch][pos] * 0.5;
                *sample += self.delay[stage * 2 + ch][if pos == max { 0 } else { pos + 1 }] * 0.5;
            } else {
                *sample += self.delay[stage * 2 + ch][pos];
            }
        }
    }
}
fn dark(sample: f64, history: &mut [f32; 100], depth: usize) -> f64 {
    let qa = sample.floor() as i32;
    let qb = (sample + 1.0).floor() as i32;
    let mut slew = 0.0f32;
    for x in 0..depth {
        slew += history[x + 1] - history[x];
    }
    slew /= depth as f32;
    let a = ((history[0] - qa as f32) - slew).abs();
    let b = ((history[0] - qb as f32) - slew).abs();
    let result = if a < b { qa } else { qb };
    history.copy_within(0..=depth, 1);
    history[0] = result as f32;
    result as f64
}
impl AirwindowsEffect for Monitoring2 {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        if i == 0 {
            self.param
        } else {
            0.0
        }
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if i == 0 {
            if let Some(v) = sanitize_param(v) {
                self.param = v;
            }
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i != 0 {
            return String::new();
        }
        [
            "Out24", "Out16", "Peaks", "Slew", "Subs", "Mono", "Side", "Vinyl", "Aurat", "MonoRat",
            "MonoLat", "Phone", "Cans A", "Cans B", "Cans C", "Cans D", "V Trick",
        ][(self.param as f64 * 16.999) as usize]
            .into()
    }
    fn reset(&mut self) {
        for d in &mut self.delay {
            d.fill(0.0);
        }
        self.index = [1; 4];
        self.dark = [[0.0; 100]; 2];
        self.last = [0.0; 2];
        self.iir = [[0.0; 26]; 2];
        self.biquad = [[0.0; 2]; 2];
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    #[allow(clippy::approx_constant, clippy::excessive_precision)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let scale = self.rate as f64 / 44100.0;
        let depth = ((17.0 * scale) as i32).clamp(3, 98) as usize;
        let mode = (self.param as f64 * 16.999) as usize;
        let maxima = [
            (149.0 * scale) as usize,
            (179.0 * scale) as usize,
            (191.0 * scale) as usize,
            (223.0 * scale) as usize,
        ];
        let (freq, reso) = match mode {
            7 => (0.0385 / scale, 0.0825),
            11 => (0.1245 / scale, 0.46),
            _ => (0.0375 / scale, 0.1575),
        };
        let k = (std::f64::consts::PI * freq).tan();
        let norm = 1.0 / (1.0 + k / reso + k * k);
        let a0 = k / reso * norm;
        let a2 = -a0;
        let b1 = 2.0 * (k * k - 1.0) * norm;
        let b2 = (1.0 - k / reso + k * k) * norm;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut v = [l as f64, r as f64];
            for (ch, sample) in v.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                xorshift(&mut self.fpd[ch]);
            }
            match mode {
                2 => {
                    for max in maxima.iter().enumerate() {
                        for s in &mut v {
                            *s = s.clamp(-1.0, 1.0).asin();
                        }
                        self.allpass(&mut v, max.0, *max.1, false);
                    }
                    for s in &mut v {
                        *s = s.clamp(-1.0, 1.0).asin() * 0.63679;
                    }
                }
                3 => {
                    for (ch, s) in v.iter_mut().enumerate() {
                        let x =
                            ((*s - self.last[ch]) * 2.302585092994045684017991).clamp(-1.0, 1.0);
                        self.last[ch] = *s;
                        *s = x;
                    }
                }
                4 => {
                    let amount = (2250.0 / 44100.0) / scale;
                    let mut gain = 1.42;
                    for s in &mut v {
                        *s *= gain;
                    }
                    gain = ((gain - 1.0) * 0.75) + 1.0;
                    for i in 0..26 {
                        for (ch, s) in v.iter_mut().enumerate() {
                            self.iir[ch][i] = (self.iir[ch][i] * (1.0 - amount)) + (*s * amount);
                            *s = self.iir[ch][i];
                            if i < 25 {
                                *s *= gain;
                            }
                            *s = s.clamp(-1.0, 1.0);
                        }
                        if i < 25 {
                            gain = ((gain - 1.0) * 0.75) + 1.0;
                        }
                    }
                }
                5 | 6 => {
                    let mut mid = v[0] + v[1];
                    let mut side = v[0] - v[1];
                    if mode == 5 {
                        side = 0.0;
                    } else {
                        mid = 0.0;
                    }
                    v = [(mid + side) / 2.0, (mid - side) / 2.0];
                }
                7..=11 => {
                    if mode == 9 {
                        v[1] = (v[0] + v[1]) * 0.5;
                        v[0] = 0.0;
                    }
                    if mode == 10 {
                        v[0] = (v[0] + v[1]) * 0.5;
                        v[1] = 0.0;
                    }
                    if mode == 11 {
                        let m = (v[0] + v[1]) * 0.5;
                        v = [m, m];
                    }
                    for (ch, s) in v.iter_mut().enumerate() {
                        *s = s.sin();
                        let t = *s * a0 + self.biquad[ch][0];
                        self.biquad[ch][0] = -t * b1 + self.biquad[ch][1];
                        self.biquad[ch][1] = *s * a2 - t * b2;
                        *s = t.clamp(-1.0, 1.0).asin();
                    }
                }
                12..=15 => {
                    let vol = [0.855, 0.748, 0.713, 0.680][mode - 12];
                    for s in &mut v {
                        *s = (*s * vol).sin();
                    }
                    let mut dry = v;
                    let bass = (mode * mode) as f64 * 0.00001 / scale;
                    let mid = v[0] + v[1];
                    let mut side = v[0] - v[1];
                    self.iir[0][0] = self.iir[0][0] * (1.0 - bass * 0.618) + side * bass * 0.618;
                    side -= self.iir[0][0];
                    v = [(mid + side) / 2.0, (mid - side) / 2.0];
                    self.allpass(&mut v, 0, maxima[0], true);
                    let cross = [0.125, 0.25, 0.30, 0.35][mode - 12];
                    for s in &mut v {
                        *s *= cross;
                    }
                    dry[0] += v[1];
                    dry[1] += v[0];
                    self.allpass(&mut v, 3, maxima[3], true);
                    for s in &mut v {
                        *s *= 0.25;
                    }
                    dry[0] += v[1];
                    dry[1] += v[0];
                    let mid = dry[0] + dry[1];
                    let mut side = dry[0] - dry[1];
                    self.iir[1][0] = self.iir[1][0] * (1.0 - bass) + side * bass;
                    side -= self.iir[1][0];
                    v = [(mid + side) / 2.0, (mid - side) / 2.0];
                    for s in &mut v {
                        *s = s.clamp(-1.0, 1.0).asin();
                    }
                }
                16 => {
                    let m = (v[0] + v[1]) * 0.5;
                    v = [-m, m];
                }
                _ => {}
            }
            let q = if mode == 1 { 32768.0 } else { 8388608.0 };
            for (ch, s) in v.iter_mut().enumerate() {
                *s = dark(*s * q, &mut self.dark[ch], depth) / q;
            }
            *out_l = v[0] as f32;
            *out_r = v[1] as f32;
        }
    }
}
