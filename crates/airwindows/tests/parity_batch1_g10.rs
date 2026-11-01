mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

fn check_reset_preserves_params(name: &str) {
    let mut effect = music_things_airwindows::create_with_seed(name, 44_100.0, 123).unwrap();
    for index in 0..effect.param_count() {
        effect.set_param(index, 0.37);
    }
    let displays: Vec<_> = (0..effect.param_count())
        .map(|i| effect.param_display(i))
        .collect();
    effect.reset();
    for (index, display) in displays.iter().enumerate() {
        assert_eq!(effect.get_param(index), 0.37, "{name} parameter {index}");
        assert_eq!(
            &effect.param_display(index),
            display,
            "{name} display {index}"
        );
    }
}
const RECORD_CONSOLE7BUSS: ParityRecord = ParityRecord {
    name: "Console7Buss",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console7buss() {
    check_reset_preserves_params("Console7Buss");
    measure(
        "Console7Buss",
        include_str!("fixtures/batch1/g10/Console7Buss.txt"),
        include_bytes!("fixtures/batch1/g10/Console7Buss.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE7BUSS,
        include_str!("fixtures/batch1/g10/Console7Buss.txt"),
        include_bytes!("fixtures/batch1/g10/Console7Buss.f32"),
    );
}

const RECORD_CONSOLE7CHANNEL: ParityRecord = ParityRecord {
    name: "Console7Channel",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console7channel() {
    check_reset_preserves_params("Console7Channel");
    measure(
        "Console7Channel",
        include_str!("fixtures/batch1/g10/Console7Channel.txt"),
        include_bytes!("fixtures/batch1/g10/Console7Channel.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE7CHANNEL,
        include_str!("fixtures/batch1/g10/Console7Channel.txt"),
        include_bytes!("fixtures/batch1/g10/Console7Channel.f32"),
    );
}

const RECORD_CONSOLE8BUSOUT: ParityRecord = ParityRecord {
    name: "Console8BusOut",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console8busout() {
    check_reset_preserves_params("Console8BusOut");
    measure(
        "Console8BusOut",
        include_str!("fixtures/batch1/g10/Console8BusOut.txt"),
        include_bytes!("fixtures/batch1/g10/Console8BusOut.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE8BUSOUT,
        include_str!("fixtures/batch1/g10/Console8BusOut.txt"),
        include_bytes!("fixtures/batch1/g10/Console8BusOut.f32"),
    );
}

const RECORD_CONSOLE8BUSSHYPE: ParityRecord = ParityRecord {
    name: "Console8BussHype",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console8busshype() {
    check_reset_preserves_params("Console8BussHype");
    measure(
        "Console8BussHype",
        include_str!("fixtures/batch1/g10/Console8BussHype.txt"),
        include_bytes!("fixtures/batch1/g10/Console8BussHype.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE8BUSSHYPE,
        include_str!("fixtures/batch1/g10/Console8BussHype.txt"),
        include_bytes!("fixtures/batch1/g10/Console8BussHype.f32"),
    );
}

const RECORD_CONSOLE8BUSSIN: ParityRecord = ParityRecord {
    name: "Console8BussIn",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console8bussin() {
    check_reset_preserves_params("Console8BussIn");
    measure(
        "Console8BussIn",
        include_str!("fixtures/batch1/g10/Console8BussIn.txt"),
        include_bytes!("fixtures/batch1/g10/Console8BussIn.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE8BUSSIN,
        include_str!("fixtures/batch1/g10/Console8BussIn.txt"),
        include_bytes!("fixtures/batch1/g10/Console8BussIn.f32"),
    );
}

const RECORD_CONSOLE8CHANNELHYPE: ParityRecord = ParityRecord {
    name: "Console8ChannelHype",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console8channelhype() {
    check_reset_preserves_params("Console8ChannelHype");
    measure(
        "Console8ChannelHype",
        include_str!("fixtures/batch1/g10/Console8ChannelHype.txt"),
        include_bytes!("fixtures/batch1/g10/Console8ChannelHype.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE8CHANNELHYPE,
        include_str!("fixtures/batch1/g10/Console8ChannelHype.txt"),
        include_bytes!("fixtures/batch1/g10/Console8ChannelHype.f32"),
    );
}

const RECORD_CONSOLE7CASCADE: ParityRecord = ParityRecord {
    name: "Console7Cascade",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console7cascade() {
    check_reset_preserves_params("Console7Cascade");
    measure(
        "Console7Cascade",
        include_str!("fixtures/batch1/g10/Console7Cascade.txt"),
        include_bytes!("fixtures/batch1/g10/Console7Cascade.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE7CASCADE,
        include_str!("fixtures/batch1/g10/Console7Cascade.txt"),
        include_bytes!("fixtures/batch1/g10/Console7Cascade.f32"),
    );
}

const RECORD_CONSOLE7CRUNCH: ParityRecord = ParityRecord {
    name: "Console7Crunch",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console7crunch() {
    check_reset_preserves_params("Console7Crunch");
    measure(
        "Console7Crunch",
        include_str!("fixtures/batch1/g10/Console7Crunch.txt"),
        include_bytes!("fixtures/batch1/g10/Console7Crunch.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE7CRUNCH,
        include_str!("fixtures/batch1/g10/Console7Crunch.txt"),
        include_bytes!("fixtures/batch1/g10/Console7Crunch.f32"),
    );
}

fn measure(name: &str, meta: &str, data: &[u8]) {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for i in 0..effect.param_count() {
                effect.set_param(
                    i,
                    [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25][i % 9],
                );
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
            max = max
                .max((l[frame] - fixture.renders[render][2 * k]).abs())
                .max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    println!("{name} max_abs={max:e}");
}
