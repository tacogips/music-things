mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

fn assert_reset_preserves_params(name: &str) {
    let mut effect = music_things_airwindows::create(name, 44100.0).unwrap();
    let values: Vec<_> = (0..effect.param_count())
        .map(|i| {
            effect.set_param(i, 0.272 + i as f32 * 0.142);
            effect.get_param(i)
        })
        .collect();
    effect.reset();
    for (i, value) in values.iter().enumerate() {
        assert_eq!(effect.get_param(i), *value, "{name} parameter {i}");
    }
}
const RECORD_DENSITY: ParityRecord = ParityRecord {
    name: "Density",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn density() {
    assert_reset_preserves_params(RECORD_DENSITY.name);
    check_plugin(
        &RECORD_DENSITY,
        include_str!("fixtures/batch1/g13/Density.txt"),
        include_bytes!("fixtures/batch1/g13/Density.f32"),
    );
}
const RECORD_DENSITY2: ParityRecord = ParityRecord {
    name: "Density2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn density2() {
    assert_reset_preserves_params(RECORD_DENSITY2.name);
    check_plugin(
        &RECORD_DENSITY2,
        include_str!("fixtures/batch1/g13/Density2.txt"),
        include_bytes!("fixtures/batch1/g13/Density2.f32"),
    );
}
const RECORD_DEREZ: ParityRecord = ParityRecord {
    name: "DeRez",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn derez() {
    assert_reset_preserves_params(RECORD_DEREZ.name);
    check_plugin(
        &RECORD_DEREZ,
        include_str!("fixtures/batch1/g13/DeRez.txt"),
        include_bytes!("fixtures/batch1/g13/DeRez.f32"),
    );
}
const RECORD_DEREZ2: ParityRecord = ParityRecord {
    name: "DeRez2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn derez2() {
    assert_reset_preserves_params(RECORD_DEREZ2.name);
    check_plugin(
        &RECORD_DEREZ2,
        include_str!("fixtures/batch1/g13/DeRez2.txt"),
        include_bytes!("fixtures/batch1/g13/DeRez2.f32"),
    );
}
const RECORD_DESK: ParityRecord = ParityRecord {
    name: "Desk",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn desk() {
    assert_reset_preserves_params(RECORD_DESK.name);
    check_plugin(
        &RECORD_DESK,
        include_str!("fixtures/batch1/g13/Desk.txt"),
        include_bytes!("fixtures/batch1/g13/Desk.f32"),
    );
}
const RECORD_DESK4: ParityRecord = ParityRecord {
    name: "Desk4",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn desk4() {
    assert_reset_preserves_params(RECORD_DESK4.name);
    check_plugin(
        &RECORD_DESK4,
        include_str!("fixtures/batch1/g13/Desk4.txt"),
        include_bytes!("fixtures/batch1/g13/Desk4.f32"),
    );
}
const RECORD_DIGITALBLACK: ParityRecord = ParityRecord {
    name: "DigitalBlack",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn digitalblack() {
    assert_reset_preserves_params(RECORD_DIGITALBLACK.name);
    check_plugin(
        &RECORD_DIGITALBLACK,
        include_str!("fixtures/batch1/g13/DigitalBlack.txt"),
        include_bytes!("fixtures/batch1/g13/DigitalBlack.f32"),
    );
}
const RECORD_DIRT: ParityRecord = ParityRecord {
    name: "Dirt",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn dirt() {
    assert_reset_preserves_params(RECORD_DIRT.name);
    check_plugin(
        &RECORD_DIRT,
        include_str!("fixtures/batch1/g13/Dirt.txt"),
        include_bytes!("fixtures/batch1/g13/Dirt.f32"),
    );
}
