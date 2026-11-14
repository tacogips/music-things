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

const RECORD_PURESTWARM: ParityRecord = ParityRecord {
    name: "PurestWarm",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn purestwarm() {
    check_reset_preserves_params("PurestWarm");
    check_plugin(
        &RECORD_PURESTWARM,
        include_str!("fixtures/batch3/g03/PurestWarm.txt"),
        include_bytes!("fixtures/batch3/g03/PurestWarm.f32"),
    );
}

const RECORD_PURESTWARM2: ParityRecord = ParityRecord {
    name: "PurestWarm2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn purestwarm2() {
    check_reset_preserves_params("PurestWarm2");
    check_plugin(
        &RECORD_PURESTWARM2,
        include_str!("fixtures/batch3/g03/PurestWarm2.txt"),
        include_bytes!("fixtures/batch3/g03/PurestWarm2.f32"),
    );
}

const RECORD_PYEWACKET: ParityRecord = ParityRecord {
    name: "Pyewacket",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pyewacket() {
    check_reset_preserves_params("Pyewacket");
    check_plugin(
        &RECORD_PYEWACKET,
        include_str!("fixtures/batch3/g03/Pyewacket.txt"),
        include_bytes!("fixtures/batch3/g03/Pyewacket.f32"),
    );
}

const RECORD_RAWGLITTERS: ParityRecord = ParityRecord {
    name: "RawGlitters",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn rawglitters() {
    check_reset_preserves_params("RawGlitters");
    check_plugin(
        &RECORD_RAWGLITTERS,
        include_str!("fixtures/batch3/g03/RawGlitters.txt"),
        include_bytes!("fixtures/batch3/g03/RawGlitters.f32"),
    );
}

const RECORD_RAWTIMBERS: ParityRecord = ParityRecord {
    name: "RawTimbers",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn rawtimbers() {
    check_reset_preserves_params("RawTimbers");
    check_plugin(
        &RECORD_RAWTIMBERS,
        include_str!("fixtures/batch3/g03/RawTimbers.txt"),
        include_bytes!("fixtures/batch3/g03/RawTimbers.f32"),
    );
}

const RECORD_RECURVE: ParityRecord = ParityRecord {
    name: "Recurve",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn recurve() {
    check_reset_preserves_params("Recurve");
    check_plugin(
        &RECORD_RECURVE,
        include_str!("fixtures/batch3/g03/Recurve.txt"),
        include_bytes!("fixtures/batch3/g03/Recurve.f32"),
    );
}

const RECORD_REMAP: ParityRecord = ParityRecord {
    name: "Remap",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn remap() {
    check_reset_preserves_params("Remap");
    check_plugin(
        &RECORD_REMAP,
        include_str!("fixtures/batch3/g03/Remap.txt"),
        include_bytes!("fixtures/batch3/g03/Remap.f32"),
    );
}

const RECORD_RESEQ: ParityRecord = ParityRecord {
    name: "ResEQ",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn reseq() {
    check_reset_preserves_params("ResEQ");
    check_plugin(
        &RECORD_RESEQ,
        include_str!("fixtures/batch3/g03/ResEQ.txt"),
        include_bytes!("fixtures/batch3/g03/ResEQ.f32"),
    );
}
