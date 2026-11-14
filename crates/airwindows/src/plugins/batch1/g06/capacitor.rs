/* Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
 * Upstream: airwindows/plugins/LinuxVST/src/Capacitor/Capacitor.h
 * Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
 * Parity: Libm; measured max-abs 0.0.
 */
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
static PARAMS: [ParamInfo; 3] = [
    ParamInfo {
        name: "Lowpass",
        label: " ",
        default: 1.0,
    },
    ParamInfo {
        name: "Highpass",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: " ",
        default: 1.0,
    },
];
static INFO: PluginInfo = PluginInfo {
    name: "Capacitor",
    upstream_name: "Capacitor",
    description: "Capacitor is a lowpass/highpass filter of a new type.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Capacitor::new(rate, seed))
}
struct Capacitor {
    params: [f32; 3],
    rate: f32,
    seed: u32,
    fpd: [u32; 2],
    hp: [[f64; 6]; 2],
    lp: [[f64; 6]; 2],
    count: usize,
    amount: [f64; 3],
    last: [f64; 3],
}
impl Capacitor {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 0.0, 1.0],
            rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            hp: [[0.0; 6]; 2],
            lp: [[0.0; 6]; 2],
            count: 0,
            amount: [1.0, 0.0, 1.0],
            last: [1000.0; 3],
        }
    }
    fn sample(&mut self, x: f32, ch: usize, stages: [usize; 3]) -> f32 {
        let mut input = x as f64;
        if input.abs() < 1.18e-23 {
            input = self.fpd[ch] as f64 * 1.18e-17;
        }
        let dry_sample = input as f32;
        for i in stages {
            self.hp[ch][i] = (self.hp[ch][i] * (1.0 - self.amount[1])) + (input * self.amount[1]);
            input -= self.hp[ch][i];
            self.lp[ch][i] = (self.lp[ch][i] * (1.0 - self.amount[0])) + (input * self.amount[0]);
            input = self.lp[ch][i];
        }
        input = (dry_sample as f64 * (1.0 - self.amount[2])) + (input * self.amount[2]);
        fpd_dither(input, &mut self.fpd[ch]) as f32
    }
}
impl AirwindowsEffect for Capacitor {
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
            .map_or_else(String::new, |v| crate::display::float2string(*v))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let chase = [
            (self.params[0] as f64).powf(2.0),
            (self.params[1] as f64).powf(2.0),
            self.params[2] as f64,
        ];
        let speed =
            std::array::from_fn::<_, 3, _>(|i| 300.0 / ((self.last[i] - chase[i]).abs() + 1.0));
        self.last = chase;
        for (((l, r), a), b) in il.iter().zip(ir).zip(ol).zip(or) {
            for i in 0..3 {
                self.amount[i] = ((self.amount[i] * speed[i]) + chase[i]) / (speed[i] + 1.0);
            }
            self.count = (self.count + 1) % 6;
            let stages = match self.count {
                0 => [0, 1, 3],
                1 => [0, 2, 4],
                2 => [0, 1, 5],
                3 => [0, 2, 3],
                4 => [0, 1, 4],
                _ => [0, 2, 5],
            };
            *a = self.sample(*l, 0, stages);
            *b = self.sample(*r, 1, stages);
        }
    }
}
