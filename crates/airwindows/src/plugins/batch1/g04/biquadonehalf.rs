//! BiquadOneHalf - Airwindows MIT license.
//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: plugins/LinuxVST/src/BiquadOneHalf/ at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: [ParamInfo; 4] = [
    ParamInfo {
        name: "Type",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Q",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Inv/Wet",
        label: "",
        default: 0.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "BiquadOneHalf",
    upstream_name: "BiquadOneHalf",
    description: "BiquadOneHalf is an interleaved biquad filter like Biquad.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(BiquadOneHalf::new(sample_rate, seed))
}
struct BiquadOneHalf {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    a: [f64; 11],
    b: [f64; 11],
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl BiquadOneHalf {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0; 4],
            sample_rate,
            seed,
            a: [0.0; 11],
            b: [0.0; 11],
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for BiquadOneHalf {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(index), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, index: usize) -> String {
        let [a, b, c, d] = self.params;
        let v = match index {
            0 => ((a as f64 * 3.999 + 0.00001).ceil()) as f32,
            1 => ((b * b * b) as f64 * 0.9999 + 0.0001) as f32,
            2 => ((c * c * c) as f64 * 29.99 + 0.01) as f32,
            3 => (d as f64 * 2.0 - 1.0) as f32,
            _ => return String::new(),
        };
        float2string(v)
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let typ = (a as f64 * 3.999 + 0.00001).ceil() as i32;
        self.a[0] = (((b * b * b) as f64 * 0.9999) + 0.0001) * 0.499;
        if self.a[0] < 0.0001 {
            self.a[0] = 0.0001;
        }
        self.a[1] = ((c * c * c) as f64 * 29.99) + 0.01;
        if self.a[1] < 0.0001 {
            self.a[1] = 0.0001;
        }
        let wet = d as f64 * 2.0 - 1.0;
        let k = (std::f64::consts::PI * self.a[0]).tan();
        let norm = 1.0 / (1.0 + k / self.a[1] + k * k);
        match typ {
            1 => {
                self.a[2] = k * k * norm;
                self.a[3] = 2.0 * self.a[2];
                self.a[4] = self.a[2];
            }
            2 => {
                self.a[2] = norm;
                self.a[3] = -2.0 * self.a[2];
                self.a[4] = self.a[2];
            }
            3 => {
                self.a[2] = k / self.a[1] * norm;
                self.a[3] = 0.0;
                self.a[4] = -self.a[2];
            }
            4 => {
                self.a[2] = (1.0 + k * k) * norm;
                self.a[3] = 2.0 * (k * k - 1.0) * norm;
                self.a[4] = self.a[2];
            }
            _ => {}
        }
        if (1..=4).contains(&typ) {
            self.a[5] = if typ == 4 {
                self.a[3]
            } else {
                2.0 * (k * k - 1.0) * norm
            };
            self.a[6] = (1.0 - k / self.a[1] + k * k) * norm;
        }
        self.b[..7].copy_from_slice(&self.a[..7]);
        let len = in_l.len().min(in_r.len()).min(out_l.len()).min(out_r.len());
        for i in 0..len {
            let mut l = in_l[i] as f64;
            let mut r = in_r[i] as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            l = l.sin();
            r = r.sin();
            {
                let biquad = if self.flip { &mut self.a } else { &mut self.b };
                self.flip = !self.flip;
                let t = l * biquad[2] + biquad[7];
                biquad[7] = l * biquad[3] - t * biquad[5] + biquad[8];
                biquad[8] = l * biquad[4] - t * biquad[6];
                l = t;
                let t = r * biquad[2] + biquad[9];
                biquad[9] = r * biquad[3] - t * biquad[5] + biquad[10];
                biquad[10] = r * biquad[4] - t * biquad[6];
                r = t;
            }
            l = l.clamp(-1.0, 1.0).asin();
            r = r.clamp(-1.0, 1.0).asin();
            if wet < 1.0 {
                l = l * wet + dry_l * (1.0 - wet.abs());
                r = r * wet + dry_r * (1.0 - wet.abs());
            }
            out_l[i] = fpd_dither(l, &mut self.fpd_l) as f32;
            out_r[i] = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
