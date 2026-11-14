mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

fn check_reset_preserves_params(name: &str) {
    let mut effect = music_things_airwindows::create_with_seed(name, 44_100.0, 123).unwrap();
    for index in 0..effect.param_count() {
        effect.set_param(index, 0.37);
    }
    effect.reset();
    for index in 0..effect.param_count() {
        assert_eq!(effect.get_param(index), 0.37, "{name} parameter {index}");
    }
}
const RECORD_ACCELERATION: ParityRecord = ParityRecord {
    name: "Acceleration",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on aarch64-apple-darwin",
};
#[test]
fn acceleration() {
    check_reset_preserves_params("Acceleration");
    check_plugin(
        &RECORD_ACCELERATION,
        include_str!("fixtures/batch1/g01/Acceleration.txt"),
        include_bytes!("fixtures/batch1/g01/Acceleration.f32"),
    );
}

const RECORD_ADT: ParityRecord = ParityRecord {
    name: "ADT",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on aarch64-apple-darwin",
};
#[test]
fn adt() {
    check_reset_preserves_params("ADT");
    check_plugin(
        &RECORD_ADT,
        include_str!("fixtures/batch1/g01/ADT.txt"),
        include_bytes!("fixtures/batch1/g01/ADT.f32"),
    );
}

const RECORD_ACCELERATION2: ParityRecord = ParityRecord {
    name: "Acceleration2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on aarch64-apple-darwin",
};
#[test]
fn acceleration2() {
    check_reset_preserves_params("Acceleration2");
    check_plugin(
        &RECORD_ACCELERATION2,
        include_str!("fixtures/batch1/g01/Acceleration2.txt"),
        include_bytes!("fixtures/batch1/g01/Acceleration2.f32"),
    );
}

const RECORD_AIR: ParityRecord = ParityRecord {
    name: "Air",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on aarch64-apple-darwin",
};
#[test]
fn air() {
    check_reset_preserves_params("Air");
    check_plugin(
        &RECORD_AIR,
        include_str!("fixtures/batch1/g01/Air.txt"),
        include_bytes!("fixtures/batch1/g01/Air.f32"),
    );
}

const RECORD_APICOLYPSE: ParityRecord = ParityRecord {
    name: "Apicolypse",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on aarch64-apple-darwin",
};
#[test]
fn apicolypse() {
    check_reset_preserves_params("Apicolypse");
    check_plugin(
        &RECORD_APICOLYPSE,
        include_str!("fixtures/batch1/g01/Apicolypse.txt"),
        include_bytes!("fixtures/batch1/g01/Apicolypse.f32"),
    );
}

const RECORD_AIR2: ParityRecord = ParityRecord {
    name: "Air2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on aarch64-apple-darwin",
};
#[test]
fn air2() {
    check_reset_preserves_params("Air2");
    check_plugin(
        &RECORD_AIR2,
        include_str!("fixtures/batch1/g01/Air2.txt"),
        include_bytes!("fixtures/batch1/g01/Air2.f32"),
    );
}

const RECORD_AQUICKVOICECLIP: ParityRecord = ParityRecord {
    name: "AQuickVoiceClip",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on aarch64-apple-darwin",
};
#[test]
fn aquickvoiceclip() {
    check_reset_preserves_params("AQuickVoiceClip");
    check_plugin(
        &RECORD_AQUICKVOICECLIP,
        include_str!("fixtures/batch1/g01/AQuickVoiceClip.txt"),
        include_bytes!("fixtures/batch1/g01/AQuickVoiceClip.f32"),
    );
}

const RECORD_ADCLIP7: ParityRecord = ParityRecord {
    name: "ADClip7",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on aarch64-apple-darwin",
};
#[test]
fn adclip7() {
    check_reset_preserves_params("ADClip7");
    check_plugin(
        &RECORD_ADCLIP7,
        include_str!("fixtures/batch1/g01/ADClip7.txt"),
        include_bytes!("fixtures/batch1/g01/ADClip7.f32"),
    );
}
