//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Monitoring3, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
use super::{dark, Monitoring3};
use crate::dither::xorshift;

impl Monitoring3 {
    #[allow(clippy::approx_constant, clippy::excessive_precision)]
    pub(super) fn process_modes(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let scale = (1.0 / 44100.0) * self.rate as f64;
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
                let sign = if self.flip { 1.0 } else { -1.0 };
                self.odd[ch] = self.odd[ch] * 0.9999999999 + sign * self.previous_shape[ch];
                self.even[ch] = self.even[ch] * 0.9999999999 - sign * self.previous_shape[ch];
                let correction = if self.flip {
                    self.odd[ch]
                } else {
                    self.even[ch]
                };
                let shaped = *s * q + correction;
                let quantized = dark(shaped, &mut self.dark[ch], depth);
                self.previous_shape[ch] = (shaped.floor() - quantized) * 0.9999999999;
                *s = quantized / q;
            }
            self.flip = !self.flip;
            *out_l = v[0] as f32;
            *out_r = v[1] as f32;
        }
    }
}
