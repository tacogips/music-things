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

const RECORD_DISTANCE: ParityRecord = ParityRecord {
    name: "Distance",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn distance() {
    check_reset_preserves_params("Distance");
    check_plugin(
        &RECORD_DISTANCE,
        include_str!("fixtures/batch1/g14/Distance.txt"),
        include_bytes!("fixtures/batch1/g14/Distance.f32"),
    );
}

const RECORD_DISTANCE2: ParityRecord = ParityRecord {
    name: "Distance2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn distance2() {
    check_reset_preserves_params("Distance2");
    check_plugin(
        &RECORD_DISTANCE2,
        include_str!("fixtures/batch1/g14/Distance2.txt"),
        include_bytes!("fixtures/batch1/g14/Distance2.f32"),
    );
}

const RECORD_DISTORTION: ParityRecord = ParityRecord {
    name: "Distortion",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn distortion() {
    check_reset_preserves_params("Distortion");
    check_plugin(
        &RECORD_DISTORTION,
        include_str!("fixtures/batch1/g14/Distortion.txt"),
        include_bytes!("fixtures/batch1/g14/Distortion.f32"),
    );
}

const RECORD_DITHERBOX: ParityRecord = ParityRecord {
    name: "Ditherbox",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn ditherbox() {
    check_reset_preserves_params("Ditherbox");
    check_plugin(
        &RECORD_DITHERBOX,
        include_str!("fixtures/batch1/g14/Ditherbox.txt"),
        include_bytes!("fixtures/batch1/g14/Ditherbox.f32"),
    );
}

const RECORD_DITHERFLOAT: ParityRecord = ParityRecord {
    name: "DitherFloat",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn ditherfloat() {
    check_reset_preserves_params("DitherFloat");
    check_plugin(
        &RECORD_DITHERFLOAT,
        include_str!("fixtures/batch1/g14/DitherFloat.txt"),
        include_bytes!("fixtures/batch1/g14/DitherFloat.f32"),
    );
}

const RECORD_DITHERMEDISKERS: ParityRecord = ParityRecord {
    name: "DitherMeDiskers",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn dithermediskers() {
    check_plugin(
        &RECORD_DITHERMEDISKERS,
        include_str!("fixtures/batch1/g14/DitherMeDiskers.txt"),
        include_bytes!("fixtures/batch1/g14/DitherMeDiskers.f32"),
    );
}
