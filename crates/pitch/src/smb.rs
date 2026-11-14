//! Streaming port of smbPitchShift 1.2 (`smbPitchShift.esp32.cpp`).
//!
//! D7: input and output are f32; upstream's int16 conversions are omitted.
//! D8: invalid frame sizes and oversampling configurations return `ConfigError`.
//!
//! COPYRIGHT 1999-2009 Stephan M. Bernsee <smb [AT] dspdimension [DOT] com>
//!
//! The Wide Open License (WOL)
//!
//! Permission to use, copy, modify, distribute and sell this software and its
//! documentation for any purpose is hereby granted without fee, provided that
//! the above copyright notice and this license appear in all source copies.
//! THIS SOFTWARE IS PROVIDED "AS IS" WITHOUT EXPRESS OR IMPLIED WARRANTY OF
//! ANY KIND. See http://www.dspguru.com/wol.htm for more information.

use crate::error::ConfigError;
use std::f64::consts::PI;

/// Maximum FFT frame size supported by the original implementation.
pub const MAX_FRAME_LENGTH: usize = 8192;

/// Stateful streaming pitch shifter. Its pitch factor is fixed at construction.
pub struct SmbPitchShifter {
    shift: f32,
    frame: usize,
    osamp: usize,
    step: usize,
    rover: usize,
    freq_per_bin: f32,
    expct: f32,
    input: Vec<f32>,
    output: Vec<f32>,
    fft: Vec<f32>,
    last_phase: Vec<f32>,
    sum_phase: Vec<f32>,
    accum: Vec<f32>,
    ana_freq: Vec<f32>,
    ana_magn: Vec<f32>,
    syn_freq: Vec<f32>,
    syn_magn: Vec<f32>,
}

impl SmbPitchShifter {
    /// Construct and allocate all processing buffers.
    pub fn new(
        pitch_shift: f32,
        fft_frame_size: usize,
        osamp: usize,
        sample_rate: f32,
    ) -> Result<Self, ConfigError> {
        if !(2..=MAX_FRAME_LENGTH).contains(&fft_frame_size) {
            return Err(ConfigError::OutOfRange {
                what: "fft_frame_size",
                value: fft_frame_size,
                min: 2,
                max: MAX_FRAME_LENGTH,
            });
        }
        if !fft_frame_size.is_power_of_two() {
            return Err(ConfigError::NotPowerOfTwo {
                what: "fft_frame_size",
                value: fft_frame_size,
            });
        }
        if osamp == 0 || fft_frame_size % osamp != 0 {
            return Err(ConfigError::InvalidValue { what: "osamp" });
        }
        let frame = fft_frame_size;
        let step = frame / osamp;
        Ok(Self {
            shift: pitch_shift,
            frame,
            osamp,
            step,
            rover: frame - step,
            freq_per_bin: sample_rate / frame as f32,
            expct: (2.0 * PI * step as f64 / frame as f64) as f32,
            input: vec![0.0; frame],
            output: vec![0.0; frame],
            fft: vec![0.0; 2 * frame],
            last_phase: vec![0.0; frame / 2 + 1],
            sum_phase: vec![0.0; frame / 2 + 1],
            accum: vec![0.0; 2 * frame],
            ana_freq: vec![0.0; frame],
            ana_magn: vec![0.0; frame],
            syn_freq: vec![0.0; frame],
            syn_magn: vec![0.0; frame],
        })
    }

    /// Algorithmic input FIFO latency, in samples.
    pub fn latency(&self) -> usize {
        self.frame - self.step
    }

    /// Clear all state without allocating.
    pub fn reset(&mut self) {
        self.rover = self.latency();
        self.input.fill(0.0);
        self.output.fill(0.0);
        self.fft.fill(0.0);
        self.last_phase.fill(0.0);
        self.sum_phase.fill(0.0);
        self.accum.fill(0.0);
        self.ana_freq.fill(0.0);
        self.ana_magn.fill(0.0);
        self.syn_freq.fill(0.0);
        self.syn_magn.fill(0.0);
    }

    /// Process the shorter of the input and output slices without allocating.
    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        for (&sample, out) in input.iter().zip(output.iter_mut()) {
            self.input[self.rover] = sample;
            *out = self.output[self.rover - self.latency()];
            self.rover += 1;
            if self.rover >= self.frame {
                self.rover = self.latency();
                self.process_frame();
            }
        }
    }

    fn process_frame(&mut self) {
        let n = self.frame;
        let half = n / 2;
        for k in 0..n {
            let window = (-0.5 * (2.0 * PI * k as f64 / n as f64).cos() + 0.5) as f32;
            self.fft[2 * k] = self.input[k] * window;
            self.fft[2 * k + 1] = 0.0;
        }
        fft(&mut self.fft, n, -1);
        for k in 0..=half {
            let real = self.fft[2 * k];
            let imag = self.fft[2 * k + 1];
            let magn = (2.0 * f64::from((real * real + imag * imag).sqrt())) as f32;
            let phase = imag.atan2(real);
            let mut tmp = phase - self.last_phase[k];
            self.last_phase[k] = phase;
            tmp -= k as f32 * self.expct;
            let mut qpd = (f64::from(tmp) / PI) as i64;
            if qpd >= 0 {
                qpd += qpd & 1;
            } else {
                qpd -= qpd & 1;
            }
            tmp = (f64::from(tmp) - PI * qpd as f64) as f32;
            tmp = (f64::from(self.osamp as f32 * tmp) / (2.0 * PI)) as f32;
            tmp = k as f32 * self.freq_per_bin + tmp * self.freq_per_bin;
            self.ana_magn[k] = magn;
            self.ana_freq[k] = tmp;
        }
        self.syn_magn.fill(0.0);
        self.syn_freq.fill(0.0);
        for k in 0..=half {
            let index = (k as f32 * self.shift) as usize;
            if index <= half {
                self.syn_magn[index] += self.ana_magn[k];
                self.syn_freq[index] = self.ana_freq[k] * self.shift;
            }
        }
        for k in 0..=half {
            let magn = self.syn_magn[k];
            let mut tmp = self.syn_freq[k];
            tmp -= k as f32 * self.freq_per_bin;
            tmp /= self.freq_per_bin;
            tmp = (2.0 * PI * f64::from(tmp) / self.osamp as f64) as f32;
            tmp += k as f32 * self.expct;
            self.sum_phase[k] += tmp;
            let phase = self.sum_phase[k];
            self.fft[2 * k] = magn * phase.cos();
            self.fft[2 * k + 1] = magn * phase.sin();
        }
        self.fft[n + 2..].fill(0.0);
        fft(&mut self.fft, n, 1);
        for k in 0..n {
            let window = (-0.5 * (2.0 * PI * k as f64 / n as f64).cos() + 0.5) as f32;
            self.accum[k] = (f64::from(self.accum[k])
                + 2.0 * f64::from(window) * f64::from(self.fft[2 * k]) / (half * self.osamp) as f64)
                as f32;
        }
        self.output[..self.step].copy_from_slice(&self.accum[..self.step]);
        self.accum.copy_within(self.step..self.step + n, 0);
        self.input.copy_within(self.step..n, 0);
    }
}

fn fft(buf: &mut [f32], frame: usize, sign: i32) {
    for i in (2..2 * frame - 2).step_by(2) {
        let mut bitm = 2;
        let mut j = 0;
        while bitm < 2 * frame {
            if i & bitm != 0 {
                j += 1;
            }
            j <<= 1;
            bitm <<= 1;
        }
        if i < j {
            buf.swap(i, j);
            buf.swap(i + 1, j + 1);
        }
    }
    let mut le = 2;
    for _ in 0..frame.ilog2() {
        le <<= 1;
        let le2 = le >> 1;
        let mut ur = 1.0_f32;
        let mut ui = 0.0_f32;
        let arg = (PI / (le2 >> 1) as f64) as f32;
        let wr = arg.cos();
        let wi = sign as f32 * arg.sin();
        for j in (0..le2).step_by(2) {
            for i in (j..2 * frame).step_by(le) {
                let p2 = i + le2;
                let tr = buf[p2] * ur - buf[p2 + 1] * ui;
                let ti = buf[p2] * ui + buf[p2 + 1] * ur;
                buf[p2] = buf[i] - tr;
                buf[p2 + 1] = buf[i + 1] - ti;
                buf[i] += tr;
                buf[i + 1] += ti;
            }
            let tr = ur * wr - ui * wi;
            ui = ur * wi + ui * wr;
            ur = tr;
        }
    }
}
