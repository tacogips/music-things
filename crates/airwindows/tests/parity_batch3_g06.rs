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

fn check_measured_plugin(record: &ParityRecord, meta: &str, data: &[u8]) {
    check_plugin(record, meta, data);
    const ALT: [f32; 9] = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25];
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(record.name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for i in 0..fixture.params.len() {
                effect.set_param(i, ALT[i % ALT.len()]);
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
    println!("{} measured max-abs: {:?}", record.name, maximum);
}

const RECORD_SPIRAL2: ParityRecord = ParityRecord {
    name: "Spiral2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn spiral2() {
    check_reset_preserves_params("Spiral2");
    check_measured_plugin(
        &RECORD_SPIRAL2,
        include_str!("fixtures/batch3/g06/Spiral2.txt"),
        include_bytes!("fixtures/batch3/g06/Spiral2.f32"),
    );
}

const RECORD_SRSLY: ParityRecord = ParityRecord {
    name: "Srsly",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn srsly() {
    check_reset_preserves_params("Srsly");
    check_measured_plugin(
        &RECORD_SRSLY,
        include_str!("fixtures/batch3/g06/Srsly.txt"),
        include_bytes!("fixtures/batch3/g06/Srsly.f32"),
    );
}

const RECORD_SRSLY2: ParityRecord = ParityRecord {
    name: "Srsly2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn srsly2() {
    check_reset_preserves_params("Srsly2");
    check_measured_plugin(
        &RECORD_SRSLY2,
        include_str!("fixtures/batch3/g06/Srsly2.txt"),
        include_bytes!("fixtures/batch3/g06/Srsly2.f32"),
    );
}

const RECORD_STARCHILD: ParityRecord = ParityRecord {
    name: "StarChild",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn starchild() {
    check_reset_preserves_params("StarChild");
    check_measured_plugin(
        &RECORD_STARCHILD,
        include_str!("fixtures/batch3/g06/StarChild.txt"),
        include_bytes!("fixtures/batch3/g06/StarChild.f32"),
    );
}

const RECORD_STEREOCHORUS: ParityRecord = ParityRecord {
    name: "StereoChorus",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn stereochorus() {
    check_reset_preserves_params("StereoChorus");
    check_measured_plugin(
        &RECORD_STEREOCHORUS,
        include_str!("fixtures/batch3/g06/StereoChorus.txt"),
        include_bytes!("fixtures/batch3/g06/StereoChorus.f32"),
    );
}

const RECORD_STEREODOUBLER: ParityRecord = ParityRecord {
    name: "StereoDoubler",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn stereodoubler() {
    check_reset_preserves_params("StereoDoubler");
    check_measured_plugin(
        &RECORD_STEREODOUBLER,
        include_str!("fixtures/batch3/g06/StereoDoubler.txt"),
        include_bytes!("fixtures/batch3/g06/StereoDoubler.f32"),
    );
}

const RECORD_STEREOENSEMBLE: ParityRecord = ParityRecord {
    name: "StereoEnsemble",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn stereoensemble() {
    check_reset_preserves_params("StereoEnsemble");
    check_measured_plugin(
        &RECORD_STEREOENSEMBLE,
        include_str!("fixtures/batch3/g06/StereoEnsemble.txt"),
        include_bytes!("fixtures/batch3/g06/StereoEnsemble.f32"),
    );
}

const RECORD_STEREOFX: ParityRecord = ParityRecord {
    name: "StereoFX",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn stereofx() {
    check_reset_preserves_params("StereoFX");
    check_measured_plugin(
        &RECORD_STEREOFX,
        include_str!("fixtures/batch3/g06/StereoFX.txt"),
        include_bytes!("fixtures/batch3/g06/StereoFX.f32"),
    );
}
