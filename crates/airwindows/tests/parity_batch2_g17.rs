mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};
const LOGICAL4: ParityRecord = ParityRecord {
    name: "Logical4",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn logical4() {
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g17/Logical4.txt"),
        include_bytes!("fixtures/batch2/g17/Logical4.f32"),
    );
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Logical4", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65, 0.95].into_iter().enumerate() {
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
    println!("Logical4 measured maximum absolute error: {maximum:?}");
    assert!(maximum <= 1e-6);
    check_plugin(
        &LOGICAL4,
        include_str!("fixtures/batch2/g17/Logical4.txt"),
        include_bytes!("fixtures/batch2/g17/Logical4.f32"),
    );
}

#[test]
fn ratio_selector_sweep() {
    // floor(sqrt(15 * B * B + 1) - 1) changes at sqrt(3/15) and sqrt(8/15).
    let first_boundary = (3.0_f32 / 15.0).sqrt();
    let second_boundary = (8.0_f32 / 15.0).sqrt();
    let midpoints = [
        first_boundary / 2.0,
        (first_boundary + second_boundary) / 2.0,
        (second_boundary + 1.0) / 2.0,
    ];
    for rate in [44100.0, 96000.0] {
        let (left, right) = common::stimulus::aw_stimulus(rate);
        for ratio in midpoints {
            let mut effect =
                music_things_airwindows::create_with_seed("Logical4", rate, 0x00c0_ffee).unwrap();
            effect.set_param(1, ratio);
            let mut l = vec![0.0; left.len()];
            let mut r = vec![0.0; right.len()];
            effect.process(&left, &right, &mut l, &mut r);
            assert!(
                l.iter().chain(&r).all(|sample| sample.is_finite()),
                "rate {rate}, ratio {ratio}"
            );
        }
    }
}
