#[path = "common/fnv.rs"]
mod fnv;
#[path = "common/stimulus.rs"]
mod stimulus;

#[test]
fn stimulus_checksum() {
    for (rate, expected) in [(44100.0, 0xd83a50634fc97283), (96000.0, 0x9079f3b5302ec7f0)] {
        let (left, right) = stimulus::aw_stimulus(rate);
        assert_eq!(left[0], 0.9);
        assert_eq!(right[0], -0.5);
        assert_eq!(left[6144], 0.0);
        assert_eq!(right[6144], 0.0);
        let interleaved: Vec<f32> = left
            .iter()
            .zip(&right)
            .flat_map(|(&l, &r)| [l, r])
            .collect();
        assert_eq!(fnv::f32_fnv(&interleaved), expected);
    }
}
