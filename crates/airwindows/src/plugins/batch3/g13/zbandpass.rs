//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ZBandpass, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Input",
        label: "",
        default: 0.1,
    },
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Poles",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "ZBandpass",
    upstream_name: "ZBandpass",
    description: "ZBandpass is a bandpass made to sound and act like the Emu e6400 Ultra bandpass.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ZBandpass::new(sample_rate, seed))
}
struct ZBandpass {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    biquad: [[f64; 15]; 6],
    iir: [f64; 2],
    fpd_l: u32,
    fpd_r: u32,
}
impl ZBandpass {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.1, 0.5, 1.0, 0.5],
            sample_rate,
            seed,
            biquad: [[0.0; 15]; 6],
            iir: [0.0; 2],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn filter(q: &mut [f64; 15], x: f64, i: usize, clipped: bool) -> f64 {
    let mut out = q[2] * x + q[3] * q[i] + q[4] * q[i + 1] - q[5] * q[i + 2] - q[6] * q[i + 3];
    q[i + 1] = q[i];
    q[i] = x;
    q[i + 3] = q[i + 2];
    if clipped {
        out = out.clamp(-1.0, 1.0);
    }
    q[i + 2] = out;
    out
}
impl AirwindowsEffect for ZBandpass {
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
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let frequency = ((b as f64).powf(4.0) * 14300.0 / self.sample_rate as f64) + 0.00079;
        let k = (std::f64::consts::PI * frequency).tan();
        let norm = 1.0 / (1.0 + k / 0.314 + k * k);
        let coeff = [
            frequency,
            0.314,
            k / 0.314 * norm,
            0.0,
            -(k / 0.314 * norm),
            2.0 * (k * k - 1.0) * norm,
            (1.0 - k / 0.314 + k * k) * norm,
        ];
        for q in &mut self.biquad[..4] {
            q[..7].copy_from_slice(&coeff);
        }
        let freq = 15500.0 / self.sample_rate as f64;
        let k = (std::f64::consts::PI * freq).tan();
        let norm = 1.0 / (1.0 + k / 0.935 + k * k);
        let coeff = [
            freq,
            0.935,
            k * k * norm,
            2.0 * (k * k * norm),
            k * k * norm,
            2.0 * (k * k - 1.0) * norm,
            (1.0 - k / 0.935 + k * k) * norm,
        ];
        for q in &mut self.biquad[4..] {
            q[..7].copy_from_slice(&coeff);
        }
        let mut in_trim = a as f64 * 10.0;
        in_trim *= in_trim;
        in_trim *= in_trim;
        let out_pad = c as f64 * 10.0;
        let iir_amount = 0.00069 / (self.sample_rate as f64 / 44100.0);
        let trim = 0.1 + 3.712 * frequency;
        let clip_factor = 1.0 - ((1.0 - b as f64) * 0.304);
        let mut wet = [1.0, 1.0, 1.0, (d as f64).powf(2.0) * 4.0];
        if wet[3] < 1.0 {
            wet = [wet[3], 0.0, 0.0, 0.0];
        } else if wet[3] < 2.0 {
            wet = [1.0, wet[3] - 1.0, 0.0, 0.0];
        } else if wet[3] < 3.0 {
            wet = [1.0, 1.0, wet[3] - 2.0, 0.0];
        } else {
            wet[3] -= 3.0;
        }
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            if input[0].abs() < 1.18e-23 {
                input[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if input[1].abs() < 1.18e-23 {
                input[1] = self.fpd_r as f64 * 1.18e-17;
            }
            let dry = input;
            for ch in 0..2 {
                let i = 7 + ch * 4;
                let mut x = input[ch];
                if in_trim != 1.0 {
                    x *= in_trim;
                }
                x = x.clamp(-1.0, 1.0);
                x *= trim;
                x = filter(&mut self.biquad[0], x, i, true);
                for (idx, amount) in wet[1..].iter().enumerate() {
                    if *amount > 0.0 {
                        let old = x;
                        x /= clip_factor;
                        x = filter(&mut self.biquad[idx + 1], x, i, true) * amount
                            + old * (1.0 - amount);
                    }
                }
                x /= clip_factor;
                if self.iir[ch].abs() < 1.18e-37 {
                    self.iir[ch] = 0.0;
                }
                self.iir[ch] = self.iir[ch] * (1.0 - iir_amount) + x * iir_amount;
                x -= self.iir[ch];
                x = filter(&mut self.biquad[4], x, i, false);
                x = x.clamp(-1.0, 1.0);
                x -= x * x * x * x * x * 0.1768;
                x = filter(&mut self.biquad[5], x, i, false);
                if out_pad != 1.0 {
                    x *= out_pad;
                }
                if wet[0] != 1.0 {
                    x = x * wet[0] + dry[ch] * (1.0 - wet[0]);
                }
                input[ch] = x;
            }
            *out_l = fpd_dither(input[0], &mut self.fpd_l) as f32;
            *out_r = fpd_dither(input[1], &mut self.fpd_r) as f32;
        }
    }
}
