// Rust port of TinyMidiLoader tml.h, revision 853a0a171759f1ddba0de1442133a75912bbeffa.
// Copyright (C) 2017, 2018, 2020 Bernhard Schelling
//
// This software is provided 'as-is', without any express or implied
// warranty.  In no event will the authors be held liable for any damages
// arising from the use of this software.
//
// Permission is granted to anyone to use this software for any purpose,
// including commercial applications, and to alter it and redistribute it
// freely, subject to the following restrictions:
//
// 1. The origin of this software must not be misrepresented; you must not
//    claim that you wrote the original software. If you use this software
//    in a product, an acknowledgment in the product documentation would be
//    appreciated but is not required.
// 2. Altered source versions must be plainly marked as such, and must not be
//    misrepresented as being the original software.
// 3. This notice may not be removed or altered from any source distribution.

//! In-memory Standard MIDI File loader, ported from TinyMidiLoader.
mod info;
mod parse;
pub use info::{info, MidiInfo};
pub use parse::{parse, MidiError};

/// Note-off message type.
pub const NOTE_OFF: u8 = 0x80;
/// Note-on message type.
pub const NOTE_ON: u8 = 0x90;
/// Polyphonic key pressure message type.
pub const KEY_PRESSURE: u8 = 0xa0;
/// Controller change message type.
pub const CONTROL_CHANGE: u8 = 0xb0;
/// Program change message type.
pub const PROGRAM_CHANGE: u8 = 0xc0;
/// Channel pressure message type.
pub const CHANNEL_PRESSURE: u8 = 0xd0;
/// Pitch bend message type.
pub const PITCH_BEND: u8 = 0xe0;
/// Set-tempo meta event type.
pub const SET_TEMPO: u8 = 0x51;
/// End-of-track event with a nonzero delay.
pub const END_OF_TRACK: u8 = 0x2f;

/// A timestamped MIDI message. Fields unused by its kind are zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MidiMessage {
    pub time_ms: u32,
    pub kind: u8,
    pub channel: u8,
    pub key: u8,
    pub velocity: u8,
    pub control: u8,
    pub control_value: u8,
    pub program: u8,
    pub channel_pressure: u8,
    pub key_pressure: u8,
    pub pitch_bend: u16,
    pub tempo: [u8; 3],
}

impl MidiMessage {
    /// Tempo in microseconds per quarter note, or zero for other kinds.
    #[must_use]
    pub fn tempo_value(&self) -> u32 {
        if self.kind != SET_TEMPO {
            return 0;
        }
        u32::from(self.tempo[0]) << 16 | u32::from(self.tempo[1]) << 8 | u32::from(self.tempo[2])
    }
}
