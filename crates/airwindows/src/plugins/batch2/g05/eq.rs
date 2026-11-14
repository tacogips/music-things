//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/EQ, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Treble",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Mid",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Bass",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Lowpass",
        label: "Khz",
        default: 1.0,
    },
    ParamInfo {
        name: "TrebFrq",
        label: "Khz",
        default: 0.4,
    },
    ParamInfo {
        name: "BassFrq",
        label: "hz",
        default: 0.4,
    },
    ParamInfo {
        name: "Hipass",
        label: "hz",
        default: 0.0,
    },
    ParamInfo {
        name: "OutGain",
        label: "dB",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "EQ",
    upstream_name: "EQ",
    description: "EQ is just the EQ parts of CStrip.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Eq::new(rate, seed))
}
#[derive(Default)]
struct Channel {
    last: f64,
    last2: f64,
    triplet: [f64; 3],
    high: [f64; 6],
    low: [f64; 6],
    hp: [[f64; 4]; 2],
    hp_tail: [f64; 2],
    lp: [[f64; 5]; 2],
    lp_tail: f64,
}
struct Eq {
    params: [f32; 8],
    rate: f32,
    seed: u32,
    state: [Channel; 2],
    flip: bool,
    phase: usize,
    fpd: [u32; 2],
}
impl Eq {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.5, 1.0, 0.4, 0.4, 0.0, 0.5],
            rate,
            seed,
            state: [Channel::default(), Channel::default()],
            flip: false,
            phase: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
struct Coeff {
    density: [f64; 3],
    high: f64,
    bass: f64,
    lowpass: f64,
    highpass: f64,
    engage_eq: bool,
    engage_hp: bool,
    engage_lp: bool,
    output: f64,
}
fn filter(state: &mut f64, sample: f64, amount: f64) {
    *state = *state * (1.0 - amount) + sample * amount;
}
#[allow(clippy::approx_constant)] // Preserve the upstream decimal coefficient.
fn drive(mut sample: f64, density: f64) -> f64 {
    sample *= density + 1.0;
    let mut bridge = (sample.abs() * 1.57079633).min(1.57079633);
    bridge = if density > 0.0 {
        bridge.sin()
    } else {
        1.0 - bridge.cos()
    };
    let out = density.abs();
    if sample > 0.0 {
        sample * (1.0 - out) + bridge * out
    } else {
        sample * (1.0 - out) - bridge * out
    }
}
fn channel(mut input: f64, state: &mut Channel, flip: bool, phase: usize, c: &Coeff) -> f64 {
    state.last2 = state.last;
    state.last = input;
    let bank = usize::from(!flip);
    if c.engage_hp {
        for x in &mut state.hp[bank] {
            filter(x, input, c.highpass);
            input -= *x;
        }
        for x in &mut state.hp_tail {
            filter(x, input, c.highpass);
            input -= *x;
        }
    }
    if c.engage_eq {
        let i = phase - 1;
        let factor = state.last2 - input;
        state.triplet[i] += factor;
        state.triplet[(i + 2) % 3] -= factor;
        let triplet = state.triplet[i] * -c.density[0];
        let stage = i + 2;
        filter(&mut state.high[stage], input, c.high);
        let mut high = input - state.high[stage];
        filter(&mut state.low[stage], input, c.bass);
        let mut bass = state.low[stage];
        for v in &mut state.triplet {
            *v /= 2.0;
        }
        high += triplet;
        filter(&mut state.high[bank], high, c.high);
        high -= state.high[bank];
        filter(&mut state.low[bank], bass, c.bass);
        bass = state.low[bank];
        filter(&mut state.high[5], high, c.high);
        high -= state.high[5];
        filter(&mut state.low[5], bass, c.bass);
        bass = state.low[5];
        let mid = (input - bass) - high;
        input = drive(mid, c.density[1]);
        input += drive(high, c.density[0]);
        input += drive(bass, c.density[2]);
    }
    if c.engage_lp {
        for x in &mut state.lp[bank] {
            filter(x, input, c.lowpass);
            input = *x;
        }
        filter(&mut state.lp_tail, input, c.lowpass);
        input = state.lp_tail * (1.0 - c.lowpass) + input * c.lowpass;
    }
    if c.output != 1.0 {
        input *= c.output;
    }
    input
}
impl AirwindowsEffect for Eq {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        let Some(&p) = self.params.get(i) else {
            return String::new();
        };
        let value = match i {
            0..=2 => p as f64 * 24.0 - 12.0,
            3..=4 => (p * p) as f64 * 15.0 + 1.0,
            5..=6 => (p * p) as f64 * 1570.0 + 30.0,
            _ => p as f64 * 36.0 - 18.0,
        };
        float2string(value as f32)
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let [a, b, c, d, e, f, g, h] = self.params;
        let raw = [
            a as f64 * 12.0 - 6.0,
            b as f64 * 12.0 - 6.0,
            c as f64 * 12.0 - 6.0,
        ];
        let lp = (d * d) as f64 * 15.0 + 1.0;
        let hp = (g * g) as f64 * 1570.0 + 30.0;
        let coeff = Coeff {
            density: raw.map(|v| 10.0_f64.powf(v / 20.0) - 1.0),
            high: ((e * e) as f64 * 15.0 + 1.0) * 1000.0 / self.rate as f64,
            bass: ((f * f) as f64 * 1570.0 + 30.0) * 10.0 / self.rate as f64,
            lowpass: (lp * 0.0188 + 0.7).min(1.0),
            highpass: hp / self.rate as f64,
            engage_eq: raw.iter().any(|&v| v != 0.0),
            engage_hp: hp > 30.01,
            engage_lp: lp < 15.99,
            output: 10.0_f64.powf((h as f64 * 36.0 - 18.0) / 20.0),
        };
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut samples = [l as f64, r as f64];
            for (n, value) in samples.iter_mut().enumerate() {
                if value.abs() < 1.18e-23 {
                    *value = self.fpd[n] as f64 * 1.18e-17;
                }
            }
            self.flip = !self.flip;
            self.phase = self.phase % 3 + 1;
            *out_l = fpd_dither(
                channel(
                    samples[0],
                    &mut self.state[0],
                    self.flip,
                    self.phase,
                    &coeff,
                ),
                &mut self.fpd[0],
            ) as f32;
            *out_r = fpd_dither(
                channel(
                    samples[1],
                    &mut self.state[1],
                    self.flip,
                    self.phase,
                    &coeff,
                ),
                &mut self.fpd[1],
            ) as f32;
        }
    }
}
