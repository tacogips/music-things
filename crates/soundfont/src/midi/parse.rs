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

use super::{
    MidiMessage, CHANNEL_PRESSURE, CONTROL_CHANGE, END_OF_TRACK, KEY_PRESSURE, NOTE_OFF, NOTE_ON,
    PITCH_BEND, PROGRAM_CHANGE, SET_TEMPO,
};
use std::fmt;

/// Errors encountered while loading a MIDI file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidiError {
    InvalidHeader,
    Truncated,
    UnsupportedFormat,
    Empty,
}
impl fmt::Display for MidiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidHeader => "invalid MIDI header",
            Self::Truncated => "truncated or malformed MIDI data",
            Self::UnsupportedFormat => "unsupported MIDI format or timing",
            Self::Empty => "MIDI contains no messages",
        })
    }
}
impl std::error::Error for MidiError {}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}
impl<'a> Reader<'a> {
    fn take(&mut self, len: usize) -> Result<&'a [u8], MidiError> {
        let end = self.pos.checked_add(len).ok_or(MidiError::Truncated)?;
        let data = self.bytes.get(self.pos..end).ok_or(MidiError::Truncated)?;
        self.pos = end;
        Ok(data)
    }
    fn byte(&mut self) -> Result<u8, MidiError> {
        Ok(self.take(1)?[0])
    }
    fn variable(&mut self) -> Result<u32, MidiError> {
        let mut n = 0;
        for _ in 0..4 {
            let b = self.byte()?;
            n = (n << 7) | u32::from(b & 0x7f);
            if b & 0x80 == 0 {
                return Ok(n);
            }
        }
        Err(MidiError::Truncated)
    }
}
#[derive(Default)]
struct Track {
    events: Vec<(u32, MidiMessage)>,
    idx: usize,
    ticks: u32,
}

/// Parse an in-memory Standard MIDI File into time-ordered messages.
pub fn parse(bytes: &[u8]) -> Result<Vec<MidiMessage>, MidiError> {
    let mut file = Reader { bytes, pos: 0 };
    let h = file.take(14)?;
    if &h[..4] != b"MThd" || h[7] != 6 {
        return Err(MidiError::InvalidHeader);
    }
    if h[9] > 2 || h[12] & 0x80 != 0 {
        return Err(MidiError::UnsupportedFormat);
    }
    let count = u16::from_be_bytes([h[10], h[11]]) as usize;
    let division = u16::from_be_bytes([h[12], h[13]]);
    if count == 0 || division == 0 {
        return Err(MidiError::InvalidHeader);
    }
    let mut tracks = Vec::with_capacity(count);
    let mut last_status = 0u8;
    for _ in 0..count {
        let Ok(header) = file.take(8) else { break };
        if &header[..4] != b"MTrk" {
            break;
        }
        let len = u32::from_be_bytes([header[4], header[5], header[6], header[7]]) as usize;
        let Ok(track_bytes) = file.take(len) else {
            break;
        };
        let mut track = Reader {
            bytes: track_bytes,
            pos: 0,
        };
        let mut events = Vec::new();
        while track.pos < track.bytes.len() {
            let parse_message = (|| -> Result<bool, MidiError> {
                // Upstream maps an invalid delta VLQ to zero and then reads status.
                let mut delta = track.variable().unwrap_or(0);
                if delta & 0xfff0_0000 != 0 {
                    delta = 0;
                }
                let mut status = track.byte()?;
                if status & 0x80 == 0 {
                    if last_status & 0x80 == 0 {
                        return Err(MidiError::Truncated);
                    }
                    track.pos -= 1;
                    status = last_status;
                } else {
                    last_status = status;
                }
                let mut msg = MidiMessage::default();
                match status {
                    0xf0 | 0xf7 => {
                        let len = track.variable()? as usize;
                        track.take(len)?;
                    }
                    0xff => {
                        let kind = track.byte()?;
                        let len = track.variable()? as usize;
                        let data = track.take(len)?;
                        match kind {
                            END_OF_TRACK => {
                                if len != 0 {
                                    return Err(MidiError::Truncated);
                                }
                                if delta == 0 {
                                    return Ok(true);
                                }
                                msg.kind = END_OF_TRACK;
                            }
                            SET_TEMPO => {
                                if len != 3 {
                                    return Err(MidiError::Truncated);
                                }
                                msg.kind = SET_TEMPO;
                                // tml_tempomsg overlays its first tempo byte on channel.
                                msg.channel = data[0];
                                msg.tempo.copy_from_slice(data);
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        let first = track.byte()? & 0x7f;
                        msg.kind = status & 0xf0;
                        msg.channel = status & 0x0f;
                        match msg.kind {
                            NOTE_OFF | NOTE_ON => {
                                msg.key = first;
                                msg.velocity = track.byte()? & 0x7f;
                            }
                            KEY_PRESSURE => {
                                msg.key = first;
                                msg.key_pressure = track.byte()? & 0x7f;
                            }
                            CONTROL_CHANGE => {
                                msg.control = first;
                                msg.control_value = track.byte()? & 0x7f;
                            }
                            PROGRAM_CHANGE => msg.program = first,
                            CHANNEL_PRESSURE => msg.channel_pressure = first,
                            PITCH_BEND => {
                                msg.pitch_bend =
                                    u16::from(first) | (u16::from(track.byte()? & 0x7f) << 7)
                            }
                            _ => {
                                msg.kind = 0;
                                msg.channel = 0;
                            }
                        }
                    }
                }
                if delta != 0 || msg.kind != 0 {
                    events.push((delta, msg));
                }
                Ok(msg.kind == END_OF_TRACK)
            })();
            if !matches!(parse_message, Ok(false)) {
                break;
            }
        }
        tracks.push(Track {
            events,
            ..Track::default()
        });
    }
    let mut result = Vec::new();
    let (mut ticks, mut tempo_ticks, mut tempo_ms) = (0u32, 0u32, 0i32);
    let mut ticks_to_ms = 500_000.0 / (1000.0 * f64::from(division));
    loop {
        let ms = tempo_ms + ((ticks.wrapping_sub(tempo_ticks) as f64) * ticks_to_ms) as i32;
        let mut next: Option<u32> = None;
        for track in &mut tracks {
            while let Some(&(delta, mut msg)) = track.events.get(track.idx) {
                let at = track.ticks.wrapping_add(delta);
                if at != ticks {
                    if at > ticks {
                        next = Some(next.map_or(at, |n| n.min(at)));
                    }
                    break;
                }
                track.ticks = at;
                track.idx += 1;
                if msg.kind == SET_TEMPO {
                    ticks_to_ms = f64::from(msg.tempo_value()) / (1000.0 * f64::from(division));
                    tempo_ms = ms;
                    tempo_ticks = ticks;
                }
                if msg.kind != 0 {
                    msg.time_ms = ms as u32;
                    result.push(msg);
                }
            }
        }
        match next {
            Some(n) => ticks = n,
            None => break,
        }
    }
    if result.is_empty() {
        Err(MidiError::Empty)
    } else {
        Ok(result)
    }
}
