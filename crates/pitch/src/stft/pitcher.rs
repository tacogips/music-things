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
use super::resampler::resample_complex;
use super::rfft::Complex;

impl Core {
    pub(super) fn pitch(&mut self, dft: &mut [Complex]) {
        let n = dft.len();
        if self.factors.len() == 1 && self.factors[0] == 1.0 {
            return;
        }
        for (i, &factor) in self.factors.iter().enumerate() {
            let out = &mut self.shifted[i * n..(i + 1) * n];
            resample_complex(dft, out, factor);
            for value in out.iter_mut() {
                value.im *= factor as f32;
                if value.im <= 0.0 || value.im as f64 >= self.nyquist {
                    value.re = 0.0;
                }
            }
        }
        for (j, value) in dft.iter_mut().enumerate() {
            let mut maximum = f32::MIN;
            for i in 0..self.factors.len() {
                let candidate = self.shifted[i * n + j];
                if candidate.re > maximum {
                    maximum = candidate.re;
                    *value = candidate;
                }
            }
        }
    }
}
