mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};
const NODE: ParityRecord = ParityRecord {
    name: "NodeDither",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn nodedither() {
    let meta = include_str!("fixtures/batch2/g29/NodeDither.txt");
    let data = include_bytes!("fixtures/batch2/g29/NodeDither.f32");
    check_plugin(&NODE, meta, data);
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("NodeDither", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
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
            max = max.max((l[frame] - fixture.renders[render][2 * k]).abs());
            max = max.max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert_eq!(max, NODE.measured_max_abs);
}

const NOISE: ParityRecord = ParityRecord {
    name: "Noise",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn noise() {
    let meta = include_str!("fixtures/batch2/g29/Noise.txt");
    let data = include_bytes!("fixtures/batch2/g29/Noise.f32");
    check_plugin(&NOISE, meta, data);
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Noise", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
            for (i, v) in [0.15, 0.65, 0.95, 0.45].into_iter().enumerate() {
                effect.set_param(i + 2, v);
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
            max = max.max((l[frame] - fixture.renders[render][2 * k]).abs());
            max = max.max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert_eq!(max, NOISE.measured_max_abs);
}

#[test]
fn noise_modes() {
    let mut effect = music_things_airwindows::create_with_seed("Noise", 44100.0, 42).unwrap();
    for mode in 0..39 {
        effect.set_param(1, mode as f32 / 38.0);
        effect.set_param(2, mode as f32 / 38.0);
        let mut left = [0.0; 8];
        let mut right = [0.0; 8];
        effect.process(&[0.25; 8], &[-0.25; 8], &mut left, &mut right);
        assert!(left.iter().chain(&right).all(|x| x.is_finite()));
    }
}
