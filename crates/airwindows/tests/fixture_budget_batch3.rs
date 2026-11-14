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
fn fixture_budget_batch3() {
    let bytes = size(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/batch3"
    )));
    assert!(
        bytes > 0 && bytes <= 3_000_000,
        "batch 3 fixture size: {bytes} bytes"
    );
}
