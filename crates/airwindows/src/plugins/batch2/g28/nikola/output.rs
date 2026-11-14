//! Wet mix and 32-bit floating point dither from processReplacing.
use crate::dither::fpd_dither;
pub(super) fn finish(mut sample: f64, dry: f64, wet: f64, fpd: &mut u32) -> f32 {
    if wet != 1.0 {
        sample = (sample * wet) + (dry * (1.0 - wet));
    }
    fpd_dither(sample, fpd) as f32
}
