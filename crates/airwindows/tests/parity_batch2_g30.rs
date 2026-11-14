mod common;

use common::parity::{check_plugin, ParityClass, ParityRecord};

const RECORD: ParityRecord = ParityRecord {
    name: "NonlinearSpace",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "All three fixture renders match bit-for-bit",
};

#[test]
fn nonlinearspace_parity() {
    check_plugin(
        &RECORD,
        include_str!("fixtures/batch2/g30/NonlinearSpace.txt"),
        include_bytes!("fixtures/batch2/g30/NonlinearSpace.f32"),
    );
}

#[test]
fn nonlinearspace_selector_sweep() {
    for rate in [44100.0, 96000.0] {
        for mode in 0..7 {
            let mut effect =
                music_things_airwindows::create_with_seed("NonlinearSpace", rate, 0x00c0_ffee)
                    .unwrap();
            let selector = (mode as f32 + 0.5) / 6.999;
            effect.set_param(0, selector);
            let (left, right) = common::stimulus::aw_stimulus(rate);
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
            assert!(out_l.iter().chain(&out_r).all(|sample| sample.is_finite()));
            effect.reset();
            assert_eq!(effect.get_param(0), selector);
        }
    }
}
