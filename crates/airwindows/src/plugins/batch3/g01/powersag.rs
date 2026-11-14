//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PowerSag, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Depth",
        label: " ",
        default: 0.0,
    },
    ParamInfo {
        name: "Speed",
        label: " ",
        default: 0.3,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PowerSag",
    upstream_name: "PowerSag",
    description: "PowerSag is for emulating power supply limitations in analog modeling.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PowerSag::new(sample_rate, seed))
}
struct PowerSag {
    params: [f32; 2],
    _sample_rate: f32,
    seed: u32,
    delay: [Box<[f64]>; 2],
    control: [f64; 2],
    count: i32,
    fpd: [u32; 2],
}
impl PowerSag {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.3],
            _sample_rate: sample_rate,
            seed,
            delay: [
                vec![0.0; 9000].into_boxed_slice(),
                vec![0.0; 9000].into_boxed_slice(),
            ],
            control: [0.0; 2],
            count: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for PowerSag {
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
        for d in &mut self.delay {
            d.fill(0.0);
        }
        self.control = [0.0; 2];
        self.count = 0;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let intensity = (self.params[0] as f64).powf(5.0) * 80.0;
        let depth = (self.params[1] as f64).powf(2.0);
        let offset = (depth * 3900.0) as usize + 1;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            if self.count < 0 || self.count > 4000 {
                self.count = 4000;
            }
            let count = self.count as usize;
            let mut outputs = [0.0f32; 2];
            for (ch, sample) in [l, r].into_iter().enumerate() {
                let mut input = sample as f64;
                if input.abs() < 1.18e-23 {
                    input = self.fpd[ch] as f64 * 1.18e-17;
                }
                let value = input.abs() * intensity;
                self.delay[ch][count] = value;
                self.delay[ch][count + 4000] = value;
                self.control[ch] += self.delay[ch][count] / offset as f64;
                self.control[ch] -= self.delay[ch][count + offset] / offset as f64;
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
                let mut bridge = input.abs();
                if bridge > 1.57079633 {
                    bridge = 1.57079633;
                }
                bridge = if thickness > 0.0 {
                    bridge.sin()
                } else {
                    1.0 - bridge.cos()
                };
                input = if input > 0.0 {
                    (input * (1.0 - out)) + (bridge * out)
                } else {
                    (input * (1.0 - out)) - (bridge * out)
                };
                input *= clamp;
                outputs[ch] = fpd_dither(input, &mut self.fpd[ch]) as f32;
            }
            self.count -= 1;
            *out_l = outputs[0];
            *out_r = outputs[1];
        }
    }
}
