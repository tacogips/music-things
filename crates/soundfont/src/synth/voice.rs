// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
use super::{
    cents_hz,
    envelope::{VoiceEnvelope, RELEASE},
    filter::Filter,
    gain_db,
    lfo::Lfo,
    Synth,
};
use crate::font::Region;
#[derive(Clone, Copy, Default)]
pub(super) struct Voice {
    pub preset: i32,
    pub key: i32,
    pub channel: usize,
    pub play_index: u32,
    pub held: bool,
    pub region: Region,
    pub gain_db: f32,
    pub pan_left: f32,
    pub pan_right: f32,
    pub pitch_input: f64,
    pub pitch_factor: f64,
    pub position: f64,
    pub loop_start: u32,
    pub loop_end: u32,
    pub amp: VoiceEnvelope,
    pub modulation: VoiceEnvelope,
    pub filter: Filter,
    pub mod_lfo: Lfo,
    pub vib_lfo: Lfo,
}
impl Voice {
    pub fn pitch(&mut self, shift: f32, rate: f32) {
        let r = self.region;
        let note = self.key as f64 + r.transpose as f64 + r.tune as f64 / 100.;
        let adjusted = r.pitch_keycenter as f64
            + (note - r.pitch_keycenter as f64) * (r.pitch_keytrack as f64 / 100.)
            + shift as f64;
        self.pitch_input = adjusted * 100.;
        self.pitch_factor = r.sample_rate as f64
            / (2_f64.powf(r.pitch_keycenter as f64 * 100. / 1200.) * rate as f64);
    }
    pub fn pan(&mut self, pan: f32) {
        let pan = self.region.pan + pan;
        if pan <= -0.5 {
            self.pan_left = 1.;
            self.pan_right = 0.;
        } else if pan >= 0.5 {
            self.pan_left = 0.;
            self.pan_right = 1.;
        } else {
            self.pan_left = (0.5 - pan).sqrt();
            self.pan_right = (0.5 + pan).sqrt();
        }
    }
    pub fn end(&mut self, rate: f32, quick: bool) {
        if quick {
            self.amp.parameters.release = 0.;
            self.modulation.parameters.release = 0.;
        }
        self.amp.next(5, rate);
        self.modulation.next(5, rate);
        if !quick && self.region.loop_mode == 2 {
            self.loop_end = self.loop_start;
        }
    }
}
impl Synth {
    pub fn note_on(&mut self, preset_index: usize, key: i32, vel: f32) -> bool {
        if preset_index >= self.font.presets.len() {
            return true;
        }
        if vel <= 0. {
            self.note_off(preset_index, key);
            return true;
        }
        let velocity = (vel * 127.) as i16;
        let play_index = self.play_index;
        self.play_index = self.play_index.wrapping_add(1);
        for index in 0..self.font.presets[preset_index].regions.len() {
            let r = self.font.presets[preset_index].regions[index];
            if key < r.lokey as i32
                || key > r.hikey as i32
                || velocity < r.lovel as i16
                || velocity > r.hivel as i16
            {
                continue;
            }
            let mut free = None;
            for (i, v) in self.voices.iter_mut().enumerate() {
                if r.group != 0 && v.preset == preset_index as i32 && v.region.group == r.group {
                    v.end(self.rate, true);
                } else if v.preset == -1 && free.is_none() {
                    free = Some(i);
                    if r.group == 0 {
                        break;
                    }
                }
            }
            let slot = free.or_else(|| {
                self.voices
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.amp.segment == RELEASE)
                    .max_by_key(|(i, v)| {
                        (
                            v.amp.release_samples(self.rate) - v.amp.samples,
                            std::cmp::Reverse(*i),
                        )
                    })
                    .map(|(i, _)| i)
            });
            let Some(slot) = slot else {
                continue;
            };
            let v = &mut self.voices[slot];
            *v = Voice::default();
            v.region = r;
            v.preset = preset_index as i32;
            v.key = key;
            v.play_index = play_index;
            v.gain_db = self.gain_db - r.attenuation - gain_db(1. / vel);
            if self.channel_num != 0 {
                let c = self.channels[self.active_channel];
                v.channel = self.active_channel;
                v.gain_db += c.gain_db;
                v.pitch(c.pitch_shift(), self.rate);
                v.pan(c.pan);
            } else {
                v.pitch(0., self.rate);
                v.pan(0.);
            }
            v.position = r.offset as f64;
            if r.loop_mode != 0 && r.loop_start < r.loop_end {
                v.loop_start = r.loop_start;
                v.loop_end = r.loop_end;
            }
            v.amp.setup(r.ampenv, key, velocity, true, self.rate);
            v.modulation
                .setup(r.modenv, key, velocity, false, self.rate);
            let fc = if r.initial_filter_fc <= 13500 {
                cents_hz(r.initial_filter_fc as f32) / self.rate
            } else {
                1.
            };
            v.filter.q_inv = 1. / 10_f64.powf((r.initial_filter_q as f32 / 10.) as f64 / 20.);
            v.filter.active = fc < 0.499;
            if v.filter.active {
                v.filter.setup(fc);
            }
            v.mod_lfo.setup(r.delay_mod_lfo, r.freq_mod_lfo, self.rate);
            v.vib_lfo.setup(r.delay_vib_lfo, r.freq_vib_lfo, self.rate);
        }
        true
    }
    pub fn bank_note_on(&mut self, bank: i32, preset_number: i32, key: i32, vel: f32) -> bool {
        self.font
            .preset_index(bank, preset_number)
            .is_some_and(|p| self.note_on(p, key, vel))
    }
    pub fn note_off(&mut self, preset_index: usize, key: i32) {
        let oldest = self
            .voices
            .iter()
            .filter(|v| v.preset == preset_index as i32 && v.key == key && v.amp.segment < RELEASE)
            .map(|v| v.play_index)
            .min();
        if let Some(index) = oldest {
            for v in &mut self.voices {
                if v.preset == preset_index as i32
                    && v.key == key
                    && v.play_index == index
                    && v.amp.segment < RELEASE
                {
                    v.end(self.rate, false);
                }
            }
        }
    }
    pub fn bank_note_off(&mut self, bank: i32, preset_number: i32, key: i32) -> bool {
        if let Some(p) = self.font.preset_index(bank, preset_number) {
            self.note_off(p, key);
            true
        } else {
            false
        }
    }
    pub fn note_off_all(&mut self) {
        for v in &mut self.voices {
            if v.preset != -1 && v.amp.segment < RELEASE {
                v.end(self.rate, false);
            }
        }
    }
    pub fn active_voice_count(&self) -> usize {
        self.voices.iter().filter(|v| v.preset != -1).count()
    }
}
