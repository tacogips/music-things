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

const RECORD_YHIGHPASS: ParityRecord = ParityRecord {
    name: "YHighpass",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn yhighpass() {
    check_reset_preserves_params("YHighpass");
    check_plugin(
        &RECORD_YHIGHPASS,
        include_str!("fixtures/batch3/g13/YHighpass.txt"),
        include_bytes!("fixtures/batch3/g13/YHighpass.f32"),
    );
}

const RECORD_YLOWPASS: ParityRecord = ParityRecord {
    name: "YLowpass",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn ylowpass() {
    check_reset_preserves_params("YLowpass");
    check_plugin(
        &RECORD_YLOWPASS,
        include_str!("fixtures/batch3/g13/YLowpass.txt"),
        include_bytes!("fixtures/batch3/g13/YLowpass.f32"),
    );
}

const RECORD_YNOTCH: ParityRecord = ParityRecord {
    name: "YNotch",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn ynotch() {
    check_reset_preserves_params("YNotch");
    check_plugin(
        &RECORD_YNOTCH,
        include_str!("fixtures/batch3/g13/YNotch.txt"),
        include_bytes!("fixtures/batch3/g13/YNotch.f32"),
    );
}

const RECORD_ZHIGHPASS: ParityRecord = ParityRecord {
    name: "ZHighpass",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn zhighpass() {
    check_reset_preserves_params("ZHighpass");
    check_plugin(
        &RECORD_ZHIGHPASS,
        include_str!("fixtures/batch3/g13/ZHighpass.txt"),
        include_bytes!("fixtures/batch3/g13/ZHighpass.f32"),
    );
}

const RECORD_ZBANDPASS: ParityRecord = ParityRecord {
    name: "ZBandpass",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn zbandpass() {
    check_reset_preserves_params("ZBandpass");
    check_plugin(
        &RECORD_ZBANDPASS,
        include_str!("fixtures/batch3/g13/ZBandpass.txt"),
        include_bytes!("fixtures/batch3/g13/ZBandpass.f32"),
    );
}

const RECORD_ZLOWPASS: ParityRecord = ParityRecord {
    name: "ZLowpass",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn zlowpass() {
    check_reset_preserves_params("ZLowpass");
    check_plugin(
        &RECORD_ZLOWPASS,
        include_str!("fixtures/batch3/g13/ZLowpass.txt"),
        include_bytes!("fixtures/batch3/g13/ZLowpass.f32"),
    );
}

const RECORD_ZBANDPASS2: ParityRecord = ParityRecord {
    name: "ZBandpass2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn zbandpass2() {
    check_reset_preserves_params("ZBandpass2");
    check_plugin(
        &RECORD_ZBANDPASS2,
        include_str!("fixtures/batch3/g13/ZBandpass2.txt"),
        include_bytes!("fixtures/batch3/g13/ZBandpass2.f32"),
    );
}

const RECORD_ZHIGHPASS2: ParityRecord = ParityRecord {
    name: "ZHighpass2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn zhighpass2() {
    check_reset_preserves_params("ZHighpass2");
    check_plugin(
        &RECORD_ZHIGHPASS2,
        include_str!("fixtures/batch3/g13/ZHighpass2.txt"),
        include_bytes!("fixtures/batch3/g13/ZHighpass2.f32"),
    );
}
