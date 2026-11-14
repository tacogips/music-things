mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};
const RECORD_DYNO: ParityRecord = ParityRecord {
    name: "Dyno",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn dyno() {
    let mut effect =
        music_things_airwindows::create_with_seed("Dyno", 44100.0, 0x00c0_ffee).unwrap();
    effect.set_param(0, 0.3);
    effect.reset();
    assert_eq!(effect.get_param(0), 0.3);
    check_plugin(
        &RECORD_DYNO,
        include_str!("fixtures/batch2/g02/Dyno.txt"),
        include_bytes!("fixtures/batch2/g02/Dyno.f32"),
    );
}

const RECORD_EDISDIM: ParityRecord = ParityRecord {
    name: "EdIsDim",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn edisdim() {
    let mut effect =
        music_things_airwindows::create_with_seed("EdIsDim", 44100.0, 0x00c0_ffee).unwrap();
    effect.set_param(0, 0.3);
    effect.reset();
    assert_eq!(effect.get_param(0), 0.3);
    check_plugin(
        &RECORD_EDISDIM,
        include_str!("fixtures/batch2/g02/EdIsDim.txt"),
        include_bytes!("fixtures/batch2/g02/EdIsDim.f32"),
    );
}
#[test]
fn measured_maxima() {
    for (name, meta, bytes) in [
        (
            "DustBunny",
            include_str!("fixtures/batch2/g02/DustBunny.txt"),
            include_bytes!("fixtures/batch2/g02/DustBunny.f32").as_slice(),
        ),
        (
            "ElectroHat",
            include_str!("fixtures/batch2/g02/ElectroHat.txt"),
            include_bytes!("fixtures/batch2/g02/ElectroHat.f32").as_slice(),
        ),
        (
            "Edge",
            include_str!("fixtures/batch2/g02/Edge.txt"),
            include_bytes!("fixtures/batch2/g02/Edge.f32").as_slice(),
        ),
        (
            "Dynamics",
            include_str!("fixtures/batch2/g02/Dynamics.txt"),
            include_bytes!("fixtures/batch2/g02/Dynamics.f32").as_slice(),
        ),
        (
            "Elation",
            include_str!("fixtures/batch2/g02/Elation.txt"),
            include_bytes!("fixtures/batch2/g02/Elation.f32").as_slice(),
        ),
        (
            "Dyno",
            include_str!("fixtures/batch2/g02/Dyno.txt"),
            include_bytes!("fixtures/batch2/g02/Dyno.f32").as_slice(),
        ),
        (
            "EdIsDim",
            include_str!("fixtures/batch2/g02/EdIsDim.txt"),
            include_bytes!("fixtures/batch2/g02/EdIsDim.f32").as_slice(),
        ),
    ] {
        let fixture = common::fixture::Fixture::parse(meta, bytes);
        let mut maximum = 0.0_f32;
        for render in 0..3 {
            let rate = if render == 2 { 96000.0 } else { 44100.0 };
            let (left, right) = common::stimulus::aw_stimulus(rate);
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
        println!("{name} maximum {maximum:e}");
        assert_eq!(maximum, 0.0);
    }
}
const RECORD_DUSTBUNNY: ParityRecord = ParityRecord {
    name: "DustBunny",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn dustbunny() {
    let mut effect =
        music_things_airwindows::create_with_seed("DustBunny", 44100.0, 0x00c0_ffee).unwrap();
    effect.set_param(0, 0.3);
    effect.reset();
    assert_eq!(effect.get_param(0), 0.3);
    check_plugin(
        &RECORD_DUSTBUNNY,
        include_str!("fixtures/batch2/g02/DustBunny.txt"),
        include_bytes!("fixtures/batch2/g02/DustBunny.f32"),
    );
}

const RECORD_ELECTROHAT: ParityRecord = ParityRecord {
    name: "ElectroHat",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn electrohat() {
    let mut effect =
        music_things_airwindows::create_with_seed("ElectroHat", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..5 {
        effect.set_param(i, (i + 1) as f32 / 7.0);
    }
    effect.reset();
    for i in 0..5 {
        assert_eq!(effect.get_param(i), (i + 1) as f32 / 7.0);
    }
    check_plugin(
        &RECORD_ELECTROHAT,
        include_str!("fixtures/batch2/g02/ElectroHat.txt"),
        include_bytes!("fixtures/batch2/g02/ElectroHat.f32"),
    );
}
#[test]
fn electrohat_mode_sweep() {
    for rate in [44100.0, 96000.0] {
        for mode in 0..6 {
            let mut effect =
                music_things_airwindows::create_with_seed("ElectroHat", rate, 0x00c0_ffee).unwrap();
            effect.set_param(0, (mode as f32 + 0.5) / 5.999);
            let (l, r) = common::stimulus::aw_stimulus(rate);
            let mut ol = vec![0.0; l.len()];
            let mut orr = vec![0.0; r.len()];
            for i in (0..l.len()).step_by(128) {
                effect.process(
                    &l[i..i + 128],
                    &r[i..i + 128],
                    &mut ol[i..i + 128],
                    &mut orr[i..i + 128],
                );
            }
            assert!(ol.iter().chain(&orr).all(|v| v.is_finite()));
        }
    }
}

const RECORD_EDGE: ParityRecord = ParityRecord {
    name: "Edge",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn edge() {
    let mut effect =
        music_things_airwindows::create_with_seed("Edge", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..5 {
        effect.set_param(i, (i + 1) as f32 / 7.0);
    }
    effect.reset();
    for i in 0..5 {
        assert_eq!(effect.get_param(i), (i + 1) as f32 / 7.0);
    }
    check_plugin(
        &RECORD_EDGE,
        include_str!("fixtures/batch2/g02/Edge.txt"),
        include_bytes!("fixtures/batch2/g02/Edge.f32"),
    );
}

const RECORD_DYNAMICS: ParityRecord = ParityRecord {
    name: "Dynamics",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn dynamics() {
    let mut effect =
        music_things_airwindows::create_with_seed("Dynamics", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..4 {
        effect.set_param(i, (i + 1) as f32 / 6.0);
    }
    effect.reset();
    for i in 0..4 {
        assert_eq!(effect.get_param(i), (i + 1) as f32 / 6.0);
    }
    check_plugin(
        &RECORD_DYNAMICS,
        include_str!("fixtures/batch2/g02/Dynamics.txt"),
        include_bytes!("fixtures/batch2/g02/Dynamics.f32"),
    );
}

const RECORD_ELATION: ParityRecord = ParityRecord {
    name: "Elation",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn elation() {
    let mut effect =
        music_things_airwindows::create_with_seed("Elation", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..4 {
        effect.set_param(i, (i + 1) as f32 / 6.0);
    }
    effect.reset();
    for i in 0..4 {
        assert_eq!(effect.get_param(i), (i + 1) as f32 / 6.0);
    }
    check_plugin(
        &RECORD_ELATION,
        include_str!("fixtures/batch2/g02/Elation.txt"),
        include_bytes!("fixtures/batch2/g02/Elation.f32"),
    );
}
