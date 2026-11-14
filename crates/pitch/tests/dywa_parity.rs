#[path = "common/fnv.rs"]
mod fnv;
#[path = "common/stimulus.rs"]
mod stimulus;
use music_things_pitch::{dywa::PitchTracker, ConfigError};

#[test]
fn parity_and_streaming() {
    let samples = stimulus::pitch_stimulus();
    let meta = include_str!("fixtures/dywa/meta.txt").trim();
    assert_eq!(meta, format!("pitch={:016x}", fnv::f32_fnv(&samples)));
    let fixture = include_str!("fixtures/dywa/pitches.txt");
    for window in [1024, 2048] {
        let expected: Vec<f64> = fixture
            .lines()
            .filter_map(|line| {
                let mut fields = line.split_whitespace();
                let w: usize = fields.next()?.parse().unwrap();
                let _start: usize = fields.next()?.parse().unwrap();
                let pitch: f64 = fields.next()?.parse().unwrap();
                (w == window).then_some(pitch)
            })
            .collect();
        let mut tracker = PitchTracker::new(44100.0, window, window).unwrap();
        let actual: Vec<_> = samples
            .chunks_exact(window)
            .map(|chunk| tracker.compute(chunk))
            .collect();
        assert_eq!(actual.len(), expected.len());
        assert!(!actual.is_empty());
        for (&actual, &expected) in actual.iter().zip(&expected) {
            assert_eq!(actual == 0.0, expected == 0.0);
            if expected != 0.0 {
                assert!(
                    (actual - expected).abs() / expected.abs() <= 1e-9,
                    "{actual} != {expected}"
                );
            }
        }
        let mut streaming = PitchTracker::new(44100.0, window, window).unwrap();
        let streamed: Vec<_> = samples
            .chunks(300)
            .filter_map(|chunk| streaming.push(chunk))
            .collect();
        assert_eq!(actual, streamed);
        tracker.reset();
        assert_eq!(tracker.compute(&samples[..window]), actual[0]);
    }
}

#[test]
fn invalid_configuration() {
    assert!(matches!(
        PitchTracker::new(44100.0, 1000, 1000),
        Err(ConfigError::NotPowerOfTwo { .. })
    ));
    assert!(matches!(
        PitchTracker::new(44100.0, 1024, 0),
        Err(ConfigError::InvalidValue { what: "hop" })
    ));
    assert!(matches!(
        PitchTracker::new(44100.0, 1024, 1025),
        Err(ConfigError::InvalidValue { what: "hop" })
    ));
    assert!(matches!(
        PitchTracker::new(44100.0, 512, 512),
        Err(ConfigError::OutOfRange { .. })
    ));
}
