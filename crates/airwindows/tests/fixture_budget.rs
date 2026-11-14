use std::path::Path;
fn size(path: &Path) -> u64 {
    match std::fs::read_dir(path) {
        Ok(entries) => entries
            .map(|entry| {
                let entry = entry.unwrap();
                let meta = entry.metadata().unwrap();
                if meta.is_dir() {
                    size(&entry.path())
                } else {
                    meta.len()
                }
            })
            .sum(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => 0,
        Err(e) => panic!("{}: {e}", path.display()),
    }
}
#[test]
fn fixture_budget() {
    assert!(
        size(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures"
        ))) - size(Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/batch3"
        ))) <= 1_600_000
    );
}
