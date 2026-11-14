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

use super::{MidiMessage, NOTE_ON, PROGRAM_CHANGE};

/// Statistics computed over a parsed MIDI message sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MidiInfo {
    pub used_channels: i32,
    pub used_programs: i32,
    pub total_notes: i32,
    pub time_first_note: u32,
    pub time_length: u32,
}

/// Compute the same statistics as `tml_get_info`.
#[must_use]
pub fn info(messages: &[MidiMessage]) -> MidiInfo {
    let mut result = MidiInfo::default();
    let mut channels = [false; 16];
    let mut programs = [false; 128];
    for msg in messages {
        result.time_length = msg.time_ms;
        if msg.kind == PROGRAM_CHANGE {
            if let Some(used) = programs.get_mut(usize::from(msg.program)) {
                if !*used {
                    *used = true;
                    result.used_programs += 1;
                }
            }
        }
        if msg.kind == NOTE_ON {
            if result.total_notes == 0 {
                result.time_first_note = msg.time_ms;
            }
            if let Some(used) = channels.get_mut(usize::from(msg.channel)) {
                if !*used {
                    *used = true;
                    result.used_channels += 1;
                }
            }
            result.total_notes += 1;
        }
    }
    result
}
