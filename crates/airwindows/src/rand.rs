use crate::dither::{fpd_from_seed, xorshift};
pub(crate) const RAND_MAX: u32 = 0x7fff_ffff;
pub(crate) struct AwRand {
    state: u32,
}
impl AwRand {
    pub(crate) fn new(seed: u32) -> Self {
        Self {
            state: fpd_from_seed(seed, 2),
        }
    }
    pub(crate) fn next(&mut self) -> u32 {
        xorshift(&mut self.state);
        self.state >> 1
    }
}
pub(crate) fn replay_fpd_seed_loop(rng: &mut AwRand) -> u32 {
    let mut fpd = 1;
    while fpd < 16386 {
        fpd = rng.next().wrapping_mul(u32::MAX);
    }
    fpd
}
