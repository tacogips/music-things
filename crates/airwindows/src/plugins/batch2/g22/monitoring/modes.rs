//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Monitoring/MonitoringProc.cpp, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Monitoring mode processing and NJAD output quantizer.
use super::Monitoring;

impl Monitoring {
    fn allpass(&mut self, stage: usize, limit: usize, l: &mut f64, r: &mut f64, dark: bool) {
        let limit = limit.min(1501);
        let index = self.index[stage];
        let previous = if index == 0 || index - 1 > limit {
            limit
        } else {
            index - 1
        };
        *l -= self.delay[stage][0][previous] * 0.5;
        self.delay[stage][0][index] = *l;
        *l *= 0.5;
        *r -= self.delay[stage][1][previous] * 0.5;
        self.delay[stage][1][index] = *r;
        *r *= 0.5;
        let next = if index == 0 || index - 1 > limit {
            limit
        } else {
            index - 1
        };
        self.index[stage] = next;
        if dark {
            let adjacent = if next == limit { 0 } else { next + 1 };
            *l += self.delay[stage][0][next] * 0.5;
            *r += self.delay[stage][1][next] * 0.5;
            *l += self.delay[stage][0][adjacent] * 0.5;
            *r += self.delay[stage][1][adjacent] * 0.5;
        } else {
            *l += self.delay[stage][0][next];
            *r += self.delay[stage][1][next];
        }
    }
    pub(super) fn mode(&mut self, mode: usize, scale: f64, l: &mut f64, r: &mut f64) {
        let limits = [149, 179, 191, 223].map(|x| (x as f64 * scale) as usize);
        match mode {
            2 => {
                for (stage, limit) in limits.into_iter().enumerate() {
                    *l = l.clamp(-1.0, 1.0).asin();
                    *r = r.clamp(-1.0, 1.0).asin();
                    self.allpass(stage, limit, l, r, false);
                }
                *l = l.clamp(-1.0, 1.0).asin() * 0.63679;
                *r = r.clamp(-1.0, 1.0).asin() * 0.63679;
            }
            3 => {
                let sl = ((*l - self.last[0]) * std::f64::consts::LN_10).clamp(-1.0, 1.0);
                let sr = ((*r - self.last[1]) * std::f64::consts::LN_10).clamp(-1.0, 1.0);
                self.last = [*l, *r];
                *l = sl;
                *r = sr;
            }
            4 => {
                let amount = (2250.0 / 44100.0) / scale;
                let mut gain = 1.42;
                *l *= gain;
                *r *= gain;
                gain = ((gain - 1.0) * 0.75) + 1.0;
                for i in 0..26 {
                    self.iir[0][i] = self.iir[0][i] * (1.0 - amount) + *l * amount;
                    self.iir[1][i] = self.iir[1][i] * (1.0 - amount) + *r * amount;
                    *l = self.iir[0][i];
                    *r = self.iir[1][i];
                    if i != 25 {
                        *l *= gain;
                        *r *= gain;
                        gain = ((gain - 1.0) * 0.75) + 1.0;
                    }
                    *l = l.clamp(-1.0, 1.0);
                    *r = r.clamp(-1.0, 1.0);
                }
            }
            5 | 6 => {
                let mid = if mode == 5 { *l + *r } else { 0.0 };
                let side = if mode == 6 { *l - *r } else { 0.0 };
                *l = (mid + side) / 2.0;
                *r = (mid - side) / 2.0;
            }
            7..=11 => {
                if mode == 9 {
                    *r = (*l + *r) * 0.5;
                    *l = 0.0;
                }
                if mode == 10 {
                    *l = (*l + *r) * 0.5;
                    *r = 0.0;
                }
                if mode == 11 {
                    let mid = (*l + *r) * 0.5;
                    *l = mid;
                    *r = mid;
                }
                for (input, b) in [l, r].into_iter().zip(self.biquad.iter_mut()) {
                    *input = input.sin();
                    let temp = *input * b[2] + b[7];
                    b[7] = -temp * b[5] + b[8];
                    b[8] = *input * b[4] - temp * b[6];
                    *input = temp.clamp(-1.0, 1.0).asin();
                }
            }
            12..=15 => {
                let gain = [0.855, 0.748, 0.713, 0.680][mode - 12];
                *l = (*l * gain).sin();
                *r = (*r * gain).sin();
                let mut dry_l = *l;
                let mut dry_r = *r;
                let bass = (mode * mode) as f64 * 0.00001 / scale;
                let mid = *l + *r;
                let mut side = *l - *r;
                self.iir[0][0] = self.iir[0][0] * (1.0 - bass * 0.618) + side * bass * 0.618;
                side -= self.iir[0][0];
                *l = (mid + side) / 2.0;
                *r = (mid - side) / 2.0;
                self.allpass(0, limits[0], l, r, true);
                let feed = [0.125, 0.25, 0.30, 0.35][mode - 12];
                *l *= feed;
                *r *= feed;
                dry_l += *r;
                dry_r += *l;
                self.allpass(3, limits[3], l, r, true);
                *l *= 0.25;
                *r *= 0.25;
                dry_l += *r;
                dry_r += *l;
                *l = dry_l;
                *r = dry_r;
                let mid = *l + *r;
                let mut side = *l - *r;
                self.iir[1][0] = self.iir[1][0] * (1.0 - bass) + side * bass;
                side -= self.iir[1][0];
                *l = ((mid + side) / 2.0).clamp(-1.0, 1.0).asin();
                *r = ((mid - side) / 2.0).clamp(-1.0, 1.0).asin();
            }
            16 => {
                let mid = (*l + *r) * 0.5;
                *l = -mid;
                *r = mid;
            }
            _ => {}
        }
    }
}

pub(super) fn quantize(input: f64, bins: &mut [f64; 13], shape: &mut f64, scale: f64) -> f32 {
    let dry = input * scale;
    let sample = dry - *shape;
    let mut cut = false;
    let mut evaluate = |value: f64| {
        let mut benford = value;
        while benford >= 1.0 {
            benford /= 10.0;
        }
        while benford < 1.0 && benford > 0.0000001 {
            benford *= 10.0;
        }
        let bin = benford.floor() as usize;
        if bin > 0 && bin < 10 {
            bins[bin] += 1.0;
            if bins[bin] > 982.0 {
                cut = true;
            }
            let mut total = 0.0;
            for (i, target) in [301.0, 176.0, 125.0, 97.0, 79.0, 67.0, 58.0, 51.0, 46.0]
                .into_iter()
                .enumerate()
            {
                total += target - bins[i + 1];
            }
            bins[bin] -= 1.0;
            (bin, total)
        } else {
            (10, 0.0)
        }
    };
    let (a, total_a) = evaluate(sample.floor());
    let (b, total_b) = evaluate(sample.ceil());
    let output = if total_a < total_b {
        bins[a] += 1.0;
        sample.floor()
    } else {
        bins[b] += 1.0;
        (sample + 1.0).floor()
    };
    if cut {
        for bin in &mut bins[1..=10] {
            *bin *= 0.99;
        }
    }
    *shape += output - dry;
    if *shape > sample.abs() {
        *shape = sample.abs();
    }
    if *shape < -sample.abs() {
        *shape = -sample.abs();
    }
    (output / scale).clamp(-1.0, 1.0) as f32
}
