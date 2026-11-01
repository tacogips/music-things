mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

fn check_reset_preserves_params(name: &str) {
    let mut effect = music_things_airwindows::create_with_seed(name, 44_100.0, 123).unwrap();
    for index in 0..effect.param_count() {
        effect.set_param(index, 0.2 + index as f32 * 0.1);
    }
    effect.reset();
    for index in 0..effect.param_count() {
        assert_eq!(
            effect.get_param(index),
            0.2 + index as f32 * 0.1,
            "{name} parameter {index}"
        );
    }
}

const RECORD_ATMOSPHEREBUSS: ParityRecord = ParityRecord {
    name: "AtmosphereBuss",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn atmospherebuss() {
    check_reset_preserves_params("AtmosphereBuss");
    check_plugin(
        &RECORD_ATMOSPHEREBUSS,
        include_str!("fixtures/batch1/g02/AtmosphereBuss.txt"),
        include_bytes!("fixtures/batch1/g02/AtmosphereBuss.f32"),
    );
}

const RECORD_ATMOSPHERECHANNEL: ParityRecord = ParityRecord {
    name: "AtmosphereChannel",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn atmospherechannel() {
    check_reset_preserves_params("AtmosphereChannel");
    check_plugin(
        &RECORD_ATMOSPHERECHANNEL,
        include_str!("fixtures/batch1/g02/AtmosphereChannel.txt"),
        include_bytes!("fixtures/batch1/g02/AtmosphereChannel.f32"),
    );
}

const RECORD_AURA: ParityRecord = ParityRecord {
    name: "Aura",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn aura() {
    check_reset_preserves_params("Aura");
    check_plugin(
        &RECORD_AURA,
        include_str!("fixtures/batch1/g02/Aura.txt"),
        include_bytes!("fixtures/batch1/g02/Aura.f32"),
    );
}

const RECORD_AUTOPAN: ParityRecord = ParityRecord {
    name: "AutoPan",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn autopan() {
    check_reset_preserves_params("AutoPan");
    check_plugin(
        &RECORD_AUTOPAN,
        include_str!("fixtures/batch1/g02/AutoPan.txt"),
        include_bytes!("fixtures/batch1/g02/AutoPan.f32"),
    );
}

const RECORD_AVERAGE: ParityRecord = ParityRecord {
    name: "Average",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn average() {
    check_reset_preserves_params("Average");
    check_plugin(
        &RECORD_AVERAGE,
        include_str!("fixtures/batch1/g02/Average.txt"),
        include_bytes!("fixtures/batch1/g02/Average.f32"),
    );
}

const RECORD_AVERMATRIX: ParityRecord = ParityRecord {
    name: "AverMatrix",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn avermatrix() {
    check_reset_preserves_params("AverMatrix");
    check_plugin(
        &RECORD_AVERMATRIX,
        include_str!("fixtures/batch1/g02/AverMatrix.txt"),
        include_bytes!("fixtures/batch1/g02/AverMatrix.f32"),
    );
}

const RECORD_BALANCED: ParityRecord = ParityRecord {
    name: "Balanced",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn balanced() {
    check_reset_preserves_params("Balanced");
    check_plugin(
        &RECORD_BALANCED,
        include_str!("fixtures/batch1/g02/Balanced.txt"),
        include_bytes!("fixtures/batch1/g02/Balanced.f32"),
    );
}

const RECORD_BASSAMP: ParityRecord = ParityRecord {
    name: "BassAmp",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn bassamp() {
    check_reset_preserves_params("BassAmp");
    check_plugin(
        &RECORD_BASSAMP,
        include_str!("fixtures/batch1/g02/BassAmp.txt"),
        include_bytes!("fixtures/batch1/g02/BassAmp.f32"),
    );
}
