mod common;
use music_things_airwindows::{catalog, create, create_with_seed};
#[test]
fn catalog_contract() {
    let mut names = std::collections::HashSet::new();
    for info in catalog() {
        assert!(names.insert(info.name));
        if info.name != info.upstream_name {
            assert!(names.insert(info.upstream_name));
        }
        assert!(!info.description.is_empty());
        let effect = create(info.name, 44100.0).unwrap();
        assert_eq!(effect.param_count(), info.params.len());
        assert!(info.params.iter().all(|p| (0.0..=1.0).contains(&p.default)));
        assert!(create_with_seed(info.upstream_name, 44100.0, 1).is_some());
    }
    assert!(create("NoSuchPlugin", 44100.0).is_none());
}
#[test]
#[should_panic(expected = "custom tolerance")]
fn custom_tolerance_requires_reason() {
    common::parity::check_plugin(
        &common::parity::ParityRecord {
            name: "x",
            class: common::parity::ParityClass::Custom(1e-7),
            measured_max_abs: 0.0,
            note: "",
        },
        "",
        &[],
    );
}

#[test]
fn fixture_metadata_matches_generated_reference() {
    let f = common::fixture::Fixture::parse(
        include_str!("fixtures/batch1/g01/Acceleration.txt"),
        include_bytes!("fixtures/batch1/g01/Acceleration.f32"),
    );
    assert_eq!(f.name, "Acceleration");
    assert_eq!(f.upstream, "Acceleration");
    assert_eq!(f.params.len(), 2);
    assert_eq!(f.params[0].name, "Limit");
    assert_eq!(
        f.renders.iter().map(Vec::len).collect::<Vec<_>>(),
        [1008; 3]
    );
    assert_eq!(f.stimulus_fnv_44100, 0xd83a50634fc97283);
    assert_eq!(f.stimulus_fnv_96000, 0x9079f3b5302ec7f0);
}
