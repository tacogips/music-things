//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Biquad, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Type",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Q",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Inv/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Biquad",
    upstream_name: "Biquad",
    description: "Biquad is the Airwindows implementation of a biquad filter.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Biquad::new(sample_rate, seed))
}
struct Biquad {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    biquad: [f64; 11],
    fpd_l: u32,
    fpd_r: u32,
}
impl Biquad {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 0.5, 0.5, 1.0],
            sample_rate,
            seed,
            biquad: [0.0; 11],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Biquad {
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
        let [a, b, c, d] = self.params;
        let x = match i {
            0 => ((a as f64 * 3.999 + 0.00001).ceil()) as f32,
            1 => ((b * b * b) as f64 * 0.9999 + 0.0001) as f32,
            2 => ((c * c * c) as f64 * 29.99 + 0.01) as f32,
            3 => (d as f64 * 2.0 - 1.0) as f32,
            _ => return String::new(),
        };
        float2string(x)
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let typ = (a as f64 * 3.999 + 0.00001).ceil() as i32;
        let q = &mut self.biquad;
        q[0] = (((b * b * b) as f64 * 0.9999) + 0.0001) * 0.499;
        if q[0] < 0.0001 {
            q[0] = 0.0001;
        }
        q[1] = ((c * c * c) as f64 * 29.99) + 0.01;
        if q[1] < 0.0001 {
            q[1] = 0.0001;
        }
        let wet = d as f64 * 2.0 - 1.0;
        let k = (std::f64::consts::PI * q[0]).tan();
        let norm = 1.0 / (1.0 + k / q[1] + k * k);
        match typ {
            1 => {
                q[2] = k * k * norm;
                q[3] = 2.0 * q[2];
                q[4] = q[2];
            }
            2 => {
                q[2] = norm;
                q[3] = -2.0 * q[2];
                q[4] = q[2];
            }
            3 => {
                q[2] = k / q[1] * norm;
                q[3] = 0.0;
                q[4] = -q[2];
            }
            4 => {
                q[2] = (1.0 + k * k) * norm;
                q[3] = 2.0 * (k * k - 1.0) * norm;
                q[4] = q[2];
            }
            _ => {}
        }
        if (1..=4).contains(&typ) {
            q[5] = 2.0 * (k * k - 1.0) * norm;
            q[6] = (1.0 - k / q[1] + k * k) * norm;
        }
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dryl = l;
            let dryr = r;
            l = l.sin();
            r = r.sin();
            let tl = l * q[2] + q[7];
            q[7] = l * q[3] - tl * q[5] + q[8];
            q[8] = l * q[4] - tl * q[6];
            let tr = r * q[2] + q[9];
            q[9] = r * q[3] - tr * q[5] + q[10];
            q[10] = r * q[4] - tr * q[6];
            l = tl.clamp(-1.0, 1.0).asin();
            r = tr.clamp(-1.0, 1.0).asin();
            if wet < 1.0 {
                l = l * wet + dryl * (1.0 - wet.abs());
                r = r * wet + dryr * (1.0 - wet.abs());
            }
            *outl = fpd_dither(l, &mut self.fpd_l) as f32;
            *outr = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
