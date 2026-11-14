use super::state::{Mv, DELAYS};
use crate::dither::fpd_dither;

impl Mv {
    pub(super) fn render(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let stage = (a as f64 * 27.0) as i32;
        let damp = ((1.0 - b as f64) * stage as f64) as i32;
        let mut feedback = c as f64;
        if feedback <= 0.0625 {
            feedback = 0.0;
        }
        if feedback > 0.0625 && feedback <= 0.125 {
            feedback = 0.0625;
        }
        if feedback > 0.125 && feedback <= 0.25 {
            feedback = 0.125;
        }
        if feedback > 0.25 && feedback <= 0.5 {
            feedback = 0.25;
        }
        if feedback > 0.5 && feedback <= 0.99 {
            feedback = 0.5;
        }
        if feedback > 0.99 {
            feedback = 1.0;
        }
        let gain = d as f64;
        let wet = e as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            l = (l + self.feedback_l).sin();
            r = (r + self.feedback_r).sin();
            for (i, &delay) in DELAYS.iter().enumerate() {
                let threshold = 26 - i as i32;
                if stage < threshold {
                    continue;
                }
                let pos = self.positions[i];
                let temp = if pos == 0 { delay } else { pos - 1 };
                l -= self.left[i][temp] * 0.5;
                self.left[i][pos] = l;
                l *= 0.5;
                r -= self.right[i][temp] * 0.5;
                self.right[i][pos] = r;
                r *= 0.5;
                self.positions[i] = temp;
                l += self.left[i][temp];
                r += self.right[i][temp];
                if damp > threshold {
                    let old_l = l;
                    l += self.avg_l[i];
                    l *= 0.5;
                    self.avg_l[i] = old_l;
                    let old_r = r;
                    r += self.avg_r[i];
                    r *= 0.5;
                    self.avg_r[i] = old_r;
                }
            }
            self.feedback_l = l * feedback;
            self.feedback_r = r * feedback;
            if gain != 1.0 {
                l *= gain;
                r *= gain;
            }
            l = l.clamp(-1.0, 1.0).asin();
            r = r.clamp(-1.0, 1.0).asin();
            if wet != 1.0 {
                l = l * wet + dry_l * (1.0 - wet);
                r = r * wet + dry_r * (1.0 - wet);
            }
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
