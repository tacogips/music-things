mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

fn check_reset_preserves_params(name: &str) {
    let mut effect = music_things_airwindows::create_with_seed(name, 44100.0, 0x00c0_ffee).unwrap();
    let count = effect.param_count();
    for i in 0..count {
        effect.set_param(i, (i + 1) as f32 / (count + 2) as f32);
    }
    effect.reset();
    for i in 0..count {
        assert_eq!(effect.get_param(i), (i + 1) as f32 / (count + 2) as f32);
    }
}

fn sweep(name: &str, param: usize, modes: usize) {
    for rate in [44100.0, 96000.0] {
        let (left, right) = common::stimulus::aw_stimulus(rate);
        for mode in 0..modes {
            let mut effect =
                music_things_airwindows::create_with_seed(name, rate, 0x00c0_ffee).unwrap();
            effect.set_param(param, (mode as f32 + 0.5) / modes as f32);
            let mut out_l = vec![0.0; left.len()];
            let mut out_r = vec![0.0; right.len()];
            for b in (0..left.len()).step_by(128) {
                effect.process(
                    &left[b..b + 128],
                    &right[b..b + 128],
                    &mut out_l[b..b + 128],
                    &mut out_r[b..b + 128],
                );
            }
            assert!(
                out_l.iter().chain(&out_r).all(|v| v.is_finite()),
                "{name} mode {mode} at {rate}"
            );
        }
    }
}

const FLIPITY: ParityRecord = ParityRecord {
    name: "Flipity",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
const FLOOR: ParityRecord = ParityRecord {
    name: "Floor",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn flipity() {
    check_reset_preserves_params("Flipity");
    check_plugin(
        &FLIPITY,
        include_str!("fixtures/batch2/g07/Flipity.txt"),
        include_bytes!("fixtures/batch2/g07/Flipity.f32"),
    );
}
#[test]
fn floor() {
    check_reset_preserves_params("Floor");
    check_plugin(
        &FLOOR,
        include_str!("fixtures/batch2/g07/Floor.txt"),
        include_bytes!("fixtures/batch2/g07/Floor.f32"),
    );
}
#[test]
fn flipity_modes() {
    sweep("Flipity", 0, 8);
    let expected = [
        (0.25, -0.5),
        (-0.25, -0.5),
        (0.25, 0.5),
        (-0.25, 0.5),
        (-0.5, 0.25),
        (0.5, 0.25),
        (-0.5, -0.25),
        (0.5, -0.25),
    ];
    let mut effect = music_things_airwindows::create_with_seed("Flipity", 44100.0, 1).unwrap();
    for (i, pair) in expected.into_iter().enumerate() {
        effect.set_param(0, (i as f32 + 0.5) / 8.0);
        let mut l = [0.0];
        let mut r = [0.0];
        effect.process(&[0.25], &[-0.5], &mut l, &mut r);
        assert_eq!((l[0], r[0]), pair);
    }
}

const FLUTTER: ParityRecord = ParityRecord {
    name: "Flutter",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const FOCUS: ParityRecord = ParityRecord {
    name: "Focus",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const FRACTURE: ParityRecord = ParityRecord {
    name: "Fracture",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn flutter() {
    check_reset_preserves_params("Flutter");
    check_plugin(
        &FLUTTER,
        include_str!("fixtures/batch2/g07/Flutter.txt"),
        include_bytes!("fixtures/batch2/g07/Flutter.f32"),
    );
}
#[test]
fn focus() {
    check_reset_preserves_params("Focus");
    check_plugin(
        &FOCUS,
        include_str!("fixtures/batch2/g07/Focus.txt"),
        include_bytes!("fixtures/batch2/g07/Focus.f32"),
    );
}
#[test]
fn fracture() {
    check_reset_preserves_params("Fracture");
    check_plugin(
        &FRACTURE,
        include_str!("fixtures/batch2/g07/Fracture.txt"),
        include_bytes!("fixtures/batch2/g07/Fracture.f32"),
    );
}
#[test]
fn focus_modes() {
    sweep("Focus", 2, 5);
}

#[test]
fn measured_maxima() {
    let cases = [
        (
            "Flipity",
            include_str!("fixtures/batch2/g07/Flipity.txt"),
            include_bytes!("fixtures/batch2/g07/Flipity.f32").as_slice(),
        ),
        (
            "Floor",
            include_str!("fixtures/batch2/g07/Floor.txt"),
            include_bytes!("fixtures/batch2/g07/Floor.f32").as_slice(),
        ),
        (
            "Flutter",
            include_str!("fixtures/batch2/g07/Flutter.txt"),
            include_bytes!("fixtures/batch2/g07/Flutter.f32").as_slice(),
        ),
        (
            "Focus",
            include_str!("fixtures/batch2/g07/Focus.txt"),
            include_bytes!("fixtures/batch2/g07/Focus.f32").as_slice(),
        ),
        (
            "Fracture",
            include_str!("fixtures/batch2/g07/Fracture.txt"),
            include_bytes!("fixtures/batch2/g07/Fracture.f32").as_slice(),
        ),
    ];
    for (name, meta, bytes) in cases {
        let fixture = common::fixture::Fixture::parse(meta, bytes);
        let mut max = 0.0f32;
        for render in 0..3 {
            let rate = if render == 2 { 96000.0 } else { 44100.0 };
            let (l, r) = common::stimulus::aw_stimulus(rate);
            let mut effect =
                music_things_airwindows::create_with_seed(name, rate, 0x00c0_ffee).unwrap();
            if render != 0 {
                for (i, v) in [0.30, 0.85, 0.15, 0.65, 0.95]
                    .into_iter()
                    .enumerate()
                    .take(effect.param_count())
                {
                    effect.set_param(i, v);
                }
            }
            let mut left = vec![0.0; l.len()];
            let mut right = vec![0.0; r.len()];
            for b in (0..l.len()).step_by(128) {
                effect.process(
                    &l[b..b + 128],
                    &r[b..b + 128],
                    &mut left[b..b + 128],
                    &mut right[b..b + 128],
                );
            }
            for k in 0..504 {
                let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
                max = max.max((left[frame] - fixture.renders[render][2 * k]).abs());
                max = max.max((right[frame] - fixture.renders[render][2 * k + 1]).abs());
            }
        }
        eprintln!("{name} measured max-abs {max:e}");
        assert!(max <= 1e-6, "{name} exceeds the parity bound: {max}");
    }
}
