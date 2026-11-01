// Port of StftPitchShift/RFFT.h at 70c96db087458f8631edbc1bb4068d42ea5b52cc.
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

use crate::ConfigError;

#[derive(Clone, Copy, Default, Debug)]
pub(super) struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }
    fn conj(self) -> Self {
        Self::new(self.re, -self.im)
    }
    fn add(self, rhs: Self) -> Self {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

pub(super) struct Rfft {
    half: usize,
    p: Vec<usize>,
    w: Vec<Complex>,
    a: Vec<Complex>,
    b: Vec<Complex>,
    cw: Vec<Complex>,
    ca: Vec<Complex>,
    cb: Vec<Complex>,
    input: Vec<Complex>,
    buffer: Vec<Complex>,
    output: Vec<Complex>,
}

impl Rfft {
    pub fn new(size: usize) -> Result<Self, ConfigError> {
        let half = size / 2;
        if !size.is_power_of_two() || !half.is_power_of_two() {
            return Err(ConfigError::NotPowerOfTwo {
                what: "framesize",
                value: size,
            });
        }
        let mut w = Vec::with_capacity(half);
        let mut a = Vec::with_capacity(half);
        let mut b = Vec::with_capacity(half);
        let pi = -2.0_f32 * (-1.0_f32).acos();
        for i in 0..half {
            let arg = (pi * i as f32) / half as f32;
            w.push(Complex::new(arg.cos(), arg.sin()));
            let arg = (pi * i as f32) / size as f32;
            let polar = Complex::new(arg.cos(), arg.sin());
            a.push(Complex::new(0.5 - 0.5 * -polar.im, 0.5 * -polar.re));
            b.push(Complex::new(0.5 - 0.5 * polar.im, 0.5 * polar.re));
        }
        let cw = w.iter().map(|x| x.conj()).collect();
        let ca = a.iter().map(|x| x.conj()).collect();
        let cb = b.iter().map(|x| x.conj()).collect();
        let bits = half.trailing_zeros();
        let p = (0..half)
            .map(|i| {
                if bits == 0 {
                    0
                } else {
                    i.reverse_bits() >> (usize::BITS - bits)
                }
            })
            .collect();
        Ok(Self {
            half,
            p,
            w,
            a,
            b,
            cw,
            ca,
            cb,
            input: vec![Complex::default(); half],
            buffer: vec![Complex::default(); half],
            output: vec![Complex::default(); half],
        })
    }

    pub fn fft(&mut self, frame: &[f32], dft: &mut [Complex]) {
        assert_eq!(frame.len(), self.half * 2);
        assert_eq!(dft.len(), self.half + 1);
        for (i, value) in self.input.iter_mut().enumerate() {
            *value = Complex::new(frame[2 * i], frame[2 * i + 1]);
        }
        transform(&self.input, &mut self.buffer, &self.p, &self.w);
        pack(&self.buffer, dft, &self.a, &self.b);
        let scale = 1.0 / self.half as f32;
        for value in &mut dft[..self.half] {
            value.re *= scale;
            value.im *= scale;
        }
    }

    pub fn ifft(&mut self, dft: &[Complex], frame: &mut [f32]) {
        assert_eq!(frame.len(), self.half * 2);
        assert_eq!(dft.len(), self.half + 1);
        pack(dft, &mut self.buffer, &self.ca, &self.cb);
        transform(&self.buffer, &mut self.output, &self.p, &self.cw);
        for (i, value) in self.output.iter().enumerate() {
            frame[2 * i] = value.re;
            frame[2 * i + 1] = value.im;
        }
    }
}

fn pack(input: &[Complex], output: &mut [Complex], a: &[Complex], b: &[Complex]) {
    let size = a.len();
    output[0] = input[0].mul(a[0]).add(input[0].conj().mul(b[0]));
    for i in 1..size {
        output[i] = input[i].mul(a[i]).add(input[size - i].conj().mul(b[i]));
    }
}

fn transform(input: &[Complex], output: &mut [Complex], p: &[usize], w: &[Complex]) {
    let size = p.len();
    for i in 0..size {
        output[p[i]] = input[i];
    }
    for bit in 1..=size.trailing_zeros() {
        let n1 = 1usize << bit;
        let n2 = n1 >> 1;
        let inc = size / n1;
        for i in (0..size).step_by(n1) {
            for j in 0..n2 {
                let left = output[i + j];
                let right = output[i + j + n2].mul(w[j * inc]);
                output[i + j] = left.add(right);
                output[i + j + n2] = left.sub(right);
            }
        }
    }
}
