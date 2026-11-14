//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Pockey, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "DeFreq",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "DeRez",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Pockey",
    upstream_name: "Pockey",
    description: "Pockey is 12 bit (and under) lo-fi hiphop in a plugin.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Pockey::new(sample_rate, seed))
}
struct Pockey {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    last: [f64; 2],
    held: [f64; 2],
    soften: [f64; 2],
    position: [f64; 2],
    freq_b: f64,
    rez_b: f64,
    fpd: [u32; 2],
}
impl Pockey {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.0, 1.0],
            sample_rate,
            seed,
            last: [0.0; 2],
            held: [0.0; 2],
            soften: [0.0; 2],
            position: [0.0; 2],
            freq_b: 0.5,
            rez_b: 0.5,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Pockey {
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
        let frames = il.len().min(ir.len()).min(ol.len()).min(or.len());
        let [a, b, c] = self.params;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let freq_min = 0.08 / overallscale;
        let freq_a = self.freq_b;
        self.freq_b = (1.0 - a as f64).powf(3.0) * (0.618_033_988_749_894_9 - freq_min) + freq_min;
        let rez_a = self.rez_b;
        self.rez_b = (b as f64 * 0.618_033_988_749_894_9).powf(3.0) + 0.000_244_140_625;
        let wet = c as f64;
        for i in 0..frames {
            let temp = (frames - 1 - i) as f64 / frames as f64;
            let freq = (freq_a * temp) + (self.freq_b * (1.0 - temp));
            let rez = (rez_a * temp) + (self.rez_b * (1.0 - temp));
            let mut samples = [il[i] as f64, ir[i] as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *sample;
                *sample = sample.clamp(-1.0, 1.0);
                if *sample > 0.0 {
                    *sample = (1.0 + 255.0 * sample.abs()).ln() / 255.0_f64.ln();
                }
                if *sample < 0.0 {
                    *sample = -(1.0 + 255.0 * sample.abs()).ln() / 255.0_f64.ln();
                }
                let mut offset = *sample;
                if *sample > 0.0 {
                    while offset > 0.0 {
                        offset -= rez;
                    }
                    *sample -= offset;
                }
                if *sample < 0.0 {
                    while offset < 0.0 {
                        offset += rez;
                    }
                    *sample -= offset;
                }
                *sample *= 1.0 - rez;
                *sample = sample.clamp(-1.0, 1.0);
                if *sample > 0.0 {
                    *sample = (256.0_f64.powf(sample.abs()) - 1.0) / 255.0;
                }
                if *sample < 0.0 {
                    *sample = -(256.0_f64.powf(sample.abs()) - 1.0) / 255.0;
                }
                self.position[ch] += freq;
                let mut output = self.held[ch];
                if self.position[ch] > 1.0 {
                    self.position[ch] -= 1.0;
                    self.held[ch] =
                        (self.last[ch] * self.position[ch]) + (*sample * (1.0 - self.position[ch]));
                    output =
                        (self.held[ch] * (1.0 - self.position[ch])) + (output * self.position[ch]);
                }
                *sample = output;
                let mut slew = (*sample - self.soften[ch]).abs() * freq;
                if slew > 0.5 {
                    slew = 0.5;
                }
                *sample = (*sample * slew) + (self.soften[ch] * (1.0 - slew));
                self.last[ch] = dry;
                self.soften[ch] = output;
                if wet != 1.0 {
                    *sample = (*sample * wet) + (dry * (1.0 - wet));
                }
            }
            ol[i] = fpd_dither(samples[0], &mut self.fpd[0]) as f32;
            or[i] = fpd_dither(samples[1], &mut self.fpd[1]) as f32;
        }
    }
}
