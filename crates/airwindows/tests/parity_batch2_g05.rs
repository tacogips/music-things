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

const EVERYTRIM: ParityRecord = ParityRecord {
    name: "EveryTrim",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn everytrim() {
    check_reset_preserves_params("EveryTrim");
    let exact = ParityRecord {
        name: "EveryTrim",
        class: ParityClass::Exact,
        measured_max_abs: 0.0,
        note: "",
    };
    check_plugin(
        &exact,
        include_str!("fixtures/batch2/g05/EveryTrim.txt"),
        include_bytes!("fixtures/batch2/g05/EveryTrim.f32"),
    );
    check_plugin(
        &EVERYTRIM,
        include_str!("fixtures/batch2/g05/EveryTrim.txt"),
        include_bytes!("fixtures/batch2/g05/EveryTrim.f32"),
    );
}

const FACET: ParityRecord = ParityRecord {
    name: "Facet",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn facet() {
    check_reset_preserves_params("Facet");
    check_plugin(
        &FACET,
        include_str!("fixtures/batch2/g05/Facet.txt"),
        include_bytes!("fixtures/batch2/g05/Facet.f32"),
    );
}

const EXCITER: ParityRecord = ParityRecord {
    name: "Exciter",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn exciter() {
    check_reset_preserves_params("Exciter");
    let exact = ParityRecord {
        name: "Exciter",
        class: ParityClass::Exact,
        measured_max_abs: 0.0,
        note: "",
    };
    check_plugin(
        &exact,
        include_str!("fixtures/batch2/g05/Exciter.txt"),
        include_bytes!("fixtures/batch2/g05/Exciter.f32"),
    );
    check_plugin(
        &EXCITER,
        include_str!("fixtures/batch2/g05/Exciter.txt"),
        include_bytes!("fixtures/batch2/g05/Exciter.f32"),
    );
}

const FATHOMFIVE: ParityRecord = ParityRecord {
    name: "FathomFive",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn fathomfive() {
    check_reset_preserves_params("FathomFive");
    let exact = ParityRecord {
        name: "FathomFive",
        class: ParityClass::Exact,
        measured_max_abs: 0.0,
        note: "",
    };
    check_plugin(
        &exact,
        include_str!("fixtures/batch2/g05/FathomFive.txt"),
        include_bytes!("fixtures/batch2/g05/FathomFive.f32"),
    );
    check_plugin(
        &FATHOMFIVE,
        include_str!("fixtures/batch2/g05/FathomFive.txt"),
        include_bytes!("fixtures/batch2/g05/FathomFive.f32"),
    );
}

const ENSEMBLE: ParityRecord = ParityRecord {
    name: "Ensemble",
    class: ParityClass::Libm,
    measured_max_abs: 1.4901161e-8,
    note: "",
};
#[test]
fn ensemble() {
    check_reset_preserves_params("Ensemble");
    check_plugin(
        &ENSEMBLE,
        include_str!("fixtures/batch2/g05/Ensemble.txt"),
        include_bytes!("fixtures/batch2/g05/Ensemble.f32"),
    );
}

#[test]
fn ensemble_measure() {
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g05/Ensemble.txt"),
        include_bytes!("fixtures/batch2/g05/Ensemble.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Ensemble", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65].into_iter().enumerate() {
                effect.set_param(i, value);
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
    println!("Ensemble max-abs {maximum:?}");
    assert_eq!(maximum, ENSEMBLE.measured_max_abs);
}

const EQ: ParityRecord = ParityRecord {
    name: "EQ",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn eq() {
    check_reset_preserves_params("EQ");
    check_plugin(
        &EQ,
        include_str!("fixtures/batch2/g05/EQ.txt"),
        include_bytes!("fixtures/batch2/g05/EQ.f32"),
    );
}

#[test]
fn eq_measure() {
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g05/EQ.txt"),
        include_bytes!("fixtures/batch2/g05/EQ.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("EQ", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70]
                .into_iter()
                .enumerate()
            {
                effect.set_param(i, value);
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
    println!("EQ max-abs {maximum:?}");
    assert_eq!(maximum, EQ.measured_max_abs);
}
