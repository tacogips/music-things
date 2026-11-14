mod common;

use common::parity::{check_plugin, ParityClass, ParityRecord};

const RECORD_BRASSRIDER: ParityRecord = ParityRecord {
    name: "BrassRider",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_BRIGHTAMBIENCE: ParityRecord = ParityRecord {
    name: "BrightAmbience",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_BRIGHTAMBIENCE2: ParityRecord = ParityRecord {
    name: "BrightAmbience2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_BRIGHTAMBIENCE3: ParityRecord = ParityRecord {
    name: "BrightAmbience3",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_BUILDATPDF: ParityRecord = ParityRecord {
    name: "BuildATPDF",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_BUSSCOLORS4: ParityRecord = ParityRecord {
    name: "BussColors4",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_BUTTERCOMP: ParityRecord = ParityRecord {
    name: "ButterComp",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
const RECORD_BUTTERCOMP2: ParityRecord = ParityRecord {
    name: "ButterComp2",
    class: ParityClass::Libm,
    measured_max_abs: 5.960_464_5e-8,
    note:
        "Upstream static noise counters persist across renders; reset restores per-instance state.",
};

fn assert_reset_preserves_params(name: &str) {
    let mut effect = music_things_airwindows::create(name, 44_100.0).unwrap();
    let expected: Vec<_> = (0..effect.param_count())
        .map(|index| {
            let value = 0.17 + index as f32 * 0.11;
            effect.set_param(index, value);
            effect.get_param(index)
        })
        .collect();
    effect.reset();
    for (index, value) in expected.into_iter().enumerate() {
        assert_eq!(effect.get_param(index), value, "{name} parameter {index}");
    }
}

#[test]
fn brassrider() {
    assert_reset_preserves_params(RECORD_BRASSRIDER.name);
    check_plugin(
        &RECORD_BRASSRIDER,
        include_str!("fixtures/batch1/g05/BrassRider.txt"),
        include_bytes!("fixtures/batch1/g05/BrassRider.f32"),
    );
}
#[test]
fn brightambience() {
    assert_reset_preserves_params(RECORD_BRIGHTAMBIENCE.name);
    check_plugin(
        &RECORD_BRIGHTAMBIENCE,
        include_str!("fixtures/batch1/g05/BrightAmbience.txt"),
        include_bytes!("fixtures/batch1/g05/BrightAmbience.f32"),
    );
}
#[test]
fn brightambience2() {
    assert_reset_preserves_params(RECORD_BRIGHTAMBIENCE2.name);
    check_plugin(
        &RECORD_BRIGHTAMBIENCE2,
        include_str!("fixtures/batch1/g05/BrightAmbience2.txt"),
        include_bytes!("fixtures/batch1/g05/BrightAmbience2.f32"),
    );
}
#[test]
fn brightambience3() {
    assert_reset_preserves_params(RECORD_BRIGHTAMBIENCE3.name);
    check_plugin(
        &RECORD_BRIGHTAMBIENCE3,
        include_str!("fixtures/batch1/g05/BrightAmbience3.txt"),
        include_bytes!("fixtures/batch1/g05/BrightAmbience3.f32"),
    );
}
#[test]
fn buildatpdf() {
    assert_reset_preserves_params(RECORD_BUILDATPDF.name);
    check_plugin(
        &RECORD_BUILDATPDF,
        include_str!("fixtures/batch1/g05/BuildATPDF.txt"),
        include_bytes!("fixtures/batch1/g05/BuildATPDF.f32"),
    );
}
#[test]
fn busscolors4() {
    assert_reset_preserves_params(RECORD_BUSSCOLORS4.name);
    check_plugin(
        &RECORD_BUSSCOLORS4,
        include_str!("fixtures/batch1/g05/BussColors4.txt"),
        include_bytes!("fixtures/batch1/g05/BussColors4.f32"),
    );
}
#[test]
fn buttercomp() {
    assert_reset_preserves_params(RECORD_BUTTERCOMP.name);
    check_plugin(
        &RECORD_BUTTERCOMP,
        include_str!("fixtures/batch1/g05/ButterComp.txt"),
        include_bytes!("fixtures/batch1/g05/ButterComp.f32"),
    );
}
#[test]
fn buttercomp2() {
    assert_reset_preserves_params(RECORD_BUTTERCOMP2.name);
    check_plugin(
        &RECORD_BUTTERCOMP2,
        include_str!("fixtures/batch1/g05/ButterComp2.txt"),
        include_bytes!("fixtures/batch1/g05/ButterComp2.f32"),
    );
}
