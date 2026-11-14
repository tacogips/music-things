#[path = "common/fnv.rs"]
mod fnv;
#[path = "common/stimulus.rs"]
mod stimulus;

#[test]
fn stimulus_checksum() {
    let samples = stimulus::pitch_stimulus();
    assert_eq!(samples.len(), 11025);
    assert_eq!(fnv::f32_fnv(&samples), 0x4ed50bb4be37bb1a);
}
