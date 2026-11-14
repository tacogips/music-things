mod common;

use common::parity::{check_plugin, ParityClass, ParityRecord};

const RECORD_LEFTOMONO: ParityRecord = ParityRecord {
    name: "LeftoMono",
    class: ParityClass::Exact,
    measured_max_abs: 0.0,
    note: "",
};

#[test]
fn leftomono() {
    check_plugin(
        &RECORD_LEFTOMONO,
        include_str!("fixtures/batch2/g15/LeftoMono.txt"),
        include_bytes!("fixtures/batch2/g15/LeftoMono.f32"),
    );
}
