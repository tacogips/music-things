mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};
const NEVERLAND: ParityRecord = ParityRecord {
    name: "Neverland",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn neverland() {
    let meta = include_str!("fixtures/batch2/g27/Neverland.txt");
    let data = include_bytes!("fixtures/batch2/g27/Neverland.f32");
    check_plugin(&NEVERLAND, meta, data);
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Neverland", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
            effect.set_param(2, 0.15);
            effect.set_param(3, 0.65);
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
    assert_eq!(max, NEVERLAND.measured_max_abs);
}

const NATURALIZE: ParityRecord = ParityRecord {
    name: "NaturalizeDither",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn naturalizedither() {
    let meta = include_str!("fixtures/batch2/g27/NaturalizeDither.txt");
    let data = include_bytes!("fixtures/batch2/g27/NaturalizeDither.f32");
    check_plugin(&NATURALIZE, meta, data);
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("NaturalizeDither", rate, 0x00c0_ffee)
                .unwrap();
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
    assert_eq!(max, NATURALIZE.measured_max_abs);
}

const NCSEVENTEEN: ParityRecord = ParityRecord {
    name: "NCSeventeen",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn ncseventeen() {
    let meta = include_str!("fixtures/batch2/g27/NCSeventeen.txt");
    let data = include_bytes!("fixtures/batch2/g27/NCSeventeen.f32");
    check_plugin(&NCSEVENTEEN, meta, data);
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("NCSeventeen", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
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
    assert_eq!(max, NCSEVENTEEN.measured_max_abs);
}
