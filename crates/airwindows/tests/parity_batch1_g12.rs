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

const RECORD_CURVE: ParityRecord = ParityRecord {
    name: "curve",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on acceptance host",
};

#[test]
fn curve() {
    check_reset_preserves_params("curve");
    check_plugin(
        &RECORD_CURVE,
        include_str!("fixtures/batch1/g12/curve.txt"),
        include_bytes!("fixtures/batch1/g12/curve.f32"),
    );
}

const RECORD_DARK: ParityRecord = ParityRecord {
    name: "Dark",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on acceptance host",
};

#[test]
fn dark() {
    check_reset_preserves_params("Dark");
    check_plugin(
        &RECORD_DARK,
        include_str!("fixtures/batch1/g12/Dark.txt"),
        include_bytes!("fixtures/batch1/g12/Dark.f32"),
    );
}

const RECORD_DARKNOISE: ParityRecord = ParityRecord {
    name: "DarkNoise",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on acceptance host",
};

#[test]
fn darknoise() {
    check_reset_preserves_params("DarkNoise");
    check_plugin(
        &RECORD_DARKNOISE,
        include_str!("fixtures/batch1/g12/DarkNoise.txt"),
        include_bytes!("fixtures/batch1/g12/DarkNoise.f32"),
    );
}

const RECORD_DCVOLTAGE: ParityRecord = ParityRecord {
    name: "DCVoltage",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "measured on acceptance host",
};

#[test]
fn dcvoltage() {
    check_reset_preserves_params("DCVoltage");
    check_plugin(
        &RECORD_DCVOLTAGE,
        include_str!("fixtures/batch1/g12/DCVoltage.txt"),
        include_bytes!("fixtures/batch1/g12/DCVoltage.f32"),
    );
}

const RECORD_DEBESS: ParityRecord = ParityRecord {
    name: "DeBess",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on acceptance host",
};

#[test]
fn debess() {
    check_reset_preserves_params("DeBess");
    check_plugin(
        &RECORD_DEBESS,
        include_str!("fixtures/batch1/g12/DeBess.txt"),
        include_bytes!("fixtures/batch1/g12/DeBess.f32"),
    );
}

const RECORD_DECKWRECKA: ParityRecord = ParityRecord {
    name: "Deckwrecka",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on acceptance host",
};

#[test]
fn deckwrecka() {
    check_reset_preserves_params("Deckwrecka");
    check_plugin(
        &RECORD_DECKWRECKA,
        include_str!("fixtures/batch1/g12/Deckwrecka.txt"),
        include_bytes!("fixtures/batch1/g12/Deckwrecka.f32"),
    );
}

const RECORD_DEESS: ParityRecord = ParityRecord {
    name: "DeEss",
    class: ParityClass::Libm,
    measured_max_abs: 5.1315766e-8,
    note: "measured on acceptance host",
};

#[test]
fn deess() {
    check_reset_preserves_params("DeEss");
    check_plugin(
        &RECORD_DEESS,
        include_str!("fixtures/batch1/g12/DeEss.txt"),
        include_bytes!("fixtures/batch1/g12/DeEss.f32"),
    );
}

const RECORD_DEHISS: ParityRecord = ParityRecord {
    name: "DeHiss",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "measured on acceptance host",
};

#[test]
fn dehiss() {
    check_reset_preserves_params("DeHiss");
    check_plugin(
        &RECORD_DEHISS,
        include_str!("fixtures/batch1/g12/DeHiss.txt"),
        include_bytes!("fixtures/batch1/g12/DeHiss.f32"),
    );
}
