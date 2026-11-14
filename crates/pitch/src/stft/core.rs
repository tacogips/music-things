// Port of StftPitchShift/{StftPitchShiftCore,Vocoder,Pitcher,Resampler,Cepster,Normalizer}.h
// at 70c96db087458f8631edbc1bb4068d42ea5b52cc.
// Copyright (c) 2022 Juergen Hock
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use super::rfft::{Complex, Rfft};
use super::StftConfig;
use crate::ConfigError;

pub(super) struct Core {
    pub(super) freq_inc: f64,
    pub(super) phase_inc: f64,
    pub(super) nyquist: f64,
    pub(super) encode_phase: Vec<f64>,
    pub(super) decode_phase: Vec<f64>,
    pub(super) offsets: Vec<f64>,
    pub(super) factors: Vec<f64>,
    pub(super) max_factors: usize,
    pub(super) shifted: Vec<Complex>,
    pub(super) envelope: Vec<f32>,
    pub(super) resampled_envelope: Vec<f32>,
    pub(super) cepstrum: Vec<f32>,
    pub(super) spectrum: Vec<Complex>,
    pub(super) fft: Rfft,
    pub(super) cutoff: usize,
    pub(super) quefrency: f64,
    pub(super) distortion: f64,
    pub(super) normalization: bool,
}

impl Core {
    pub fn new(config: StftConfig) -> Result<Self, ConfigError> {
        let n = config.analysis_framesize;
        let bins = n / 2 + 1;
        let pi = 2.0 * (-1.0_f64).acos();
        let mut offsets = vec![0.0; bins];
        if n != config.synthesis_framesize {
            for (i, value) in offsets.iter_mut().enumerate() {
                *value = pi * i as f64 * config.synthesis_framesize as f64 / bins as f64;
            }
        }
        let mut factors = Vec::with_capacity(config.max_factors);
        factors.push(1.0);
        Ok(Self {
            freq_inc: config.sample_rate as f64 / n as f64,
            phase_inc: pi * config.hopsize as f64 / n as f64,
            nyquist: config.sample_rate as f64 / 2.0,
            encode_phase: vec![0.0; bins],
            decode_phase: vec![0.0; bins],
            offsets,
            factors,
            max_factors: config.max_factors,
            shifted: vec![Complex::default(); bins * config.max_factors],
            envelope: vec![0.0; bins],
            resampled_envelope: vec![0.0; bins],
            cepstrum: vec![0.0; n],
            spectrum: vec![Complex::default(); bins],
            fft: Rfft::new(n)?,
            cutoff: (config.quefrency as f64 * config.sample_rate as f64) as usize,
            quefrency: config.quefrency as f64,
            distortion: config.distortion as f64,
            normalization: config.normalization,
        })
    }

    pub fn reset(&mut self) {
        self.encode_phase.fill(0.0);
        self.decode_phase.fill(0.0);
        self.spectrum.fill(Complex::default());
        self.cepstrum.fill(0.0);
    }

    pub fn set_factors(&mut self, factors: &[f32]) -> Result<(), ConfigError> {
        if factors.is_empty() {
            return Err(ConfigError::InvalidValue { what: "factors" });
        }
        if factors.len() > self.max_factors {
            return Err(ConfigError::TooManyFactors {
                max: self.max_factors,
                got: factors.len(),
            });
        }
        self.factors.clear();
        self.factors.extend(factors.iter().map(|&x| x as f64));
        Ok(())
    }

    pub fn process(&mut self, dft: &mut [Complex]) {
        self.encode(dft);
        let target = if self.normalization { rms(dft) } else { 0.0 };
        if self.quefrency != 0.0 {
            self.prepare_envelope(dft);
        }
        self.pitch(dft);
        if self.quefrency != 0.0 {
            for (value, &e) in dft.iter_mut().zip(self.resampled_envelope.iter()) {
                value.re *= e;
            }
        }
        if self.normalization {
            let b = rms(dft);
            if b != 0.0 {
                let c = (target / b).sqrt();
                for value in dft.iter_mut() {
                    value.re *= c;
                }
            }
        }
        self.decode(dft);
    }
}

fn rms(data: &[Complex]) -> f32 {
    data.iter().fold(0.0, |sum, v| sum + v.re * v.re)
}
