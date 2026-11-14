mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

const RECORD: ParityRecord = ParityRecord {
    name: "PocketVerbs",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn pocketverbs_parity() {
    let meta = include_str!("fixtures/batch2/g33/PocketVerbs.txt");
    let data = include_bytes!("fixtures/batch2/g33/PocketVerbs.f32");
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("PocketVerbs", rate, 0x00c0_ffee).unwrap();
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
    println!("PocketVerbs max-abs {maximum}");
    assert!(maximum <= 1e-6, "PocketVerbs max-abs {maximum}");
    check_plugin(&RECORD, meta, data);
}

#[test]
fn pocketverbs_mode_sweep() {
    for rate in [44100.0, 96000.0] {
        for mode in 0..6 {
            let mut effect =
                music_things_airwindows::create_with_seed("PocketVerbs", rate, 0x00c0_ffee)
                    .unwrap();
            effect.set_param(0, (mode as f32 + 0.5) / 5.999);
            effect.set_param(1, 1.0);
            let (l, r) = common::stimulus::aw_stimulus(rate);
            let mut ol = vec![0.0; l.len()];
            let mut or = vec![0.0; r.len()];
            for block in (0..l.len()).step_by(128) {
                effect.process(
                    &l[block..block + 128],
                    &r[block..block + 128],
                    &mut ol[block..block + 128],
                    &mut or[block..block + 128],
                );
            }
            assert!(
                ol.iter().chain(&or).all(|x| x.is_finite()),
                "mode {mode}, rate {rate}"
            );
        }
    }
}
