//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Floor, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Floor",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Drive",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "Floor",
    upstream_name: "Floor",
    description: "Floor is fake bottom octave for fun and profit!",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Floor::new(rate, seed))
}
struct Floor {
    params: [f32; 3],
    rate: f32,
    seed: u32,
    states: [[f64; 5]; 2],
    fpd: [u32; 2],
}
impl Floor {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 0.0, 1.0],
            rate,
            seed,
            states: [[0.0; 5]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
// Preserve the two distinct constants from the upstream float path.
#[allow(clippy::approx_constant)]
const LOWS_FACTOR: f64 = 1.5707963267949;
#[allow(clippy::approx_constant)]
const DRIVE_FACTOR: f64 = 1.57079633;
fn channel(
    input: f32,
    states: &mut [f64; 5],
    fpd: &mut u32,
    iir: f64,
    gain: f64,
    density: f64,
    wet: f64,
) -> f32 {
    let mut x = input as f64;
    if x.abs() < 1.18e-23 {
        x = *fpd as f64 * 1.18e-17;
    }
    let dry = x;
    let tight = -1.0 / 3.0;
    for state in states {
        let offset = ((1.0 + tight) + ((1.0 - x.abs()) * tight)).clamp(0.0, 1.0);
        *state = (*state * (1.0 - offset * iir)) + (x * (offset * iir));
        let mut lows = *state;
        x -= lows;
        let temp = lows;
        if lows < 0.0 {
            lows = -(-lows * LOWS_FACTOR).sin();
        }
        if lows > 0.0 {
            lows = (lows * LOWS_FACTOR).sin();
        }
        lows -= temp;
        x += lows;
        x *= gain;
    }
    x = x.clamp(-1.0, 1.0);
    let mut bridge = x.abs() * DRIVE_FACTOR;
    bridge = bridge.sin() * DRIVE_FACTOR;
    bridge = (x.abs() * (1.0 - density)) + (bridge * density);
    bridge = bridge.sin();
    if x > 0.0 {
        x = (x * (1.0 - density)) + (bridge * density);
    } else {
        x = (x * (1.0 - density)) - (bridge * density);
    }
    if wet != 1.0 {
        x = (x * wet) + (dry * (1.0 - wet));
    }
    fpd_dither(x, fpd) as f32
}
impl AirwindowsEffect for Floor {
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
            .map_or_else(String::new, |p| float2string(*p))
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c] = self.params;
        let setting = (a as f64).powf(2.0);
        let mut iir = (setting / 4.0) / (1.0 / 44100.0 * self.rate as f64);
        iir += iir;
        iir = iir.clamp(0.0, 1.0);
        let gain = 1.0 + setting / 4.0;
        for (((&l, &r), left), right) in il.iter().zip(ir).zip(ol).zip(or) {
            *left = channel(
                l,
                &mut self.states[0],
                &mut self.fpd[0],
                iir,
                gain,
                b as f64,
                c as f64,
            );
            *right = channel(
                r,
                &mut self.states[1],
                &mut self.fpd[1],
                iir,
                gain,
                b as f64,
                c as f64,
            );
        }
    }
}
