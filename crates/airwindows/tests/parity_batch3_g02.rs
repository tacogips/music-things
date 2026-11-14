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
const RECORD_PURESTCONSOLE2CHANNEL: ParityRecord = ParityRecord {
    name: "PurestConsole2Channel",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn purestconsole2channel() {
    let meta = include_str!("fixtures/batch3/g02/PurestConsole2Channel.txt");
    let data = include_bytes!("fixtures/batch3/g02/PurestConsole2Channel.f32");
    check_reset_preserves_params("PurestConsole2Channel");
    check_plugin(&RECORD_PURESTCONSOLE2CHANNEL, meta, data);
}

const RECORD_PURESTCONSOLEBUSS: ParityRecord = ParityRecord {
    name: "PurestConsoleBuss",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn purestconsolebuss() {
    let meta = include_str!("fixtures/batch3/g02/PurestConsoleBuss.txt");
    let data = include_bytes!("fixtures/batch3/g02/PurestConsoleBuss.f32");
    check_reset_preserves_params("PurestConsoleBuss");
    check_plugin(&RECORD_PURESTCONSOLEBUSS, meta, data);
}

const RECORD_PURESTCONSOLECHANNEL: ParityRecord = ParityRecord {
    name: "PurestConsoleChannel",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn purestconsolechannel() {
    let meta = include_str!("fixtures/batch3/g02/PurestConsoleChannel.txt");
    let data = include_bytes!("fixtures/batch3/g02/PurestConsoleChannel.f32");
    check_reset_preserves_params("PurestConsoleChannel");
    check_plugin(&RECORD_PURESTCONSOLECHANNEL, meta, data);
}

const RECORD_PURESTDRIVE: ParityRecord = ParityRecord {
    name: "PurestDrive",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn purestdrive() {
    let meta = include_str!("fixtures/batch3/g02/PurestDrive.txt");
    let data = include_bytes!("fixtures/batch3/g02/PurestDrive.f32");
    check_reset_preserves_params("PurestDrive");
    check_plugin(&RECORD_PURESTDRIVE, meta, data);
}

const RECORD_PURESTECHO: ParityRecord = ParityRecord {
    name: "PurestEcho",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn purestecho() {
    let meta = include_str!("fixtures/batch3/g02/PurestEcho.txt");
    let data = include_bytes!("fixtures/batch3/g02/PurestEcho.f32");
    check_reset_preserves_params("PurestEcho");
    check_plugin(&RECORD_PURESTECHO, meta, data);
}

const RECORD_PURESTFADE: ParityRecord = ParityRecord {
    name: "PurestFade",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn purestfade() {
    let meta = include_str!("fixtures/batch3/g02/PurestFade.txt");
    let data = include_bytes!("fixtures/batch3/g02/PurestFade.f32");
    check_reset_preserves_params("PurestFade");
    check_plugin(&RECORD_PURESTFADE, meta, data);
}

const RECORD_PURESTGAIN: ParityRecord = ParityRecord {
    name: "PurestGain",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn purestgain() {
    let meta = include_str!("fixtures/batch3/g02/PurestGain.txt");
    let data = include_bytes!("fixtures/batch3/g02/PurestGain.f32");
    check_reset_preserves_params("PurestGain");
    check_plugin(&RECORD_PURESTGAIN, meta, data);
}

const RECORD_PURESTSQUISH: ParityRecord = ParityRecord {
    name: "PurestSquish",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn purestsquish() {
    let meta = include_str!("fixtures/batch3/g02/PurestSquish.txt");
    let data = include_bytes!("fixtures/batch3/g02/PurestSquish.f32");
    check_reset_preserves_params("PurestSquish");
    check_plugin(&RECORD_PURESTSQUISH, meta, data);
}
