//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/TubeDesk, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};

// Upstream uses this rounded threshold, which differs from FRAC_PI_2.
#[allow(clippy::approx_constant)]
const BRIDGE_LIMIT: f64 = 1.57079633;
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "TubeDesk",
    upstream_name: "TubeDesk",
    description: "TubeDesk is a tube recording console type tone coloring.",
    params: &[],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(TubeDesk::new(rate, seed))
}
struct TubeDesk {
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    delay: [Box<[f64]>; 2],
    count: i32,
    control: [f64; 2],
    last: [f64; 2],
    last_out: [f64; 2],
    last_slew: [f64; 2],
}
impl TubeDesk {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            delay: [
                vec![0.0; 5000].into_boxed_slice(),
                vec![0.0; 5000].into_boxed_slice(),
            ],
            count: 0,
            control: [0.0; 2],
            last: [0.0; 2],
            last_out: [0.0; 2],
            last_slew: [0.0; 2],
        }
    }
}
impl AirwindowsEffect for TubeDesk {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, _: usize) -> f32 {
        0.0
    }
    fn set_param(&mut self, _: usize, _: f32) {}
    fn param_display(&self, _: usize) -> String {
        String::new()
    }
    fn reset(&mut self) {
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
        self.delay[0].fill(0.0);
        self.delay[1].fill(0.0);
        self.count = 0;
        self.control = [0.0; 2];
        self.last = [0.0; 2];
        self.last_out = [0.0; 2];
        self.last_slew = [0.0; 2];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let overall = 1.0 / 44100.0 * self.sample_rate as f64;
        let offset = ((549.0 * overall) as i32).clamp(1, 2440) as usize;
        let slewgain = 0.128 * overall;
        let prevslew = 0.105 * overall;
        let balance_b = 0.0001 / overall;
        let balance_a = 1.0 - balance_b;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            if self.count < 0 || self.count > 2450 {
                self.count = 2450;
            }
            let index = self.count as usize;
            let mut result = [0.0f32; 2];
            for (ch, input) in [l, r].into_iter().enumerate() {
                let mut x = input as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = x;
                let val = x.abs() * 0.4384938;
                self.delay[ch][index] = val;
                self.delay[ch][index + 2450] = val;
                self.control[ch] += self.delay[ch][index] / offset as f64;
                self.control[ch] -= self.delay[ch][index + offset] / offset as f64;
                self.control[ch] -= 0.000001;
                let mut clamp = 1.0;
                if self.control[ch] < 0.0 {
                    self.control[ch] = 0.0;
                }
                if self.control[ch] > 1.0 {
                    clamp -= self.control[ch] - 1.0;
                    self.control[ch] = 1.0;
                }
                if clamp < 0.5 {
                    clamp = 0.5;
                }
                let thickness = ((1.0 - self.control[ch]) * 2.0) - 1.0;
                let out = thickness.abs();
                let mut bridge = x.abs();
                if bridge > BRIDGE_LIMIT {
                    bridge = BRIDGE_LIMIT;
                }
                bridge = if thickness > 0.0 {
                    bridge.sin()
                } else {
                    1.0 - bridge.cos()
                };
                if x > 0.0 {
                    x = (x * (1.0 - out)) + (bridge * out);
                } else {
                    x = (x * (1.0 - out)) - (bridge * out);
                }
                x *= clamp;
                let mut slew = x - self.last[ch];
                self.last[ch] = x;
                bridge = (slew * slewgain).abs();
                bridge = if bridge > BRIDGE_LIMIT {
                    1.0
                } else {
                    bridge.sin()
                };
                slew = if slew > 0.0 {
                    bridge / slewgain
                } else {
                    -(bridge / slewgain)
                };
                x = (self.last_out[ch] * balance_a) + (self.last[ch] * balance_b) + slew;
                self.last_out[ch] = x;
                let comb = (dry * self.last[ch]).abs().min(1.0);
                x -= self.last_slew[ch] * comb * prevslew;
                self.last_slew[ch] = slew;
                x *= 0.5;
                bridge = x.abs();
                bridge = if bridge > BRIDGE_LIMIT {
                    1.0
                } else {
                    bridge.sin()
                };
                x = if x > 0.0 { bridge } else { -bridge };
                x /= 0.5;
                result[ch] = fpd_dither(x, &mut self.fpd[ch]) as f32;
            }
            self.count -= 1;
            *out_l = result[0];
            *out_r = result[1];
        }
    }
}
