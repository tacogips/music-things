//! Envelope, crossover, and pulse selection from processReplacing.
use super::waveform::WAVEFORM;

#[derive(Default)]
pub(super) struct Channel {
    pub frame: i32,
    pub was_negative: bool,
    pub level: f64,
}

impl Channel {
    pub fn render(&mut self, input: f64, decay: f64, highs_boost: i32) -> f64 {
        let rectified = input.abs();
        if rectified > self.level {
            self.level = rectified;
        } else {
            self.level = ((self.level * decay) + rectified) / (decay + 1.0);
        }
        self.frame += 1;
        if input > 0.0 {
            if self.was_negative {
                self.frame = highs_boost;
            }
            self.was_negative = false;
        } else {
            self.was_negative = true;
        }
        let pulse = if self.frame > 0 && self.frame < 469 {
            WAVEFORM[self.frame as usize]
        } else {
            input
        };
        pulse * self.level
    }
}
