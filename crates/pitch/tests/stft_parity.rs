#[path = "common/fnv.rs"]
mod fnv;
#[path = "common/stimulus.rs"]
mod stimulus;

use music_things_pitch::{
    error::ConfigError,
    stft::{StftConfig, StftPitchShift},
};

#[test]
fn offline_matches_upstream() {
    let input = stimulus::pitch_stimulus();
    assert_eq!(fnv::f32_fnv(&input), 0x4ed50bb4be37bb1a);
    let mut c1 = StftConfig::symmetric(1024, 256, 44100.0);
    c1.quefrency = 0.001;
    let mut c2 = StftConfig::symmetric(1024, 256, 44100.0);
    c2.normalization = true;
    c2.max_factors = 2;
    let mut c3 = StftConfig::symmetric(1024, 128, 44100.0);
    c3.synthesis_framesize = 512;
    let cases: [(StftConfig, &[f32], &[u8]); 3] = [
        (c1, &[1.5], include_bytes!("fixtures/stft/c1.f32")),
        (c2, &[0.75, 1.25], include_bytes!("fixtures/stft/c2.f32")),
        (c3, &[1.25], include_bytes!("fixtures/stft/c3.f32")),
    ];
    for (index, (config, factors, bytes)) in cases.into_iter().enumerate() {
        assert_eq!(bytes.len(), input.len() * 4);
        let mut processor = StftPitchShift::new(config).unwrap();
        processor.set_factors(factors).unwrap();
        let mut output = vec![0.0; input.len()];
        processor.shift_pitch(&input, &mut output);
        let mut max_error = 0.0_f32;
        for (actual, expected) in output.iter().zip(bytes.chunks_exact(4)) {
            let expected = f32::from_le_bytes(expected.try_into().unwrap());
            assert!(actual.is_finite() && expected.is_finite());
            max_error = max_error.max((actual - expected).abs());
        }
        println!("C{} offline max_abs={max_error:.9}", index + 1);
        assert!(max_error <= 1e-5, "C{}: {max_error}", index + 1);
    }
}

#[test]
fn configuration_validation() {
    for size in [1, 1000] {
        assert!(
            matches!(StftPitchShift::new(StftConfig::symmetric(size, 1, 44100.0)), Err(ConfigError::NotPowerOfTwo { what: "framesize", value }) if value == size)
        );
    }
    let mut config = StftConfig::symmetric(512, 128, 44100.0);
    config.synthesis_framesize = 1024;
    assert!(matches!(
        StftPitchShift::new(config),
        Err(ConfigError::InvalidValue { what: "framesize" })
    ));
    assert!(matches!(
        StftPitchShift::new(StftConfig::symmetric(1024, 0, 44100.0)),
        Err(ConfigError::InvalidValue { what: "hopsize" })
    ));
    assert!(StftPitchShift::new(StftConfig::symmetric(1024, 384, 44100.0)).is_ok());
    let mut asymmetric = StftConfig::symmetric(1024, 256, 44100.0);
    asymmetric.synthesis_framesize = 513;
    assert!(StftPitchShift::new(asymmetric).is_ok());
    let mut config = StftConfig::symmetric(1024, 256, 44100.0);
    config.max_factors = 2;
    let mut processor = StftPitchShift::new(config).unwrap();
    assert!(matches!(
        processor.set_factors(&[]),
        Err(ConfigError::InvalidValue { .. })
    ));
    assert!(matches!(
        processor.set_factors(&[1.0, 1.0, 1.0]),
        Err(ConfigError::TooManyFactors { max: 2, got: 3 })
    ));
}
