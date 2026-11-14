//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Srsly2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Center",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Space",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Level",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Q",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Srsly2",
    upstream_name: "Srsly2",
    description: "Srsly2 is a revisit of Srsly, to make the stereo widening more extreme.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Srsly2::new(sample_rate, seed))
}
struct Srsly2 {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    biquads: [[f64; 11]; 9],
    fpd: [u32; 2],
}
impl Srsly2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.5, 1.0, 0.5, 1.0],
            sample_rate,
            seed,
            biquads: [[0.0; 11]; 9],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn filter(b: &mut [f64; 11], x: f64, pos: usize) -> f64 {
    let t = x * b[2] + b[pos];
    b[pos] = (-t * b[5]) + b[pos + 1];
    b[pos + 1] = x * b[4] - t * b[6];
    t
}
impl AirwindowsEffect for Srsly2 {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(v)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        self.params
            .get(i)
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let rate = (self.sample_rate as f64).max(22000.0);
        let fm = 15.0 - (a as f64 * 10.0);
        let fs = 21.0 - (b as f64 * 15.0);
        let q = d as f64 + 0.25;
        for (i, biq) in self.biquads.iter_mut().enumerate() {
            let hz = [
                2000.0, 7000.0, 10000.0, 3000.0, 7000.0, 3000.0, 7000.0, 3000.0, 5000.0,
            ][i];
            biq[0] = hz / rate;
            biq[1] = if i == 0 {
                fm * 0.25 * q
            } else if (3..=6).contains(&i) {
                fs * q
            } else {
                fm * q
            };
            let k = (std::f64::consts::PI * biq[0]).tan();
            let norm = 1.0 / (1.0 + k / biq[1] + k * k);
            biq[2] = k / biq[1] * norm;
            biq[4] = -biq[2];
            biq[5] = 2.0 * (k * k - 1.0) * norm;
            biq[6] = (1.0 - k / biq[1] + k * k) * norm;
        }
        let dm = (a as f64).powf(2.0) * 2.0;
        let ds = (b as f64).powf(2.0) * 2.0;
        let level = c as f64;
        let wet = e as f64;
        let mut gain_m = a as f64 * 2.0;
        let gain_s = b as f64 * 2.0;
        if gain_s > 1.0 {
            gain_m /= gain_s;
        }
        if gain_m > 1.0 {
            gain_m = 1.0;
        }
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd[0] as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd[1] as f64 * 1.18e-17;
            }
            let dl = l;
            let dr = r;
            l = l.sin();
            r = r.sin();
            let mut mid = l + r;
            let rawmid = mid * 0.5;
            let mut side = l - r;
            let boost = side * ds;
            let m2 = filter(&mut self.biquads[0], mid, 7);
            let m7 = -filter(&mut self.biquads[1], mid, 7) * 2.0;
            let m10 = -filter(&mut self.biquads[2], mid, 7) * 2.0;
            let s3 = filter(&mut self.biquads[7], side, 7) * 2.0;
            let s5 = -filter(&mut self.biquads[8], side, 7) * 5.0;
            mid = (m2 + m7 + m10) * dm;
            side = (s3 + s5 + boost) * ds;
            let msl = (mid + side) / 2.0;
            let msr = (mid - side) / 2.0;
            let iso_l = l - rawmid;
            let iso_r = r - rawmid;
            let l3 = filter(&mut self.biquads[3], iso_l, 7);
            let r3 = filter(&mut self.biquads[5], iso_r, 9);
            let l7 = filter(&mut self.biquads[4], iso_l, 7) * 3.0;
            let r7 = filter(&mut self.biquads[6], iso_r, 9) * 3.0;
            let processing_l = msl + ((l3 + l7) * ds);
            let processing_r = msr + ((r3 + r7) * ds);
            mid = l + r;
            side = l - r;
            mid *= gain_m;
            side *= gain_s;
            side = side.clamp(-1.57079633, 1.57079633).sin();
            side *= gain_s;
            l = ((mid + side) / 2.0) + processing_l;
            r = ((mid - side) / 2.0) + processing_r;
            if level < 1.0 {
                l *= level;
                r *= level;
            }
            l = l.clamp(-1.0, 1.0).asin();
            r = r.clamp(-1.0, 1.0).asin();
            if wet < 1.0 {
                l = (l * wet) + (dl * (1.0 - wet));
                r = (r * wet) + (dr * (1.0 - wet));
            }
            *outl = fpd_dither(l, &mut self.fpd[0]) as f32;
            *outr = fpd_dither(r, &mut self.fpd[1]) as f32;
        }
    }
}
