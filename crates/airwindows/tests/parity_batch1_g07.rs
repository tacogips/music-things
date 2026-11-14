mod common;
use common::parity::{ParityClass, ParityRecord};
fn measure(name: &str, meta: &str, data: &[u8]) {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (l, r) = common::stimulus::aw_stimulus(rate);
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
        let mut ol = vec![0.0; l.len()];
        let mut or = vec![0.0; r.len()];
        for n in (0..l.len()).step_by(128) {
            effect.process(
                &l[n..n + 128],
                &r[n..n + 128],
                &mut ol[n..n + 128],
                &mut or[n..n + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            for (c, actual) in [ol[frame], or[frame]].into_iter().enumerate() {
                max = max.max((actual - fixture.renders[render][2 * k + c]).abs());
            }
        }
    }
    eprintln!("{name} max-abs={max:?}");
}
const RECORD_CHANNEL4: ParityRecord = ParityRecord {
    name: "Channel4",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn channel4() {
    let meta = include_str!("fixtures/batch1/g07/Channel4.txt");
    let data = include_bytes!("fixtures/batch1/g07/Channel4.f32");
    common::parity::check_plugin(&RECORD_CHANNEL4, meta, data);
    measure("Channel4", meta, data);
}
const RECORD_CHANNEL5: ParityRecord = ParityRecord {
    name: "Channel5",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn channel5() {
    let meta = include_str!("fixtures/batch1/g07/Channel5.txt");
    let data = include_bytes!("fixtures/batch1/g07/Channel5.f32");
    common::parity::check_plugin(&RECORD_CHANNEL5, meta, data);
    measure("Channel5", meta, data);
}
const RECORD_CHANNEL6: ParityRecord = ParityRecord {
    name: "Channel6",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn channel6() {
    let meta = include_str!("fixtures/batch1/g07/Channel6.txt");
    let data = include_bytes!("fixtures/batch1/g07/Channel6.f32");
    common::parity::check_plugin(&RECORD_CHANNEL6, meta, data);
    measure("Channel6", meta, data);
}
const RECORD_CHANNEL7: ParityRecord = ParityRecord {
    name: "Channel7",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn channel7() {
    let meta = include_str!("fixtures/batch1/g07/Channel7.txt");
    let data = include_bytes!("fixtures/batch1/g07/Channel7.f32");
    common::parity::check_plugin(&RECORD_CHANNEL7, meta, data);
    measure("Channel7", meta, data);
}
const RECORD_CHANNEL8: ParityRecord = ParityRecord {
    name: "Channel8",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn channel8() {
    let meta = include_str!("fixtures/batch1/g07/Channel8.txt");
    let data = include_bytes!("fixtures/batch1/g07/Channel8.f32");
    common::parity::check_plugin(&RECORD_CHANNEL8, meta, data);
    measure("Channel8", meta, data);
}
const RECORD_CHANNEL9: ParityRecord = ParityRecord {
    name: "Channel9",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn channel9() {
    let meta = include_str!("fixtures/batch1/g07/Channel9.txt");
    let data = include_bytes!("fixtures/batch1/g07/Channel9.f32");
    common::parity::check_plugin(&RECORD_CHANNEL9, meta, data);
    measure("Channel9", meta, data);
}
const RECORD_CHORUS: ParityRecord = ParityRecord {
    name: "Chorus",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn chorus() {
    let meta = include_str!("fixtures/batch1/g07/Chorus.txt");
    let data = include_bytes!("fixtures/batch1/g07/Chorus.f32");
    common::parity::check_plugin(&RECORD_CHORUS, meta, data);
    measure("Chorus", meta, data);
}
const RECORD_CHORUSENSEMBLE: ParityRecord = ParityRecord {
    name: "ChorusEnsemble",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn chorusensemble() {
    let meta = include_str!("fixtures/batch1/g07/ChorusEnsemble.txt");
    let data = include_bytes!("fixtures/batch1/g07/ChorusEnsemble.f32");
    common::parity::check_plugin(&RECORD_CHORUSENSEMBLE, meta, data);
    measure("ChorusEnsemble", meta, data);
}
