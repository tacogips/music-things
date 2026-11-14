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
const RECORD_BIQUADDOUBLE: ParityRecord = ParityRecord {
    name: "BiquadDouble",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "Measured on aarch64-apple-darwin",
};
#[test]
fn biquaddouble() {
    check_plugin(
        &RECORD_BIQUADDOUBLE,
        include_str!("fixtures/batch1/g04/BiquadDouble.txt"),
        include_bytes!("fixtures/batch1/g04/BiquadDouble.f32"),
    );
    check_reset_preserves_params("BiquadDouble");
}

const RECORD_BIQUADONEHALF: ParityRecord = ParityRecord {
    name: "BiquadOneHalf",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "Measured on aarch64-apple-darwin",
};
#[test]
fn biquadonehalf() {
    check_plugin(
        &RECORD_BIQUADONEHALF,
        include_str!("fixtures/batch1/g04/BiquadOneHalf.txt"),
        include_bytes!("fixtures/batch1/g04/BiquadOneHalf.f32"),
    );
    check_reset_preserves_params("BiquadOneHalf");
}

const RECORD_BIQUADPLUS: ParityRecord = ParityRecord {
    name: "BiquadPlus",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "Measured on aarch64-apple-darwin",
};
#[test]
fn biquadplus() {
    check_plugin(
        &RECORD_BIQUADPLUS,
        include_str!("fixtures/batch1/g04/BiquadPlus.txt"),
        include_bytes!("fixtures/batch1/g04/BiquadPlus.f32"),
    );
    check_reset_preserves_params("BiquadPlus");
}

const RECORD_BIQUADTRIPLE: ParityRecord = ParityRecord {
    name: "BiquadTriple",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "Measured on aarch64-apple-darwin",
};
#[test]
fn biquadtriple() {
    check_plugin(
        &RECORD_BIQUADTRIPLE,
        include_str!("fixtures/batch1/g04/BiquadTriple.txt"),
        include_bytes!("fixtures/batch1/g04/BiquadTriple.f32"),
    );
    check_reset_preserves_params("BiquadTriple");
}

const RECORD_BITE: ParityRecord = ParityRecord {
    name: "Bite",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "Measured on aarch64-apple-darwin",
};
#[test]
fn bite() {
    check_plugin(
        &RECORD_BITE,
        include_str!("fixtures/batch1/g04/Bite.txt"),
        include_bytes!("fixtures/batch1/g04/Bite.f32"),
    );
    check_reset_preserves_params("Bite");
}

const RECORD_BITGLITTER: ParityRecord = ParityRecord {
    name: "BitGlitter",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "Measured on aarch64-apple-darwin",
};
#[test]
fn bitglitter() {
    check_plugin(
        &RECORD_BITGLITTER,
        include_str!("fixtures/batch1/g04/BitGlitter.txt"),
        include_bytes!("fixtures/batch1/g04/BitGlitter.f32"),
    );
    check_reset_preserves_params("BitGlitter");
}

const RECORD_BITSHIFTGAIN: ParityRecord = ParityRecord {
    name: "BitShiftGain",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "Measured on aarch64-apple-darwin",
};
#[test]
fn bitshiftgain() {
    check_plugin(
        &RECORD_BITSHIFTGAIN,
        include_str!("fixtures/batch1/g04/BitShiftGain.txt"),
        include_bytes!("fixtures/batch1/g04/BitShiftGain.f32"),
    );
    check_reset_preserves_params("BitShiftGain");
}

const RECORD_BLOCKPARTY: ParityRecord = ParityRecord {
    name: "BlockParty",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "Measured on aarch64-apple-darwin",
};
#[test]
fn blockparty() {
    check_plugin(
        &RECORD_BLOCKPARTY,
        include_str!("fixtures/batch1/g04/BlockParty.txt"),
        include_bytes!("fixtures/batch1/g04/BlockParty.f32"),
    );
    check_reset_preserves_params("BlockParty");
}
