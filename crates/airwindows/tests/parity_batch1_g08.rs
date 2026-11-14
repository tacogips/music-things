mod common;

use common::parity::{check_plugin, ParityClass, ParityRecord};

fn assert_reset_preserves_params(name: &str) {
    let mut effect = music_things_airwindows::create_with_seed(name, 44100.0, 123).unwrap();
    let values: Vec<_> = (0..effect.param_count())
        .map(|i| 0.272 + 0.142 * i as f32)
        .collect();
    for (i, &value) in values.iter().enumerate() {
        effect.set_param(i, value);
    }
    effect.reset();
    for (i, &value) in values.iter().enumerate() {
        assert_eq!(
            effect.get_param(i).to_bits(),
            value.to_bits(),
            "{name} parameter {i}"
        );
    }
}

const RECORD_CLIPSOFTLY: ParityRecord = ParityRecord {
    name: "ClipSoftly",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn clipsoftly() {
    assert_reset_preserves_params(RECORD_CLIPSOFTLY.name);
    check_plugin(
        &RECORD_CLIPSOFTLY,
        include_str!("fixtures/batch1/g08/ClipSoftly.txt"),
        include_bytes!("fixtures/batch1/g08/ClipSoftly.f32"),
    );
}

const RECORD_CHROMEOXIDE: ParityRecord = ParityRecord {
    name: "ChromeOxide",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn chromeoxide() {
    assert_reset_preserves_params(RECORD_CHROMEOXIDE.name);
    check_plugin(
        &RECORD_CHROMEOXIDE,
        include_str!("fixtures/batch1/g08/ChromeOxide.txt"),
        include_bytes!("fixtures/batch1/g08/ChromeOxide.f32"),
    );
}

const RECORD_CLIPONLY: ParityRecord = ParityRecord {
    name: "ClipOnly",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn cliponly() {
    assert_reset_preserves_params(RECORD_CLIPONLY.name);
    check_plugin(
        &RECORD_CLIPONLY,
        include_str!("fixtures/batch1/g08/ClipOnly.txt"),
        include_bytes!("fixtures/batch1/g08/ClipOnly.f32"),
    );
}

const RECORD_CLIPONLY2: ParityRecord = ParityRecord {
    name: "ClipOnly2",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn cliponly2() {
    assert_reset_preserves_params(RECORD_CLIPONLY2.name);
    check_plugin(
        &RECORD_CLIPONLY2,
        include_str!("fixtures/batch1/g08/ClipOnly2.txt"),
        include_bytes!("fixtures/batch1/g08/ClipOnly2.f32"),
    );
}

const RECORD_COILS: ParityRecord = ParityRecord {
    name: "Coils",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn coils() {
    assert_reset_preserves_params(RECORD_COILS.name);
    check_plugin(
        &RECORD_COILS,
        include_str!("fixtures/batch1/g08/Coils.txt"),
        include_bytes!("fixtures/batch1/g08/Coils.f32"),
    );
}

const RECORD_COILS2: ParityRecord = ParityRecord {
    name: "Coils2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn coils2() {
    assert_reset_preserves_params(RECORD_COILS2.name);
    check_plugin(
        &RECORD_COILS2,
        include_str!("fixtures/batch1/g08/Coils2.txt"),
        include_bytes!("fixtures/batch1/g08/Coils2.f32"),
    );
}

const RECORD_COJONES: ParityRecord = ParityRecord {
    name: "Cojones",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn cojones() {
    assert_reset_preserves_params(RECORD_COJONES.name);
    check_plugin(
        &RECORD_COJONES,
        include_str!("fixtures/batch1/g08/Cojones.txt"),
        include_bytes!("fixtures/batch1/g08/Cojones.f32"),
    );
}

const RECORD_CIDER: ParityRecord = ParityRecord {
    name: "Cider",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn cider() {
    assert_reset_preserves_params(RECORD_CIDER.name);
    check_plugin(
        &RECORD_CIDER,
        include_str!("fixtures/batch1/g08/Cider.txt"),
        include_bytes!("fixtures/batch1/g08/Cider.f32"),
    );
}
