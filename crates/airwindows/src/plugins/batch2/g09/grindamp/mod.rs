//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/GrindAmp, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
mod cabinet;
mod stage;
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Gain",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Tone",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 0.8,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "GrindAmp",
    upstream_name: "GrindAmp",
    description: "GrindAmp is a heavier amp sim for many purposes.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(GrindAmp::new(sample_rate, seed))
}
struct GrindAmp {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    stage: stage::Stage,
    cab: [[f64; 90]; 2],
    smooth_a: [f64; 2],
    smooth_b: [f64; 2],
    last_cab: [f64; 2],
    refs: [[f64; 10]; 2],
    cycle: usize,
    fpd: [u32; 2],
}
impl GrindAmp {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.5, 0.8, 1.0],
            sample_rate,
            seed,
            stage: stage::Stage::new(),
            cab: [[0.0; 90]; 2],
            smooth_a: [0.0; 2],
            smooth_b: [0.0; 2],
            last_cab: [0.0; 2],
            refs: [[0.0; 10]; 2],
            cycle: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for GrindAmp {
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
        let rate = self.sample_rate as f64;
        let tone = self.params[1] as f64;
        self.stage.setup(rate, tone);
        let inputlevel = (self.params[0] as f64).powi(2);
        let mut trim = 1.1 - tone;
        let tone_eq = trim / 1.2;
        trim /= 50.0;
        trim += 0.165;
        let eq = ((trim - tone_eq / 6.1) / rate) * 22050.0;
        let beq = ((trim + tone_eq / 2.1) / rate) * 22050.0;
        #[allow(clippy::approx_constant)]
        let bassdrive = 1.57079633 * (2.5 - tone_eq);
        let output = self.params[2] as f64;
        let wet = self.params[3] as f64;
        let cycle_end = ((rate / 44100.0).floor() as usize).clamp(1, 4);
        self.cycle = self.cycle.min(cycle_end - 1);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol.iter_mut()).zip(orr.iter_mut()) {
            let mut input = [l as f64, r as f64];
            for (i, sample) in input.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[i] as f64 * 1.18e-17;
                }
            }
            let dry = input;
            let (l, r) = self.stage.process(
                input[0],
                input[1],
                dry[0],
                dry[1],
                inputlevel,
                eq,
                beq,
                tone_eq,
                bassdrive,
                output,
                wet,
                self.fpd[0],
                self.fpd[1],
            );
            input = [l, r];
            self.cycle += 1;
            if self.cycle == cycle_end {
                for ch in 0..2 {
                    let temp = (input[ch] + self.smooth_a[ch]) / 3.0;
                    self.smooth_a[ch] = input[ch];
                    let sample = cabinet::convolve(&mut self.cab[ch], temp);
                    let temp = (sample + self.smooth_b[ch]) / 3.0;
                    self.smooth_b[ch] = sample;
                    let sample = temp / 4.0;
                    let randy =
                        self.fpd[ch] as f64 / u32::MAX as f64 * if ch == 0 { 0.044 } else { 0.04 };
                    let result = (((sample * (1.0 - randy) + self.last_cab[ch] * randy) * wet)
                        + (dry[ch] * (1.0 - wet)))
                        * output;
                    self.last_cab[ch] = sample;
                    let refs = &mut self.refs[ch];
                    match cycle_end {
                        4 => {
                            refs[0] = refs[4];
                            refs[2] = (refs[0] + result) / 2.0;
                            refs[1] = (refs[0] + refs[2]) / 2.0;
                            refs[3] = (refs[2] + result) / 2.0;
                            refs[4] = result;
                        }
                        3 => {
                            refs[0] = refs[3];
                            refs[2] = (refs[0] + refs[0] + result) / 3.0;
                            refs[1] = (refs[0] + result + result) / 3.0;
                            refs[3] = result;
                        }
                        2 => {
                            refs[0] = refs[2];
                            refs[1] = (refs[0] + result) / 2.0;
                            refs[2] = result;
                        }
                        _ => refs[0] = result,
                    }
                }
                self.cycle = 0;
            }
            for (ch, sample) in input.iter_mut().enumerate() {
                *sample = self.refs[ch][self.cycle];
                for stage in (2..=cycle_end).rev() {
                    let idx = stage + 3;
                    let temp = *sample;
                    *sample = (*sample + self.refs[ch][idx]) * 0.5;
                    self.refs[ch][idx] = temp;
                }
            }
            *out_l = fpd_dither(input[0], &mut self.fpd[0]) as f32;
            *out_r = fpd_dither(input[1], &mut self.fpd[1]) as f32;
        }
    }
}
