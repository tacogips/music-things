//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/MultiBandDistortion, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Freqcy",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "H Gain",
        label: "dB",
        default: 0.0,
    },
    ParamInfo {
        name: "L Gain",
        label: "dB",
        default: 0.0,
    },
    ParamInfo {
        name: "H Hard",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "L Hard",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "H Mtrx",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "L Mtrx",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Stable",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Output",
        label: "dB",
        default: 0.75,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "MultiBandDistortion",
    upstream_name: "MultiBandDistortion",
    description: "MultiBandDistortion is an old weird gnarly sound wrecker :)",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(MultiBand::new(rate, seed))
}
#[derive(Default)]
struct Channel {
    last: [f64; 3],
    iir: [f64; 2],
    a: f64,
    b: f64,
}
struct MultiBand {
    params: [f32; 9],
    rate: f32,
    seed: u32,
    l: Channel,
    r: Channel,
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl MultiBand {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.75],
            rate,
            seed,
            l: Channel::default(),
            r: Channel::default(),
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
struct Settings {
    iir: f64,
    gain_h: f64,
    gain_l: f64,
    threshold_h: f64,
    threshold_l: f64,
    hardness_h: f64,
    hardness_l: f64,
    out_h: f64,
    out_l: f64,
    out_d: f64,
    global: f64,
}
#[allow(clippy::approx_constant)]
fn distort(sample: f64, threshold: f64, hardness: f64) -> f64 {
    if sample.abs() > threshold {
        let bridge = ((sample.abs() - threshold) * hardness)
            .min(1.5707963267949)
            .sin()
            / hardness;
        if sample > 0.0 {
            bridge + threshold
        } else {
            -(bridge + threshold)
        }
    } else {
        sample
    }
}
fn bands(input: f64, state: &mut f64, s: &Settings) -> f64 {
    *state = (*state * (1.0 - s.iir)) + (input * s.iir);
    let low = *state;
    let high = input - *state;
    let high = distort(high * s.gain_h, s.threshold_h, s.hardness_h);
    let low = distort(low * s.gain_l, s.threshold_l, s.hardness_l);
    (low * s.out_l) + (high * s.out_h)
}
#[allow(clippy::excessive_precision)]
fn channel(input: f64, state: &mut Channel, flip: bool, s: &Settings) -> f64 {
    let half =
        (input + state.last[0] + ((-state.last[1] + state.last[2]) * 0.0414213562373095048801688))
            / 2.0;
    state.last[2] = state.last[1];
    state.last[1] = state.last[0];
    state.last[0] = input;
    let half_out = bands(half, &mut state.iir[0], s);
    let raw_out = bands(input, &mut state.iir[1], s);
    let decay = 0.915965594177219015;
    let mut c = half_out - half;
    let half_diff = if flip {
        state.a *= decay;
        state.b *= decay;
        state.a += c;
        state.b -= c;
        state.a * decay
    } else {
        state.b *= decay;
        state.a *= decay;
        state.b += c;
        state.a -= c;
        state.b * decay
    };
    c = raw_out - input;
    let diff = if flip {
        state.a *= decay;
        state.b *= decay;
        state.a += c;
        state.b -= c;
        state.a * decay
    } else {
        state.b *= decay;
        state.a *= decay;
        state.b += c;
        state.a -= c;
        state.b * decay
    };
    let mut out = input * s.out_d;
    out += diff + half_diff;
    out * s.global
}
impl AirwindowsEffect for MultiBand {
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
        if i >= 9 {
            return String::new();
        }
        let value = match i {
            1 | 2 => self.params[i] as f64 * 48.0,
            8 => (self.params[i] as f64 - 1.0) * 48.0,
            _ => self.params[i] as f64,
        };
        float2string(value as f32)
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let p = self.params.map(f64::from);
        let oh = p[5];
        let lo = p[6];
        let dry = p[7] * 0.597;
        let trim = oh + lo + dry;
        let hardness = |x: f64| {
            if x < 1.0 {
                1.0 / (1.0 - x)
            } else {
                999999999999999999999.0
            }
        };
        let s = Settings {
            iir: p[0].powf(3.0) / (self.rate as f64 / 44100.0),
            gain_h: 10.0_f64.powf((p[1] * 48.0) / 20.0),
            gain_l: 10.0_f64.powf((p[2] * 48.0) / 20.0),
            threshold_h: p[3],
            threshold_l: p[4],
            hardness_h: hardness(p[3]),
            hardness_l: hardness(p[4]),
            out_h: oh * trim,
            out_l: lo * trim,
            out_d: dry * trim,
            global: 10.0_f64.powf(((p[8] - 1.0) * 48.0) / 20.0),
        };
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            l = channel(l, &mut self.l, self.flip, &s);
            r = channel(r, &mut self.r, self.flip, &s);
            self.flip = !self.flip;
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
