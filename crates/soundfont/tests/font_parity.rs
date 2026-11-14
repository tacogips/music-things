#[path = "common/fnv.rs"]
mod fnv;
#[path = "common/sf2_builder.rs"]
mod sf2_builder;

use music_things_soundfont::font::SoundFont;
use std::path::PathBuf;

fn fixture(name: &str) -> String {
    std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/font")
            .join(name),
    )
    .unwrap()
}

#[test]
#[ignore]
fn export_synthetic_sf2() {
    let dir = PathBuf::from(
        std::env::var_os("SOUNDFONT_FIXTURE_OUT").expect("SOUNDFONT_FIXTURE_OUT required"),
    );
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("synthetic.sf2"), sf2_builder::synthetic_sf2()).unwrap();
    std::fs::write(
        dir.join("truncated.sf2"),
        sf2_builder::broken_sf2_truncated(),
    )
    .unwrap();
}

#[test]
fn synthetic_parity() {
    let bytes = sf2_builder::synthetic_sf2();
    let hashes = fixture("inputs.txt");
    assert!(hashes
        .lines()
        .any(|s| s == format!("synthetic={:016x}", fnv::fnv1a64(&bytes))));
    let font = SoundFont::load(&bytes).unwrap();
    assert_eq!(font.preset_count(), 3);
    let drum = font.preset_index(128, 0).unwrap();
    assert_eq!(font.preset_name(drum), Some("Drums"));
    assert_eq!(font.bank_preset_name(0, 1), Some("SynthSaw"));
    assert_eq!(font.preset_index(999, 999), None);
    let actual = font.debug_dump();
    let expected = fixture("synthetic.txt");
    let actual_lines: Vec<_> = actual.lines().collect();
    let expected_lines: Vec<_> = expected.lines().collect();
    assert_eq!(actual_lines.len(), expected_lines.len());
    for (line_number, (a, e)) in actual_lines.iter().zip(expected_lines.iter()).enumerate() {
        let aa: Vec<_> = a.split_whitespace().collect();
        let ee: Vec<_> = e.split_whitespace().collect();
        assert_eq!(aa.len(), ee.len(), "line {line_number}");
        for (i, (a, e)) in aa.iter().zip(ee.iter()).enumerate() {
            if aa[0] == "region" && ((16..=33).contains(&i) || i == 40 || i == 43) {
                let a = f32::from_bits(u32::from_str_radix(a, 16).unwrap());
                let e = f32::from_bits(u32::from_str_radix(e, 16).unwrap());
                assert!(
                    a.is_finite()
                        && e.is_finite()
                        && (a - e).abs() <= 1e-6 * a.abs().max(e.abs()).max(f32::MIN_POSITIVE),
                    "line {line_number} field {i}: {a} vs {e}"
                );
            } else {
                assert_eq!(a, e, "line {line_number} field {i}");
            }
        }
    }
}

#[test]
fn truncated_is_error() {
    let bytes = sf2_builder::broken_sf2_truncated();
    assert!(fixture("inputs.txt")
        .lines()
        .any(|s| s == format!("truncated={:016x}", fnv::fnv1a64(&bytes))));
    assert!(SoundFont::load(&bytes).is_err());
    assert_eq!(fixture("truncated.txt").trim(), "error");
}
