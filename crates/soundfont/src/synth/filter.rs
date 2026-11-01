// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
#[derive(Clone, Copy, Default)]
pub(super) struct Filter {
    pub active: bool,
    pub q_inv: f64,
    a0: f64,
    a1: f64,
    b1: f64,
    b2: f64,
    z1: f64,
    z2: f64,
}
impl Filter {
    pub fn setup(&mut self, fc: f32) {
        let k = (std::f64::consts::PI * fc as f64).tan();
        let kk = k * k;
        let norm = 1. / (1. + k * self.q_inv + kk);
        self.a0 = kk * norm;
        self.a1 = 2. * self.a0;
        self.b1 = 2. * (kk - 1.) * norm;
        self.b2 = (1. - k * self.q_inv + kk) * norm;
    }
    pub fn process(&mut self, input: f32) -> f32 {
        let input = input as f64;
        let out = input * self.a0 + self.z1;
        self.z1 = input * self.a1 + self.z2 - self.b1 * out;
        self.z2 = input * self.a0 - self.b2 * out;
        out as f32
    }
}
