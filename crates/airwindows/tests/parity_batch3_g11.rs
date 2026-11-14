mod common;

use common::parity::{check_plugin, ParityClass, ParityRecord};

fn check_reset_preserves_params(name: &str) {
    let mut effect = music_things_airwindows::create_with_seed(name, 44100.0, 0x00c0_ffee).unwrap();
    let count = effect.info().params.len();
    for i in 0..count {
        effect.set_param(i, (i + 1) as f32 / (count + 2) as f32);
    }
    effect.reset();
    for i in 0..count {
        assert_eq!(
            effect.get_param(i),
            (i + 1) as f32 / (count + 2) as f32,
            "{name} parameter {i}"
        );
    }
}

const RECORD_UNBOX: ParityRecord = ParityRecord {
    name: "UnBox",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn unbox() {
    check_reset_preserves_params("UnBox");
    check_plugin(
        &RECORD_UNBOX,
        include_str!("fixtures/batch3/g11/UnBox.txt"),
        include_bytes!("fixtures/batch3/g11/UnBox.f32"),
    );
}

const RECORD_VOICEOFTHESTARSHIP: ParityRecord = ParityRecord {
    name: "VoiceOfTheStarship",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn voiceofthestarship() {
    check_reset_preserves_params("VoiceOfTheStarship");
    check_plugin(
        &RECORD_VOICEOFTHESTARSHIP,
        include_str!("fixtures/batch3/g11/VoiceOfTheStarship.txt"),
        include_bytes!("fixtures/batch3/g11/VoiceOfTheStarship.f32"),
    );
}

const RECORD_VARIMU: ParityRecord = ParityRecord {
    name: "VariMu",
    class: ParityClass::Libm,
    measured_max_abs: 5.3941392e-8,
    note: "Upstream static noise counters persist across fixture renders",
};
#[test]
fn varimu() {
    check_reset_preserves_params("VariMu");
    check_plugin(
        &RECORD_VARIMU,
        include_str!("fixtures/batch3/g11/VariMu.txt"),
        include_bytes!("fixtures/batch3/g11/VariMu.f32"),
    );
}

const RECORD_VERBITY: ParityRecord = ParityRecord {
    name: "Verbity",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn verbity() {
    check_reset_preserves_params("Verbity");
    check_plugin(
        &RECORD_VERBITY,
        include_str!("fixtures/batch3/g11/Verbity.txt"),
        include_bytes!("fixtures/batch3/g11/Verbity.f32"),
    );
}

const RECORD_VERBITY2: ParityRecord = ParityRecord {
    name: "Verbity2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn verbity2() {
    check_reset_preserves_params("Verbity2");
    check_plugin(
        &RECORD_VERBITY2,
        include_str!("fixtures/batch3/g11/Verbity2.txt"),
        include_bytes!("fixtures/batch3/g11/Verbity2.f32"),
    );
}

const RECORD_VIBRATO: ParityRecord = ParityRecord {
    name: "Vibrato",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_VINYLDITHER: ParityRecord = ParityRecord {
    name: "VinylDither",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn vinyldither() {
    check_reset_preserves_params("VinylDither");
    check_plugin(
        &RECORD_VINYLDITHER,
        include_str!("fixtures/batch3/g11/VinylDither.txt"),
        include_bytes!("fixtures/batch3/g11/VinylDither.f32"),
    );
}
#[test]
fn vibrato() {
    check_reset_preserves_params("Vibrato");
    check_plugin(
        &RECORD_VIBRATO,
        include_str!("fixtures/batch3/g11/Vibrato.txt"),
        include_bytes!("fixtures/batch3/g11/Vibrato.f32"),
    );
}

const RECORD_ULTRASONX: ParityRecord = ParityRecord {
    name: "UltrasonX",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn ultrasonx() {
    check_reset_preserves_params("UltrasonX");
    check_plugin(
        &RECORD_ULTRASONX,
        include_str!("fixtures/batch3/g11/UltrasonX.txt"),
        include_bytes!("fixtures/batch3/g11/UltrasonX.f32"),
    );
}
