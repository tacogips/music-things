//! Bite, upstream plugins/LinuxVST/src/Bite/Bite.h
//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm, measured max-abs 0.
use crate::{
    display::float2string,
    dither::{fpd_dither, fpd_from_seed},
    params::sanitize_param,
    rand::{replay_fpd_seed_loop, AwRand},
    registry::PluginEntry,
    AirwindowsEffect, ParamInfo, PluginInfo,
};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Bite",
        label: " ",
        default: 0.5,
    },
    ParamInfo {
        name: "Output Level",
        label: " ",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Bite",
    upstream_name: "Bite",
    description: "Bite is an unusual edge-maker.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Bite::new(sample_rate, seed))
}
struct Bite {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    samples: [[f64; 9]; 2],
    fpd: [u32; 2],
}
impl Bite {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        let _ = replay_fpd_seed_loop(&mut rng);
        let _ = replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 1.0],
            sample_rate,
            seed,
            samples: [[0.0; 9]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    fn channel(&mut self, channel: usize, input: f32, gain: f64, outputgain: f64) -> f32 {
        let s = &mut self.samples[channel];
        s.copy_within(0..8, 1);
        s[0] = f64::from(input);
        if s[0].abs() < 1.18e-23 {
            s[0] = f64::from(self.fpd[channel]) * 1.18e-17;
        }
        let mut a = s[0] - s[4];
        let mut b = s[8] - s[4];
        let mut c = s[2] - s[4];
        let mut d = s[6] - s[4];
        a *= (((s[1] + s[2] + s[3]) / 3.0) - ((s[0] + s[4]) / 2.0)) * gain;
        b *= (((s[5] + s[6] + s[7]) / 3.0) - ((s[4] + s[8]) / 2.0)) * gain;
        c *= (s[3] - ((s[2] + s[4]) / 2.0)) * gain;
        d *= (s[5] - ((s[4] + s[6]) / 2.0)) * gain;
        let mut output = s[4] + ((a + b + c + d).sin() * 8.0);
        if outputgain != 1.0 {
            output *= outputgain;
        }
        fpd_dither(output, &mut self.fpd[channel]) as f32
    }
}
impl AirwindowsEffect for Bite {
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
        match index {
            0 => float2string((f64::from(self.params[0]) * 2.0 - 1.0) as f32),
            1 => float2string(self.params[1]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.samples = [[0.0; 9]; 2];
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let mut scale = 1.3;
        scale /= 44100.0;
        scale *= f64::from(self.sample_rate);
        let gain = (f64::from(self.params[0]) * 2.0 - 1.0) * scale;
        let output = f64::from(self.params[1]);
        for (((l, r), ol), or) in in_l
            .iter()
            .zip(in_r)
            .zip(out_l.iter_mut())
            .zip(out_r.iter_mut())
        {
            *ol = self.channel(0, *l, gain, output);
            *or = self.channel(1, *r, gain, output);
        }
    }
}
