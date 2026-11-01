//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ChorusEnsemble/ChorusEnsemble.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Speed",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Range",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.8,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "ChorusEnsemble",
    upstream_name: "ChorusEnsemble",
    description: "ChorusEnsemble is a more complex, multi-tap mono chorus.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ChorusEnsemble::new(rate, seed))
}
struct ChorusEnsemble {
    params: [f32; 3],
    rate: f32,
    seed: u32,
    delay: [Box<[f32]>; 2],
    sweep: f64,
    count: i32,
    prev: [f64; 2],
    even: [f64; 2],
    odd: [f64; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl ChorusEnsemble {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.5, 0.8],
            rate,
            seed,
            delay: [
                vec![0.0; 16386].into_boxed_slice(),
                vec![0.0; 16386].into_boxed_slice(),
            ],
            sweep: std::f64::consts::PI / 2.0,
            count: 0,
            prev: [0.0; 2],
            even: [0.0; 2],
            odd: [0.0; 2],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for ChorusEnsemble {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(slot), Some(value)) = (self.params.get_mut(index), sanitize_param(value)) {
            *slot = value;
        }
    }
    fn param_display(&self, index: usize) -> String {
        self.params
            .get(index)
            .map_or_else(String::new, |&x| float2string(x))
    }
    fn reset(&mut self) {
        for d in &mut self.delay {
            d.fill(0.0);
        }
        self.sweep = std::f64::consts::PI / 2.0;
        self.count = 0;
        self.prev = [0.0; 2];
        self.even = [0.0; 2];
        self.odd = [0.0; 2];
        self.flip = true;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        const LIMIT: i32 = (16386.0 * 0.499) as i32;
        let scale = (1.0 / 44100.0) * self.rate as f64;
        let speed = (self.params[0] as f64).powf(3.0) * 0.001 * scale;
        let range = (self.params[1] as f64).powf(3.0) * LIMIT as f64 * 0.12;
        let wet = self.params[2] as f64;
        let modulation = range * wet;
        for (((&l, &r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            let mut s = [l as f64, r as f64];
            for (c, sample) in s.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[c] as f64 * 1.18e-17;
                }
            }
            let dry = s;
            for (c, sample) in s.iter_mut().enumerate() {
                let mut factor = self.prev[c] - *sample;
                if self.flip {
                    self.even[c] += factor;
                    self.odd[c] -= factor;
                    factor = self.even[c];
                } else {
                    self.odd[c] += factor;
                    self.even[c] -= factor;
                    factor = self.odd[c];
                }
                self.odd[c] = (self.odd[c] - ((self.odd[c] - self.even[c]) / 256.0)) / 1.0001;
                self.even[c] = (self.even[c] - ((self.even[c] - self.odd[c]) / 256.0)) / 1.0001;
                self.prev[c] = *sample;
                *sample += factor * wet;
            }
            if self.count < 1 || self.count > LIMIT {
                self.count = LIMIT;
            }
            let count = self.count as usize;
            for (c, &sample) in s.iter().enumerate() {
                self.delay[c][count] = sample as f32;
                self.delay[c][count + LIMIT as usize] = self.delay[c][count];
            }
            self.count -= 1;
            let mut result = [0.0; 2];
            for tap in 0..4 {
                let offset =
                    range * (tap + 1) as f64 + modulation * (self.sweep + tap as f64).sin();
                let count = (self.count + offset.floor() as i32) as usize;
                for (c, sample) in result.iter_mut().enumerate() {
                    let d = &self.delay[c];
                    *sample += d[count] as f64 * (1.0 - (offset - offset.floor()));
                    *sample += d[count + 1] as f64;
                    *sample += d[count + 2] as f64 * (offset - offset.floor());
                    *sample -=
                        (((d[count] - d[count + 1]) - (d[count + 1] - d[count + 2])) / 50.0) as f64;
                }
            }
            s = [result[0] * 0.125, result[1] * 0.125];
            self.sweep += speed;
            if self.sweep > std::f64::consts::PI * 2.0 {
                self.sweep -= std::f64::consts::PI * 2.0;
            }
            if wet != 1.0 {
                for c in 0..2 {
                    s[c] = (s[c] * wet) + (dry[c] * (1.0 - wet));
                }
            }
            self.flip = !self.flip;
            *ol = fpd_dither(s[0], &mut self.fpd[0]) as f32;
            *or = fpd_dither(s[1], &mut self.fpd[1]) as f32;
        }
    }
}
