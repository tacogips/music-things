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
fn measured(name: &str, meta: &str, data: &[u8]) -> f32 {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for i in 0..fixture.params.len() {
                effect.set_param(
                    i,
                    [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25][i % 9],
                );
            }
        }
        let mut l = vec![0.0; left.len()];
        let mut r = vec![0.0; right.len()];
        for block in (0..left.len()).step_by(128) {
            effect.process(
                &left[block..block + 128],
                &right[block..block + 128],
                &mut l[block..block + 128],
                &mut r[block..block + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            maximum = maximum.max((l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    maximum
}
const RECORD_GRINGER: ParityRecord = ParityRecord {
    name: "Gringer",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn gringer() {
    let meta = include_str!("fixtures/batch2/g10/Gringer.txt");
    let data = include_bytes!("fixtures/batch2/g10/Gringer.f32");
    check_reset_preserves_params("Gringer");
    check_plugin(&RECORD_GRINGER, meta, data);
    assert_eq!(
        measured("Gringer", meta, data),
        RECORD_GRINGER.measured_max_abs
    );
}
const RECORD_GROOVEWEAR: ParityRecord = ParityRecord {
    name: "GrooveWear",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn groovewear() {
    let meta = include_str!("fixtures/batch2/g10/GrooveWear.txt");
    let data = include_bytes!("fixtures/batch2/g10/GrooveWear.f32");
    check_reset_preserves_params("GrooveWear");
    check_plugin(&RECORD_GROOVEWEAR, meta, data);
    assert_eq!(
        measured("GrooveWear", meta, data),
        RECORD_GROOVEWEAR.measured_max_abs
    );
}
const RECORD_GUITARCONDITIONER: ParityRecord = ParityRecord {
    name: "GuitarConditioner",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn guitarconditioner() {
    let meta = include_str!("fixtures/batch2/g10/GuitarConditioner.txt");
    let data = include_bytes!("fixtures/batch2/g10/GuitarConditioner.f32");
    check_reset_preserves_params("GuitarConditioner");
    check_plugin(&RECORD_GUITARCONDITIONER, meta, data);
    assert_eq!(
        measured("GuitarConditioner", meta, data),
        RECORD_GUITARCONDITIONER.measured_max_abs
    );
}
const RECORD_HARDVACUUM: ParityRecord = ParityRecord {
    name: "HardVacuum",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn hardvacuum() {
    let meta = include_str!("fixtures/batch2/g10/HardVacuum.txt");
    let data = include_bytes!("fixtures/batch2/g10/HardVacuum.f32");
    check_reset_preserves_params("HardVacuum");
    check_plugin(&RECORD_HARDVACUUM, meta, data);
    assert_eq!(
        measured("HardVacuum", meta, data),
        RECORD_HARDVACUUM.measured_max_abs
    );
}
const RECORD_HERMEPASS: ParityRecord = ParityRecord {
    name: "Hermepass",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn hermepass() {
    let meta = include_str!("fixtures/batch2/g10/Hermepass.txt");
    let data = include_bytes!("fixtures/batch2/g10/Hermepass.f32");
    check_reset_preserves_params("Hermepass");
    check_plugin(&RECORD_HERMEPASS, meta, data);
    assert_eq!(
        measured("Hermepass", meta, data),
        RECORD_HERMEPASS.measured_max_abs
    );
}
const RECORD_HERMETRIM: ParityRecord = ParityRecord {
    name: "HermeTrim",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn hermetrim() {
    let meta = include_str!("fixtures/batch2/g10/HermeTrim.txt");
    let data = include_bytes!("fixtures/batch2/g10/HermeTrim.f32");
    check_reset_preserves_params("HermeTrim");
    check_plugin(&RECORD_HERMETRIM, meta, data);
    assert_eq!(
        measured("HermeTrim", meta, data),
        RECORD_HERMETRIM.measured_max_abs
    );
}
const RECORD_HIGHGLOSSDITHER: ParityRecord = ParityRecord {
    name: "HighGlossDither",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn highglossdither() {
    let meta = include_str!("fixtures/batch2/g10/HighGlossDither.txt");
    let data = include_bytes!("fixtures/batch2/g10/HighGlossDither.f32");
    check_reset_preserves_params("HighGlossDither");
    check_plugin(&RECORD_HIGHGLOSSDITHER, meta, data);
    assert_eq!(
        measured("HighGlossDither", meta, data),
        RECORD_HIGHGLOSSDITHER.measured_max_abs
    );
}
