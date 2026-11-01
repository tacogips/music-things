mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};
const RECORD_C5RAWBUSS: ParityRecord = ParityRecord {
    name: "C5RawBuss",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn c5rawbuss() {
    measure(
        "C5RawBuss",
        include_str!("fixtures/batch1/g06/C5RawBuss.txt"),
        include_bytes!("fixtures/batch1/g06/C5RawBuss.f32"),
    );
    check_plugin(
        &RECORD_C5RAWBUSS,
        include_str!("fixtures/batch1/g06/C5RawBuss.txt"),
        include_bytes!("fixtures/batch1/g06/C5RawBuss.f32"),
    );
}

const RECORD_C5RAWCHANNEL: ParityRecord = ParityRecord {
    name: "C5RawChannel",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn c5rawchannel() {
    measure(
        "C5RawChannel",
        include_str!("fixtures/batch1/g06/C5RawChannel.txt"),
        include_bytes!("fixtures/batch1/g06/C5RawChannel.f32"),
    );
    check_plugin(
        &RECORD_C5RAWCHANNEL,
        include_str!("fixtures/batch1/g06/C5RawChannel.txt"),
        include_bytes!("fixtures/batch1/g06/C5RawChannel.f32"),
    );
}

fn measure(name: &str, meta: &str, data: &[u8]) {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect = music_things_airwindows::create_with_seed(name, rate, 0x00c0ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25]
                .iter()
                .cycle()
                .take(effect.param_count())
                .enumerate()
            {
                effect.set_param(i, *value);
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
            let i = if k < 256 { k } else { 256 + 32 * (k - 256) };
            max = max
                .max((l[i] - fixture.renders[render][2 * k]).abs())
                .max((r[i] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    println!("{name} max_abs={max:e}");
}

const RECORD_CAPACITOR: ParityRecord = ParityRecord {
    name: "Capacitor",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn capacitor() {
    measure(
        "Capacitor",
        include_str!("fixtures/batch1/g06/Capacitor.txt"),
        include_bytes!("fixtures/batch1/g06/Capacitor.f32"),
    );
    check_plugin(
        &RECORD_CAPACITOR,
        include_str!("fixtures/batch1/g06/Capacitor.txt"),
        include_bytes!("fixtures/batch1/g06/Capacitor.f32"),
    );
}

const RECORD_CAPACITOR2: ParityRecord = ParityRecord {
    name: "Capacitor2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn capacitor2() {
    measure(
        "Capacitor2",
        include_str!("fixtures/batch1/g06/Capacitor2.txt"),
        include_bytes!("fixtures/batch1/g06/Capacitor2.f32"),
    );
    check_plugin(
        &RECORD_CAPACITOR2,
        include_str!("fixtures/batch1/g06/Capacitor2.txt"),
        include_bytes!("fixtures/batch1/g06/Capacitor2.f32"),
    );
}

const RECORD_CALIBRE: ParityRecord = ParityRecord {
    name: "Calibre",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn calibre() {
    measure(
        "Calibre",
        include_str!("fixtures/batch1/g06/Calibre.txt"),
        include_bytes!("fixtures/batch1/g06/Calibre.f32"),
    );
    check_plugin(
        &RECORD_CALIBRE,
        include_str!("fixtures/batch1/g06/Calibre.txt"),
        include_bytes!("fixtures/batch1/g06/Calibre.f32"),
    );
}

const RECORD_CHAMBER: ParityRecord = ParityRecord {
    name: "Chamber",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn chamber() {
    measure(
        "Chamber",
        include_str!("fixtures/batch1/g06/Chamber.txt"),
        include_bytes!("fixtures/batch1/g06/Chamber.f32"),
    );
    check_plugin(
        &RECORD_CHAMBER,
        include_str!("fixtures/batch1/g06/Chamber.txt"),
        include_bytes!("fixtures/batch1/g06/Chamber.f32"),
    );
}

const RECORD_CHAMBER2: ParityRecord = ParityRecord {
    name: "Chamber2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn chamber2() {
    measure(
        "Chamber2",
        include_str!("fixtures/batch1/g06/Chamber2.txt"),
        include_bytes!("fixtures/batch1/g06/Chamber2.f32"),
    );
    check_plugin(
        &RECORD_CHAMBER2,
        include_str!("fixtures/batch1/g06/Chamber2.txt"),
        include_bytes!("fixtures/batch1/g06/Chamber2.f32"),
    );
}

const RECORD_CABS: ParityRecord = ParityRecord {
    name: "Cabs",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn cabs() {
    measure(
        "Cabs",
        include_str!("fixtures/batch1/g06/Cabs.txt"),
        include_bytes!("fixtures/batch1/g06/Cabs.f32"),
    );
    check_plugin(
        &RECORD_CABS,
        include_str!("fixtures/batch1/g06/Cabs.txt"),
        include_bytes!("fixtures/batch1/g06/Cabs.f32"),
    );
}

#[test]
fn reset_preserves_params() {
    for name in [
        "C5RawBuss",
        "C5RawChannel",
        "Cabs",
        "Calibre",
        "Capacitor",
        "Capacitor2",
        "Chamber",
        "Chamber2",
    ] {
        let mut effect = music_things_airwindows::create_with_seed(name, 44_100.0, 123).unwrap();
        for index in 0..effect.param_count() {
            effect.set_param(index, 0.37);
        }
        effect.reset();
        for index in 0..effect.param_count() {
            assert_eq!(effect.get_param(index), 0.37, "{name} parameter {index}");
        }
    }
}
