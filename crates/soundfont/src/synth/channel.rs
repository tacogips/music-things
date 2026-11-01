// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
use super::{db_gain, envelope::RELEASE, gain_db, Synth};
#[derive(Clone, Copy)]
pub(super) struct Channel {
    pub preset: u16,
    pub bank: u16,
    pub wheel: u16,
    pub pan: f32,
    pub gain_db: f32,
    pub range: f32,
    pub tuning: f32,
    pub sustain: bool,
    midi_pan: u16,
    midi_volume: u16,
    midi_expression: u16,
    midi_rpn: u16,
    midi_data: u16,
}
impl Default for Channel {
    fn default() -> Self {
        Self {
            preset: 0,
            bank: 0,
            wheel: 8192,
            pan: 0.,
            gain_db: 0.,
            range: 2.,
            tuning: 0.,
            sustain: false,
            midi_pan: 8192,
            midi_volume: 16383,
            midi_expression: 16383,
            midi_rpn: 0xffff,
            midi_data: 0,
        }
    }
}
impl Channel {
    pub fn pitch_shift(self) -> f32 {
        if self.wheel == 8192 {
            self.tuning
        } else {
            self.wheel as f32 / 16383. * self.range * 2. - self.range + self.tuning
        }
    }
}
impl Synth {
    fn activate(&mut self, ch: usize) -> bool {
        if ch >= 16 {
            return false;
        }
        while self.channel_num <= ch {
            self.channels[self.channel_num] = Channel::default();
            self.channel_num += 1;
        }
        true
    }
    fn apply_pitch(&mut self, ch: usize) {
        let pitch = self.channels[ch].pitch_shift();
        for v in &mut self.voices {
            if v.preset != -1 && v.channel == ch {
                v.pitch(pitch, self.rate);
            }
        }
    }
    pub fn channel_set_presetindex(&mut self, ch: usize, preset: i32) -> bool {
        if !self.activate(ch) {
            return false;
        }
        self.channels[ch].preset = preset as u16;
        true
    }
    pub fn channel_set_presetnumber(&mut self, ch: usize, preset: i32, drums: bool) -> bool {
        if !self.activate(ch) {
            return false;
        }
        let bank = (self.channels[ch].bank & 0x7fff) as i32;
        let p = if drums {
            self.font
                .preset_index(128 | bank, preset)
                .or_else(|| self.font.preset_index(128, preset))
                .or_else(|| self.font.preset_index(128, 0))
                .or_else(|| self.font.preset_index(bank, preset))
        } else {
            self.font.preset_index(bank, preset)
        }
        .or_else(|| self.font.preset_index(0, preset));
        if let Some(p) = p {
            self.channels[ch].preset = p as u16;
            true
        } else {
            false
        }
    }
    pub fn channel_set_bank(&mut self, ch: usize, bank: i32) -> bool {
        if !self.activate(ch) {
            return false;
        }
        self.channels[ch].bank = bank as u16;
        true
    }
    pub fn channel_set_bank_preset(&mut self, ch: usize, bank: i32, preset: i32) -> bool {
        if !self.activate(ch) {
            return false;
        }
        if let Some(p) = self.font.preset_index(bank, preset) {
            self.channels[ch].preset = p as u16;
            self.channels[ch].bank = bank as u16;
            true
        } else {
            false
        }
    }
    pub fn channel_set_pan(&mut self, ch: usize, pan: f32) -> bool {
        if !self.activate(ch) {
            return false;
        }
        for v in &mut self.voices {
            if v.preset != -1 && v.channel == ch {
                v.pan(pan - 0.5);
            }
        }
        self.channels[ch].pan = pan - 0.5;
        true
    }
    pub fn channel_set_volume(&mut self, ch: usize, volume: f32) -> bool {
        if !self.activate(ch) {
            return false;
        }
        let gain = gain_db(volume);
        let change = gain - self.channels[ch].gain_db;
        for v in &mut self.voices {
            if v.preset != -1 && v.channel == ch {
                v.gain_db += change;
            }
        }
        self.channels[ch].gain_db = gain;
        true
    }
    pub fn channel_set_pitchwheel(&mut self, ch: usize, wheel: i32) -> bool {
        if !self.activate(ch) {
            return false;
        }
        self.channels[ch].wheel = wheel as u16;
        self.apply_pitch(ch);
        true
    }
    pub fn channel_set_pitchrange(&mut self, ch: usize, range: f32) -> bool {
        if !self.activate(ch) {
            return false;
        }
        self.channels[ch].range = range;
        if self.channels[ch].wheel != 8192 {
            self.apply_pitch(ch);
        }
        true
    }
    pub fn channel_set_tuning(&mut self, ch: usize, tuning: f32) -> bool {
        if !self.activate(ch) {
            return false;
        }
        self.channels[ch].tuning = tuning;
        self.apply_pitch(ch);
        true
    }
    pub fn channel_set_sustain(&mut self, ch: usize, sustain: bool) -> bool {
        if !self.activate(ch) {
            return false;
        }
        if self.channels[ch].sustain == sustain {
            return true;
        }
        self.channels[ch].sustain = sustain;
        if !sustain {
            for v in &mut self.voices {
                if v.preset != -1 && v.channel == ch && v.amp.segment < RELEASE && v.held {
                    v.end(self.rate, false);
                }
            }
        }
        true
    }
    pub fn channel_note_on(&mut self, ch: usize, key: i32, velocity: f32) -> bool {
        if ch >= 16 {
            return false;
        }
        if ch >= self.channel_num {
            return true;
        }
        self.active_channel = ch;
        if velocity == 0. {
            self.channel_note_off(ch, key);
            true
        } else {
            self.note_on(self.channels[ch].preset as usize, key, velocity)
        }
    }
    pub fn channel_note_off(&mut self, ch: usize, key: i32) {
        if ch >= 16 {
            return;
        }
        let oldest = self
            .voices
            .iter()
            .filter(|v| {
                v.preset != -1
                    && v.channel == ch
                    && v.key == key
                    && v.amp.segment < RELEASE
                    && !v.held
            })
            .map(|v| v.play_index)
            .min();
        if let Some(index) = oldest {
            for v in &mut self.voices {
                if v.preset != -1
                    && v.channel == ch
                    && v.key == key
                    && v.play_index == index
                    && v.amp.segment < RELEASE
                {
                    if self.channels[ch].sustain {
                        v.held = true;
                    } else {
                        v.end(self.rate, false);
                    }
                }
            }
        }
    }
    pub fn channel_note_off_all(&mut self, ch: usize) {
        if ch >= 16 {
            return;
        }
        for v in &mut self.voices {
            if v.preset != -1 && v.channel == ch && v.amp.segment < RELEASE {
                v.end(self.rate, false);
            }
        }
    }
    pub fn channel_sounds_off_all(&mut self, ch: usize) {
        if ch >= 16 {
            return;
        }
        for v in &mut self.voices {
            if v.preset != -1
                && v.channel == ch
                && (v.amp.segment < RELEASE || v.amp.parameters.release != 0.)
            {
                v.end(self.rate, true);
            }
        }
    }
    pub fn channel_get_preset_index(&self, ch: usize) -> i32 {
        if ch < self.channel_num {
            self.channels[ch].preset as i32
        } else {
            0
        }
    }
    pub fn channel_get_preset_bank(&self, ch: usize) -> i32 {
        if ch < self.channel_num {
            (self.channels[ch].bank & 0x7fff) as i32
        } else {
            0
        }
    }
    pub fn channel_get_preset_number(&self, ch: usize) -> i32 {
        if ch < self.channel_num {
            self.font
                .presets
                .get(self.channels[ch].preset as usize)
                .map_or(0, |p| p.preset as i32)
        } else {
            0
        }
    }
    pub fn channel_get_pan(&self, ch: usize) -> f32 {
        if ch < self.channel_num {
            self.channels[ch].pan - 0.5
        } else {
            0.5
        }
    }
    pub fn channel_get_volume(&self, ch: usize) -> f32 {
        if ch < self.channel_num {
            db_gain(self.channels[ch].gain_db)
        } else {
            1.
        }
    }
    pub fn channel_get_pitchwheel(&self, ch: usize) -> i32 {
        if ch < self.channel_num {
            self.channels[ch].wheel as i32
        } else {
            8192
        }
    }
    pub fn channel_get_pitchrange(&self, ch: usize) -> f32 {
        if ch < self.channel_num {
            self.channels[ch].range
        } else {
            2.
        }
    }
    pub fn channel_get_tuning(&self, ch: usize) -> f32 {
        if ch < self.channel_num {
            self.channels[ch].tuning
        } else {
            0.
        }
    }
    pub fn channel_midi_control(&mut self, ch: usize, controller: i32, value: i32) -> bool {
        if !self.activate(ch) {
            return false;
        }
        let c = &mut self.channels[ch];
        let val = value as u16;
        match controller {
            7 => c.midi_volume = (c.midi_volume & 0x7f) | val.wrapping_shl(7),
            39 => c.midi_volume = (c.midi_volume & 0x3f80) | val,
            11 => c.midi_expression = (c.midi_expression & 0x7f) | val.wrapping_shl(7),
            43 => c.midi_expression = (c.midi_expression & 0x3f80) | val,
            10 => c.midi_pan = (c.midi_pan & 0x7f) | val.wrapping_shl(7),
            42 => c.midi_pan = (c.midi_pan & 0x3f80) | val,
            6 => c.midi_data = (c.midi_data & 0x7f) | val.wrapping_shl(7),
            38 => c.midi_data = (c.midi_data & 0x3f80) | val,
            0 => c.bank = 0x8000 | val,
            32 => {
                c.bank = (if c.bank & 0x8000 != 0 {
                    (c.bank & 0x7f) << 7
                } else {
                    0
                }) | val
            }
            101 => {
                c.midi_rpn =
                    (if c.midi_rpn == 0xffff { 0 } else { c.midi_rpn } & 0x7f) | val.wrapping_shl(7)
            }
            100 => c.midi_rpn = (if c.midi_rpn == 0xffff { 0 } else { c.midi_rpn } & 0x3f80) | val,
            98 | 99 => c.midi_rpn = 0xffff,
            64 => {
                self.channel_set_sustain(ch, value >= 64);
            }
            120 => self.channel_sounds_off_all(ch),
            123 => self.channel_note_off_all(ch),
            121 => {
                c.midi_volume = 16383;
                c.midi_expression = 16383;
                c.midi_pan = 8192;
                c.bank = 0;
                c.midi_rpn = 0xffff;
                c.midi_data = 0;
                self.channel_set_volume(ch, 1.);
                self.channel_set_pan(ch, 0.5);
                self.channel_set_pitchrange(ch, 2.);
                self.channel_set_tuning(ch, 0.);
            }
            _ => {}
        }
        let c = self.channels[ch];
        match controller {
            7 | 39 | 11 | 43 => {
                self.channel_set_volume(
                    ch,
                    ((c.midi_volume as f32 / 16383.) * (c.midi_expression as f32 / 16383.))
                        .powf(3.),
                );
            }
            10 | 42 => {
                self.channel_set_pan(ch, c.midi_pan as f32 / 16383.);
            }
            6 | 38 => match c.midi_rpn {
                0 => {
                    self.channel_set_pitchrange(
                        ch,
                        (c.midi_data >> 7) as f32 + 0.01 * (c.midi_data & 0x7f) as f32,
                    );
                }
                1 => {
                    self.channel_set_tuning(
                        ch,
                        c.tuning as i32 as f32 + (c.midi_data as f32 - 8192.) / 8192.,
                    );
                }
                2 if controller == 6 => {
                    self.channel_set_tuning(
                        ch,
                        (value as f32 - 64.) + (c.tuning - c.tuning as i32 as f32),
                    );
                }
                _ => {}
            },
            _ => {}
        }
        true
    }
}
