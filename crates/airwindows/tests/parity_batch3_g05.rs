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

fn measured(name: &str, meta: &str, data: &[u8]) -> f32 {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25]
                .into_iter()
                .enumerate()
                .take(fixture.params.len())
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
    maximum
}
const RECORD_SLEW: ParityRecord = ParityRecord {
    name: "Slew",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn slew() {
    check_reset_preserves_params("Slew");
    check_plugin(
        &RECORD_SLEW,
        include_str!("fixtures/batch3/g05/Slew.txt"),
        include_bytes!("fixtures/batch3/g05/Slew.f32"),
    );
    assert_eq!(
        measured(
            "Slew",
            include_str!("fixtures/batch3/g05/Slew.txt"),
            include_bytes!("fixtures/batch3/g05/Slew.f32")
        ),
        RECORD_SLEW.measured_max_abs
    );
}

const RECORD_SLEW2: ParityRecord = ParityRecord {
    name: "Slew2",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn slew2() {
    check_reset_preserves_params("Slew2");
    assert_eq!(
        measured(
            "Slew2",
            include_str!("fixtures/batch3/g05/Slew2.txt"),
            include_bytes!("fixtures/batch3/g05/Slew2.f32")
        ),
        RECORD_SLEW2.measured_max_abs
    );
    check_plugin(
        &RECORD_SLEW2,
        include_str!("fixtures/batch3/g05/Slew2.txt"),
        include_bytes!("fixtures/batch3/g05/Slew2.f32"),
    );
}

const RECORD_SLEW3: ParityRecord = ParityRecord {
    name: "Slew3",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn slew3() {
    check_reset_preserves_params("Slew3");
    assert_eq!(
        measured(
            "Slew3",
            include_str!("fixtures/batch3/g05/Slew3.txt"),
            include_bytes!("fixtures/batch3/g05/Slew3.f32")
        ),
        RECORD_SLEW3.measured_max_abs
    );
    check_plugin(
        &RECORD_SLEW3,
        include_str!("fixtures/batch3/g05/Slew3.txt"),
        include_bytes!("fixtures/batch3/g05/Slew3.f32"),
    );
}

const RECORD_SLEWONLY: ParityRecord = ParityRecord {
    name: "SlewOnly",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn slewonly() {
    check_reset_preserves_params("SlewOnly");
    assert_eq!(
        measured(
            "SlewOnly",
            include_str!("fixtures/batch3/g05/SlewOnly.txt"),
            include_bytes!("fixtures/batch3/g05/SlewOnly.f32")
        ),
        RECORD_SLEWONLY.measured_max_abs
    );
    check_plugin(
        &RECORD_SLEWONLY,
        include_str!("fixtures/batch3/g05/SlewOnly.txt"),
        include_bytes!("fixtures/batch3/g05/SlewOnly.f32"),
    );
}

const RECORD_SMOOTH: ParityRecord = ParityRecord {
    name: "Smooth",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn smooth() {
    check_reset_preserves_params("Smooth");
    assert_eq!(
        measured(
            "Smooth",
            include_str!("fixtures/batch3/g05/Smooth.txt"),
            include_bytes!("fixtures/batch3/g05/Smooth.f32")
        ),
        RECORD_SMOOTH.measured_max_abs
    );
    check_plugin(
        &RECORD_SMOOTH,
        include_str!("fixtures/batch3/g05/Smooth.txt"),
        include_bytes!("fixtures/batch3/g05/Smooth.f32"),
    );
}

const RECORD_SOFTGATE: ParityRecord = ParityRecord {
    name: "SoftGate",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn softgate() {
    check_reset_preserves_params("SoftGate");
    assert_eq!(
        measured(
            "SoftGate",
            include_str!("fixtures/batch3/g05/SoftGate.txt"),
            include_bytes!("fixtures/batch3/g05/SoftGate.f32")
        ),
        RECORD_SOFTGATE.measured_max_abs
    );
    check_plugin(
        &RECORD_SOFTGATE,
        include_str!("fixtures/batch3/g05/SoftGate.txt"),
        include_bytes!("fixtures/batch3/g05/SoftGate.f32"),
    );
}

const RECORD_SPATIALIZEDITHER: ParityRecord = ParityRecord {
    name: "SpatializeDither",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn spatializedither() {
    check_reset_preserves_params("SpatializeDither");
    assert_eq!(
        measured(
            "SpatializeDither",
            include_str!("fixtures/batch3/g05/SpatializeDither.txt"),
            include_bytes!("fixtures/batch3/g05/SpatializeDither.f32")
        ),
        RECORD_SPATIALIZEDITHER.measured_max_abs
    );
    check_plugin(
        &RECORD_SPATIALIZEDITHER,
        include_str!("fixtures/batch3/g05/SpatializeDither.txt"),
        include_bytes!("fixtures/batch3/g05/SpatializeDither.f32"),
    );
}

const RECORD_SPIRAL: ParityRecord = ParityRecord {
    name: "Spiral",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn spiral() {
    check_reset_preserves_params("Spiral");
    assert_eq!(
        measured(
            "Spiral",
            include_str!("fixtures/batch3/g05/Spiral.txt"),
            include_bytes!("fixtures/batch3/g05/Spiral.f32")
        ),
        RECORD_SPIRAL.measured_max_abs
    );
    check_plugin(
        &RECORD_SPIRAL,
        include_str!("fixtures/batch3/g05/Spiral.txt"),
        include_bytes!("fixtures/batch3/g05/Spiral.f32"),
    );
}
