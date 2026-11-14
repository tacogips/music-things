//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PodcastDeluxe, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "Boost",
    label: "",
    default: 0.5,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PodcastDeluxe",
    upstream_name: "PodcastDeluxe",
    description: "PodcastDeluxe is a pile of compressors (curve style) and phase rotators.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PodcastDeluxe::new(sample_rate, seed))
}
struct PodcastDeluxe {
    params: [f32; 1],
    sample_rate: f32,
    seed: u32,
    delay: [[[f64; 503]; 5]; 2],
    tap: [usize; 5],
    comp: [[f64; 5]; 2],
    last: [f64; 2],
    fpd: [u32; 2],
}
impl PodcastDeluxe {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5],
            sample_rate,
            seed,
            delay: [[[0.0; 503]; 5]; 2],
            tap: [1; 5],
            comp: [[2.0; 5]; 2],
            last: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for PodcastDeluxe {
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
            .map_or_else(String::new, |&v| float2string(v))
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let compress = 1.0 + (self.params[0] as f64 * 0.8).powf(2.0);
        let mut speed1 = 128.0 / compress.powf(2.0);
        speed1 *= scale;
        let speed2 = speed1 * 1.4;
        let speed3 = speed2 * 1.5;
        let speed4 = speed3 * 1.6;
        let speed5 = speed4 * 1.7;
        let speeds = [speed5, speed4, speed3, speed2, speed1];
        let maxdelay = [
            (23.0 * scale) as usize,
            (19.0 * scale) as usize,
            (17.0 * scale) as usize,
            (13.0 * scale) as usize,
            (11.0 * scale) as usize,
        ];
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            for (ch, s) in samples.iter_mut().enumerate() {
                if s.abs() < 1.18e-23 {
                    *s = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            for stage in 0..5 {
                let tap = self.tap[stage];
                let temp = if tap == 0 || tap - 1 > maxdelay[stage] {
                    maxdelay[stage]
                } else {
                    tap - 1
                };
                for (ch, s) in samples.iter_mut().enumerate() {
                    *s -= self.delay[ch][stage][temp] * 0.618_033_988_749_894_9;
                    self.delay[ch][stage][tap] = *s;
                    *s *= 0.618_033_988_749_894_9;
                    *s += self.delay[ch][stage][temp];
                    *s *= self.comp[ch][stage];
                    let mut trigger = s.abs() * 4.7;
                    if trigger > 4.7 {
                        trigger = 4.7;
                    }
                    trigger = trigger.sin();
                    if trigger < 0.0 {
                        trigger *= 8.0;
                    }
                    if trigger < -4.2 {
                        trigger = -4.2;
                    }
                    self.comp[ch][stage] += trigger / speeds[stage];
                    if self.comp[ch][stage] > compress {
                        self.comp[ch][stage] = compress;
                    }
                }
                self.tap[stage] = if tap == 0 || tap - 1 > maxdelay[stage] {
                    maxdelay[stage]
                } else {
                    tap - 1
                };
            }
            for (ch, s) in samples.iter_mut().enumerate() {
                if compress > 1.0 {
                    *s /= compress;
                }
                let last = &mut self.last[ch];
                if *last >= 0.999 {
                    if *s < 0.999 {
                        *last = 0.545 + (*s * 0.435);
                    } else {
                        *last = 0.999;
                    }
                }
                if *last <= -0.999 {
                    if *s > -0.999 {
                        *last = -0.545 + (*s * 0.435);
                    } else {
                        *last = -0.999;
                    }
                }
                if *s > 0.999 {
                    if *last < 0.999 {
                        *s = 0.56 + (*last * 0.435);
                    } else {
                        *s = 0.999;
                    }
                }
                if *s < -0.999 {
                    if *last > -0.999 {
                        *s = -0.56 + (*last * 0.435);
                    } else {
                        *s = -0.999;
                    }
                }
            }
            *out_l = fpd_dither(samples[0], &mut self.fpd[0]) as f32;
            *out_r = fpd_dither(samples[1], &mut self.fpd[1]) as f32;
        }
    }
}
