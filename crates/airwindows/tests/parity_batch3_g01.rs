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

fn check_measured_max(record: &ParityRecord, meta: &str, data: &[u8]) {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let alt = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25];
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(record.name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for i in 0..fixture.params.len() {
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
            maximum = maximum.max((out_l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((out_r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    println!("{} measured max-abs: {}", record.name, maximum);
    assert_eq!(
        maximum, record.measured_max_abs,
        "{} measured max-abs",
        record.name
    );
}

const RECORD_POWERSAG: ParityRecord = ParityRecord {
    name: "PowerSag",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn powersag() {
    check_reset_preserves_params("PowerSag");
    check_measured_max(
        &RECORD_POWERSAG,
        include_str!("fixtures/batch3/g01/PowerSag.txt"),
        include_bytes!("fixtures/batch3/g01/PowerSag.f32"),
    );
    check_plugin(
        &RECORD_POWERSAG,
        include_str!("fixtures/batch3/g01/PowerSag.txt"),
        include_bytes!("fixtures/batch3/g01/PowerSag.f32"),
    );
}

const RECORD_POWERSAG2: ParityRecord = ParityRecord {
    name: "PowerSag2",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn powersag2() {
    check_reset_preserves_params("PowerSag2");
    check_measured_max(
        &RECORD_POWERSAG2,
        include_str!("fixtures/batch3/g01/PowerSag2.txt"),
        include_bytes!("fixtures/batch3/g01/PowerSag2.f32"),
    );
    check_plugin(
        &RECORD_POWERSAG2,
        include_str!("fixtures/batch3/g01/PowerSag2.txt"),
        include_bytes!("fixtures/batch3/g01/PowerSag2.f32"),
    );
}

const RECORD_PRECIOUS: ParityRecord = ParityRecord {
    name: "Precious",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn precious() {
    check_reset_preserves_params("Precious");
    check_measured_max(
        &RECORD_PRECIOUS,
        include_str!("fixtures/batch3/g01/Precious.txt"),
        include_bytes!("fixtures/batch3/g01/Precious.f32"),
    );
    check_plugin(
        &RECORD_PRECIOUS,
        include_str!("fixtures/batch3/g01/Precious.txt"),
        include_bytes!("fixtures/batch3/g01/Precious.f32"),
    );
}

const RECORD_PREPONDERANT: ParityRecord = ParityRecord {
    name: "Preponderant",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn preponderant() {
    check_reset_preserves_params("Preponderant");
    check_measured_max(
        &RECORD_PREPONDERANT,
        include_str!("fixtures/batch3/g01/Preponderant.txt"),
        include_bytes!("fixtures/batch3/g01/Preponderant.f32"),
    );
    check_plugin(
        &RECORD_PREPONDERANT,
        include_str!("fixtures/batch3/g01/Preponderant.txt"),
        include_bytes!("fixtures/batch3/g01/Preponderant.f32"),
    );
}

const RECORD_PRESSURE4: ParityRecord = ParityRecord {
    name: "Pressure4",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pressure4() {
    check_reset_preserves_params("Pressure4");
    check_measured_max(
        &RECORD_PRESSURE4,
        include_str!("fixtures/batch3/g01/Pressure4.txt"),
        include_bytes!("fixtures/batch3/g01/Pressure4.f32"),
    );
    check_plugin(
        &RECORD_PRESSURE4,
        include_str!("fixtures/batch3/g01/Pressure4.txt"),
        include_bytes!("fixtures/batch3/g01/Pressure4.f32"),
    );
}

const RECORD_PRESSURE5: ParityRecord = ParityRecord {
    name: "Pressure5",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pressure5() {
    check_reset_preserves_params("Pressure5");
    check_measured_max(
        &RECORD_PRESSURE5,
        include_str!("fixtures/batch3/g01/Pressure5.txt"),
        include_bytes!("fixtures/batch3/g01/Pressure5.f32"),
    );
    check_plugin(
        &RECORD_PRESSURE5,
        include_str!("fixtures/batch3/g01/Pressure5.txt"),
        include_bytes!("fixtures/batch3/g01/Pressure5.f32"),
    );
}

const RECORD_PURESTAIR: ParityRecord = ParityRecord {
    name: "PurestAir",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn purestair() {
    check_reset_preserves_params("PurestAir");
    check_measured_max(
        &RECORD_PURESTAIR,
        include_str!("fixtures/batch3/g01/PurestAir.txt"),
        include_bytes!("fixtures/batch3/g01/PurestAir.f32"),
    );
    check_plugin(
        &RECORD_PURESTAIR,
        include_str!("fixtures/batch3/g01/PurestAir.txt"),
        include_bytes!("fixtures/batch3/g01/PurestAir.f32"),
    );
}

const RECORD_PURESTCONSOLE2BUSS: ParityRecord = ParityRecord {
    name: "PurestConsole2Buss",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn purestconsole2buss() {
    check_reset_preserves_params("PurestConsole2Buss");
    check_plugin(
        &RECORD_PURESTCONSOLE2BUSS,
        include_str!("fixtures/batch3/g01/PurestConsole2Buss.txt"),
        include_bytes!("fixtures/batch3/g01/PurestConsole2Buss.f32"),
    );
    check_measured_max(
        &RECORD_PURESTCONSOLE2BUSS,
        include_str!("fixtures/batch3/g01/PurestConsole2Buss.txt"),
        include_bytes!("fixtures/batch3/g01/PurestConsole2Buss.f32"),
    );
}
