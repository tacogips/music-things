// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
use crate::font::Envelope;

pub(super) const RELEASE: i32 = 6;
pub(super) const DONE: i32 = 7;
#[derive(Clone, Copy, Default)]
pub(super) struct VoiceEnvelope {
    pub parameters: Envelope,
    pub segment: i32,
    pub samples: i32,
    pub level: f32,
    slope: f32,
    exponential: bool,
    velocity: i16,
    amp: bool,
}
impl VoiceEnvelope {
    pub fn setup(&mut self, p: Envelope, key: i32, velocity: i16, amp: bool, rate: f32) {
        self.parameters = p;
        if p.keynum_to_hold != 0.0 {
            self.parameters.hold += p.keynum_to_hold * (60.0 - key as f32);
            self.parameters.hold = seconds(self.parameters.hold);
        }
        if p.keynum_to_decay != 0.0 {
            self.parameters.decay += p.keynum_to_decay * (60.0 - key as f32);
            self.parameters.decay = seconds(self.parameters.decay);
        }
        self.velocity = velocity;
        self.amp = amp;
        self.next(0, rate);
    }
    pub fn release_samples(&self, rate: f32) -> i32 {
        ((if self.parameters.release <= 0.0 {
            0.01
        } else {
            self.parameters.release
        }) * rate) as i32
    }
    pub fn next(&mut self, mut segment: i32, rate: f32) {
        loop {
            match segment {
                0 => {
                    self.samples = (self.parameters.delay * rate) as i32;
                    if self.samples > 0 {
                        self.segment = 1;
                        self.exponential = false;
                        self.level = 0.;
                        self.slope = 0.;
                        return;
                    }
                }
                1 => {
                    self.samples = (self.parameters.attack * rate) as i32;
                    if self.samples > 0 {
                        if !self.amp {
                            self.samples = (self.parameters.attack
                                * ((145 - self.velocity as i32) as f32 / 144.)
                                * rate) as i32;
                        }
                        self.segment = 2;
                        self.exponential = false;
                        self.level = 0.;
                        self.slope = 1. / self.samples as f32;
                        return;
                    }
                }
                2 => {
                    self.samples = (self.parameters.hold * rate) as i32;
                    if self.samples > 0 {
                        self.segment = 3;
                        self.exponential = false;
                        self.level = 1.;
                        self.slope = 0.;
                        return;
                    }
                }
                3 => {
                    self.samples = (self.parameters.decay * rate) as i32;
                    if self.samples > 0 {
                        self.segment = 4;
                        self.level = 1.;
                        if self.amp {
                            let slope = -9.226_f32 / self.samples as f32;
                            self.slope = slope.exp();
                            self.exponential = true;
                            if self.parameters.sustain > 0. {
                                self.samples =
                                    ((self.parameters.sustain as f64).ln() / slope as f64) as i32;
                            }
                        } else {
                            self.slope = -1. / self.samples as f32;
                            self.samples = (self.parameters.decay
                                * (1. - self.parameters.sustain)
                                * rate) as i32;
                            self.exponential = false;
                        }
                        return;
                    }
                }
                4 => {
                    self.segment = 5;
                    self.level = self.parameters.sustain;
                    self.slope = 0.;
                    self.samples = i32::MAX;
                    self.exponential = false;
                    return;
                }
                5 => {
                    self.segment = RELEASE;
                    self.samples = self.release_samples(rate);
                    if self.amp {
                        self.slope = (-9.226_f32 / self.samples as f32).exp();
                        self.exponential = true;
                    } else {
                        self.slope = -self.level / self.samples as f32;
                        self.exponential = false;
                    }
                    return;
                }
                _ => {
                    self.segment = DONE;
                    self.exponential = false;
                    self.level = 0.;
                    self.slope = 0.;
                    self.samples = 0x7ffffff;
                    return;
                }
            }
            segment += 1;
        }
    }
    pub fn process(&mut self, count: i32, rate: f32) {
        if self.slope != 0. {
            if self.exponential {
                self.level *= self.slope.powf(count as f32);
            } else {
                self.level += self.slope * count as f32;
            }
        }
        self.samples -= count;
        if self.samples <= 0 {
            self.next(self.segment, rate);
        }
    }
}
fn seconds(cents: f32) -> f32 {
    if cents < -10000. {
        0.
    } else {
        2_f32.powf(cents / 1200.)
    }
}
