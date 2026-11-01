use std::path::Path;

fn size(path: &Path) -> u64 {
    match std::fs::read_dir(path) {
        Ok(entries) => entries
            .map(|entry| {
                let entry = entry.expect("read fixture entry");
                let metadata = entry.metadata().expect("read fixture metadata");
                if metadata.is_dir() {
                    size(&entry.path())
                } else {
                    metadata.len()
                }
            })
            .sum(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0,
        Err(error) => panic!("read fixture directory: {error}"),
    }
}

#[test]
fn fixture_budget() {
    let path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures"));
    assert!(size(path) <= 400_000);
}
