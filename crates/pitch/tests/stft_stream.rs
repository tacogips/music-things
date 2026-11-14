#[path = "common/stimulus.rs"]
mod stimulus;

use music_things_pitch::stft::{StftConfig, StftPitchShift, StftPitchShiftStream};

#[test]
fn stream_matches_offline_after_latency() {
    let input = stimulus::pitch_stimulus();
    let mut c1 = StftConfig::symmetric(1024, 256, 44100.0);
    c1.quefrency = 0.001;
    let mut c2 = StftConfig::symmetric(1024, 256, 44100.0);
    c2.normalization = true;
    c2.max_factors = 2;
    let mut c3 = StftConfig::symmetric(1024, 128, 44100.0);
    c3.synthesis_framesize = 512;
    let cases: [(StftConfig, &[f32]); 3] = [(c1, &[1.5]), (c2, &[0.75, 1.25]), (c3, &[1.25])];
    for (index, (config, factors)) in cases.into_iter().enumerate() {
        let framesize = config.analysis_framesize;
        let mut offline = StftPitchShift::new(config).unwrap();
        offline.set_factors(factors).unwrap();
        let mut expected = vec![0.0; input.len()];
        offline.shift_pitch(&input, &mut expected);
        let mut stream = StftPitchShiftStream::new(config).unwrap();
        stream.set_factors(factors).unwrap();
        let latency = stream.latency();
        let mut actual = vec![0.0; input.len()];
        for (source, destination) in input.chunks(128).zip(actual.chunks_mut(128)) {
            stream.process(source, destination);
        }
        let end = input.len().checked_sub(2 * framesize + latency).unwrap();
        assert!(end > framesize, "comparison region must not be empty");
        let mut max_error = 0.0_f32;
        for n in framesize..end {
            assert!(actual[n + latency].is_finite() && expected[n].is_finite());
            max_error = max_error.max((actual[n + latency] - expected[n]).abs());
        }
        println!(
            "C{} stream max_abs={max_error:.9} latency={latency}",
            index + 1
        );
        assert!(max_error <= 1e-6, "C{}: {max_error}", index + 1);
    }
}
