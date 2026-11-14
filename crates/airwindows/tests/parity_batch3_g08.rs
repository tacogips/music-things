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
        assert_eq!(effect.get_param(i), (i + 1) as f32 / (count + 2) as f32);
    }
}

const RECORD_TAPEDITHER: ParityRecord = ParityRecord {
    name: "TapeDither",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn tapedither() {
    check_reset_preserves_params("TapeDither");
    check_plugin(
        &RECORD_TAPEDITHER,
        include_str!("fixtures/batch3/g08/TapeDither.txt"),
        include_bytes!("fixtures/batch3/g08/TapeDither.f32"),
    );
}

const RECORD_TEXTURIZEMS: ParityRecord = ParityRecord {
    name: "TexturizeMS",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn texturizems() {
    check_reset_preserves_params("TexturizeMS");
    check_plugin(
        &RECORD_TEXTURIZEMS,
        include_str!("fixtures/batch3/g08/TexturizeMS.txt"),
        include_bytes!("fixtures/batch3/g08/TexturizeMS.f32"),
    );
}

const RECORD_TONESLANT: ParityRecord = ParityRecord {
    name: "ToneSlant",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn toneslant() {
    check_reset_preserves_params("ToneSlant");
    check_plugin(
        &RECORD_TONESLANT,
        include_str!("fixtures/batch3/g08/ToneSlant.txt"),
        include_bytes!("fixtures/batch3/g08/ToneSlant.f32"),
    );
}

const RECORD_TAPEDUST: ParityRecord = ParityRecord {
    name: "TapeDust",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn tapedust() {
    check_reset_preserves_params("TapeDust");
    check_plugin(
        &RECORD_TAPEDUST,
        include_str!("fixtures/batch3/g08/TapeDust.txt"),
        include_bytes!("fixtures/batch3/g08/TapeDust.f32"),
    );
}
const RECORD_THUNDER: ParityRecord = ParityRecord {
    name: "Thunder",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn thunder() {
    check_reset_preserves_params("Thunder");
    check_plugin(
        &RECORD_THUNDER,
        include_str!("fixtures/batch3/g08/Thunder.txt"),
        include_bytes!("fixtures/batch3/g08/Thunder.f32"),
    );
}
const RECORD_TOTAPE5: ParityRecord = ParityRecord {
    name: "ToTape5",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn totape5() {
    check_reset_preserves_params("ToTape5");
    check_plugin(
        &RECORD_TOTAPE5,
        include_str!("fixtures/batch3/g08/ToTape5.txt"),
        include_bytes!("fixtures/batch3/g08/ToTape5.f32"),
    );
}

const RECORD_TAPEFAT: ParityRecord = ParityRecord {
    name: "TapeFat",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn tapefat() {
    check_reset_preserves_params("TapeFat");
    check_plugin(
        &RECORD_TAPEFAT,
        include_str!("fixtures/batch3/g08/TapeFat.txt"),
        include_bytes!("fixtures/batch3/g08/TapeFat.f32"),
    );
}
const RECORD_TEXTURIZE: ParityRecord = ParityRecord {
    name: "Texturize",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn texturize() {
    check_reset_preserves_params("Texturize");
    check_plugin(
        &RECORD_TEXTURIZE,
        include_str!("fixtures/batch3/g08/Texturize.txt"),
        include_bytes!("fixtures/batch3/g08/Texturize.f32"),
    );
}
