mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};
const DITHERMETIMBERS: ParityRecord = ParityRecord {
    name: "DitherMeTimbers",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn dithermetimbers() {
    check_plugin(
        &DITHERMETIMBERS,
        include_str!("fixtures/batch2/g01/DitherMeTimbers.txt"),
        include_bytes!("fixtures/batch2/g01/DitherMeTimbers.f32"),
    );
}

const DRIVE: ParityRecord = ParityRecord {
    name: "Drive",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn drive() {
    check_plugin(
        &DRIVE,
        include_str!("fixtures/batch2/g01/Drive.txt"),
        include_bytes!("fixtures/batch2/g01/Drive.f32"),
    );
}

const DOUBLEPAUL: ParityRecord = ParityRecord {
    name: "DoublePaul",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn doublepaul() {
    check_plugin(
        &DOUBLEPAUL,
        include_str!("fixtures/batch2/g01/DoublePaul.txt"),
        include_bytes!("fixtures/batch2/g01/DoublePaul.f32"),
    );
}

const DUBLY: ParityRecord = ParityRecord {
    name: "Dubly",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn dubly() {
    check_plugin(
        &DUBLY,
        include_str!("fixtures/batch2/g01/Dubly.txt"),
        include_bytes!("fixtures/batch2/g01/Dubly.f32"),
    );
}

const DRUMSLAM: ParityRecord = ParityRecord {
    name: "DrumSlam",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn drumslam() {
    check_plugin(
        &DRUMSLAM,
        include_str!("fixtures/batch2/g01/DrumSlam.txt"),
        include_bytes!("fixtures/batch2/g01/DrumSlam.f32"),
    );
}

const DUBCENTER: ParityRecord = ParityRecord {
    name: "DubCenter",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn dubcenter() {
    check_plugin(
        &DUBCENTER,
        include_str!("fixtures/batch2/g01/DubCenter.txt"),
        include_bytes!("fixtures/batch2/g01/DubCenter.f32"),
    );
}

const DOUBLELAY: ParityRecord = ParityRecord {
    name: "Doublelay",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn doublelay() {
    check_plugin(
        &DOUBLELAY,
        include_str!("fixtures/batch2/g01/Doublelay.txt"),
        include_bytes!("fixtures/batch2/g01/Doublelay.f32"),
    );
}

#[test]
fn measured_maxima() {
    let cases = [
        (
            "DitherMeTimbers",
            include_str!("fixtures/batch2/g01/DitherMeTimbers.txt"),
            &include_bytes!("fixtures/batch2/g01/DitherMeTimbers.f32")[..],
        ),
        (
            "Doublelay",
            include_str!("fixtures/batch2/g01/Doublelay.txt"),
            &include_bytes!("fixtures/batch2/g01/Doublelay.f32")[..],
        ),
        (
            "DoublePaul",
            include_str!("fixtures/batch2/g01/DoublePaul.txt"),
            &include_bytes!("fixtures/batch2/g01/DoublePaul.f32")[..],
        ),
        (
            "Drive",
            include_str!("fixtures/batch2/g01/Drive.txt"),
            &include_bytes!("fixtures/batch2/g01/Drive.f32")[..],
        ),
        (
            "DrumSlam",
            include_str!("fixtures/batch2/g01/DrumSlam.txt"),
            &include_bytes!("fixtures/batch2/g01/DrumSlam.f32")[..],
        ),
        (
            "DubCenter",
            include_str!("fixtures/batch2/g01/DubCenter.txt"),
            &include_bytes!("fixtures/batch2/g01/DubCenter.f32")[..],
        ),
        (
            "Dubly",
            include_str!("fixtures/batch2/g01/Dubly.txt"),
            &include_bytes!("fixtures/batch2/g01/Dubly.f32")[..],
        ),
    ];
    let alt = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25];
    for (name, meta, data) in cases {
        let fixture = common::fixture::Fixture::parse(meta, data);
        let mut maximum = 0.0_f32;
        for render in 0..3 {
            let rate = if render == 2 { 96000.0 } else { 44100.0 };
            let (left, right) = common::stimulus::aw_stimulus(rate);
            let mut effect =
                music_things_airwindows::create_with_seed(name, rate, 0x00c0_ffee).unwrap();
            if render != 0 {
                for i in 0..effect.param_count() {
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
                maximum = maximum
                    .max((out_l[frame] - fixture.renders[render][2 * k]).abs())
                    .max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
            }
        }
        println!("{name} max-abs {maximum:?}");
        assert_eq!(maximum, 0.0, "{name} recorded maximum");
    }
}
