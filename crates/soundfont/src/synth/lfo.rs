// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
#[derive(Clone, Copy, Default)]
pub(super) struct Lfo {
    pub level: f32,
    pub delta: f32,
    samples_until: i32,
}
impl Lfo {
    pub fn setup(&mut self, delay: f32, frequency: i32, rate: f32) {
        self.samples_until = (delay * rate) as i32;
        self.delta = 4. * super::cents_hz(frequency as f32) / rate;
        self.level = 0.;
    }
    pub fn process(&mut self, count: i32) {
        if self.samples_until > count {
            self.samples_until -= count;
            return;
        }
        self.level += self.delta * count as f32;
        if self.level > 1. {
            self.delta = -self.delta;
            self.level = 2. - self.level;
        } else if self.level < -1. {
            self.delta = -self.delta;
            self.level = -2. - self.level;
        }
    }
}
