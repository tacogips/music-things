//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Tube, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "TUBE",
    label: "",
    default: 0.0,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Tube",
    upstream_name: "Tube",
    description: "Tube is a tube style SoundBetterIzer using a new algorithm for analog modeling!",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Tube::new(rate, seed))
}
struct Tube {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    previous: [f64; 6],
}
impl Tube {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0],
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            previous: [0.0; 6],
        }
    }
}
impl AirwindowsEffect for Tube {
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
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let gain = 1.0 + (self.params[0] as f64 * 0.2246161992650486);
        let iterations = 1.0 - self.params[0] as f64;
        let power = (5.0 * iterations + 1.0) as i32;
        let scaling = 1.0 / (power + 1) as f64;
        let output = 1.0 + 1.0 / power as f64;
        let high = (1.0 / 44100.0 * self.sample_rate as f64) > 1.9;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut result = [0.0; 2];
            for (ch, input) in [l, r].into_iter().enumerate() {
                let mut x = input as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[ch] as f64 * 1.18e-17;
                }
                if high {
                    let stored = x;
                    x += self.previous[ch];
                    self.previous[ch] = stored;
                    x *= 0.5;
                }
                x *= gain;
                x = x.clamp(-1.0, 1.0);
                let mut factor = x;
                for _ in 0..power {
                    factor *= x;
                }
                if power % 2 == 1 && x != 0.0 {
                    factor = (factor / x) * x.abs();
                }
                factor *= scaling;
                x -= factor;
                x *= output;
                if high {
                    let stored = x;
                    x += self.previous[2 + ch];
                    self.previous[2 + ch] = stored;
                    x *= 0.5;
                }
                result[ch] = fpd_dither(x, &mut self.fpd[ch]) as f32;
            }
            *out_l = result[0];
            *out_r = result[1];
        }
    }
}
