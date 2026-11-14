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

const RECORD_MACKEQ: ParityRecord = ParityRecord {
    name: "MackEQ",
    class: ParityClass::Libm,
    measured_max_abs: 5.9604645e-8,
    note: "",
};

#[test]
fn mackeq() {
    check_reset_preserves_params("MackEQ");
    check_plugin(
        &RECORD_MACKEQ,
        include_str!("fixtures/batch2/g19/MackEQ.txt"),
        include_bytes!("fixtures/batch2/g19/MackEQ.f32"),
    );
}

const RECORD_MACKITY: ParityRecord = ParityRecord {
    name: "Mackity",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn mackity() {
    check_reset_preserves_params("Mackity");
    check_plugin(
        &RECORD_MACKITY,
        include_str!("fixtures/batch2/g19/Mackity.txt"),
        include_bytes!("fixtures/batch2/g19/Mackity.f32"),
    );
}

const RECORD_MATRIXVERB: ParityRecord = ParityRecord {
    name: "MatrixVerb",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn matrixverb() {
    check_reset_preserves_params("MatrixVerb");
    check_plugin(
        &RECORD_MATRIXVERB,
        include_str!("fixtures/batch2/g19/MatrixVerb.txt"),
        include_bytes!("fixtures/batch2/g19/MatrixVerb.f32"),
    );
}

const RECORD_MELT: ParityRecord = ParityRecord {
    name: "Melt",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn melt() {
    check_reset_preserves_params("Melt");
    check_plugin(
        &RECORD_MELT,
        include_str!("fixtures/batch2/g19/Melt.txt"),
        include_bytes!("fixtures/batch2/g19/Melt.f32"),
    );
}
