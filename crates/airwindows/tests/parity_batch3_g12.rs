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

const RECORD_VOICETRICK: ParityRecord = ParityRecord {
    name: "VoiceTrick",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn voicetrick() {
    check_reset_preserves_params("VoiceTrick");
    check_plugin(
        &RECORD_VOICETRICK,
        include_str!("fixtures/batch3/g12/VoiceTrick.txt"),
        include_bytes!("fixtures/batch3/g12/VoiceTrick.f32"),
    );
}

const RECORD_WIDER: ParityRecord = ParityRecord {
    name: "Wider",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn wider() {
    check_reset_preserves_params("Wider");
    check_plugin(
        &RECORD_WIDER,
        include_str!("fixtures/batch3/g12/Wider.txt"),
        include_bytes!("fixtures/batch3/g12/Wider.f32"),
    );
}

const RECORD_XBANDPASS: ParityRecord = ParityRecord {
    name: "XBandpass",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn xbandpass() {
    check_reset_preserves_params("XBandpass");
    check_plugin(
        &RECORD_XBANDPASS,
        include_str!("fixtures/batch3/g12/XBandpass.txt"),
        include_bytes!("fixtures/batch3/g12/XBandpass.f32"),
    );
}

const RECORD_XHIGHPASS: ParityRecord = ParityRecord {
    name: "XHighpass",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn xhighpass() {
    check_reset_preserves_params("XHighpass");
    check_plugin(
        &RECORD_XHIGHPASS,
        include_str!("fixtures/batch3/g12/XHighpass.txt"),
        include_bytes!("fixtures/batch3/g12/XHighpass.f32"),
    );
}

const RECORD_XLOWPASS: ParityRecord = ParityRecord {
    name: "XLowpass",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn xlowpass() {
    check_reset_preserves_params("XLowpass");
    check_plugin(
        &RECORD_XLOWPASS,
        include_str!("fixtures/batch3/g12/XLowpass.txt"),
        include_bytes!("fixtures/batch3/g12/XLowpass.f32"),
    );
}

const RECORD_XNOTCH: ParityRecord = ParityRecord {
    name: "XNotch",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn xnotch() {
    check_reset_preserves_params("XNotch");
    check_plugin(
        &RECORD_XNOTCH,
        include_str!("fixtures/batch3/g12/XNotch.txt"),
        include_bytes!("fixtures/batch3/g12/XNotch.f32"),
    );
}

const RECORD_XREGION: ParityRecord = ParityRecord {
    name: "XRegion",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn xregion() {
    check_reset_preserves_params("XRegion");
    check_plugin(
        &RECORD_XREGION,
        include_str!("fixtures/batch3/g12/XRegion.txt"),
        include_bytes!("fixtures/batch3/g12/XRegion.f32"),
    );
}

const RECORD_YBANDPASS: ParityRecord = ParityRecord {
    name: "YBandpass",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn ybandpass() {
    check_reset_preserves_params("YBandpass");
    check_plugin(
        &RECORD_YBANDPASS,
        include_str!("fixtures/batch3/g12/YBandpass.txt"),
        include_bytes!("fixtures/batch3/g12/YBandpass.f32"),
    );
}
