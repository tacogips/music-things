mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};

fn check_modes(name: &str) {
    // The upstream selector casts parameter * 1.999 to an integer.
    let boundary = 1.0_f32 / 1.999;
    for rate in [44100.0, 96000.0] {
        for mode in [boundary / 2.0, (boundary + 1.0) / 2.0] {
            let mut effect =
                music_things_airwindows::create_with_seed(name, rate, 0x00c0_ffee).unwrap();
            effect.set_param(0, mode);
            let (left, right) = common::stimulus::aw_stimulus(rate);
            let mut output_left = vec![0.0; left.len()];
            let mut output_right = vec![0.0; right.len()];
            for block in (0..left.len()).step_by(128) {
                effect.process(
                    &left[block..block + 128],
                    &right[block..block + 128],
                    &mut output_left[block..block + 128],
                    &mut output_right[block..block + 128],
                );
            }
            assert!(output_left
                .into_iter()
                .chain(output_right)
                .all(f32::is_finite));
        }
    }
}
const RECORD_PAULDITHER: ParityRecord = ParityRecord {
    name: "PaulDither",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn pauldither() {
    let record = &RECORD_PAULDITHER;
    check_plugin(
        record,
        include_str!("fixtures/batch2/g31/PaulDither.txt"),
        include_bytes!("fixtures/batch2/g31/PaulDither.f32"),
    );
}
#[test]
fn pauldither_measure() {
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g31/PaulDither.txt"),
        include_bytes!("fixtures/batch2/g31/PaulDither.f32"),
    );
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("PaulDither", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
        }
        let mut l = vec![0.0; left.len()];
        let mut r = l.clone();
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
            max = max
                .max((l[frame] - fixture.renders[render][2 * k]).abs())
                .max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    println!("PaulDither maximum: {max:?}");
    assert!(max <= 1e-6);
}
#[test]
fn pauldither_modes() {
    check_modes("PaulDither");
}

const RECORD_PAULWIDE: ParityRecord = ParityRecord {
    name: "PaulWide",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn paulwide() {
    let record = &RECORD_PAULWIDE;
    check_plugin(
        record,
        include_str!("fixtures/batch2/g31/PaulWide.txt"),
        include_bytes!("fixtures/batch2/g31/PaulWide.f32"),
    );
}
#[test]
fn paulwide_measure() {
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g31/PaulWide.txt"),
        include_bytes!("fixtures/batch2/g31/PaulWide.f32"),
    );
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("PaulWide", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
        }
        let mut l = vec![0.0; left.len()];
        let mut r = l.clone();
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
            max = max
                .max((l[frame] - fixture.renders[render][2 * k]).abs())
                .max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    println!("PaulWide maximum: {max:?}");
    assert!(max <= 1e-6);
}
#[test]
fn paulwide_modes() {
    check_modes("PaulWide");
}

const RECORD_ONECORNERCLIP: ParityRecord = ParityRecord {
    name: "OneCornerClip",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn onecornerclip() {
    let record = &RECORD_ONECORNERCLIP;
    check_plugin(
        record,
        include_str!("fixtures/batch2/g31/OneCornerClip.txt"),
        include_bytes!("fixtures/batch2/g31/OneCornerClip.f32"),
    );
}
#[test]
fn onecornerclip_measure() {
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g31/OneCornerClip.txt"),
        include_bytes!("fixtures/batch2/g31/OneCornerClip.f32"),
    );
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("OneCornerClip", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
            effect.set_param(2, 0.15);
            effect.set_param(3, 0.65);
            effect.set_param(4, 0.95);
        }
        let mut l = vec![0.0; left.len()];
        let mut r = l.clone();
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
            max = max
                .max((l[frame] - fixture.renders[render][2 * k]).abs())
                .max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    println!("OneCornerClip maximum: {max:?}");
    assert!(max <= 1e-6);
}

const RECORD_PAFNUTY: ParityRecord = ParityRecord {
    name: "Pafnuty",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn pafnuty() {
    let record = &RECORD_PAFNUTY;
    check_plugin(
        record,
        include_str!("fixtures/batch2/g31/Pafnuty.txt"),
        include_bytes!("fixtures/batch2/g31/Pafnuty.f32"),
    );
}
#[test]
fn pafnuty_measure() {
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g31/Pafnuty.txt"),
        include_bytes!("fixtures/batch2/g31/Pafnuty.f32"),
    );
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("Pafnuty", rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for i in 0..13 {
                effect.set_param(
                    i,
                    [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25][i % 9],
                );
            }
        }
        let mut l = vec![0.0; left.len()];
        let mut r = l.clone();
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
            max = max
                .max((l[frame] - fixture.renders[render][2 * k]).abs())
                .max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    println!("Pafnuty maximum: {max:?}");
    assert!(max <= 1e-6);
}

const RECORD_NOTJUSTANOTHERDITHER: ParityRecord = ParityRecord {
    name: "NotJustAnotherDither",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn notjustanotherdither() {
    let record = &RECORD_NOTJUSTANOTHERDITHER;
    check_plugin(
        record,
        include_str!("fixtures/batch2/g31/NotJustAnotherDither.txt"),
        include_bytes!("fixtures/batch2/g31/NotJustAnotherDither.f32"),
    );
}
#[test]
fn notjustanotherdither_measure() {
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g31/NotJustAnotherDither.txt"),
        include_bytes!("fixtures/batch2/g31/NotJustAnotherDither.f32"),
    );
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("NotJustAnotherDither", rate, 0x00c0_ffee)
                .unwrap();
        if render != 0 {
            effect.set_param(0, 0.30);
            effect.set_param(1, 0.85);
        }
        let mut l = vec![0.0; left.len()];
        let mut r = l.clone();
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
            max = max
                .max((l[frame] - fixture.renders[render][2 * k]).abs())
                .max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    println!("NotJustAnotherDither maximum: {max:?}");
    assert!(max <= 1e-6);
}
#[test]
fn notjustanotherdither_modes() {
    check_modes("NotJustAnotherDither");
}

const RECORD_NOTJUSTANOTHERCD: ParityRecord = ParityRecord {
    name: "NotJustAnotherCD",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn notjustanothercd() {
    let record = &RECORD_NOTJUSTANOTHERCD;
    check_plugin(
        record,
        include_str!("fixtures/batch2/g31/NotJustAnotherCD.txt"),
        include_bytes!("fixtures/batch2/g31/NotJustAnotherCD.f32"),
    );
}
#[test]
fn notjustanothercd_measure() {
    let fixture = common::fixture::Fixture::parse(
        include_str!("fixtures/batch2/g31/NotJustAnotherCD.txt"),
        include_bytes!("fixtures/batch2/g31/NotJustAnotherCD.f32"),
    );
    let mut max = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed("NotJustAnotherCD", rate, 0x00c0_ffee)
                .unwrap();
        let mut l = vec![0.0; left.len()];
        let mut r = l.clone();
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
            max = max
                .max((l[frame] - fixture.renders[render][2 * k]).abs())
                .max((r[frame] - fixture.renders[render][2 * k + 1]).abs());
        }
    }
    println!("NotJustAnotherCD maximum: {max:?}");
    assert!(max <= 1e-6);
}
