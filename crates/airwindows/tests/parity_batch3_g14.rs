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

fn check_measured(record: &ParityRecord, meta: &str, data: &[u8]) {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(record.name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25]
                .into_iter()
                .take(fixture.params.len())
                .enumerate()
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
            maximum = maximum.max((out_l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    eprintln!("{} measured max-abs: {maximum}", record.name);
    assert!(
        maximum <= 1e-6,
        "{} measured max-abs: {maximum}",
        record.name
    );
}

const RECORD_ZLOWPASS2: ParityRecord = ParityRecord {
    name: "ZLowpass2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn zlowpass2() {
    check_reset_preserves_params("ZLowpass2");
    let meta = include_str!("fixtures/batch3/g14/ZLowpass2.txt");
    let data = include_bytes!("fixtures/batch3/g14/ZLowpass2.f32");
    check_plugin(&RECORD_ZLOWPASS2, meta, data);
    check_measured(&RECORD_ZLOWPASS2, meta, data);
}

const RECORD_ZNOTCH: ParityRecord = ParityRecord {
    name: "ZNotch",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn znotch() {
    check_reset_preserves_params("ZNotch");
    let meta = include_str!("fixtures/batch3/g14/ZNotch.txt");
    let data = include_bytes!("fixtures/batch3/g14/ZNotch.f32");
    check_plugin(&RECORD_ZNOTCH, meta, data);
    check_measured(&RECORD_ZNOTCH, meta, data);
}

const RECORD_ZNOTCH2: ParityRecord = ParityRecord {
    name: "ZNotch2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn znotch2() {
    check_reset_preserves_params("ZNotch2");
    let meta = include_str!("fixtures/batch3/g14/ZNotch2.txt");
    let data = include_bytes!("fixtures/batch3/g14/ZNotch2.f32");
    check_plugin(&RECORD_ZNOTCH2, meta, data);
    check_measured(&RECORD_ZNOTCH2, meta, data);
}

const RECORD_ZREGION: ParityRecord = ParityRecord {
    name: "ZRegion",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn zregion() {
    check_reset_preserves_params("ZRegion");
    let meta = include_str!("fixtures/batch3/g14/ZRegion.txt");
    let data = include_bytes!("fixtures/batch3/g14/ZRegion.f32");
    check_plugin(&RECORD_ZREGION, meta, data);
    check_measured(&RECORD_ZREGION, meta, data);
}

const RECORD_ZREGION2: ParityRecord = ParityRecord {
    name: "ZRegion2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn zregion2() {
    check_reset_preserves_params("ZRegion2");
    let meta = include_str!("fixtures/batch3/g14/ZRegion2.txt");
    let data = include_bytes!("fixtures/batch3/g14/ZRegion2.f32");
    check_plugin(&RECORD_ZREGION2, meta, data);
    check_measured(&RECORD_ZREGION2, meta, data);
}
