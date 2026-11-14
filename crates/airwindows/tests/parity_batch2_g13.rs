mod common;

use common::parity::{check_plugin, ParityClass, ParityRecord};

fn check_reset_preserves_params(name: &str) {
    let mut effect = music_things_airwindows::create_with_seed(name, 44100.0, 123).unwrap();
    for i in 0..effect.param_count() {
        effect.set_param(i, 0.37);
    }
    effect.reset();
    for i in 0..effect.param_count() {
        assert_eq!(effect.get_param(i), 0.37);
    }
}

fn measure(name: &str, meta: &str, data: &[u8]) -> f32 {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for (i, value) in [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25]
                .into_iter()
                .cycle()
                .enumerate()
                .take(effect.param_count())
            {
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
    eprintln!("{name} measured max-abs {maximum:?}");
    maximum
}

const RECORD_INTERSTAGE: ParityRecord = ParityRecord {
    name: "Interstage",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn interstage() {
    check_reset_preserves_params("Interstage");
    assert_eq!(
        measure(
            "Interstage",
            include_str!("fixtures/batch2/g13/Interstage.txt"),
            include_bytes!("fixtures/batch2/g13/Interstage.f32"),
        ),
        0.0
    );
    check_plugin(
        &RECORD_INTERSTAGE,
        include_str!("fixtures/batch2/g13/Interstage.txt"),
        include_bytes!("fixtures/batch2/g13/Interstage.f32"),
    );
}

const RECORD_IRONOXIDE5: ParityRecord = ParityRecord {
    name: "IronOxide5",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn ironoxide5() {
    check_reset_preserves_params("IronOxide5");
    assert_eq!(
        measure(
            "IronOxide5",
            include_str!("fixtures/batch2/g13/IronOxide5.txt"),
            include_bytes!("fixtures/batch2/g13/IronOxide5.f32"),
        ),
        0.0
    );
    check_plugin(
        &RECORD_IRONOXIDE5,
        include_str!("fixtures/batch2/g13/IronOxide5.txt"),
        include_bytes!("fixtures/batch2/g13/IronOxide5.f32"),
    );
}

const RECORD_IRONOXIDECLASSIC: ParityRecord = ParityRecord {
    name: "IronOxideClassic",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn ironoxideclassic() {
    check_reset_preserves_params("IronOxideClassic");
    assert_eq!(
        measure(
            "IronOxideClassic",
            include_str!("fixtures/batch2/g13/IronOxideClassic.txt"),
            include_bytes!("fixtures/batch2/g13/IronOxideClassic.f32"),
        ),
        0.0
    );
    check_plugin(
        &RECORD_IRONOXIDECLASSIC,
        include_str!("fixtures/batch2/g13/IronOxideClassic.txt"),
        include_bytes!("fixtures/batch2/g13/IronOxideClassic.f32"),
    );
}

const RECORD_IRONOXIDECLASSIC2: ParityRecord = ParityRecord {
    name: "IronOxideClassic2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn ironoxideclassic2() {
    check_reset_preserves_params("IronOxideClassic2");
    assert_eq!(
        measure(
            "IronOxideClassic2",
            include_str!("fixtures/batch2/g13/IronOxideClassic2.txt"),
            include_bytes!("fixtures/batch2/g13/IronOxideClassic2.f32"),
        ),
        0.0
    );
    check_plugin(
        &RECORD_IRONOXIDECLASSIC2,
        include_str!("fixtures/batch2/g13/IronOxideClassic2.txt"),
        include_bytes!("fixtures/batch2/g13/IronOxideClassic2.f32"),
    );
}

const RECORD_ISOLATOR: ParityRecord = ParityRecord {
    name: "Isolator",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn isolator() {
    check_reset_preserves_params("Isolator");
    assert_eq!(
        measure(
            "Isolator",
            include_str!("fixtures/batch2/g13/Isolator.txt"),
            include_bytes!("fixtures/batch2/g13/Isolator.f32"),
        ),
        0.0
    );
    check_plugin(
        &RECORD_ISOLATOR,
        include_str!("fixtures/batch2/g13/Isolator.txt"),
        include_bytes!("fixtures/batch2/g13/Isolator.f32"),
    );
}

const RECORD_ISOLATOR2: ParityRecord = ParityRecord {
    name: "Isolator2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn isolator2() {
    check_reset_preserves_params("Isolator2");
    assert_eq!(
        measure(
            "Isolator2",
            include_str!("fixtures/batch2/g13/Isolator2.txt"),
            include_bytes!("fixtures/batch2/g13/Isolator2.f32"),
        ),
        0.0
    );
    check_plugin(
        &RECORD_ISOLATOR2,
        include_str!("fixtures/batch2/g13/Isolator2.txt"),
        include_bytes!("fixtures/batch2/g13/Isolator2.f32"),
    );
}
