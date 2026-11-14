//! Dynamic wavelet pitch tracking, ported from dywapitchtrack 0.1.0.1.
//! D3: the sample rate is explicit; at 44100 Hz this matches upstream.
//! The streaming API requires a nonzero hop no greater than the window.
//!
//! Copyright (c) 2010 Antoine Schmitt
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in
//! all copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
//! THE SOFTWARE.
//!
//! Source: dywapitchtrack-0.1.0.1/cbits/dywapitchtrack.c.

use crate::ConfigError;

/// Stateful wavelet pitch detector with preallocated streaming and scratch storage.
pub struct PitchTracker {
    sample_rate: f64,
    window: usize,
    hop: usize,
    ring: Vec<f32>,
    write: usize,
    filled: usize,
    since: usize,
    sam: Vec<f64>,
    distances: Vec<i32>,
    mins: Vec<usize>,
    maxs: Vec<usize>,
    prev_pitch: f64,
    confidence: i32,
}

impl PitchTracker {
    /// Construct a tracker. A window must have at least 1024 samples.
    pub fn new(sample_rate: f32, window: usize, hop: usize) -> Result<Self, ConfigError> {
        if !window.is_power_of_two() {
            return Err(ConfigError::NotPowerOfTwo {
                what: "window",
                value: window,
            });
        }
        if !(1024..=i32::MAX as usize).contains(&window) {
            return Err(ConfigError::OutOfRange {
                what: "window",
                value: window,
                min: 1024,
                max: i32::MAX as usize,
            });
        }
        if hop == 0 || hop > window {
            return Err(ConfigError::InvalidValue { what: "hop" });
        }
        if !sample_rate.is_finite() || sample_rate <= 0.0 {
            return Err(ConfigError::InvalidValue {
                what: "sample_rate",
            });
        }
        Ok(Self {
            sample_rate: f64::from(sample_rate),
            window,
            hop,
            ring: vec![0.0; window],
            write: 0,
            filled: 0,
            since: 0,
            sam: vec![0.0; window],
            distances: vec![0; window],
            mins: vec![0; window],
            maxs: vec![0; window],
            prev_pitch: -1.0,
            confidence: -1,
        })
    }
    /// Compute pitch from the first window of samples. Panics if the slice is shorter.
    pub fn compute(&mut self, samples: &[f32]) -> f64 {
        for (dst, src) in self.sam.iter_mut().zip(&samples[..self.window]) {
            *dst = f64::from(*src);
        }
        self.finish()
    }
    /// Process input, returning the latest completed window's pitch, if any.
    pub fn push(&mut self, input: &[f32]) -> Option<f64> {
        let mut result = None;
        for &sample in input {
            self.ring[self.write] = sample;
            self.write = (self.write + 1) % self.window;
            if self.filled < self.window {
                self.filled += 1;
                if self.filled != self.window {
                    continue;
                }
            } else {
                self.since += 1;
                if self.since < self.hop {
                    continue;
                }
            }
            self.since = 0;
            for i in 0..self.window {
                self.sam[i] = f64::from(self.ring[(self.write + i) % self.window]);
            }
            result = Some(self.finish());
        }
        result
    }
    /// Reset tracking and streaming state.
    pub fn reset(&mut self) {
        self.prev_pitch = -1.0;
        self.confidence = -1;
        self.write = 0;
        self.filled = 0;
        self.since = 0;
    }
    fn finish(&mut self) -> f64 {
        let pitch = self.wavelet();
        self.dynamic(pitch)
    }
    fn wavelet(&mut self) -> f64 {
        let n = self.window;
        let (mut dc, mut max, mut min) = (0.0, 0.0_f64, 0.0_f64);
        for &s in &self.sam {
            dc += s;
            if s > max {
                max = s;
            }
            if s < min {
                min = s;
            }
        }
        dc /= n as f64;
        max -= dc;
        min -= dc;
        let threshold = max.max(-min) * 0.75;
        let mut count = n;
        let mut mode = -1.0;
        for level in 0..6 {
            let delta = (self.sample_rate / ((1 << level) as f64 * 3000.0)) as i32;
            if count < 2 {
                break;
            }
            let (mut previous, mut last_min, mut last_max) = (-1000.0, -1000000, -1000000);
            let (mut find_min, mut find_max) = (false, false);
            let (mut nm, mut nx) = (0, 0);
            for i in 2..count {
                let s = self.sam[i] - dc;
                let s1 = self.sam[i - 1] - dc;
                if s1 <= 0.0 && s > 0.0 {
                    find_max = true;
                }
                if s1 >= 0.0 && s < 0.0 {
                    find_min = true;
                }
                let dv = s - s1;
                if previous > -1000.0 {
                    if find_min
                        && previous < 0.0
                        && dv >= 0.0
                        && s.abs() >= threshold
                        && i as i32 > last_min + delta
                    {
                        self.mins[nm] = i;
                        nm += 1;
                        last_min = i as i32;
                        find_min = false;
                    }
                    if find_max
                        && previous > 0.0
                        && dv <= 0.0
                        && s.abs() >= threshold
                        && i as i32 > last_max + delta
                    {
                        self.maxs[nx] = i;
                        nx += 1;
                        last_max = i as i32;
                        find_max = false;
                    }
                }
                previous = dv;
            }
            if nm == 0 && nx == 0 {
                break;
            }
            self.distances.fill(0);
            for (indexes, len) in [(&self.mins, nm), (&self.maxs, nx)] {
                for i in 0..len {
                    for j in 1..3 {
                        if i + j < len {
                            let d = indexes[i + j] - indexes[i];
                            self.distances[d] += 1;
                        }
                    }
                }
            }
            let (mut best, mut best_value) = (-1_i32, -1);
            for i in 0..count as i32 {
                let mut sum = 0;
                for j in -delta..=delta {
                    if i + j >= 0 && i + j < count as i32 {
                        sum += self.distances[(i + j) as usize];
                    }
                }
                match sum.cmp(&best_value) {
                    std::cmp::Ordering::Equal if i == 2 * best => best = i,
                    std::cmp::Ordering::Greater => {
                        best_value = sum;
                        best = i;
                    }
                    _ => {}
                }
            }
            let (mut avg, mut nd) = (0.0, 0.0);
            for j in -delta..=delta {
                let idx = best + j;
                if idx >= 0 && idx < n as i32 {
                    let d = self.distances[idx as usize];
                    if d > 0 {
                        nd += f64::from(d);
                        avg += f64::from(idx * d);
                    }
                }
            }
            avg /= nd;
            if mode > -1.0 && (avg * 2.0 - mode).abs() <= f64::from(2 * delta) {
                return self.sample_rate / ((1 << (level - 1)) as f64 * mode);
            }
            mode = avg;
            for i in 0..count / 2 {
                self.sam[i] = (self.sam[2 * i] + self.sam[2 * i + 1]) / 2.0;
            }
            count /= 2;
        }
        0.0
    }
    fn dynamic(&mut self, mut pitch: f64) -> f64 {
        if pitch == 0.0 {
            pitch = -1.0;
        }
        let mut estimated = -1.0;
        // C's abs takes an int, and 0.2f is promoted from float.
        let error = f64::from(0.2_f32);
        let abs = |v: f64| (v as i32).abs() as f64;
        if pitch != -1.0 {
            if self.prev_pitch == -1.0 {
                estimated = pitch;
                self.prev_pitch = pitch;
                self.confidence = 1;
            } else if abs(self.prev_pitch - pitch) / pitch < error {
                self.prev_pitch = pitch;
                estimated = pitch;
                self.confidence = (self.confidence + 1).min(5);
            } else if self.confidence >= 3
                && abs(self.prev_pitch - 2.0 * pitch) / (2.0 * pitch) < error
            {
                estimated = 2.0 * pitch;
                self.prev_pitch = estimated;
            } else if self.confidence >= 3
                && abs(self.prev_pitch - 0.5 * pitch) / (0.5 * pitch) < error
            {
                estimated = 0.5 * pitch;
                self.prev_pitch = estimated;
            } else if self.confidence >= 1 {
                estimated = self.prev_pitch;
                self.confidence = (self.confidence - 1).max(0);
            } else {
                estimated = pitch;
                self.prev_pitch = pitch;
                self.confidence = 1;
            }
        } else if self.prev_pitch != -1.0 {
            if self.confidence >= 1 {
                estimated = self.prev_pitch;
                self.confidence = (self.confidence - 1).max(0);
            } else {
                self.prev_pitch = -1.0;
                self.confidence = 0;
            }
        }
        if self.confidence >= 1 {
            estimated
        } else {
            0.0
        }
    }
}
