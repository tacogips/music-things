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

use super::core::Core;
use super::rfft::Complex;

impl Core {
    pub(super) fn encode(&mut self, dft: &mut [Complex]) {
        for (i, value) in dft.iter_mut().enumerate() {
            let phase = (value.im as f64).atan2(value.re as f64);
            let delta = phase - self.encode_phase[i];
            self.encode_phase[i] = phase;
            let t = delta - i as f64 * self.phase_inc;
            let j = (t - std::f64::consts::TAU * (t / std::f64::consts::TAU + 0.5).floor())
                / self.phase_inc;
            let frequency = (i as f64 + j) * self.freq_inc;
            *value = Complex {
                re: value.re.hypot(value.im),
                im: frequency as f32,
            };
        }
    }

    pub(super) fn decode(&mut self, dft: &mut [Complex]) {
        for (i, value) in dft.iter_mut().enumerate() {
            let frequency = value.im as f64;
            let j = (frequency - i as f64 * self.freq_inc) / self.freq_inc;
            let delta = (i as f64 + j) * self.phase_inc;
            self.decode_phase[i] += delta;
            let phase = (self.decode_phase[i] - self.offsets[i]) as f32;
            *value = Complex {
                re: value.re * phase.cos(),
                im: value.re * phase.sin(),
            };
        }
    }
}
