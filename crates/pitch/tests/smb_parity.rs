#[path = "common/fnv.rs"]
mod fnv;
#[path = "common/stimulus.rs"]
mod stimulus;

use music_things_pitch::{error::ConfigError, smb::SmbPitchShifter};

fn run(input: &[f32], shift: f32, frame: usize, osamp: usize, chunk: usize) -> Vec<f32> {
    let mut shifter = SmbPitchShifter::new(shift, frame, osamp, 44100.0).unwrap();
    let mut out = vec![0.0; input.len()];
    for (source, dest) in input.chunks(chunk).zip(out.chunks_mut(chunk)) {
        shifter.process(source, dest);
    }
    out
}

#[test]
fn reference_parity() {
    let samples = stimulus::pitch_stimulus();
    assert_eq!(fnv::f32_fnv(&samples), 0x4ed50bb4be37bb1a);
    let metadata = include_str!("fixtures/smb/meta.txt");
    assert!(metadata.to_lowercase().contains("4ed50bb4be37bb1a"));
    let input: Vec<f32> = samples
        .iter()
        .map(|x| {
            let scaled = f64::from(*x) * 32768.0;
            assert!((-32768.0..32768.0).contains(&scaled));
            f32::from(scaled as i16) / 32768.0
        })
        .collect();
    for (shift, frame, osamp, fixture) in [
        (1.5, 1024, 4, &include_bytes!("fixtures/smb/c1.i16")[..]),
        (0.75, 2048, 8, &include_bytes!("fixtures/smb/c2.i16")[..]),
    ] {
        let result = run(&input, shift, frame, osamp, 256);
        assert_eq!(fixture.len(), 2 * result.len());
        let mut max_error = 0_i32;
        for (actual, expected) in result.iter().zip(fixture.chunks_exact(2)) {
            let scaled = f64::from(*actual) * 32768.0;
            assert!(scaled.abs() < 32767.0);
            let reference = i16::from_le_bytes([expected[0], expected[1]]);
            max_error =
                max_error.max((i32::from(scaled.trunc() as i16) - i32::from(reference)).abs());
        }
        println!("frame {frame}: max error {max_error} LSB");
        assert!(max_error <= 3, "frame {frame}: {max_error} LSB");
    }
}

#[test]
fn invalid_configs_and_chunk_invariance() {
    assert!(matches!(
        SmbPitchShifter::new(1.5, 1000, 4, 44100.0),
        Err(ConfigError::NotPowerOfTwo {
            what: "fft_frame_size",
            ..
        })
    ));
    assert!(matches!(
        SmbPitchShifter::new(1.5, 16384, 4, 44100.0),
        Err(ConfigError::OutOfRange {
            what: "fft_frame_size",
            ..
        })
    ));
    for osamp in [0, 3] {
        assert!(matches!(
            SmbPitchShifter::new(1.5, 1024, osamp, 44100.0),
            Err(ConfigError::InvalidValue { what: "osamp" })
        ));
    }
    let input = stimulus::pitch_stimulus();
    let a = run(&input, 1.5, 1024, 4, 64);
    let b = run(&input, 1.5, 1024, 4, 256);
    assert!(a.iter().zip(&b).all(|(a, b)| a.to_bits() == b.to_bits()));
    let mut shifter = SmbPitchShifter::new(1.5, 1024, 4, 44100.0).unwrap();
    assert_eq!(shifter.latency(), 768);
    let mut result = vec![0.0; input.len()];
    shifter.process(&input, &mut result);
    shifter.reset();
    shifter.process(&input, &mut result);
    assert!(a
        .iter()
        .zip(&result)
        .all(|(a, b)| a.to_bits() == b.to_bits()));
}
