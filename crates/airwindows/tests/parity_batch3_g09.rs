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

fn check_measured(record: &ParityRecord, meta: &str, data: &[u8]) {
    let fixture = common::fixture::Fixture::parse(meta, data);
    let alt = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25];
    let mut maximum = 0.0_f32;
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = common::stimulus::aw_stimulus(rate);
        let mut effect =
            music_things_airwindows::create_with_seed(record.name, rate, 0x00c0_ffee).unwrap();
        if render != 0 {
            for i in 0..fixture.params.len() {
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
    eprintln!("{} measured max-abs {maximum:?}", record.name);
    assert_eq!(
        maximum, record.measured_max_abs,
        "{} measured maximum",
        record.name
    );
}

const RECORD_TOTAPE6: ParityRecord = ParityRecord {
    name: "ToTape6",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn totape6() {
    check_reset_preserves_params("ToTape6");
    check_plugin(
        &RECORD_TOTAPE6,
        include_str!("fixtures/batch3/g09/ToTape6.txt"),
        include_bytes!("fixtures/batch3/g09/ToTape6.f32"),
    );
    check_measured(
        &RECORD_TOTAPE6,
        include_str!("fixtures/batch3/g09/ToTape6.txt"),
        include_bytes!("fixtures/batch3/g09/ToTape6.f32"),
    );
}

const RECORD_TOVINYL4: ParityRecord = ParityRecord {
    name: "ToVinyl4",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn tovinyl4() {
    check_reset_preserves_params("ToVinyl4");
    check_plugin(
        &RECORD_TOVINYL4,
        include_str!("fixtures/batch3/g09/ToVinyl4.txt"),
        include_bytes!("fixtures/batch3/g09/ToVinyl4.f32"),
    );
    check_measured(
        &RECORD_TOVINYL4,
        include_str!("fixtures/batch3/g09/ToVinyl4.txt"),
        include_bytes!("fixtures/batch3/g09/ToVinyl4.f32"),
    );
}

const RECORD_TPDFDITHER: ParityRecord = ParityRecord {
    name: "TPDFDither",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn tpdfdither() {
    check_reset_preserves_params("TPDFDither");
    check_plugin(
        &RECORD_TPDFDITHER,
        include_str!("fixtures/batch3/g09/TPDFDither.txt"),
        include_bytes!("fixtures/batch3/g09/TPDFDither.f32"),
    );
    check_measured(
        &RECORD_TPDFDITHER,
        include_str!("fixtures/batch3/g09/TPDFDither.txt"),
        include_bytes!("fixtures/batch3/g09/TPDFDither.f32"),
    );
}

const RECORD_TPDFWIDE: ParityRecord = ParityRecord {
    name: "TPDFWide",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn tpdfwide() {
    check_reset_preserves_params("TPDFWide");
    check_plugin(
        &RECORD_TPDFWIDE,
        include_str!("fixtures/batch3/g09/TPDFWide.txt"),
        include_bytes!("fixtures/batch3/g09/TPDFWide.f32"),
    );
    check_measured(
        &RECORD_TPDFWIDE,
        include_str!("fixtures/batch3/g09/TPDFWide.txt"),
        include_bytes!("fixtures/batch3/g09/TPDFWide.f32"),
    );
}

const RECORD_TRANSDESK: ParityRecord = ParityRecord {
    name: "TransDesk",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn transdesk() {
    check_reset_preserves_params("TransDesk");
    check_plugin(
        &RECORD_TRANSDESK,
        include_str!("fixtures/batch3/g09/TransDesk.txt"),
        include_bytes!("fixtures/batch3/g09/TransDesk.f32"),
    );
    check_measured(
        &RECORD_TRANSDESK,
        include_str!("fixtures/batch3/g09/TransDesk.txt"),
        include_bytes!("fixtures/batch3/g09/TransDesk.f32"),
    );
}

const RECORD_TREMOLO: ParityRecord = ParityRecord {
    name: "Tremolo",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn tremolo() {
    check_reset_preserves_params("Tremolo");
    check_plugin(
        &RECORD_TREMOLO,
        include_str!("fixtures/batch3/g09/Tremolo.txt"),
        include_bytes!("fixtures/batch3/g09/Tremolo.f32"),
    );
    check_measured(
        &RECORD_TREMOLO,
        include_str!("fixtures/batch3/g09/Tremolo.txt"),
        include_bytes!("fixtures/batch3/g09/Tremolo.f32"),
    );
}

const RECORD_TREMOSQUARE: ParityRecord = ParityRecord {
    name: "TremoSquare",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn tremosquare() {
    check_reset_preserves_params("TremoSquare");
    check_plugin(
        &RECORD_TREMOSQUARE,
        include_str!("fixtures/batch3/g09/TremoSquare.txt"),
        include_bytes!("fixtures/batch3/g09/TremoSquare.f32"),
    );
    check_measured(
        &RECORD_TREMOSQUARE,
        include_str!("fixtures/batch3/g09/TremoSquare.txt"),
        include_bytes!("fixtures/batch3/g09/TremoSquare.f32"),
    );
}

const RECORD_TRIPLESPREAD: ParityRecord = ParityRecord {
    name: "TripleSpread",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn triplespread() {
    check_reset_preserves_params("TripleSpread");
    check_plugin(
        &RECORD_TRIPLESPREAD,
        include_str!("fixtures/batch3/g09/TripleSpread.txt"),
        include_bytes!("fixtures/batch3/g09/TripleSpread.f32"),
    );
    check_measured(
        &RECORD_TRIPLESPREAD,
        include_str!("fixtures/batch3/g09/TripleSpread.txt"),
        include_bytes!("fixtures/batch3/g09/TripleSpread.f32"),
    );
}
