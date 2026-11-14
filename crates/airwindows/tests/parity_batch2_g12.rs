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

fn check(record: &ParityRecord, meta: &str, data: &[u8]) {
    check_reset_preserves_params(record.name);
    check_plugin(record, meta, data);
    let fixture = common::fixture::Fixture::parse(meta, data);
    let alt = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25];
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(record.name, rate, 0x00c0_ffee).unwrap();
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
    eprintln!("{} measured max-abs: {maximum:?}", record.name);
}

const RECORD_HYPE: ParityRecord = ParityRecord {
    name: "Hype",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_HYPERSONIC: ParityRecord = ParityRecord {
    name: "Hypersonic",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_HYPERSONX: ParityRecord = ParityRecord {
    name: "HypersonX",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_INFINITY: ParityRecord = ParityRecord {
    name: "Infinity",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_INFINITY2: ParityRecord = ParityRecord {
    name: "Infinity2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_INFRASONIC: ParityRecord = ParityRecord {
    name: "Infrasonic",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn hype() {
    check(
        &RECORD_HYPE,
        include_str!("fixtures/batch2/g12/Hype.txt"),
        include_bytes!("fixtures/batch2/g12/Hype.f32"),
    );
}
#[test]
fn hypersonic() {
    check(
        &RECORD_HYPERSONIC,
        include_str!("fixtures/batch2/g12/Hypersonic.txt"),
        include_bytes!("fixtures/batch2/g12/Hypersonic.f32"),
    );
}
#[test]
fn hypersonx() {
    check(
        &RECORD_HYPERSONX,
        include_str!("fixtures/batch2/g12/HypersonX.txt"),
        include_bytes!("fixtures/batch2/g12/HypersonX.f32"),
    );
}
#[test]
fn infinity() {
    check(
        &RECORD_INFINITY,
        include_str!("fixtures/batch2/g12/Infinity.txt"),
        include_bytes!("fixtures/batch2/g12/Infinity.f32"),
    );
}
#[test]
fn infinity2() {
    check(
        &RECORD_INFINITY2,
        include_str!("fixtures/batch2/g12/Infinity2.txt"),
        include_bytes!("fixtures/batch2/g12/Infinity2.f32"),
    );
}
#[test]
fn infrasonic() {
    check(
        &RECORD_INFRASONIC,
        include_str!("fixtures/batch2/g12/Infrasonic.txt"),
        include_bytes!("fixtures/batch2/g12/Infrasonic.f32"),
    );
}

#[test]
fn hypersonx_mode_sweep() {
    for rate in [44100.0, 96000.0] {
        for mode in 0..7 {
            let mut effect =
                music_things_airwindows::create_with_seed("HypersonX", rate, 0x00c0_ffee).unwrap();
            effect.set_param(0, (mode as f32 + 0.5) / 6.999);
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
                "mode {mode} rate {rate}"
            );
        }
    }
}
