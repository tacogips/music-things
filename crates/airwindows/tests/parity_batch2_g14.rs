mod common;

use common::parity::{check_plugin, ParityClass, ParityRecord};

const RECORD_LEADAMP: ParityRecord = ParityRecord {
    name: "LeadAmp",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn leadamp() {
    let meta = include_str!("fixtures/batch2/g14/LeadAmp.txt");
    let data = include_bytes!("fixtures/batch2/g14/LeadAmp.f32");
    let mut effect = music_things_airwindows::create_with_seed("LeadAmp", 44100.0, 123).unwrap();
    for i in 0..effect.param_count() {
        effect.set_param(i, 0.37);
    }
    effect.reset();
    for i in 0..effect.param_count() {
        assert_eq!(effect.get_param(i), 0.37);
    }
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("LeadAmp", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65].into_iter().enumerate() {
                effect.set_param(i, value);
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
    eprintln!("LeadAmp measured max-abs {maximum:?}");
    check_plugin(&RECORD_LEADAMP, meta, data);
}
