//! TinySoundFont synthesis with a fixed voice pool and sixteen lazy channels.
//!
//! `tsf_set_output` and `tsf_set_max_voices` map to [`Synth::new`];
//! `tsf_set_volume`, `tsf_reset`, `tsf_note_on`, `tsf_bank_note_on`,
//! `tsf_note_off`, `tsf_bank_note_off`, `tsf_note_off_all`,
//! `tsf_active_voice_count`, and `tsf_render_float` map to identically named
//! methods without the `tsf_` prefix. Channel methods also drop `tsf_`:
//! `channel_set_presetindex`, `channel_set_presetnumber`, `channel_set_bank`,
//! `channel_set_bank_preset`, `channel_set_pan`, `channel_set_volume`,
//! `channel_set_pitchwheel`, `channel_set_pitchrange`, `channel_set_tuning`,
//! `channel_set_sustain`, `channel_note_on`, `channel_note_off`,
//! `channel_note_off_all`, `channel_sounds_off_all`, `channel_midi_control`,
//! `channel_get_preset_index`, `channel_get_preset_bank`,
//! `channel_get_preset_number`, `channel_get_pan`, `channel_get_volume`,
//! `channel_get_pitchwheel`, `channel_get_pitchrange`, and `channel_get_tuning`.
//! Channels 16 and above are ignored.
// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
mod channel;
mod envelope;
mod filter;
mod lfo;
mod render;
mod voice;
use crate::font::SoundFont;
use channel::Channel;
use envelope::RELEASE;
use std::sync::Arc;
use voice::Voice;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    StereoInterleaved,
    StereoUnweaved,
    Mono,
}
#[derive(Debug, Clone, Copy)]
pub struct SynthConfig {
    pub output_mode: OutputMode,
    pub sample_rate: u32,
    pub global_gain_db: f32,
    pub max_voices: usize,
}
impl Default for SynthConfig {
    fn default() -> Self {
        Self {
            output_mode: OutputMode::StereoInterleaved,
            sample_rate: 44100,
            global_gain_db: 0.,
            max_voices: 64,
        }
    }
}
pub struct Synth {
    font: Arc<SoundFont>,
    voices: Vec<Voice>,
    channels: [Channel; 16],
    channel_num: usize,
    active_channel: usize,
    play_index: u32,
    mode: OutputMode,
    rate: f32,
    gain_db: f32,
}
impl Synth {
    pub fn new(font: Arc<SoundFont>, config: SynthConfig) -> Self {
        let mut voices = vec![Voice::default(); config.max_voices];
        for voice in &mut voices {
            voice.preset = -1;
        }
        Self {
            font,
            voices,
            channels: [Channel::default(); 16],
            channel_num: 0,
            active_channel: 0,
            play_index: 0,
            mode: config.output_mode,
            rate: if config.sample_rate == 0 {
                44100.
            } else {
                config.sample_rate as f32
            },
            gain_db: config.global_gain_db,
        }
    }
    pub fn set_volume(&mut self, volume: f32) {
        self.gain_db = if volume == 1. {
            0.
        } else {
            -gain_db(1. / volume)
        };
    }
    pub fn reset(&mut self) {
        for v in &mut self.voices {
            if v.preset != -1 && (v.amp.segment < RELEASE || v.amp.parameters.release != 0.) {
                v.end(self.rate, true);
            }
        }
        self.channel_num = 0;
        self.active_channel = 0;
    }
}
fn cents_hz(cents: f32) -> f32 {
    8.176 * 2_f32.powf(cents / 1200.)
}
fn db_gain(db: f32) -> f32 {
    if db > -100. {
        10_f32.powf(db * 0.05)
    } else {
        0.
    }
}
fn gain_db(gain: f32) -> f32 {
    if gain <= 0.00001 {
        -100.
    } else {
        (20. * (gain as f64).log10()) as f32
    }
}
