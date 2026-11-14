mod common;

use common::parity::{check_plugin, ParityClass, ParityRecord};

fn measured_check(record: &ParityRecord, meta: &str, data: &[u8]) {
    let mut effect =
        music_things_airwindows::create_with_seed(record.name, 44100.0, 0x00c0_ffee).unwrap();
    let count = effect.param_count();
    for i in 0..count {
        effect.set_param(i, (i + 1) as f32 / (count + 2) as f32);
    }
    effect.reset();
    for i in 0..count {
        assert_eq!(
            effect.get_param(i),
            (i + 1) as f32 / (count + 2) as f32,
            "{} parameter {i}",
            record.name
        );
    }
    check_plugin(record, meta, data);
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(record.name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25]
                .into_iter()
                .enumerate()
                .take(effect.param_count())
            {
                effect.set_param(i, value);
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
            max = max.max((out_l[frame] - fixture.renders[render][2 * k]).abs());
            max = max.max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert_eq!(
        max, record.measured_max_abs,
        "{} measured max-abs",
        record.name
    );
}

const RECORD_FROMTAPE: ParityRecord = ParityRecord {
    name: "FromTape",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn fromtape() {
    measured_check(
        &RECORD_FROMTAPE,
        include_str!("fixtures/batch2/g08/FromTape.txt"),
        include_bytes!("fixtures/batch2/g08/FromTape.f32"),
    );
}

const RECORD_GALACTIC: ParityRecord = ParityRecord {
    name: "Galactic",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn galactic() {
    measured_check(
        &RECORD_GALACTIC,
        include_str!("fixtures/batch2/g08/Galactic.txt"),
        include_bytes!("fixtures/batch2/g08/Galactic.f32"),
    );
}

const RECORD_GATELOPE: ParityRecord = ParityRecord {
    name: "Gatelope",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn gatelope() {
    measured_check(
        &RECORD_GATELOPE,
        include_str!("fixtures/batch2/g08/Gatelope.txt"),
        include_bytes!("fixtures/batch2/g08/Gatelope.f32"),
    );
}

const RECORD_GLITCHSHIFTER: ParityRecord = ParityRecord {
    name: "GlitchShifter",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn glitchshifter() {
    measured_check(
        &RECORD_GLITCHSHIFTER,
        include_str!("fixtures/batch2/g08/GlitchShifter.txt"),
        include_bytes!("fixtures/batch2/g08/GlitchShifter.f32"),
    );
}

const RECORD_GOLEM: ParityRecord = ParityRecord {
    name: "Golem",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn golem() {
    measured_check(
        &RECORD_GOLEM,
        include_str!("fixtures/batch2/g08/Golem.txt"),
        include_bytes!("fixtures/batch2/g08/Golem.f32"),
    );
}

#[test]
fn golem_mode_sweep() {
    for rate in [44100.0, 96000.0] {
        for mode in 0..6 {
            let mut effect =
                music_things_airwindows::create_with_seed("Golem", rate, 0x00c0_ffee).unwrap();
            effect.set_param(2, (mode as f32 + 0.5) / 5.999);
            let (left, right) = common::stimulus::aw_stimulus(rate);
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
            assert!(
                out_l.iter().chain(&out_r).all(|sample| sample.is_finite()),
                "rate {rate}, mode {mode}"
            );
        }
    }
}

#[test]
fn glitchshifter_note_sweep() {
    for rate in [44100.0, 96000.0] {
        for note in 0..25 {
            let mut effect =
                music_things_airwindows::create_with_seed("GlitchShifter", rate, 0x00c0_ffee)
                    .unwrap();
            effect.set_param(0, (note as f32 + 0.5) / 25.0);
            let (left, right) = common::stimulus::aw_stimulus(rate);
            let mut out_l = vec![0.0; left.len()];
            let mut out_r = vec![0.0; right.len()];
            effect.process(&left, &right, &mut out_l, &mut out_r);
            assert!(
                out_l.iter().chain(&out_r).all(|sample| sample.is_finite()),
                "rate {rate}, note {note}"
            );
        }
    }
}
