pub(crate) fn fpd_from_seed(seed: u32, lane: u32) -> u32 {
    let mut x = seed.wrapping_add(lane.wrapping_mul(0x9e37_79b9));
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^= x >> 16;
    if x < 16386 {
        x += 16386;
    }
    x
}
pub(crate) fn xorshift(x: &mut u32) {
    *x ^= *x << 13;
    *x ^= *x >> 17;
    *x ^= *x << 5;
}
pub(crate) fn frexpf_exponent(x: f32) -> i32 {
    let bits = x.to_bits() & 0x7fff_ffff;
    let e = (bits >> 23) as i32;
    if e == 255 || bits == 0 {
        0
    } else if e == 0 {
        -126 - ((bits & 0x7f_ffff).leading_zeros() as i32 - 9)
    } else {
        e - 126
    }
}
pub(crate) fn pow2(e: i32) -> f64 {
    debug_assert!((-1022..=1023).contains(&e));
    f64::from_bits(((e + 1023) as u64) << 52)
}
pub(crate) fn fpd_dither(sample: f64, fpd: &mut u32) -> f64 {
    let expon = frexpf_exponent(sample as f32);
    xorshift(fpd);
    sample + ((*fpd as f64 - 2147483647.0) * 5.5e-36) * pow2(expon + 62)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exponent_and_seed() {
        assert_eq!(frexpf_exponent(1.0), 1);
        assert_eq!(frexpf_exponent(0.5), 0);
        assert_eq!(frexpf_exponent(0.0), 0);
        assert_eq!(frexpf_exponent(f32::MIN_POSITIVE), -125);
        assert_eq!(frexpf_exponent(f32::from_bits(1)), -148);
        assert!(fpd_from_seed(0, 0) >= 16386);
        assert_ne!(fpd_from_seed(1, 0), fpd_from_seed(1, 1));
        let mut f = fpd_from_seed(1, 0);
        let prev = f;
        let _ = fpd_dither(0.25, &mut f);
        assert_ne!(f, prev);
    }
}
