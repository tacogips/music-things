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

fn measure_maximum(name: &str, meta: &str, data: &[u8]) -> f32 {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut maximum = 0.0_f32;
    let alt = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25];
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for i in 0..fixture.params.len() {
                effect.set_param(i, alt[i % alt.len()]);
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
            maximum = maximum.max((out_l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    maximum
}

const RECORD_STUDIOTAN: ParityRecord = ParityRecord {
    name: "StudioTan",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn studiotan() {
    let meta = include_str!("fixtures/batch3/g07/StudioTan.txt");
    let data = include_bytes!("fixtures/batch3/g07/StudioTan.f32");
    check_reset_preserves_params("StudioTan");
    let maximum = measure_maximum("StudioTan", meta, data);
    check_plugin(&RECORD_STUDIOTAN, meta, data);
    assert_eq!(
        maximum, RECORD_STUDIOTAN.measured_max_abs,
        "StudioTan measured max-abs: {maximum:?}"
    );
}

const RECORD_SUBSONLY: ParityRecord = ParityRecord {
    name: "SubsOnly",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn subsonly() {
    let meta = include_str!("fixtures/batch3/g07/SubsOnly.txt");
    let data = include_bytes!("fixtures/batch3/g07/SubsOnly.f32");
    check_reset_preserves_params("SubsOnly");
    let maximum = measure_maximum("SubsOnly", meta, data);
    check_plugin(&RECORD_SUBSONLY, meta, data);
    assert_eq!(
        maximum, RECORD_SUBSONLY.measured_max_abs,
        "SubsOnly measured max-abs: {maximum:?}"
    );
}

const RECORD_SURGE: ParityRecord = ParityRecord {
    name: "Surge",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn surge() {
    let meta = include_str!("fixtures/batch3/g07/Surge.txt");
    let data = include_bytes!("fixtures/batch3/g07/Surge.f32");
    check_reset_preserves_params("Surge");
    let maximum = measure_maximum("Surge", meta, data);
    check_plugin(&RECORD_SURGE, meta, data);
    assert_eq!(
        maximum, RECORD_SURGE.measured_max_abs,
        "Surge measured max-abs: {maximum:?}"
    );
}

const RECORD_SURGETIDE: ParityRecord = ParityRecord {
    name: "SurgeTide",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn surgetide() {
    let meta = include_str!("fixtures/batch3/g07/SurgeTide.txt");
    let data = include_bytes!("fixtures/batch3/g07/SurgeTide.f32");
    check_reset_preserves_params("SurgeTide");
    let maximum = measure_maximum("SurgeTide", meta, data);
    check_plugin(&RECORD_SURGETIDE, meta, data);
    assert_eq!(
        maximum, RECORD_SURGETIDE.measured_max_abs,
        "SurgeTide measured max-abs: {maximum:?}"
    );
}

const RECORD_SWELL: ParityRecord = ParityRecord {
    name: "Swell",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn swell() {
    let meta = include_str!("fixtures/batch3/g07/Swell.txt");
    let data = include_bytes!("fixtures/batch3/g07/Swell.f32");
    check_reset_preserves_params("Swell");
    let maximum = measure_maximum("Swell", meta, data);
    check_plugin(&RECORD_SWELL, meta, data);
    assert_eq!(
        maximum, RECORD_SWELL.measured_max_abs,
        "Swell measured max-abs: {maximum:?}"
    );
}

const RECORD_TAPE: ParityRecord = ParityRecord {
    name: "Tape",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn tape() {
    let meta = include_str!("fixtures/batch3/g07/Tape.txt");
    let data = include_bytes!("fixtures/batch3/g07/Tape.f32");
    check_reset_preserves_params("Tape");
    let maximum = measure_maximum("Tape", meta, data);
    check_plugin(&RECORD_TAPE, meta, data);
    assert_eq!(
        maximum, RECORD_TAPE.measured_max_abs,
        "Tape measured max-abs: {maximum:?}"
    );
}

const RECORD_TAPEDELAY: ParityRecord = ParityRecord {
    name: "TapeDelay",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn tapedelay() {
    let meta = include_str!("fixtures/batch3/g07/TapeDelay.txt");
    let data = include_bytes!("fixtures/batch3/g07/TapeDelay.f32");
    check_reset_preserves_params("TapeDelay");
    let maximum = measure_maximum("TapeDelay", meta, data);
    check_plugin(&RECORD_TAPEDELAY, meta, data);
    assert_eq!(
        maximum, RECORD_TAPEDELAY.measured_max_abs,
        "TapeDelay measured max-abs: {maximum:?}"
    );
}

const RECORD_TAPEDELAY2: ParityRecord = ParityRecord {
    name: "TapeDelay2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn tapedelay2() {
    let meta = include_str!("fixtures/batch3/g07/TapeDelay2.txt");
    let data = include_bytes!("fixtures/batch3/g07/TapeDelay2.f32");
    check_reset_preserves_params("TapeDelay2");
    let maximum = measure_maximum("TapeDelay2", meta, data);
    check_plugin(&RECORD_TAPEDELAY2, meta, data);
    assert_eq!(
        maximum, RECORD_TAPEDELAY2.measured_max_abs,
        "TapeDelay2 measured max-abs: {maximum:?}"
    );
}
