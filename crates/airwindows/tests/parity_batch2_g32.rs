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

fn measure(record: &ParityRecord, meta: &str, bytes: &[u8]) {
    check_reset_preserves_params(record.name);
    check_plugin(record, meta, bytes);
    let fixture = common::fixture::Fixture::parse(meta, bytes);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(record.name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            let alt = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25];
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
    println!("{} measured max-abs: {maximum:e}", record.name);
}

const RECORD_PDBUSS: ParityRecord = ParityRecord {
    name: "PDBuss",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pdbuss() {
    measure(
        &RECORD_PDBUSS,
        include_str!("fixtures/batch2/g32/PDBuss.txt"),
        include_bytes!("fixtures/batch2/g32/PDBuss.f32"),
    );
}

const RECORD_PDCHANNEL: ParityRecord = ParityRecord {
    name: "PDChannel",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pdchannel() {
    measure(
        &RECORD_PDCHANNEL,
        include_str!("fixtures/batch2/g32/PDChannel.txt"),
        include_bytes!("fixtures/batch2/g32/PDChannel.f32"),
    );
}

const RECORD_PEAKSONLY: ParityRecord = ParityRecord {
    name: "PeaksOnly",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn peaksonly() {
    measure(
        &RECORD_PEAKSONLY,
        include_str!("fixtures/batch2/g32/PeaksOnly.txt"),
        include_bytes!("fixtures/batch2/g32/PeaksOnly.f32"),
    );
}

const RECORD_PHASENUDGE: ParityRecord = ParityRecord {
    name: "PhaseNudge",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn phasenudge() {
    measure(
        &RECORD_PHASENUDGE,
        include_str!("fixtures/batch2/g32/PhaseNudge.txt"),
        include_bytes!("fixtures/batch2/g32/PhaseNudge.f32"),
    );
}

const RECORD_PITCHDELAY: ParityRecord = ParityRecord {
    name: "PitchDelay",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pitchdelay() {
    measure(
        &RECORD_PITCHDELAY,
        include_str!("fixtures/batch2/g32/PitchDelay.txt"),
        include_bytes!("fixtures/batch2/g32/PitchDelay.f32"),
    );
}

const RECORD_PITCHNASTY: ParityRecord = ParityRecord {
    name: "PitchNasty",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pitchnasty() {
    measure(
        &RECORD_PITCHNASTY,
        include_str!("fixtures/batch2/g32/PitchNasty.txt"),
        include_bytes!("fixtures/batch2/g32/PitchNasty.f32"),
    );
}
