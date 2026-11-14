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

const RECORD_ENERGY: ParityRecord = ParityRecord {
    name: "Energy",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn energy() {
    check_reset_preserves_params("Energy");
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g03/Energy.txt"),
        include_bytes!("fixtures/batch2/g03/Energy.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Energy", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            let alt = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25];
            for i in 0..effect.info().params.len() {
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
    assert!(maximum <= 1e-6, "Energy max-abs {maximum}");
    check_plugin(
        &RECORD_ENERGY,
        include_str!("fixtures/batch2/g03/Energy.txt"),
        include_bytes!("fixtures/batch2/g03/Energy.f32"),
    );
}
