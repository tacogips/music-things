// Port of StftPitchShift/STFT.h and StftPitchShift.cpp at 70c96db087458f8631edbc1bb4068d42ea5b52cc.
// Copyright (c) 2022 Juergen Hock. MIT license; see rfft.rs for the license text.
use super::{
    core::Core,
    rfft::{Complex, Rfft},
    StftConfig,
};
use crate::ConfigError;

/// Offline STFT pitch shifter with preallocated working buffers.
pub struct StftPitchShift {
    pub(super) config: StftConfig,
    fft: Rfft,
    core: Core,
    analysis: Vec<f32>,
    synthesis: Vec<f32>,
    time: Vec<f32>,
    freq: Vec<Complex>,
}

fn window(n: usize) -> Vec<f32> {
    let pi = 2.0_f32 * (-1.0_f32).acos() / n as f32;
    (0..n).map(|i| 0.5 - 0.5 * (pi * i as f32).cos()).collect()
}

impl StftPitchShift {
    /// Allocates the working buffers and validates the frame configuration.
    pub fn new(config: StftConfig) -> Result<Self, ConfigError> {
        let n = config.analysis_framesize;
        let s = config.synthesis_framesize;
        if !n.is_power_of_two() || n < 2 {
            return Err(ConfigError::NotPowerOfTwo {
                what: "framesize",
                value: n,
            });
        }
        if n < s {
            return Err(ConfigError::InvalidValue { what: "framesize" });
        }
        if config.hopsize == 0 {
            return Err(ConfigError::InvalidValue { what: "hopsize" });
        }
        if config.max_factors == 0 {
            return Err(ConfigError::InvalidValue {
                what: "max_factors",
            });
        }
        let (analysis, mut synthesis) = if n == s {
            let w = window(n);
            (w.clone(), w)
        } else {
            let m = s / 2;
            let left = window(2 * n - 2 * m);
            let right = window(2 * m);
            let mut a = vec![0.0; n];
            let mut b = vec![0.0; n];
            a[..n - m].copy_from_slice(&left[..n - m]);
            a[n - m..].copy_from_slice(&right[m..]);
            for i in 0..m {
                b[i + n - 2 * m] = right[i] * right[i] / left[i + n - 2 * m];
            }
            b[n - m..].copy_from_slice(&right[m..]);
            (a, b)
        };
        let dot = analysis
            .iter()
            .zip(&synthesis)
            .fold(0.0_f32, |v, (a, b)| v + a * b);
        let scale = config.hopsize as f32 / dot;
        for v in &mut synthesis {
            *v *= scale;
        }
        Ok(Self {
            config,
            fft: Rfft::new(n)?,
            core: Core::new(config)?,
            analysis,
            synthesis,
            time: vec![0.0; n],
            freq: vec![Complex::default(); n / 2 + 1],
        })
    }
    /// Replaces pitch factors without allocating.
    pub fn set_factors(&mut self, factors: &[f32]) -> Result<(), ConfigError> {
        self.core.set_factors(factors)
    }
    pub(super) fn reset(&mut self) {
        self.core.reset();
    }
    pub(super) fn process_frame(&mut self, input: &[f32], output: &mut [f32]) {
        for ((time, &sample), &window) in self.time.iter_mut().zip(input).zip(&self.analysis) {
            *time = sample * window;
        }
        self.fft.fft(&self.time, &mut self.freq);
        self.core.process(&mut self.freq);
        self.freq[0] = Complex::default();
        let last = self.freq.len() - 1;
        self.freq[last] = Complex::default();
        self.fft.ifft(&self.freq, &mut self.time);
        for (i, value) in output.iter_mut().enumerate().take(self.time.len()) {
            *value = self.time[i] * self.synthesis[i];
        }
    }
    /// Shifts the input using upstream's offline framing and output length semantics.
    pub fn shift_pitch(&mut self, input: &[f32], output: &mut [f32]) {
        output.fill(0.0);
        self.reset();
        let n = self.config.analysis_framesize;
        let samples = input.len().min(output.len());
        let mut hop = 0;
        while hop + n < samples {
            for i in 0..n {
                self.time[i] = input[hop + i] * self.analysis[i];
            }
            self.fft.fft(&self.time, &mut self.freq);
            self.core.process(&mut self.freq);
            self.freq[0] = Complex::default();
            let last = self.freq.len() - 1;
            self.freq[last] = Complex::default();
            self.fft.ifft(&self.freq, &mut self.time);
            for i in 0..n {
                output[hop + i] += self.time[i] * self.synthesis[i];
            }
            hop += self.config.hopsize;
        }
    }
}
