// SPDX-License-Identifier: MIT
// Based on stftPitchShift by Juergen Hock, copyright (c) 2022.
// Upstream revision 70c96db087458f8631edbc1bb4068d42ea5b52cc.

use super::{StftConfig, StftPitchShift};
use crate::ConfigError;

/// Streaming STFT pitch shifter with a latency of `analysis_framesize + 1` samples.
///
/// Like the offline processor, only complete frames followed by another input
/// sample are processed. A caller may feed silence to drain the output.
pub struct StftPitchShiftStream {
    core: StftPitchShift,
    input_ring: Vec<f32>,
    output_ring: Vec<f32>,
    input_frame: Vec<f32>,
    output_frame: Vec<f32>,
    position: usize,
    hopsize: usize,
    latency: usize,
}

impl StftPitchShiftStream {
    /// Constructs a stream and allocates its fixed-size buffers.
    pub fn new(config: StftConfig) -> Result<Self, ConfigError> {
        let framesize = config.analysis_framesize;
        let hopsize = config.hopsize;
        let core = StftPitchShift::new(config)?;
        let latency = framesize + 1;
        Ok(Self {
            core,
            input_ring: vec![0.0; framesize],
            output_ring: vec![0.0; 2 * framesize + 2],
            input_frame: vec![0.0; framesize],
            output_frame: vec![0.0; framesize],
            position: 0,
            hopsize,
            latency,
        })
    }

    /// Sets the pitch factors without allocating.
    pub fn set_factors(&mut self, factors: &[f32]) -> Result<(), ConfigError> {
        self.core.set_factors(factors)
    }

    /// Returns the output delay in samples.
    pub fn latency(&self) -> usize {
        self.latency
    }

    /// Processes the paired portion of the input and output slices.
    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let framesize = self.input_ring.len();
        let output_size = self.output_ring.len();
        for (&sample, result) in input.iter().zip(output.iter_mut()) {
            let position = self.position;
            let output_index = position % output_size;
            *result = self.output_ring[output_index];
            self.output_ring[output_index] = 0.0;

            if position >= framesize && (position - framesize) % self.hopsize == 0 {
                let start = position - framesize;
                for (i, value) in self.input_frame.iter_mut().enumerate() {
                    *value = self.input_ring[(start + i) % framesize];
                }
                self.core
                    .process_frame(&self.input_frame, &mut self.output_frame);
                for (i, value) in self.output_frame.iter().enumerate() {
                    self.output_ring[(start + self.latency + i) % output_size] += value;
                }
            }
            self.input_ring[position % framesize] = sample;
            self.position += 1;
        }
    }
}
