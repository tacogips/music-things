mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

const MONITORING2: ParityRecord = ParityRecord {
    name: "Monitoring2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn monitoring2() {
    check_plugin(
        &MONITORING2,
        include_str!("fixtures/batch2/g23/Monitoring2.txt"),
        include_bytes!("fixtures/batch2/g23/Monitoring2.f32"),
    );
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g23/Monitoring2.txt"),
        include_bytes!("fixtures/batch2/g23/Monitoring2.f32"),
    );
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Monitoring2", rate, 0x00c0_ffee).unwrap();
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
    assert_eq!(max, MONITORING2.measured_max_abs);
}
#[test]
fn monitoring2_modes() {
    let mut effect =
        music_things_airwindows::create_with_seed("Monitoring2", 44100.0, 123).unwrap();
    for mode in 0..17 {
        effect.set_param(0, (mode as f32 + 0.5) / 17.0);
        let mut l = [0.0; 2];
        let mut r = [0.0; 2];
        effect.process(&[0.2, -0.3], &[0.4, -0.1], &mut l, &mut r);
        assert!(l.iter().chain(&r).all(|x| x.is_finite()));
    }
}
