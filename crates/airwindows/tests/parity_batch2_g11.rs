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

fn measure(record: &ParityRecord, meta: &str, data: &[u8]) {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(record.name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for i in 0..fixture.params.len() {
                effect.set_param(
                    i,
                    [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25][i % 9],
                );
            }
        }
        let mut out_l = vec![0.0; left.len()];
        let mut out_r = vec![0.0; right.len()];
        for block in (0..left.len()).step_by(128) {
            effect.process(
                &left[block..block + 128],
                &right[block..block + 128],
                &mut out_l[block..block + 128],
                &mut out_r[block..block + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            maximum = maximum
                .max((out_l[frame] - fixture.renders[render][2 * k]).abs())
                .max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert_eq!(
        maximum, record.measured_max_abs,
        "{} measured max-abs",
        record.name
    );
}

const RECORD_HIGHIMPACT: ParityRecord = ParityRecord {
    name: "HighImpact",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn highimpact() {
    check_reset_preserves_params("HighImpact");
    measure(
        &RECORD_HIGHIMPACT,
        include_str!("fixtures/batch2/g11/HighImpact.txt"),
        include_bytes!("fixtures/batch2/g11/HighImpact.f32"),
    );
    check_plugin(
        &RECORD_HIGHIMPACT,
        include_str!("fixtures/batch2/g11/HighImpact.txt"),
        include_bytes!("fixtures/batch2/g11/HighImpact.f32"),
    );
}

const RECORD_HIGHPASS: ParityRecord = ParityRecord {
    name: "Highpass",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn highpass() {
    check_reset_preserves_params("Highpass");
    measure(
        &RECORD_HIGHPASS,
        include_str!("fixtures/batch2/g11/Highpass.txt"),
        include_bytes!("fixtures/batch2/g11/Highpass.f32"),
    );
    check_plugin(
        &RECORD_HIGHPASS,
        include_str!("fixtures/batch2/g11/Highpass.txt"),
        include_bytes!("fixtures/batch2/g11/Highpass.f32"),
    );
}

const RECORD_HIGHPASS2: ParityRecord = ParityRecord {
    name: "Highpass2",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn highpass2() {
    check_reset_preserves_params("Highpass2");
    measure(
        &RECORD_HIGHPASS2,
        include_str!("fixtures/batch2/g11/Highpass2.txt"),
        include_bytes!("fixtures/batch2/g11/Highpass2.f32"),
    );
    check_plugin(
        &RECORD_HIGHPASS2,
        include_str!("fixtures/batch2/g11/Highpass2.txt"),
        include_bytes!("fixtures/batch2/g11/Highpass2.f32"),
    );
}

const RECORD_HOLT: ParityRecord = ParityRecord {
    name: "Holt",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn holt() {
    check_reset_preserves_params("Holt");
    measure(
        &RECORD_HOLT,
        include_str!("fixtures/batch2/g11/Holt.txt"),
        include_bytes!("fixtures/batch2/g11/Holt.f32"),
    );
    check_plugin(
        &RECORD_HOLT,
        include_str!("fixtures/batch2/g11/Holt.txt"),
        include_bytes!("fixtures/batch2/g11/Holt.f32"),
    );
}

const RECORD_HOLT2: ParityRecord = ParityRecord {
    name: "Holt2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn holt2() {
    check_reset_preserves_params("Holt2");
    measure(
        &RECORD_HOLT2,
        include_str!("fixtures/batch2/g11/Holt2.txt"),
        include_bytes!("fixtures/batch2/g11/Holt2.f32"),
    );
    check_plugin(
        &RECORD_HOLT2,
        include_str!("fixtures/batch2/g11/Holt2.txt"),
        include_bytes!("fixtures/batch2/g11/Holt2.f32"),
    );
}

const RECORD_HOMBRE: ParityRecord = ParityRecord {
    name: "Hombre",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn hombre() {
    check_reset_preserves_params("Hombre");
    measure(
        &RECORD_HOMBRE,
        include_str!("fixtures/batch2/g11/Hombre.txt"),
        include_bytes!("fixtures/batch2/g11/Hombre.f32"),
    );
    check_plugin(
        &RECORD_HOMBRE,
        include_str!("fixtures/batch2/g11/Hombre.txt"),
        include_bytes!("fixtures/batch2/g11/Hombre.f32"),
    );
}

const RECORD_HULL: ParityRecord = ParityRecord {
    name: "Hull",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn hull() {
    check_reset_preserves_params("Hull");
    measure(
        &RECORD_HULL,
        include_str!("fixtures/batch2/g11/Hull.txt"),
        include_bytes!("fixtures/batch2/g11/Hull.f32"),
    );
    check_plugin(
        &RECORD_HULL,
        include_str!("fixtures/batch2/g11/Hull.txt"),
        include_bytes!("fixtures/batch2/g11/Hull.f32"),
    );
}
