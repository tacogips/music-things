//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! LeadAmp state, upstream revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
use crate::dither::fpd_from_seed;

pub(super) struct State {
    pub params: [f32; 4],
    pub sample_rate: f32,
    pub seed: u32,
    pub store_sample: [f64; 2],
    pub smooth: [[f64; 11]; 2],
    pub iir: [[f64; 11]; 2],
    pub lowpass: [f64; 2],
    pub speaker_a: [f64; 2],
    pub speaker_b: [f64; 2],
    pub sub: [f64; 2],
    pub odd: [[f64; 257]; 2],
    pub even: [[f64; 257]; 2],
    pub flip: bool,
    pub count: i32,
    pub cab: [[f64; 90]; 2],
    pub last_cab: [f64; 2],
    pub smooth_cab_a: [f64; 2],
    pub smooth_cab_b: [f64; 2],
    pub last_ref: [[f64; 10]; 2],
    pub cycle: usize,
    pub fix: [[f64; 11]; 6],
    pub fpd: [u32; 2],
}
impl State {
    pub fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.8, 1.0],
            sample_rate,
            seed,
            store_sample: [0.0; 2],
            smooth: [[0.0; 11]; 2],
            iir: [[0.0; 11]; 2],
            lowpass: [0.0; 2],
            speaker_a: [0.0; 2],
            speaker_b: [0.0; 2],
            sub: [0.0; 2],
            odd: [[0.0; 257]; 2],
            even: [[0.0; 257]; 2],
            flip: false,
            count: 0,
            cab: [[0.0; 90]; 2],
            last_cab: [0.0; 2],
            smooth_cab_a: [0.0; 2],
            smooth_cab_b: [0.0; 2],
            last_ref: [[0.0; 10]; 2],
            cycle: 0,
            fix: [[0.0; 11]; 6],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
