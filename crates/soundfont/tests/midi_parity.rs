#[path = "common/fnv.rs"]
mod fnv;
#[path = "common/midi_builder.rs"]
mod midi_builder;

use music_things_soundfont::midi::{self, MidiMessage};
use std::fmt::Write as _;
use std::path::PathBuf;

fn fixture(name: &str) -> String {
    std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/tml")
            .join(name),
    )
    .expect("fixture must exist")
}

fn check_hash(name: &str, bytes: &[u8]) {
    let hashes = fixture("inputs.txt");
    let expected = hashes
        .lines()
        .find_map(|line| line.strip_prefix(&format!("{name}=")))
        .expect("input hash must exist");
    assert_eq!(format!("{:016x}", fnv::fnv1a64(bytes)), expected);
}

fn dump(messages: &[MidiMessage]) -> String {
    let mut output = String::new();
    for message in messages {
        let (first, second) = match message.kind {
            midi::NOTE_OFF | midi::NOTE_ON => (u32::from(message.key), u32::from(message.velocity)),
            midi::KEY_PRESSURE => (u32::from(message.key), u32::from(message.key_pressure)),
            midi::CONTROL_CHANGE => (u32::from(message.control), u32::from(message.control_value)),
            midi::PROGRAM_CHANGE => (u32::from(message.program), 0),
            midi::CHANNEL_PRESSURE => (u32::from(message.channel_pressure), 0),
            midi::PITCH_BEND => (u32::from(message.pitch_bend), 0),
            midi::SET_TEMPO => (message.tempo_value(), 0),
            _ => (0, 0),
        };
        writeln!(
            output,
            "{} {:02x} {} {first} {second}",
            message.time_ms, message.kind, message.channel
        )
        .unwrap();
    }
    let info = midi::info(messages);
    writeln!(
        output,
        "info {} {} {} {} {}",
        info.used_channels,
        info.used_programs,
        info.total_notes,
        info.time_first_note,
        info.time_length
    )
    .unwrap();
    output
}

#[test]
#[ignore]
fn export_synthetic_midi() {
    let dir = std::env::var_os("SOUNDFONT_FIXTURE_OUT").expect("SOUNDFONT_FIXTURE_OUT required");
    std::fs::create_dir_all(&dir).unwrap();
    for (name, bytes) in [
        ("scenario.mid", midi_builder::scenario_midi()),
        ("format0.mid", midi_builder::format0_midi()),
        ("truncated.mid", midi_builder::truncated_midi()),
        ("truncated_late.mid", midi_builder::truncated_late_midi()),
        ("bad_meta.mid", midi_builder::bad_meta_midi()),
    ] {
        std::fs::write(PathBuf::from(&dir).join(name), bytes).unwrap();
    }
}

#[test]
fn scenario() {
    let bytes = midi_builder::scenario_midi();
    check_hash("scenario", &bytes);
    assert_eq!(dump(&midi::parse(&bytes).unwrap()), fixture("scenario.txt"));
}

#[test]
fn format0() {
    let bytes = midi_builder::format0_midi();
    check_hash("format0", &bytes);
    assert_eq!(dump(&midi::parse(&bytes).unwrap()), fixture("format0.txt"));
}

#[test]
fn truncated() {
    let bytes = midi_builder::truncated_midi();
    check_hash("truncated", &bytes);
    assert!(midi::parse(&bytes).is_err());
    assert_eq!(fixture("truncated.txt"), "error\n");
}

#[test]
fn truncated_late() {
    let bytes = midi_builder::truncated_late_midi();
    check_hash("truncated_late", &bytes);
    assert_eq!(
        dump(&midi::parse(&bytes).unwrap()),
        fixture("truncated_late.txt")
    );
}

#[test]
fn bad_meta() {
    let bytes = midi_builder::bad_meta_midi();
    check_hash("bad_meta", &bytes);
    assert_eq!(dump(&midi::parse(&bytes).unwrap()), fixture("bad_meta.txt"));
}

#[test]
fn tempo_values() {
    let messages = midi::parse(&midi_builder::scenario_midi()).unwrap();
    let tempos: Vec<_> = messages
        .iter()
        .filter(|message| message.kind == midi::SET_TEMPO)
        .map(MidiMessage::tempo_value)
        .collect();
    assert_eq!(tempos, [500_000, 400_000]);
}
