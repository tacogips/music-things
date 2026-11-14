use music_things_airwindows::{catalog, create, create_with_seed};
use std::{fs, path::Path};

// DubSub (list line 118) is deferred: its upstream reference relies on signed overflow and is not reproducible.
const BATCH2: [(&str, &str); 109] = [
    ("DitherMeTimbers", "g01"),
    ("Doublelay", "g01"),
    ("DoublePaul", "g01"),
    ("Drive", "g01"),
    ("DrumSlam", "g01"),
    ("DubCenter", "g01"),
    ("Dubly", "g01"),
    ("DustBunny", "g02"),
    ("Dynamics", "g02"),
    ("Dyno", "g02"),
    ("Edge", "g02"),
    ("EdIsDim", "g02"),
    ("Elation", "g02"),
    ("ElectroHat", "g02"),
    ("Energy", "g03"),
    ("Energy2", "g04"),
    ("Ensemble", "g05"),
    ("EQ", "g05"),
    ("EveryTrim", "g05"),
    ("Exciter", "g05"),
    ("Facet", "g05"),
    ("FathomFive", "g05"),
    ("FireAmp", "g06"),
    ("Flipity", "g07"),
    ("Floor", "g07"),
    ("Flutter", "g07"),
    ("Focus", "g07"),
    ("Fracture", "g07"),
    ("FromTape", "g08"),
    ("Galactic", "g08"),
    ("Gatelope", "g08"),
    ("GlitchShifter", "g08"),
    ("Golem", "g08"),
    ("GrindAmp", "g09"),
    ("Gringer", "g10"),
    ("GrooveWear", "g10"),
    ("GuitarConditioner", "g10"),
    ("HardVacuum", "g10"),
    ("Hermepass", "g10"),
    ("HermeTrim", "g10"),
    ("HighGlossDither", "g10"),
    ("HighImpact", "g11"),
    ("Highpass", "g11"),
    ("Highpass2", "g11"),
    ("Holt", "g11"),
    ("Holt2", "g11"),
    ("Hombre", "g11"),
    ("Hull", "g11"),
    ("Hype", "g12"),
    ("Hypersonic", "g12"),
    ("HypersonX", "g12"),
    ("Infinity", "g12"),
    ("Infinity2", "g12"),
    ("Infrasonic", "g12"),
    ("Interstage", "g13"),
    ("IronOxide5", "g13"),
    ("IronOxideClassic", "g13"),
    ("IronOxideClassic2", "g13"),
    ("Isolator", "g13"),
    ("Isolator2", "g13"),
    ("LeadAmp", "g14"),
    ("LeftoMono", "g15"),
    ("LilAmp", "g16"),
    ("Logical4", "g17"),
    ("Loud", "g18"),
    ("Lowpass", "g18"),
    ("Lowpass2", "g18"),
    ("LRFlipTimer", "g18"),
    ("Luxor", "g18"),
    ("MackEQ", "g19"),
    ("Mackity", "g19"),
    ("MatrixVerb", "g19"),
    ("Melt", "g19"),
    ("MidAmp", "g20"),
    ("MidSide", "g21"),
    ("Mojo", "g21"),
    ("Monitoring", "g22"),
    ("Monitoring2", "g23"),
    ("Monitoring3", "g24"),
    ("MoNoam", "g25"),
    ("MultiBandDistortion", "g25"),
    ("MV", "g26"),
    ("NaturalizeDither", "g27"),
    ("NCSeventeen", "g27"),
    ("Neverland", "g27"),
    ("Nikola", "g28"),
    ("NodeDither", "g29"),
    ("Noise", "g29"),
    ("NonlinearSpace", "g30"),
    ("NotJustAnotherCD", "g31"),
    ("NotJustAnotherDither", "g31"),
    ("OneCornerClip", "g31"),
    ("Pafnuty", "g31"),
    ("PaulDither", "g31"),
    ("PaulWide", "g31"),
    ("PDBuss", "g32"),
    ("PDChannel", "g32"),
    ("PeaksOnly", "g32"),
    ("PhaseNudge", "g32"),
    ("PitchDelay", "g32"),
    ("PitchNasty", "g32"),
    ("PocketVerbs", "g33"),
    ("Pockey", "g34"),
    ("Pockey2", "g34"),
    ("Podcast", "g34"),
    ("PodcastDeluxe", "g34"),
    ("Point", "g34"),
    ("Pop", "g34"),
    ("Pop2", "g34"),
];

fn directory_bytes(path: &Path) -> u64 {
    if !path.exists() {
        return 0;
    }
    fs::read_dir(path)
        .expect("read fixture directory")
        .map(|entry| {
            let entry = entry.expect("read fixture entry");
            let metadata = entry.metadata().expect("read fixture metadata");
            if metadata.is_dir() {
                directory_bytes(&entry.path())
            } else {
                metadata.len()
            }
        })
        .sum()
}

#[test]
fn batch2_fixture_budget() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/batch2");
    assert!(directory_bytes(&root) <= 3_000_000);
    for (name, group) in BATCH2 {
        let dir = root.join(group);
        assert!(
            dir.join(format!("{name}.txt")).is_file(),
            "{group}/{name}.txt missing"
        );
        let binary = dir.join(format!("{name}.f32"));
        assert_eq!(
            fs::metadata(&binary).expect("fixture missing").len(),
            12_096,
            "{group}/{name}"
        );
    }
}

#[test]
fn batch2_catalog_exact() {
    let entries: Vec<_> = catalog().collect();
    let start = entries
        .iter()
        .position(|info| info.name == "DitherMeDiskers")
        .expect("DitherMeDiskers missing")
        + 1;
    let names: Vec<_> = entries
        .iter()
        .skip(start)
        .take(BATCH2.len())
        .map(|info| info.name)
        .collect();
    let expected: Vec<_> = BATCH2.iter().map(|(name, _)| *name).collect();
    assert_eq!(names, expected, "batch-2 catalog order");
    for (name, _) in BATCH2 {
        let matches: Vec<_> = entries.iter().filter(|info| info.name == name).collect();
        assert_eq!(matches.len(), 1, "{name} catalog count");
        assert_eq!(matches[0].upstream_name, name, "{name} upstream name");
        assert!(create(name, 44100.0).is_some(), "{name} create");
        assert!(
            create_with_seed(name, 96000.0, 1).is_some(),
            "{name} seeded create"
        );
    }
}

#[test]
fn batch2_parity_coverage() {
    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    for (name, group) in BATCH2 {
        let path = tests.join(format!("parity_batch2_{group}.rs"));
        let source = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{group}/{name}: {}: {error}", path.display()));
        assert!(
            source.contains(&format!("name: \"{name}\"")),
            "{group}/{name} parity record missing"
        );
        assert!(
            !source.contains("#[ignore"),
            "{group}/{name} parity file contains ignore"
        );
    }
}
