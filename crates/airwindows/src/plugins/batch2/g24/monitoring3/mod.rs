//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Monitoring3, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
mod modes;
use crate::dither::fpd_from_seed;
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Monitor",
    label: "",
    default: 0.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Monitoring3",
    upstream_name: "Monitoring3",
    description:
        "Monitoring3 is Monitoring, but with Ten Nines into Dark as the wordlength reducer.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Monitoring3::new(rate, seed))
}
struct Monitoring3 {
    param: f32,
    rate: f32,
    seed: u32,
    delay: [Box<[f64]>; 8],
    index: [usize; 4],
    dark: [[f64; 100]; 2],
    last: [f64; 2],
    iir: [[f64; 26]; 2],
    biquad: [[f64; 2]; 2],
    fpd: [u32; 2],
    odd: [f64; 2],
    even: [f64; 2],
    previous_shape: [f64; 2],
    flip: bool,
}
impl Monitoring3 {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            param: 0.0,
            rate,
            seed,
            delay: std::array::from_fn(|_| vec![0.0; 1503].into_boxed_slice()),
            index: [1; 4],
            dark: [[0.0; 100]; 2],
            last: [0.0; 2],
            iir: [[0.0; 26]; 2],
            biquad: [[0.0; 2]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            odd: [0.0; 2],
            even: [0.0; 2],
            previous_shape: [0.0; 2],
            flip: true,
        }
    }
    fn allpass(&mut self, input: &mut [f64; 2], stage: usize, max: usize, darkened: bool) {
        let pos = self.index[stage];
        let prev = if pos == 0 || pos - 1 > max {
            max
        } else {
            pos - 1
        };
        for (ch, sample) in input.iter_mut().enumerate() {
            *sample -= self.delay[stage * 2 + ch][prev] * 0.5;
            self.delay[stage * 2 + ch][pos] = *sample;
            *sample *= 0.5;
        }
        let pos = if pos == 0 || pos - 1 > max {
            max
        } else {
            pos - 1
        };
        self.index[stage] = pos;
        for (ch, sample) in input.iter_mut().enumerate() {
            if darkened {
                *sample += self.delay[stage * 2 + ch][pos] * 0.5;
                *sample += self.delay[stage * 2 + ch][if pos == max { 0 } else { pos + 1 }] * 0.5;
            } else {
                *sample += self.delay[stage * 2 + ch][pos];
            }
        }
    }
}
fn dark(sample: f64, history: &mut [f64; 100], depth: usize) -> f64 {
    let qa = sample.floor() as i32;
    let qb = (sample + 1.0).floor() as i32;
    let mut slew = 0.0f32;
    for x in 0..depth {
        slew = (slew as f64 + (history[x + 1] - history[x])) as f32;
    }
    slew /= depth as f32;
    let a = ((history[0] - qa as f64) - slew as f64).abs() as f32;
    let b = ((history[0] - qb as f64) - slew as f64).abs() as f32;
    let result = if a < b { qa } else { qb };
    history.copy_within(0..=depth, 1);
    history[0] = result as f64;
    result as f64
}
impl AirwindowsEffect for Monitoring3 {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        if i == 0 {
            self.param
        } else {
            0.0
        }
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if i == 0 {
            if let Some(v) = sanitize_param(v) {
                self.param = v;
            }
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i != 0 {
            return String::new();
        }
        [
            "Out24", "Out16", "Peaks", "Slew", "Subs", "Mono", "Side", "Vinyl", "Aurat", "MonoRat",
            "MonoLat", "Phone", "Cans A", "Cans B", "Cans C", "Cans D", "V Trick",
        ][(self.param as f64 * 16.999) as usize]
            .into()
    }
    fn reset(&mut self) {
        for d in &mut self.delay {
            d.fill(0.0);
        }
        self.index = [1; 4];
        self.dark = [[0.0; 100]; 2];
        self.last = [0.0; 2];
        self.iir = [[0.0; 26]; 2];
        self.biquad = [[0.0; 2]; 2];
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
        self.odd = [0.0; 2];
        self.even = [0.0; 2];
        self.previous_shape = [0.0; 2];
        self.flip = true;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        self.process_modes(il, ir, ol, or);
    }
}
