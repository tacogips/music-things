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

fn measure(record: &ParityRecord, meta: &str, data: &[u8]) {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(record.name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25]
                .into_iter()
                .enumerate()
                .take(effect.param_count())
            {
                effect.set_param(i, value);
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
            maximum = maximum.max((l[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert_eq!(
        maximum, record.measured_max_abs,
        "{} measured max-abs",
        record.name
    );
}

const RECORD_TUBE: ParityRecord = ParityRecord {
    name: "Tube",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn tube() {
    check_reset_preserves_params("Tube");
    check_plugin(
        &RECORD_TUBE,
        include_str!("fixtures/batch3/g10/Tube.txt"),
        include_bytes!("fixtures/batch3/g10/Tube.f32"),
    );
    measure(
        &RECORD_TUBE,
        include_str!("fixtures/batch3/g10/Tube.txt"),
        include_bytes!("fixtures/batch3/g10/Tube.f32"),
    );
}

const RECORD_TUBE2: ParityRecord = ParityRecord {
    name: "Tube2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn tube2() {
    check_reset_preserves_params("Tube2");
    measure(
        &RECORD_TUBE2,
        include_str!("fixtures/batch3/g10/Tube2.txt"),
        include_bytes!("fixtures/batch3/g10/Tube2.f32"),
    );
    check_plugin(
        &RECORD_TUBE2,
        include_str!("fixtures/batch3/g10/Tube2.txt"),
        include_bytes!("fixtures/batch3/g10/Tube2.f32"),
    );
}

const RECORD_TUBEDESK: ParityRecord = ParityRecord {
    name: "TubeDesk",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn tubedesk() {
    check_reset_preserves_params("TubeDesk");
    measure(
        &RECORD_TUBEDESK,
        include_str!("fixtures/batch3/g10/TubeDesk.txt"),
        include_bytes!("fixtures/batch3/g10/TubeDesk.f32"),
    );
    check_plugin(
        &RECORD_TUBEDESK,
        include_str!("fixtures/batch3/g10/TubeDesk.txt"),
        include_bytes!("fixtures/batch3/g10/TubeDesk.f32"),
    );
}

const RECORD_ULAWDECODE: ParityRecord = ParityRecord {
    name: "uLawDecode",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn ulawdecode() {
    check_reset_preserves_params("uLawDecode");
    measure(
        &RECORD_ULAWDECODE,
        include_str!("fixtures/batch3/g10/uLawDecode.txt"),
        include_bytes!("fixtures/batch3/g10/uLawDecode.f32"),
    );
    check_plugin(
        &RECORD_ULAWDECODE,
        include_str!("fixtures/batch3/g10/uLawDecode.txt"),
        include_bytes!("fixtures/batch3/g10/uLawDecode.f32"),
    );
}

const RECORD_ULAWENCODE: ParityRecord = ParityRecord { name: "uLawEncode", class: ParityClass::Libm, measured_max_abs: 5.394141e-8, note: "Upstream static noise counters persist across fixture renders; Rust instances reset independently." };
#[test]
fn ulawencode() {
    check_reset_preserves_params("uLawEncode");
    measure(
        &RECORD_ULAWENCODE,
        include_str!("fixtures/batch3/g10/uLawEncode.txt"),
        include_bytes!("fixtures/batch3/g10/uLawEncode.f32"),
    );
    check_plugin(
        &RECORD_ULAWENCODE,
        include_str!("fixtures/batch3/g10/uLawEncode.txt"),
        include_bytes!("fixtures/batch3/g10/uLawEncode.f32"),
    );
}

const RECORD_ULTRASONIC: ParityRecord = ParityRecord {
    name: "Ultrasonic",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn ultrasonic() {
    check_reset_preserves_params("Ultrasonic");
    measure(
        &RECORD_ULTRASONIC,
        include_str!("fixtures/batch3/g10/Ultrasonic.txt"),
        include_bytes!("fixtures/batch3/g10/Ultrasonic.f32"),
    );
    check_plugin(
        &RECORD_ULTRASONIC,
        include_str!("fixtures/batch3/g10/Ultrasonic.txt"),
        include_bytes!("fixtures/batch3/g10/Ultrasonic.f32"),
    );
}

const RECORD_ULTRASONICLITE: ParityRecord = ParityRecord {
    name: "UltrasonicLite",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn ultrasoniclite() {
    check_reset_preserves_params("UltrasonicLite");
    measure(
        &RECORD_ULTRASONICLITE,
        include_str!("fixtures/batch3/g10/UltrasonicLite.txt"),
        include_bytes!("fixtures/batch3/g10/UltrasonicLite.f32"),
    );
    check_plugin(
        &RECORD_ULTRASONICLITE,
        include_str!("fixtures/batch3/g10/UltrasonicLite.txt"),
        include_bytes!("fixtures/batch3/g10/UltrasonicLite.f32"),
    );
}

const RECORD_ULTRASONICMED: ParityRecord = ParityRecord {
    name: "UltrasonicMed",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn ultrasonicmed() {
    check_reset_preserves_params("UltrasonicMed");
    measure(
        &RECORD_ULTRASONICMED,
        include_str!("fixtures/batch3/g10/UltrasonicMed.txt"),
        include_bytes!("fixtures/batch3/g10/UltrasonicMed.f32"),
    );
    check_plugin(
        &RECORD_ULTRASONICMED,
        include_str!("fixtures/batch3/g10/UltrasonicMed.txt"),
        include_bytes!("fixtures/batch3/g10/UltrasonicMed.f32"),
    );
}
