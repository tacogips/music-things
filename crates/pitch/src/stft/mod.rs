//! Short-time Fourier transform pitch shifting.
// Port of StftPitchShift/StftPitchShift.h at 70c96db087458f8631edbc1bb4068d42ea5b52cc.
// Copyright (c) 2022 Juergen Hock. MIT license; see rfft.rs for the license text.

mod cepster;
mod core;
#[path = "stft.rs"]
mod framing;
mod pitcher;
mod resampler;
mod rfft;
mod stream;
mod vocoder;

pub use framing::StftPitchShift;
pub use stream::StftPitchShiftStream;

/// Configuration for offline and streaming pitch shifting.
#[derive(Clone, Copy, Debug)]
pub struct StftConfig {
    pub analysis_framesize: usize,
    pub synthesis_framesize: usize,
    pub hopsize: usize,
    pub sample_rate: f32,
    pub quefrency: f32,
    pub distortion: f32,
    pub normalization: bool,
    pub max_factors: usize,
}

impl StftConfig {
    /// Constructs a configuration with equal analysis and synthesis windows.
    pub fn symmetric(framesize: usize, hopsize: usize, sample_rate: f32) -> Self {
        Self {
            analysis_framesize: framesize,
            synthesis_framesize: framesize,
            hopsize,
            sample_rate,
            quefrency: 0.0,
            distortion: 1.0,
            normalization: false,
            max_factors: 1,
        }
    }
}
