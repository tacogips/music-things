//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PowerSag2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Range",
        label: "",
        default: 0.3,
    },
    ParamInfo {
        name: "Inv/Wet",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PowerSag2",
    upstream_name: "PowerSag2",
    description: "PowerSag2 is my improved circuit-starve plugin, now with inverse effect!",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PowerSag2::new(sample_rate, seed))
}
struct PowerSag2 {
    params: [f32; 2],
    _sample_rate: f32,
    seed: u32,
    delay: [Box<[f64]>; 2],
    control: [f64; 2],
    count: i32,
    fpd: [u32; 2],
}
impl PowerSag2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.3, 0.5],
            _sample_rate: sample_rate,
            seed,
            delay: [
                vec![0.0; 16386].into_boxed_slice(),
                vec![0.0; 16386].into_boxed_slice(),
            ],
            control: [0.0; 2],
            count: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for PowerSag2 {
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
        match i {
            0 => float2string(self.params[0]),
            1 => float2string((self.params[1] as f64 * 2.0 - 1.0) as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        for d in &mut self.delay {
            d.fill(0.0);
        }
        self.control = [0.0; 2];
        self.count = 0;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    #[allow(clippy::manual_clamp)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let depth = (self.params[0] as f64).powf(4.0);
        let offset = (depth * 16383.0) as usize + 1;
        let wet = self.params[1] as f64 * 2.0 - 1.0;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            if self.count < 0 || self.count > 16384 {
                self.count = 16384;
            }
            let count = self.count as usize;
            let mut outputs = [0.0f32; 2];
            for (ch, sample) in [l, r].into_iter().enumerate() {
                let mut input = sample as f64;
                if input.abs() < 1.18e-23 {
                    input = self.fpd[ch] as f64 * 1.18e-17;
                }
                let mut dry = input;
                self.delay[ch][count] = input.abs();
                self.control[ch] += self.delay[ch][count];
                let index = count + offset - if count + offset > 16384 { 16384 } else { 0 };
                self.control[ch] -= self.delay[ch][index];
                if self.control[ch] > offset as f64 {
                    self.control[ch] = offset as f64;
                }
                if self.control[ch] < 0.0 {
                    self.control[ch] = 0.0;
                }
                let burst = input * (self.control[ch] / (offset as f64).sqrt());
                let mut clamp = input / if burst == 0.0 { 1.0 } else { burst };
                if clamp > 1.0 {
                    clamp = 1.0;
                }
                if clamp < 0.0 {
                    clamp = 0.0;
                }
                input *= clamp;
                let difference = dry - input;
                if wet < 0.0 {
                    dry *= wet + 1.0;
                }
                input = dry - (difference * wet);
                outputs[ch] = fpd_dither(input, &mut self.fpd[ch]) as f32;
            }
            self.count -= 1;
            *out_l = outputs[0];
            *out_r = outputs[1];
        }
    }
}
