mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};
const RECORD_COMPRESATURATOR: ParityRecord = ParityRecord {
    name: "Compresaturator",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn compresaturator() {
    measure(
        "Compresaturator",
        include_str!("fixtures/batch1/g09/Compresaturator.txt"),
        include_bytes!("fixtures/batch1/g09/Compresaturator.f32"),
    );
    check_plugin(
        &RECORD_COMPRESATURATOR,
        include_str!("fixtures/batch1/g09/Compresaturator.txt"),
        include_bytes!("fixtures/batch1/g09/Compresaturator.f32"),
    );
}
const RECORD_CONSOLE4BUSS: ParityRecord = ParityRecord {
    name: "Console4Buss",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console4buss() {
    measure(
        "Console4Buss",
        include_str!("fixtures/batch1/g09/Console4Buss.txt"),
        include_bytes!("fixtures/batch1/g09/Console4Buss.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE4BUSS,
        include_str!("fixtures/batch1/g09/Console4Buss.txt"),
        include_bytes!("fixtures/batch1/g09/Console4Buss.f32"),
    );
}
const RECORD_CONSOLE4CHANNEL: ParityRecord = ParityRecord {
    name: "Console4Channel",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console4channel() {
    measure(
        "Console4Channel",
        include_str!("fixtures/batch1/g09/Console4Channel.txt"),
        include_bytes!("fixtures/batch1/g09/Console4Channel.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE4CHANNEL,
        include_str!("fixtures/batch1/g09/Console4Channel.txt"),
        include_bytes!("fixtures/batch1/g09/Console4Channel.f32"),
    );
}
const RECORD_CONSOLE5BUSS: ParityRecord = ParityRecord {
    name: "Console5Buss",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console5buss() {
    measure(
        "Console5Buss",
        include_str!("fixtures/batch1/g09/Console5Buss.txt"),
        include_bytes!("fixtures/batch1/g09/Console5Buss.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE5BUSS,
        include_str!("fixtures/batch1/g09/Console5Buss.txt"),
        include_bytes!("fixtures/batch1/g09/Console5Buss.f32"),
    );
}
const RECORD_CONSOLE5CHANNEL: ParityRecord = ParityRecord {
    name: "Console5Channel",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console5channel() {
    measure(
        "Console5Channel",
        include_str!("fixtures/batch1/g09/Console5Channel.txt"),
        include_bytes!("fixtures/batch1/g09/Console5Channel.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE5CHANNEL,
        include_str!("fixtures/batch1/g09/Console5Channel.txt"),
        include_bytes!("fixtures/batch1/g09/Console5Channel.f32"),
    );
}
const RECORD_CONSOLE5DARKCH: ParityRecord = ParityRecord {
    name: "Console5DarkCh",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console5darkch() {
    measure(
        "Console5DarkCh",
        include_str!("fixtures/batch1/g09/Console5DarkCh.txt"),
        include_bytes!("fixtures/batch1/g09/Console5DarkCh.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE5DARKCH,
        include_str!("fixtures/batch1/g09/Console5DarkCh.txt"),
        include_bytes!("fixtures/batch1/g09/Console5DarkCh.f32"),
    );
}
const RECORD_CONSOLE6BUSS: ParityRecord = ParityRecord {
    name: "Console6Buss",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console6buss() {
    measure(
        "Console6Buss",
        include_str!("fixtures/batch1/g09/Console6Buss.txt"),
        include_bytes!("fixtures/batch1/g09/Console6Buss.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE6BUSS,
        include_str!("fixtures/batch1/g09/Console6Buss.txt"),
        include_bytes!("fixtures/batch1/g09/Console6Buss.f32"),
    );
}
const RECORD_CONSOLE6CHANNEL: ParityRecord = ParityRecord {
    name: "Console6Channel",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console6channel() {
    measure(
        "Console6Channel",
        include_str!("fixtures/batch1/g09/Console6Channel.txt"),
        include_bytes!("fixtures/batch1/g09/Console6Channel.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE6CHANNEL,
        include_str!("fixtures/batch1/g09/Console6Channel.txt"),
        include_bytes!("fixtures/batch1/g09/Console6Channel.f32"),
    );
}

fn measure(name: &str, meta: &str, data: &[u8]) {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let alt = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25];
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for i in 0..fixture.params.len() {
                effect.set_param(i, alt[i % alt.len()]);
            }
        }
        let mut l = vec![0.0; left.len()];
        let mut r = vec![0.0; right.len()];
        for b in (0..left.len()).step_by(128) {
            effect.process(
                &left[b..b + 128],
                &right[b..b + 128],
                &mut l[b..b + 128],
                &mut r[b..b + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            max = max
                .max((l[frame] - fixture.renders[render][2 * k]).abs())
                .max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    println!("{name} max_abs={max:e}");
}

#[test]
fn reset_keeps_params() {
    for name in [
        "Compresaturator",
        "Console4Buss",
        "Console4Channel",
        "Console5Buss",
        "Console5Channel",
        "Console5DarkCh",
        "Console6Buss",
        "Console6Channel",
    ] {
        let mut effect =
            music_things_airwindows::create_with_seed(name, 44100.0, 0x00c0_ffee).unwrap();
        let values = [0.3_f32, 0.4, 0.6, 0.7, 0.8];
        for (i, value) in values
            .iter()
            .copied()
            .enumerate()
            .take(effect.param_count())
        {
            assert_ne!(
                effect.get_param(i).to_bits(),
                value.to_bits(),
                "{name} param {i}"
            );
            effect.set_param(i, value);
        }
        effect.reset();
        for (i, value) in values
            .iter()
            .copied()
            .enumerate()
            .take(effect.param_count())
        {
            assert_eq!(
                effect.get_param(i).to_bits(),
                value.to_bits(),
                "{name} param {i}"
            );
        }
        let input = [0.0; 128];
        let mut left = [0.0; 128];
        let mut right = [0.0; 128];
        effect.process(&input, &input, &mut left, &mut right);
        assert!(
            left.iter()
                .chain(right.iter())
                .all(|sample| sample.is_finite()),
            "{name}"
        );
    }
}
