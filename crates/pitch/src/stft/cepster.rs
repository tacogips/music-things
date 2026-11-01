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
use super::resampler::resample_real;
use super::rfft::Complex;

impl Core {
    pub(super) fn prepare_envelope(&mut self, dft: &mut [Complex]) {
        for (e, value) in self.envelope.iter_mut().zip(dft.iter()) {
            *e = value.re;
        }
        for (s, &e) in self.spectrum.iter_mut().zip(self.envelope.iter()) {
            *s = Complex {
                re: if e != 0.0 { e.log10() } else { -12.0 },
                im: 0.0,
            };
        }
        self.fft.ifft(&self.spectrum, &mut self.cepstrum);
        for i in 1..self.cutoff.min(self.cepstrum.len()) {
            self.cepstrum[i] *= 2.0;
        }
        for i in self.cutoff.saturating_add(1)..self.cepstrum.len() {
            self.cepstrum[i] = 0.0;
        }
        self.fft.fft(&self.cepstrum, &mut self.spectrum);
        for (e, s) in self.envelope.iter_mut().zip(self.spectrum.iter()) {
            *e = 10.0_f32.powf(s.re);
        }
        for (e, value) in self.envelope.iter_mut().zip(dft.iter_mut()) {
            if e.is_normal() {
                value.re /= *e;
            } else {
                *e = 0.0;
                value.re = 0.0;
            }
        }
        resample_real(
            &self.envelope,
            &mut self.resampled_envelope,
            self.distortion,
        );
    }
}
