mod common;

use common::parity::{check_plugin, ParityClass, ParityRecord};

const RECORD_MV: ParityRecord = ParityRecord {
    name: "MV",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "Measured max-abs 0.0 across all three renders",
};

#[test]
fn mv_parity() {
    check_plugin(
        &RECORD_MV,
        include_str!("fixtures/batch2/g26/MV.txt"),
        include_bytes!("fixtures/batch2/g26/MV.f32"),
    );
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g26/MV.txt"),
        include_bytes!("fixtures/batch2/g26/MV.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("MV", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (index, value) in [0.30, 0.85, 0.15, 0.65, 0.95].into_iter().enumerate() {
                effect.set_param(index, value);
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
    println!("MV measured max-abs: {maximum:?}");
    assert!(maximum <= 1e-6);
}

#[test]
fn mv_stage_sweep() {
    for rate in [44100.0, 96000.0] {
        let (input_l, input_r) = common::stimulus::aw_stimulus(rate);
        for stage in 0..=27 {
            let mut effect =
                music_things_airwindows::create_with_seed("MV", rate, 0x00c0_ffee).unwrap();
            let depth = if stage == 27 {
                1.0
            } else {
                (stage as f32 + 0.5) / 27.0
            };
            effect.set_param(0, depth);
            let mut left = [0.0_f32; 128];
            let mut right = [0.0_f32; 128];
            for block in (0..input_l.len()).step_by(128) {
                effect.process(
                    &input_l[block..block + 128],
                    &input_r[block..block + 128],
                    &mut left,
                    &mut right,
                );
                assert!(
                    left.iter().chain(&right).all(|sample| sample.is_finite()),
                    "stage {stage}, sample rate {rate}, block {block}"
                );
            }
        }
    }
}
