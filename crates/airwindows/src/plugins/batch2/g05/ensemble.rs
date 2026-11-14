//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Ensemble, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 1.4901161e-8.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Ensemble",
        label: "vox",
        default: 0.5,
    },
    ParamInfo {
        name: "Fullness",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Brighten",
        label: " ",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Ensemble",
    upstream_name: "Ensemble",
    description: "Ensemble is a weird flangey little modulation effect.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Ensemble::new(rate, seed))
}
#[derive(Default)]
struct Air {
    prev: f64,
    even: f64,
    odd: f64,
}
impl Air {
    fn apply(&mut self, input: f64, flip: bool, bright: f64) -> f64 {
        let mut factor = self.prev - input;
        if flip {
            self.even += factor;
            self.odd -= factor;
            factor = self.even;
        } else {
            self.odd += factor;
            self.even -= factor;
            factor = self.odd;
        }
        self.odd = (self.odd - ((self.odd - self.even) / 256.0)) / 1.0001;
        self.even = (self.even - ((self.even - self.odd) / 256.0)) / 1.0001;
        self.prev = input;
        input + factor * bright
    }
}
struct Ensemble {
    params: [f32; 4],
    rate: f32,
    seed: u32,
    delay: [Box<[f32]>; 2],
    sweep: [f64; 49],
    count: i32,
    air: [Air; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl Ensemble {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.0, 1.0, 1.0],
            rate,
            seed,
            delay: [
                vec![0.0; 65540].into_boxed_slice(),
                vec![0.0; 65540].into_boxed_slice(),
            ],
            sweep: [std::f64::consts::PI / 2.0; 49],
            count: 0,
            air: [Air::default(), Air::default()],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Ensemble {
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
        match i {
            0 => float2string((self.params[0] as f64 * 46.0 + 2.9).floor() as f32),
            1..=3 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        for d in &mut self.delay {
            d.fill(0.0);
        }
        self.sweep.fill(std::f64::consts::PI / 2.0);
        self.count = 0;
        self.air = [Air::default(), Air::default()];
        self.flip = true;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let spd = (0.4 + (b / 12.0) as f64).powf(10.0) * (self.rate as f64 / 44100.0);
        let depth = 0.002 / spd;
        let taps = (a as f64 * 46.0 + 2.9).floor() as usize;
        let hapi = std::f64::consts::PI / taps as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut dry = [l as f64, r as f64];
            for (n, sample) in dry.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[n] as f64 * 1.18e-17;
                }
            }
            let mut temp = [
                self.air[0].apply(dry[0], self.flip, c as f64),
                self.air[1].apply(dry[1], self.flip, c as f64),
            ];
            self.flip = !self.flip;
            if self.count < 1 || self.count > 32767 {
                self.count = 32767;
            }
            let ix = self.count as usize;
            for (buf, &value) in self.delay.iter_mut().zip(temp.iter()) {
                buf[ix] = value as f32;
                buf[ix + 32767] = value as f32;
            }
            for voice in 1..=taps {
                let offset = depth * voice as f64
                    + depth * (self.sweep[voice] + hapi * (voice - 1) as f64).sin();
                let frac = offset - offset.floor();
                let k = ix + offset.floor() as usize;
                for (buf, value) in self.delay.iter().zip(temp.iter_mut()) {
                    *value += buf[k] as f64 * (1.0 - frac);
                    *value += buf[k + 1] as f64;
                    *value += buf[k + 2] as f64 * frac;
                    *value -= ((buf[k] - buf[k + 1]) - (buf[k + 1] - buf[k + 2])) as f64 / 50.0;
                }
                self.sweep[voice] += spd / (1.0 + voice as f64 / taps as f64);
                if self.sweep[voice] > std::f64::consts::TAU {
                    self.sweep[voice] -= std::f64::consts::TAU;
                }
            }
            self.count -= 1;
            for n in 0..2 {
                let mut value = temp[n] / (4.0 * (taps as f64).sqrt());
                if d != 1.0 {
                    value = value * d as f64 + dry[n] * (1.0 - d as f64);
                }
                let result = fpd_dither(value, &mut self.fpd[n]) as f32;
                if n == 0 {
                    *out_l = result;
                } else {
                    *out_r = result;
                }
            }
        }
    }
}
