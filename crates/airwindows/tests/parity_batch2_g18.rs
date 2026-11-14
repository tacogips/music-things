mod common;

use common::parity::{check_plugin, ParityClass, ParityRecord};

const RECORD_LOUD: ParityRecord = ParityRecord {
    name: "Loud",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn loud() {
    check_plugin(
        &RECORD_LOUD,
        include_str!("fixtures/batch2/g18/Loud.txt"),
        include_bytes!("fixtures/batch2/g18/Loud.f32"),
    );
}

const RECORD_LOWPASS: ParityRecord = ParityRecord {
    name: "Lowpass",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn lowpass() {
    check_plugin(
        &RECORD_LOWPASS,
        include_str!("fixtures/batch2/g18/Lowpass.txt"),
        include_bytes!("fixtures/batch2/g18/Lowpass.f32"),
    );
}

const RECORD_LOWPASS2: ParityRecord = ParityRecord {
    name: "Lowpass2",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn lowpass2() {
    check_plugin(
        &RECORD_LOWPASS2,
        include_str!("fixtures/batch2/g18/Lowpass2.txt"),
        include_bytes!("fixtures/batch2/g18/Lowpass2.f32"),
    );
}

const RECORD_LRFLIPTIMER: ParityRecord = ParityRecord {
    name: "LRFlipTimer",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn lrfliptimer() {
    check_plugin(
        &RECORD_LRFLIPTIMER,
        include_str!("fixtures/batch2/g18/LRFlipTimer.txt"),
        include_bytes!("fixtures/batch2/g18/LRFlipTimer.f32"),
    );
}

const RECORD_LUXOR: ParityRecord = ParityRecord {
    name: "Luxor",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn luxor() {
    check_plugin(
        &RECORD_LUXOR,
        include_str!("fixtures/batch2/g18/Luxor.txt"),
        include_bytes!("fixtures/batch2/g18/Luxor.f32"),
    );
}
