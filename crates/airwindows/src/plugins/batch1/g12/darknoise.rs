//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: plugins/LinuxVST/src/DarkNoise, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo { name: "DarkNoise", upstream_name: "DarkNoise", description: "DarkNoise is an alternative method for producing bassier noise directly. Sound design stuff.", params: &[
    ParamInfo { name: "Freq", label: "", default: 0.5 }, ParamInfo { name: "Dark", label: "", default: 1.0 }, ParamInfo { name: "Output", label: "", default: 1.0 }, ParamInfo { name: "Dry/Wet", label: "", default: 1.0 }
] };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DarkNoise::new(sample_rate, seed))
}
struct DarkNoise {
    sample_rate: f32,
    seed: u32,
    params: [f32; 4],
    bins: [Box<[f64]>; 2],
    b: [[[f64; 11]; 11]; 2],
    f: [f64; 11],
    last: [f64; 2],
    noise: [f64; 2],
    freq: usize,
    fpd: [u32; 2],
}
impl DarkNoise {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rand = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rand);
        replay_fpd_seed_loop(&mut rand);
        Self {
            sample_rate,
            seed,
            params: [0.5, 1.0, 1.0, 1.0],
            bins: [
                vec![0.5; 8193].into_boxed_slice(),
                vec![0.5; 8193].into_boxed_slice(),
            ],
            b: [[[0.0; 11]; 11]; 2],
            f: [0.0; 11],
            last: [0.0; 2],
            noise: [0.5; 2],
            freq: 1,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for DarkNoise {
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
        self.params
            .get(index)
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        let _ = self.sample_rate;
        self.bins[0].fill(0.5);
        self.bins[1].fill(0.5);
        self.b = [[[0.0; 11]; 11]; 2];
        self.f = [0.0; 11];
        self.last = [0.0; 2];
        self.noise = [0.5; 2];
        self.freq = 1;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    #[allow(clippy::needless_range_loop)]
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let a = f64::from(self.params[0]);
        let target = ((a.powf(3.0) * 8192.0) as usize).max(2);
        let volume = (1.0 / target as f64) * (target as f64).sqrt();
        let overall_taps = a.powf(2.0) * 8.0 + 2.0;
        let mut taps = overall_taps;
        let poles = a * 9.0 + 1.0;
        let y_limit = poles.floor() as usize + 1;
        let partial = poles - poles.floor();
        let mut x_limit = 1;
        for x in 0..11 {
            if taps > 1.0 {
                self.f[x] = 1.0;
                taps -= 1.0;
                x_limit += 1;
            } else {
                self.f[x] = taps;
                taps = 0.0;
            }
        }
        x_limit = x_limit.min(9);
        for x in 0..x_limit {
            self.f[x] /= overall_taps.max(1.0);
        }
        let dark = f64::from(self.params[1]);
        let output = f64::from(self.params[2]) * 0.5;
        let wet = f64::from(self.params[3]);
        for (((l, r), ol), or) in in_l.iter().zip(in_r).zip(out_l).zip(out_r) {
            let mut input = [f64::from(*l), f64::from(*r)];
            for (c, s) in input.iter_mut().enumerate() {
                if s.abs() < 1.18e-23 {
                    *s = f64::from(self.fpd[c]) * 1.18e-17;
                }
            }
            let dry = input;
            if target < self.freq {
                for c in 0..2 {
                    self.noise[c] =
                        ((self.noise[c] - 0.5) * (1.0 - (1.0 / self.freq as f64))) + 0.5;
                    self.bins[c][self.freq] = 0.5;
                }
                self.freq -= 1;
            }
            if target > self.freq {
                self.freq += 1;
            }
            for c in 0..2 {
                let rand = f64::from(self.fpd[c]) / f64::from(u32::MAX);
                let bin = (rand * self.freq as f64) as usize;
                self.noise[c] -= self.bins[c][bin];
                self.bins[c][bin] = self.last[c];
                self.noise[c] += self.last[c];
                self.last[c] = rand;
                input[c] = (-0.5 + self.noise[c]) * volume;
            }
            let nondark = input;
            let mut previous = [0.0; 2];
            for y in 0..y_limit {
                for x in (0..=x_limit).rev() {
                    for c in 0..2 {
                        self.b[c][x + 1][y] = self.b[c][x][y];
                    }
                }
                for c in 0..2 {
                    self.b[c][0][y] = input[c];
                    previous[c] = input[c];
                    input[c] = 0.0;
                }
                for x in 0..x_limit {
                    for c in 0..2 {
                        input[c] += self.b[c][x][y] * self.f[x];
                    }
                }
            }
            for c in 0..2 {
                input[c] = previous[c] * (1.0 - partial) + input[c] * partial;
                if dark != 1.0 {
                    input[c] = input[c] * dark + nondark[c] * (1.0 - dark);
                }
                if output != 1.0 {
                    input[c] *= output;
                }
                if wet != 1.0 {
                    input[c] = input[c] * wet + dry[c] * (1.0 - wet);
                }
            }
            *ol = fpd_dither(input[0], &mut self.fpd[0]) as f32;
            *or = fpd_dither(input[1], &mut self.fpd[1]) as f32;
        }
    }
}
