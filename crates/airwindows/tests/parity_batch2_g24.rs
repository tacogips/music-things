mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

const MONITORING3: ParityRecord = ParityRecord {
    name: "Monitoring3",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn monitoring3() {
    check_plugin(
        &MONITORING3,
        include_str!("fixtures/batch2/g24/Monitoring3.txt"),
        include_bytes!("fixtures/batch2/g24/Monitoring3.f32"),
    );
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g24/Monitoring3.txt"),
        include_bytes!("fixtures/batch2/g24/Monitoring3.f32"),
    );
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Monitoring3", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
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
            max = max.max((l[frame] - fixture.renders[render][2 * k]).abs());
            max = max.max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    assert_eq!(max, MONITORING3.measured_max_abs);
}
#[test]
fn monitoring3_modes() {
    for rate in [44100.0, 96000.0] {
        let (left, right) = common::stimulus::aw_stimulus(rate);
        for mode in 0..17 {
            let mut effect =
                music_things_airwindows::create_with_seed("Monitoring3", rate, 123).unwrap();
            effect.set_param(0, (mode as f32 + 0.5) / 16.999);
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
            assert!(l.iter().chain(&r).all(|x| x.is_finite()));
        }
    }
}
