mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};
const RECORD_CONSOLE8CHANNELIN: ParityRecord = ParityRecord {
    name: "Console8ChannelIn",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console8channelin() {
    measure(
        "Console8ChannelIn",
        include_str!("fixtures/batch1/g11/Console8ChannelIn.txt"),
        include_bytes!("fixtures/batch1/g11/Console8ChannelIn.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE8CHANNELIN,
        include_str!("fixtures/batch1/g11/Console8ChannelIn.txt"),
        include_bytes!("fixtures/batch1/g11/Console8ChannelIn.f32"),
    );
}
const RECORD_CONSOLE8CHANNELOUT: ParityRecord = ParityRecord {
    name: "Console8ChannelOut",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console8channelout() {
    measure(
        "Console8ChannelOut",
        include_str!("fixtures/batch1/g11/Console8ChannelOut.txt"),
        include_bytes!("fixtures/batch1/g11/Console8ChannelOut.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE8CHANNELOUT,
        include_str!("fixtures/batch1/g11/Console8ChannelOut.txt"),
        include_bytes!("fixtures/batch1/g11/Console8ChannelOut.f32"),
    );
}
const RECORD_CONSOLE8SUBHYPE: ParityRecord = ParityRecord {
    name: "Console8SubHype",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console8subhype() {
    measure(
        "Console8SubHype",
        include_str!("fixtures/batch1/g11/Console8SubHype.txt"),
        include_bytes!("fixtures/batch1/g11/Console8SubHype.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE8SUBHYPE,
        include_str!("fixtures/batch1/g11/Console8SubHype.txt"),
        include_bytes!("fixtures/batch1/g11/Console8SubHype.f32"),
    );
}
const RECORD_CONSOLE8SUBIN: ParityRecord = ParityRecord {
    name: "Console8SubIn",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console8subin() {
    measure(
        "Console8SubIn",
        include_str!("fixtures/batch1/g11/Console8SubIn.txt"),
        include_bytes!("fixtures/batch1/g11/Console8SubIn.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE8SUBIN,
        include_str!("fixtures/batch1/g11/Console8SubIn.txt"),
        include_bytes!("fixtures/batch1/g11/Console8SubIn.f32"),
    );
}
const RECORD_CONSOLE8SUBOUT: ParityRecord = ParityRecord {
    name: "Console8SubOut",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn console8subout() {
    measure(
        "Console8SubOut",
        include_str!("fixtures/batch1/g11/Console8SubOut.txt"),
        include_bytes!("fixtures/batch1/g11/Console8SubOut.f32"),
    );
    check_plugin(
        &RECORD_CONSOLE8SUBOUT,
        include_str!("fixtures/batch1/g11/Console8SubOut.txt"),
        include_bytes!("fixtures/batch1/g11/Console8SubOut.f32"),
    );
}

const RECORD_CRYSTAL: ParityRecord = ParityRecord {
    name: "Crystal",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn crystal() {
    measure(
        "Crystal",
        include_str!("fixtures/batch1/g11/Crystal.txt"),
        include_bytes!("fixtures/batch1/g11/Crystal.f32"),
    );
    check_plugin(
        &RECORD_CRYSTAL,
        include_str!("fixtures/batch1/g11/Crystal.txt"),
        include_bytes!("fixtures/batch1/g11/Crystal.f32"),
    );
}

const RECORD_CRUNCHYGROOVEWEAR: ParityRecord = ParityRecord {
    name: "CrunchyGrooveWear",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn crunchygroovewear() {
    measure(
        "CrunchyGrooveWear",
        include_str!("fixtures/batch1/g11/CrunchyGrooveWear.txt"),
        include_bytes!("fixtures/batch1/g11/CrunchyGrooveWear.f32"),
    );
    check_plugin(
        &RECORD_CRUNCHYGROOVEWEAR,
        include_str!("fixtures/batch1/g11/CrunchyGrooveWear.txt"),
        include_bytes!("fixtures/batch1/g11/CrunchyGrooveWear.f32"),
    );
}

const RECORD_CSTRIP: ParityRecord = ParityRecord {
    name: "CStrip",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn cstrip() {
    measure(
        "CStrip",
        include_str!("fixtures/batch1/g11/CStrip.txt"),
        include_bytes!("fixtures/batch1/g11/CStrip.f32"),
    );
    check_plugin(
        &RECORD_CSTRIP,
        include_str!("fixtures/batch1/g11/CStrip.txt"),
        include_bytes!("fixtures/batch1/g11/CStrip.f32"),
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
