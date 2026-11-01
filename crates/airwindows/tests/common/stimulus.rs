#![allow(dead_code)]

pub const AW_FRAMES: usize = 8192;
pub const PITCH_SAMPLES: usize = 11025;

pub fn xorshift32(state: &mut u32) -> u32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    x
}

fn parabolic(p: f64) -> f64 {
    let x = 2.0 * p - 1.0;
    4.0 * x * (1.0 - x.abs())
}

pub fn aw_stimulus(sample_rate: f32) -> (Vec<f32>, Vec<f32>) {
    let mut left = vec![0.0; AW_FRAMES];
    let mut right = vec![0.0; AW_FRAMES];
    left[0] = 0.9;
    right[0] = -0.5;
    let (mut pl, mut pr) = (0.0, 0.0);
    let (il, ir) = (
        220.0 / f64::from(sample_rate),
        330.0 / f64::from(sample_rate),
    );
    for n in 256..4096 {
        let amp = 0.05 + 1.15 * ((n - 256) as f64 / 3840.0);
        left[n] = (amp * parabolic(pl)) as f32;
        right[n] = (amp * parabolic(pr)) as f32;
        pl += il;
        pr += ir;
        if pl >= 1.0 {
            pl -= 1.0;
        }
        if pr >= 1.0 {
            pr -= 1.0;
        }
    }
    let mut state = 0x12345678;
    for n in 4096..6144 {
        left[n] = ((f64::from(xorshift32(&mut state)) / 4294967296.0 * 2.0 - 1.0) * 0.5) as f32;
        right[n] = ((f64::from(xorshift32(&mut state)) / 4294967296.0 * 2.0 - 1.0) * 0.5) as f32;
    }
    (left, right)
}

pub fn pitch_stimulus() -> Vec<f32> {
    let mut out = Vec::with_capacity(PITCH_SAMPLES);
    let mut phase = 0.0;
    let mut state = 0x9e3779b9;
    for n in 0..PITCH_SAMPLES {
        let frequency = 220.0 + 110.0 * (n as f64 / 11025.0);
        let mut q = 2.0 * phase;
        if q >= 1.0 {
            q -= 1.0;
        }
        let noise = f64::from(xorshift32(&mut state)) / 4294967296.0 * 2.0 - 1.0;
        out.push((0.3 * parabolic(phase) + 0.15 * parabolic(q) + 0.05 * noise) as f32);
        phase += frequency / 44100.0;
        if phase >= 1.0 {
            phase -= 1.0;
        }
    }
    out
}
