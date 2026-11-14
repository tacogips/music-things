mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

const RECORD: ParityRecord = ParityRecord {
    name: "Nikola",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn nikola() {
    let text = include_str!("fixtures/batch2/g28/Nikola.txt");
    let bytes = include_bytes!("fixtures/batch2/g28/Nikola.f32");
    let fixture = common::fixture::Fixture::parse(text, bytes);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Nikola", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
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
    eprintln!("Nikola measured max-abs: {maximum:?}");
    assert!(maximum <= 1e-6);
    check_plugin(&RECORD, text, bytes);
}
