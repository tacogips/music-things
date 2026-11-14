//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! LeadAmp gain and tone stages, upstream revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
use super::state::State;
impl State {
    pub(super) fn gain_stages(
        &mut self,
        c: usize,
        mut x: f64,
        start: f64,
        eq: f64,
        bassfactor: f64,
    ) -> f64 {
        let mut level = start;
        let mut basscut = 0.99;
        for stage in 0..11 {
            let filter = match stage {
                0 => Some(0),
                1 => Some(1),
                2 => Some(2),
                4 => Some(3),
                6 => Some(4),
                8 => Some(5),
                _ => None,
            };
            if let Some(i) = filter {
                x = fixed(&mut self.fix[i], c, x);
            }
            x *= level;
            level = ((level * 7.0) + 1.0) / 8.0;
            self.iir[c][stage] = (self.iir[c][stage] * (1.0 - eq)) + (x * eq);
            basscut *= bassfactor;
            x -= self.iir[c][stage] * basscut;
            if stage == 0 {
                let rect = x.abs().min(1.57079633).sin();
                x = if x > 0.0 { rect } else { -rect };
            } else {
                x = x.clamp(-1.57079633, 1.57079633).sin();
            }
            let rect = self.smooth[c][stage] + x;
            self.smooth[c][stage] = x;
            x = rect;
        }
        x
    }
}
fn fixed(f: &mut [f64; 11], c: usize, x: f64) -> f64 {
    let i = if c == 0 { 7 } else { 9 };
    let out = x * f[2] + f[i];
    f[i] = x * f[3] - out * f[5] + f[i + 1];
    f[i + 1] = x * f[4] - out * f[6];
    out
}
