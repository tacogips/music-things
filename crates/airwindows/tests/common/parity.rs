use super::{fixture::Fixture, fnv, stimulus};
use music_things_airwindows::create_with_seed;
const ALT: [f32; 9] = [0.30, 0.85, 0.15, 0.65, 0.95, 0.45, 0.05, 0.70, 0.25];
pub enum ParityClass {
    Exact,
    Libm,
    Custom(f32),
}
pub struct ParityRecord {
    pub name: &'static str,
    pub class: ParityClass,
    pub measured_max_abs: f32,
    pub note: &'static str,
}
pub fn check_plugin(record: &ParityRecord, meta: &str, data: &[u8]) {
    if let ParityClass::Custom(limit) = record.class {
        assert!(
            limit > 1e-6 && !record.note.trim().is_empty(),
            "custom tolerance requires a reason and limit > 1e-6"
        );
    }
    let fixture = Fixture::parse(meta, data);
    assert_eq!(record.name, fixture.name);
    for render in 0..3 {
        let rate = if render == 2 { 96000.0 } else { 44100.0 };
        let (left, right) = stimulus::aw_stimulus(rate);
        let interleaved: Vec<f32> = left
            .iter()
            .zip(&right)
            .flat_map(|(&l, &r)| [l, r])
            .collect();
        assert_eq!(
            fnv::f32_fnv(&interleaved),
            if render == 2 {
                fixture.stimulus_fnv_96000
            } else {
                fixture.stimulus_fnv_44100
            }
        );
        let mut effect = create_with_seed(record.name, rate, 0x00c0_ffee).expect("plugin missing");
        assert_eq!(effect.info().name, fixture.name);
        assert_eq!(effect.info().upstream_name, fixture.upstream);
        assert_eq!(effect.info().description, fixture.description);
        assert_eq!(effect.param_count(), fixture.params.len());
        for (i, p) in fixture.params.iter().enumerate() {
            let info = effect.param_info(i).unwrap();
            assert_eq!(info.name, p.name);
            assert_eq!(info.default.to_bits(), p.default_bits);
            assert_eq!(info.label, p.label_r0);
            assert_eq!(effect.get_param(i).to_bits(), p.default_bits);
            assert_eq!(effect.param_label(i), p.label_r0);
            assert_eq!(effect.param_display(i), p.display_r0);
        }
        if render != 0 {
            for i in 0..fixture.params.len() {
                effect.set_param(i, ALT[i % ALT.len()]);
            }
        }
        for (i, p) in fixture.params.iter().enumerate() {
            if render != 0 {
                assert_eq!(effect.param_label(i), p.label_r1);
                assert_eq!(effect.param_display(i), p.display_r1);
            }
        }
        let mut out_l = vec![0.0; left.len()];
        let mut out_r = vec![0.0; right.len()];
        for block in (0..left.len()).step_by(128) {
            effect.process(
                &left[block..block + 128],
                &right[block..block + 128],
                &mut out_l[block..block + 128],
                &mut out_r[block..block + 128],
            );
        }
        let mut max = 0.0_f32;
        for k in 0..504 {
            let frame = if k < 256 { k } else { 256 + 32 * (k - 256) };
            for (channel, actual) in [(0, out_l[frame]), (1, out_r[frame])] {
                let expected = fixture.renders[render][2 * k + channel];
                let diff = (actual - expected).abs();
                assert!(diff.is_finite(),"render {render} frame {frame} channel {channel}: expected {expected:?}, got {actual:?}");
                max = max.max(diff);
                match record.class { ParityClass::Exact=>assert_eq!(actual.to_bits(),expected.to_bits(),"render {render} frame {frame} channel {channel}: expected {expected:?}, got {actual:?}"), ParityClass::Libm=>assert!(diff<=1e-6,"render {render} frame {frame} channel {channel}: expected {expected:?}, got {actual:?}"), ParityClass::Custom(limit)=>assert!(diff<=limit,"render {render} frame {frame} channel {channel}: expected {expected:?}, got {actual:?}") }
            }
        }
        if render == 0 {
            effect.reset();
            let mut l = vec![0.0; left.len()];
            let mut r = vec![0.0; right.len()];
            for block in (0..left.len()).step_by(128) {
                effect.process(
                    &left[block..block + 128],
                    &right[block..block + 128],
                    &mut l[block..block + 128],
                    &mut r[block..block + 128],
                );
            }
            for i in 0..left.len() {
                assert_eq!(
                    l[i].to_bits(),
                    out_l[i].to_bits(),
                    "reset frame {i} channel 0 expected {:?}, got {:?}",
                    out_l[i],
                    l[i]
                );
                assert_eq!(
                    r[i].to_bits(),
                    out_r[i].to_bits(),
                    "reset frame {i} channel 1 expected {:?}, got {:?}",
                    out_r[i],
                    r[i]
                );
            }
        }
        let _ = (max, record.measured_max_abs);
    }
}
