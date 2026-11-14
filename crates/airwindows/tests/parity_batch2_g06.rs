mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};
const RECORD: ParityRecord = ParityRecord {
    name: "FireAmp",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn fireamp() {
    check_plugin(
        &RECORD,
        include_str!("fixtures/batch2/g06/FireAmp.txt"),
        include_bytes!("fixtures/batch2/g06/FireAmp.f32"),
    );
}

#[test]
fn measured_error() {
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g06/FireAmp.txt"),
        include_bytes!("fixtures/batch2/g06/FireAmp.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("FireAmp", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, v) in [0.30, 0.85, 0.15, 0.65].into_iter().enumerate() {
                effect.set_param(i, v);
            }
        }
        let mut ol = vec![0.0; left.len()];
        let mut orr = vec![0.0; right.len()];
        for block in (0..left.len()).step_by(128) {
            effect.process(
                &left[block..block + 128],
                &right[block..block + 128],
                &mut ol[block..block + 128],
                &mut orr[block..block + 128],
            );
        }
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            maximum = maximum.max((ol[frame] - fixture.renders[render][2 * k]).abs());
            maximum = maximum.max((orr[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    println!("measured maximum: {maximum:?}");
    assert!(maximum <= 1.0e-6, "maximum error {maximum}");
}

#[test]
fn reset_preserves_parameters() {
    let mut effect =
        music_things_airwindows::create_with_seed("FireAmp", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..effect.info().params.len() {
        effect.set_param(i, (i + 1) as f32 / 6.0);
    }
    effect.reset();
    for i in 0..effect.info().params.len() {
        assert_eq!(effect.get_param(i), (i + 1) as f32 / 6.0);
    }
}
