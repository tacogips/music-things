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

const RECORD_MIDSIDE: ParityRecord = ParityRecord {
    name: "MidSide",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn midside() {
    check_reset_preserves_params("MidSide");
    check_plugin(
        &RECORD_MIDSIDE,
        include_str!("fixtures/batch2/g21/MidSide.txt"),
        include_bytes!("fixtures/batch2/g21/MidSide.f32"),
    );
}

const RECORD_MOJO: ParityRecord = ParityRecord {
    name: "Mojo",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn mojo() {
    check_reset_preserves_params("Mojo");
    check_plugin(
        &RECORD_MOJO,
        include_str!("fixtures/batch2/g21/Mojo.txt"),
        include_bytes!("fixtures/batch2/g21/Mojo.f32"),
    );
}
