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

use super::rfft::Complex;

fn indices(n: usize, factor: f64, i: usize) -> Option<(usize, f32)> {
    let m = (n as f64 * factor) as isize;
    if !(factor > 0.0 && factor < 1.0 && m < n as isize || factor > 1.0 && m > n as isize) {
        return None;
    }
    if i as isize >= m && factor < 1.0 {
        return Some((n, 0.0));
    }
    let numerator = i as isize * n as isize;
    let j = numerator / m;
    let r = numerator % m;
    if j <= 0 || j >= n as isize - 1 {
        Some((n, 0.0))
    } else {
        Some((j as usize, r as f32 / m as f32))
    }
}

pub(super) fn resample_complex(input: &[Complex], output: &mut [Complex], factor: f64) {
    let n = input.len();
    for i in 0..n {
        output[i] = match indices(n, factor, i) {
            None => input[i],
            Some((j, k)) if j < n => Complex {
                re: k * input[j + 1].re + (1.0 - k) * input[j].re,
                im: k * input[j + 1].im + (1.0 - k) * input[j].im,
            },
            _ => Complex::default(),
        };
    }
}

pub(super) fn resample_real(input: &[f32], output: &mut [f32], factor: f64) {
    let n = input.len();
    for i in 0..n {
        output[i] = match indices(n, factor, i) {
            None => input[i],
            Some((j, k)) if j < n => k * input[j + 1] + (1.0 - k) * input[j],
            _ => 0.0,
        };
    }
}
