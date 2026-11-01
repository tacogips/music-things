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

const RECORD_BASSDRIVE: ParityRecord = ParityRecord {
    name: "BassDrive",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn bassdrive() {
    check_reset_preserves_params("BassDrive");
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch1/g03/BassDrive.txt"),
        include_bytes!("fixtures/batch1/g03/BassDrive.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("BassDrive", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65, 0.95].into_iter().enumerate() {
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
    assert_eq!(maximum, RECORD_BASSDRIVE.measured_max_abs);
    check_plugin(
        &RECORD_BASSDRIVE,
        include_str!("fixtures/batch1/g03/BassDrive.txt"),
        include_bytes!("fixtures/batch1/g03/BassDrive.f32"),
    );
}

const RECORD_BASSKIT: ParityRecord = ParityRecord {
    name: "BassKit",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn basskit() {
    check_reset_preserves_params("BassKit");
    check_plugin(
        &RECORD_BASSKIT,
        include_str!("fixtures/batch1/g03/BassKit.txt"),
        include_bytes!("fixtures/batch1/g03/BassKit.f32"),
    );
}

const RECORD_BAXANDALL: ParityRecord = ParityRecord {
    name: "Baxandall",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn baxandall() {
    check_reset_preserves_params("Baxandall");
    check_plugin(
        &RECORD_BAXANDALL,
        include_str!("fixtures/batch1/g03/Baxandall.txt"),
        include_bytes!("fixtures/batch1/g03/Baxandall.f32"),
    );
}

const RECORD_BAXANDALL2: ParityRecord = ParityRecord {
    name: "Baxandall2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn baxandall2() {
    check_reset_preserves_params("Baxandall2");
    check_plugin(
        &RECORD_BAXANDALL2,
        include_str!("fixtures/batch1/g03/Baxandall2.txt"),
        include_bytes!("fixtures/batch1/g03/Baxandall2.f32"),
    );
}

const RECORD_BEAM: ParityRecord = ParityRecord {
    name: "Beam",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn beam() {
    check_reset_preserves_params("Beam");
    check_plugin(
        &RECORD_BEAM,
        include_str!("fixtures/batch1/g03/Beam.txt"),
        include_bytes!("fixtures/batch1/g03/Beam.f32"),
    );
}

const RECORD_BIGAMP: ParityRecord = ParityRecord {
    name: "BigAmp",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn bigamp() {
    check_reset_preserves_params("BigAmp");
    check_plugin(
        &RECORD_BIGAMP,
        include_str!("fixtures/batch1/g03/BigAmp.txt"),
        include_bytes!("fixtures/batch1/g03/BigAmp.f32"),
    );
}

const RECORD_BIQUAD: ParityRecord = ParityRecord {
    name: "Biquad",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn biquad() {
    check_reset_preserves_params("Biquad");
    check_plugin(
        &RECORD_BIQUAD,
        include_str!("fixtures/batch1/g03/Biquad.txt"),
        include_bytes!("fixtures/batch1/g03/Biquad.f32"),
    );
}

const RECORD_BIQUAD2: ParityRecord = ParityRecord {
    name: "Biquad2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn biquad2() {
    check_reset_preserves_params("Biquad2");
    check_plugin(
        &RECORD_BIQUAD2,
        include_str!("fixtures/batch1/g03/Biquad2.txt"),
        include_bytes!("fixtures/batch1/g03/Biquad2.f32"),
    );
}
