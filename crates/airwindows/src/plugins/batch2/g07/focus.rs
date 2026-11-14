//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Focus, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Boost",
        label: "dB",
        default: 0.0,
    },
    ParamInfo {
        name: "Focus",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Mode",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Focus",
    upstream_name: "Focus",
    description: "Focus brings out clarity by distorting. Aggressive, subtle, flexible.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Focus::new(rate, seed))
}
struct Focus {
    params: [f32; 5],
    rate: f32,
    seed: u32,
    state: [[f64; 2]; 2],
    fpd: [u32; 2],
}
impl Focus {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 0.5, 0.5, 1.0, 1.0],
            rate,
            seed,
            state: [[0.0; 2]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
// Preserve the upstream saturation threshold exactly.
#[allow(clippy::approx_constant)]
const DENSITY_LIMIT: f64 = 1.570796326794897;
fn shape(mut x: f64, mode: usize) -> f64 {
    match mode {
        0 => x.clamp(-DENSITY_LIMIT, DENSITY_LIMIT).sin(),
        1 => {
            x = x.clamp(-1.0, 1.0);
            x -= x * (x.abs() * 0.6) * (x.abs() * 0.6);
            x * 1.6
        }
        2 => {
            x = x.clamp(-1.2533141373155, 1.2533141373155);
            (x * x.abs()).sin() / if x.abs() == 0.0 { 1.0 } else { x.abs() }
        }
        3 => {
            let m = x.abs().powf(0.25);
            if m > 0.0 {
                (x * m * std::f64::consts::PI * 0.5).sin() / m * 0.987654321
            } else {
                x
            }
        }
        _ => {
            let m = x.abs().powf(4.0);
            if m > 0.0 {
                (x * m).sin() / m * 1.1654321
            } else {
                x
            }
        }
    }
}
impl AirwindowsEffect for Focus {
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
        match i {
            0 => float2string((self.params[0] as f64 * 12.0) as f32),
            2 => ["Density", "Drive", "Spiral", "Mojo", "Dyno"]
                [(self.params[2] as f64 * 4.999) as usize]
                .into(),
            1 | 3 | 4 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let boost = 10.0f64.powf((a as f64 * 12.0) / 20.0);
        let resonance = ((b as f64).powf(3.0) * 2.0).powf(2.0) + 0.0001;
        let k = (std::f64::consts::PI * 3515.775 / self.rate as f64).tan();
        let norm = 1.0 / (1.0 + k / resonance + k * k);
        let b0 = k / resonance * norm;
        let a1 = 2.0 * (k * k - 1.0) * norm;
        let a2 = (1.0 - k / resonance + k * k) * norm;
        let mode = (c as f64 * 4.999) as usize;
        for (((&l, &r), left), right) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut samples = [l as f64, r as f64];
            for (i, x) in samples.iter_mut().enumerate() {
                if x.abs() < 1.18e-23 {
                    *x = self.fpd[i] as f64 * 1.18e-17;
                }
                let dry = *x;
                let encoded = x.sin();
                let filtered = encoded * b0 + self.state[i][0];
                self.state[i][0] = -(filtered * a1) + self.state[i][1];
                self.state[i][1] = encoded * (-b0) - (filtered * a2);
                let band = filtered.clamp(-1.0, 1.0).asin();
                let ground = dry - band;
                *x = shape(band * boost, mode);
                if d != 1.0 {
                    *x *= d as f64;
                }
                *x += ground;
                if e != 1.0 {
                    *x = (*x * e as f64) + (dry * (1.0 - e as f64));
                }
                *x = fpd_dither(*x, &mut self.fpd[i]);
            }
            *left = samples[0] as f32;
            *right = samples[1] as f32;
        }
    }
}
