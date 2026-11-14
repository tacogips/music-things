mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

const RECORD_RIGHTOMONO: ParityRecord = ParityRecord {
    name: "RightoMono",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn rightomono() {
    let mut effect =
        music_things_airwindows::create_with_seed("RightoMono", 44100.0, 0x00c0_ffee).unwrap();
    effect.reset();
    check_plugin(
        &RECORD_RIGHTOMONO,
        include_str!("fixtures/batch3/g04/RightoMono.txt"),
        include_bytes!("fixtures/batch3/g04/RightoMono.f32"),
    );
}

const RECORD_SIDEPASS: ParityRecord = ParityRecord {
    name: "Sidepass",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn sidepass() {
    let mut effect =
        music_things_airwindows::create_with_seed("Sidepass", 44100.0, 0x00c0_ffee).unwrap();
    effect.set_param(0, 0.25);
    effect.reset();
    assert_eq!(effect.get_param(0), 0.25);
    check_plugin(
        &RECORD_SIDEPASS,
        include_str!("fixtures/batch3/g04/Sidepass.txt"),
        include_bytes!("fixtures/batch3/g04/Sidepass.f32"),
    );
}

const RECORD_SIDEDULL: ParityRecord = ParityRecord {
    name: "SideDull",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn sidedull() {
    let mut effect =
        music_things_airwindows::create_with_seed("SideDull", 44100.0, 0x00c0_ffee).unwrap();
    effect.set_param(0, 0.25);
    effect.reset();
    assert_eq!(effect.get_param(0), 0.25);
    check_plugin(
        &RECORD_SIDEDULL,
        include_str!("fixtures/batch3/g04/SideDull.txt"),
        include_bytes!("fixtures/batch3/g04/SideDull.f32"),
    );
}

const RECORD_SHAPE: ParityRecord = ParityRecord {
    name: "Shape",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn shape() {
    let mut effect =
        music_things_airwindows::create_with_seed("Shape", 44100.0, 0x00c0_ffee).unwrap();
    effect.set_param(0, 0.25);
    effect.set_param(1, 0.75);
    effect.reset();
    assert_eq!(effect.get_param(0), 0.25);
    assert_eq!(effect.get_param(1), 0.75);
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch3/g04/Shape.txt"),
        include_bytes!("fixtures/batch3/g04/Shape.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Shape", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
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
    assert_eq!(maximum, RECORD_SHAPE.measured_max_abs);
    check_plugin(
        &RECORD_SHAPE,
        include_str!("fixtures/batch3/g04/Shape.txt"),
        include_bytes!("fixtures/batch3/g04/Shape.f32"),
    );
}

const RECORD_SINGLEENDEDTRIODE: ParityRecord = ParityRecord {
    name: "SingleEndedTriode",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn singleendedtriode() {
    let mut effect =
        music_things_airwindows::create_with_seed("SingleEndedTriode", 44100.0, 0x00c0_ffee)
            .unwrap();
    for i in 0..4 {
        effect.set_param(i, (i + 1) as f32 / 6.0);
    }
    effect.reset();
    for i in 0..4 {
        assert_eq!(effect.get_param(i), (i + 1) as f32 / 6.0);
    }
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch3/g04/SingleEndedTriode.txt"),
        include_bytes!("fixtures/batch3/g04/SingleEndedTriode.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("SingleEndedTriode", rate, 0x00c0_ffee)
                .unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
            effect.set_param(2, 0.15);
            effect.set_param(3, 0.65);
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
    assert_eq!(maximum, RECORD_SINGLEENDEDTRIODE.measured_max_abs);
    check_plugin(
        &RECORD_SINGLEENDEDTRIODE,
        include_str!("fixtures/batch3/g04/SingleEndedTriode.txt"),
        include_bytes!("fixtures/batch3/g04/SingleEndedTriode.f32"),
    );
}

const RECORD_SILHOUETTE: ParityRecord = ParityRecord {
    name: "Silhouette",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn silhouette() {
    let mut effect =
        music_things_airwindows::create_with_seed("Silhouette", 44100.0, 0x00c0_ffee).unwrap();
    effect.set_param(0, 0.25);
    effect.reset();
    assert_eq!(effect.get_param(0), 0.25);
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch3/g04/Silhouette.txt"),
        include_bytes!("fixtures/batch3/g04/Silhouette.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Silhouette", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
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
    assert_eq!(maximum, RECORD_SILHOUETTE.measured_max_abs);
    check_plugin(
        &RECORD_SILHOUETTE,
        include_str!("fixtures/batch3/g04/Silhouette.txt"),
        include_bytes!("fixtures/batch3/g04/Silhouette.f32"),
    );
}

const RECORD_REVERB: ParityRecord = ParityRecord {
    name: "Reverb",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn reverb() {
    let mut effect =
        music_things_airwindows::create_with_seed("Reverb", 44100.0, 0x00c0_ffee).unwrap();
    effect.set_param(0, 0.25);
    effect.set_param(1, 0.75);
    effect.reset();
    assert_eq!(effect.get_param(0), 0.25);
    assert_eq!(effect.get_param(1), 0.75);
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch3/g04/Reverb.txt"),
        include_bytes!("fixtures/batch3/g04/Reverb.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Reverb", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
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
    assert_eq!(maximum, RECORD_REVERB.measured_max_abs);
    check_plugin(
        &RECORD_REVERB,
        include_str!("fixtures/batch3/g04/Reverb.txt"),
        include_bytes!("fixtures/batch3/g04/Reverb.f32"),
    );
}

const RECORD_RIGHTEOUS4: ParityRecord = ParityRecord {
    name: "Righteous4",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn righteous4() {
    let mut effect =
        music_things_airwindows::create_with_seed("Righteous4", 44100.0, 0x00c0_ffee).unwrap();
    effect.set_param(0, 0.25);
    effect.set_param(1, 0.75);
    effect.reset();
    assert_eq!(effect.get_param(0), 0.25);
    assert_eq!(effect.get_param(1), 0.75);
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch3/g04/Righteous4.txt"),
        include_bytes!("fixtures/batch3/g04/Righteous4.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Righteous4", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
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
    assert_eq!(maximum, RECORD_RIGHTEOUS4.measured_max_abs);
    check_plugin(
        &RECORD_RIGHTEOUS4,
        include_str!("fixtures/batch3/g04/Righteous4.txt"),
        include_bytes!("fixtures/batch3/g04/Righteous4.f32"),
    );
}
