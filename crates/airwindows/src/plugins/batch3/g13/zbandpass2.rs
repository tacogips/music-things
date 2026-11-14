//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ZBandpass2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
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
    name: "ZBandpass2",
    upstream_name: "ZBandpass2",
    description:
        "ZBandpass2 acts more like the Emu e6400 Ultra bandpass in motion, with control smoothing.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ZBandpass2::new(sample_rate, seed))
}
struct ZBandpass2 {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    states: [[[f64; 2]; 2]; 6],
    iir: [f64; 2],
    coeff: [f64; 5],
    trims: [f64; 3],
    fpd_l: u32,
    fpd_r: u32,
}
impl ZBandpass2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.1, 0.5, 1.0, 0.5],
            sample_rate,
            seed,
            states: [[[0.0; 2]; 2]; 6],
            iir: [0.0; 2],
            coeff: [0.0; 5],
            trims: [0.1, 1.0, 0.5],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
fn filter(s: &mut [f64; 2], x: f64, c: &[f64; 5], clip: bool) -> f64 {
    let mut y = x * c[0] + s[0];
    if clip {
        y = y.clamp(-1.0, 1.0);
    }
    s[0] = x * c[1] - y * c[3] + s[1];
    s[1] = x * c[2] - y * c[4];
    y
}
impl AirwindowsEffect for ZBandpass2 {
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
        let next = [
            k / 0.314 * norm,
            0.0,
            -(k / 0.314 * norm),
            2.0 * (k * k - 1.0) * norm,
            (1.0 - k / 0.314 + k * k) * norm,
        ];
        let old = self.coeff;
        self.coeff = next;
        let mut in_trim = a as f64 * 10.0;
        in_trim *= in_trim;
        in_trim *= in_trim;
        let trims = [in_trim, c as f64 * 10.0, (d as f64).powf(2.0)];
        let old_trims = self.trims;
        self.trims = trims;
        let freq = 15500.0 / self.sample_rate as f64;
        let k = (std::f64::consts::PI * freq).tan();
        let norm = 1.0 / (1.0 + k / 0.935 + k * k);
        let a0 = k * k * norm;
        let fixed = [
            a0,
            2.0 * a0,
            a0,
            2.0 * (k * k - 1.0) * norm,
            (1.0 - k / 0.935 + k * k) * norm,
        ];
        let iir_amount = 0.00069 / (self.sample_rate as f64 / 44100.0);
        let trim = 0.1 + 3.712 * frequency;
        let clip_factor = 1.0 - ((1.0 - b as f64) * 0.304);
        let frames = il.len().min(ir.len()).min(ol.len()).min(orr.len());
        for frame in 0..frames {
            let pos = (frames - 1 - frame) as f64 / frames as f64;
            let mut cs = [0.0; 5];
            for j in 0..5 {
                cs[j] = old[j] * pos + next[j] * (1.0 - pos);
            }
            let int = old_trims[0] * pos + trims[0] * (1.0 - pos);
            let out = old_trims[1] * pos + trims[1] * (1.0 - pos);
            let wet = old_trims[2] * pos + trims[2] * (1.0 - pos);
            let mut w = [1.0, 1.0, 1.0, wet * 4.0];
            if w[3] < 1.0 {
                w = [w[3], 0.0, 0.0, 0.0];
            } else if w[3] < 2.0 {
                w = [1.0, w[3] - 1.0, 0.0, 0.0];
            } else if w[3] < 3.0 {
                w = [1.0, 1.0, w[3] - 2.0, 0.0];
            } else {
                w[3] -= 3.0;
            }
            let mut input = [il[frame] as f64, ir[frame] as f64];
            if input[0].abs() < 1.18e-23 {
                input[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if input[1].abs() < 1.18e-23 {
                input[1] = self.fpd_r as f64 * 1.18e-17;
            }
            let dry = input;
            for ch in 0..2 {
                let mut x = input[ch];
                if int != 1.0 {
                    x *= int;
                }
                x = x.clamp(-1.0, 1.0);
                x *= trim;
                x /= clip_factor;
                x = filter(&mut self.states[0][ch], x, &cs, true);
                for (idx, amount) in w[1..].iter().enumerate() {
                    if *amount > 0.0 {
                        let prior = x;
                        x /= clip_factor;
                        x = filter(&mut self.states[idx + 1][ch], x, &cs, true) * amount
                            + prior * (1.0 - amount);
                    }
                }
                x /= clip_factor;
                if self.iir[ch].abs() < 1.18e-37 {
                    self.iir[ch] = 0.0;
                }
                self.iir[ch] = self.iir[ch] * (1.0 - iir_amount) + x * iir_amount;
                x -= self.iir[ch];
                x = filter(&mut self.states[4][ch], x, &fixed, false);
                x = x.clamp(-1.0, 1.0);
                x -= x * x * x * x * x * 0.1768;
                x = filter(&mut self.states[5][ch], x, &fixed, false);
                if out != 1.0 {
                    x *= out;
                }
                if w[0] != 1.0 {
                    x = x * w[0] + dry[ch] * (1.0 - w[0]);
                }
                input[ch] = x;
            }
            ol[frame] = fpd_dither(input[0], &mut self.fpd_l) as f32;
            orr[frame] = fpd_dither(input[1], &mut self.fpd_r) as f32;
        }
    }
}
