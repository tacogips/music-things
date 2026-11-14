mod common;
use common::parity::{check_plugin, ParityClass, ParityRecord};
const RECORD: ParityRecord = ParityRecord {
    name: "Energy2",
    class: ParityClass::Libm,
    measured_max_abs: 0.0,
    note: "",
};
#[test]
fn energy2() {
    let mut effect =
        music_things_airwindows::create_with_seed("Energy2", 44100.0, 0x00c0_ffee).unwrap();
    for i in 0..9 {
        effect.set_param(i, (i + 1) as f32 / 11.0);
    }
    effect.reset();
    for i in 0..9 {
        assert_eq!(effect.get_param(i), (i + 1) as f32 / 11.0);
    }
    check_plugin(
        &RECORD,
        include_str!("fixtures/batch2/g04/Energy2.txt"),
        include_bytes!("fixtures/batch2/g04/Energy2.f32"),
    );
}
