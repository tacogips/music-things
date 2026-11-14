mod common;

use common::parity::{check_plugin, ParityClass, ParityRecord};

const RECORD_MONITORING: ParityRecord = ParityRecord {
    name: "Monitoring",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn monitoring() {
    check_plugin(
        &RECORD_MONITORING,
        include_str!("fixtures/batch2/g22/Monitoring.txt"),
        include_bytes!("fixtures/batch2/g22/Monitoring.f32"),
    );
}

#[test]
fn monitoring_mode_sweep() {
    let names = [
        "Out24", "Out16", "Peaks", "Slew", "Subs", "Mono", "Side", "Vinyl", "Aurat", "MonoRat",
        "MonoLat", "Phone", "Cans A", "Cans B", "Cans C", "Cans D", "V Trick",
    ];
    for rate in [44_100.0, 96_000.0] {
        let (left, right) = common::stimulus::aw_stimulus(rate);
        for (mode, name) in names.iter().enumerate() {
            let mut effect =
                music_things_airwindows::create_with_seed("Monitoring", rate, 0x00c0_ffee).unwrap();
            let parameter = (mode as f32 + 0.5) / 16.999;
            effect.set_param(0, parameter);
            effect.reset();
            assert_eq!(effect.get_param(0), parameter);
            assert_eq!(effect.param_display(0), *name);
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
            assert!(
                out_l.iter().chain(&out_r).all(|sample| sample.is_finite()),
                "mode {name} at {rate}"
            );
        }
    }
}
