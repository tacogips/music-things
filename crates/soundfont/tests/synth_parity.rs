#[path = "common/fnv.rs"]
mod fnv;
#[path = "common/midi_builder.rs"]
mod midi_builder;
#[path = "common/sf2_builder.rs"]
mod sf2_builder;

use music_things_soundfont::{
    font::SoundFont,
    midi,
    synth::{OutputMode, Synth, SynthConfig},
};
use std::{path::PathBuf, sync::Arc};

const BLOCKS: usize = 1378;
const FRAMES: usize = 64;

fn synth(mode: OutputMode, gain: f32, voices: usize) -> Synth {
    Synth::new(
        Arc::new(SoundFont::load(&sf2_builder::synthetic_sf2()).unwrap()),
        SynthConfig {
            output_mode: mode,
            sample_rate: 44100,
            global_gain_db: gain,
            max_voices: voices,
        },
    )
}
fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/synth")
            .join(name),
    )
    .unwrap()
}
#[test]
#[ignore]
fn export_synth_inputs() {
    let dir = PathBuf::from(
        std::env::var_os("SOUNDFONT_FIXTURE_OUT").expect("SOUNDFONT_FIXTURE_OUT required"),
    );
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("synthetic.sf2"), sf2_builder::synthetic_sf2()).unwrap();
    std::fs::write(dir.join("scenario.mid"), midi_builder::scenario_midi()).unwrap();
}
#[test]
fn input_hashes() {
    let hashes = String::from_utf8(fixture("inputs.txt")).unwrap();
    for (name, bytes) in [
        ("synthetic", sf2_builder::synthetic_sf2()),
        ("scenario", midi_builder::scenario_midi()),
    ] {
        assert!(hashes
            .lines()
            .any(|line| line == format!("{name}={:016x}", fnv::fnv1a64(&bytes))));
    }
}
// Pinned upstream on aarch64-apple-darwin: S1 and S2 max-abs 0.000000000.
fn compare(name: &str, actual: &[f32]) {
    let bytes = fixture(name);
    assert_eq!(bytes.len(), actual.len() * 4);
    let mut max = 0.0_f32;
    for (index, (raw, sample)) in bytes.chunks_exact(4).zip(actual).enumerate() {
        let reference = f32::from_le_bytes(raw.try_into().unwrap());
        assert!(
            reference.is_finite() && sample.is_finite(),
            "non-finite at {index}"
        );
        max = max.max((reference - sample).abs());
    }
    eprintln!("{name}: max-abs {max:.9}");
    assert!(max <= 1e-5, "{name}: max-abs {max}");
}
fn retain(block: &[f32], frame_base: usize, channels: usize, out: &mut Vec<f32>) {
    for frame in 0..FRAMES {
        let position = frame_base + frame;
        if position < 2048 || position % 16 == 0 {
            out.extend_from_slice(&block[frame * channels..(frame + 1) * channels]);
        }
    }
}
#[test]
fn render_s1_parity() {
    let mut synth = synth(OutputMode::StereoInterleaved, 0.0, 32);
    synth.channel_set_bank_preset(9, 128, 0);
    let messages = midi::parse(&midi_builder::scenario_midi()).unwrap();
    let mut next = 0;
    let mut msec = 0.0_f64;
    let mut actual = Vec::new();
    let mut block = [0.0; FRAMES * 2];
    for k in 0..BLOCKS {
        msec += 64.0 * (1000.0 / 44100.0);
        while next < messages.len() && f64::from(messages[next].time_ms) <= msec {
            let msg = messages[next];
            let ch = usize::from(msg.channel);
            match msg.kind {
                midi::PROGRAM_CHANGE => {
                    synth.channel_set_presetnumber(ch, i32::from(msg.program), ch == 9);
                }
                midi::NOTE_ON => {
                    synth.channel_note_on(ch, i32::from(msg.key), f32::from(msg.velocity) / 127.0);
                }
                midi::NOTE_OFF => {
                    synth.channel_note_off(ch, i32::from(msg.key));
                }
                midi::PITCH_BEND => {
                    synth.channel_set_pitchwheel(ch, i32::from(msg.pitch_bend));
                }
                midi::CONTROL_CHANGE => {
                    synth.channel_midi_control(
                        ch,
                        i32::from(msg.control),
                        i32::from(msg.control_value),
                    );
                }
                _ => {}
            }
            next += 1;
        }
        synth.render_float(&mut block, FRAMES, false);
        retain(&block, k * FRAMES, 2, &mut actual);
    }
    compare("render_s1.f32", &actual);
}
#[test]
fn render_s2_parity() {
    let mut synth = synth(OutputMode::Mono, -6.0, 8);
    let mut actual = Vec::new();
    let mut block = [0.125; FRAMES];
    for k in 0..BLOCKS {
        match k {
            0 => {
                synth.channel_set_presetnumber(0, 0, false);
                for key in 48..=60 {
                    synth.channel_note_on(0, key, 0.8);
                }
            }
            100 => {
                synth.channel_set_pitchrange(0, 12.0);
                synth.channel_set_pitchwheel(0, 12000);
            }
            200 => {
                synth.channel_midi_control(0, 64, 127);
            }
            220 => {
                for key in 48..=60 {
                    synth.channel_note_off(0, key);
                }
            }
            400 => {
                synth.channel_midi_control(0, 64, 0);
            }
            500 => {
                synth.note_on(1, 72, 1.0);
            }
            600 => {
                synth.channel_set_pan(0, 0.2);
                synth.channel_set_volume(0, 0.5);
            }
            800 => synth.note_off_all(),
            900 => {
                synth.channel_note_on(0, 60, 0.5);
            }
            1000 => {
                synth.channel_sounds_off_all(0);
            }
            _ => {}
        }
        block.fill(0.125);
        synth.render_float(&mut block, FRAMES, true);
        retain(&block, k * FRAMES, 1, &mut actual);
    }
    compare("render_s2.f32", &actual);
}
#[test]
fn inactive_channels_and_cap() {
    let mut s = synth(OutputMode::Mono, 0.0, 8);
    assert!(s.channel_note_on(3, 60, 1.0));
    assert_eq!(s.active_voice_count(), 0);
    assert_eq!(s.channel_get_pan(3), 0.5);
    assert_eq!(s.channel_get_pitchrange(3), 2.0);
    assert!(!s.channel_set_pan(16, 0.5));
}
#[test]
fn activating_channel_initializes_preceding_channels() {
    let mut s = synth(OutputMode::Mono, 0.0, 8);
    assert!(s.channel_set_pan(5, 0.25));
    for ch in 0..5 {
        assert_eq!(s.channel_get_pan(ch), -0.5);
        assert_eq!(s.channel_get_pitchrange(ch), 2.0);
    }
    assert_eq!(s.channel_get_pan(5), -0.75);
    assert!(s.channel_note_on(2, 60, 0.8));
    assert!(s.active_voice_count() > 0);
}
#[test]
fn reset_deactivates_channels() {
    let mut s = synth(OutputMode::Mono, 0.0, 8);
    s.channel_set_pan(0, 0.5);
    s.reset();
    assert!(s.channel_note_on(0, 60, 0.8));
    assert_eq!(s.active_voice_count(), 0);
    s.channel_set_pan(0, 0.5);
    assert!(s.channel_note_on(0, 60, 0.8));
    assert!(s.active_voice_count() > 0);
}
#[test]
fn inactive_channel_still_scans_releasing_voices() {
    let mut s = synth(OutputMode::Mono, 0.0, 8);
    s.channel_set_presetnumber(0, 0, false);
    s.channel_note_on(0, 60, 0.8);
    assert!(s.active_voice_count() > 0);
    s.render_float(&mut [0.0; FRAMES], FRAMES, false);
    s.reset();
    s.channel_sounds_off_all(0);
    s.render_float(&mut [0.0; 512], 512, false);
    assert_eq!(s.active_voice_count(), 0);
}
