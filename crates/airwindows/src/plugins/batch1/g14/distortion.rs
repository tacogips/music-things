//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Distortion, b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs pending.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Input",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Mode",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Distortion",
    upstream_name: "Distortion",
    description:
        "Distortion is a slightly dark analog-style distortion with several presets, like Focus.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Distortion::new(rate, seed))
}
struct Distortion {
    rate: f32,
    seed: u32,
    params: [f32; 4],
    input: [[f64; 9]; 2],
    output: [[f64; 9]; 2],
    fpd: [u32; 2],
}
impl Distortion {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            rate,
            seed,
            params: [0.5, 0.5, 0.5, 1.0],
            input: [[0.0; 9]; 2],
            output: [[0.0; 9]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    #[allow(clippy::approx_constant, clippy::too_many_arguments)]
    fn channel(
        &mut self,
        c: usize,
        s: f32,
        stages: usize,
        gain: f64,
        mode: i32,
        out: f64,
        wet: f64,
    ) -> f32 {
        let mut x = s as f64;
        if x.abs() < 1.18e-23 {
            x = self.fpd[c] as f64 * 1.18e-17;
        }
        let dry = x;
        x *= gain;
        for i in 0..stages {
            let t = (x + self.input[c][i]) * 0.5;
            self.input[c][i] = x;
            x = t;
        }
        match mode {
            0 => {
                x = x.clamp(-1.570796326794897, 1.570796326794897);
                x = x.sin();
            }
            1 => {
                x = x.clamp(-1.0, 1.0);
                x -= x * (x.abs() * 0.6) * (x.abs() * 0.6);
                x *= 1.5;
            }
            2 => {
                x = x.clamp(-1.2533141373155, 1.2533141373155);
                x = (x * x.abs()).sin() / if x.abs() == 0.0 { 1.0 } else { x.abs() };
            }
            3 => {
                let mojo = x.abs().powf(0.25);
                if mojo > 0.0 {
                    x = ((x * mojo * std::f64::consts::PI * 0.5).sin() / mojo) * 0.987654321;
                }
            }
            4 => {
                let dyno = x.abs().powf(4.0);
                if dyno > 0.0 {
                    x = ((x * dyno).sin() / dyno) * 1.1654321;
                }
            }
            _ => {}
        }
        for i in 1..(stages / 2) {
            let t = (x + self.output[c][i]) * 0.5;
            self.output[c][i] = x;
            x = t;
        }
        if out != 1.0 {
            x *= out;
        }
        if wet != 1.0 {
            x = x * wet + dry * (1.0 - wet);
        }
        fpd_dither(x, &mut self.fpd[c]) as f32
    }
}
impl AirwindowsEffect for Distortion {
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
            0 | 2 => float2string(((self.params[i] as f64 - 0.5) * 24.0) as f32),
            1 => ["Density", "Drive", "Spiral", "Mojo", "Dyno"]
                [(self.params[1] as f64 * 4.999) as usize]
                .into(),
            3 => float2string(self.params[3]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let stages = ((self.rate as f64 / 25000.0).floor() as usize).min(8);
        let input = 10.0_f64.powf(((self.params[0] as f64 - 0.5) * 24.0) / 20.0);
        let mode = (self.params[1] as f64 * 4.999) as i32;
        let output = 10.0_f64.powf(((self.params[2] as f64 - 0.5) * 24.0) / 20.0);
        let wet = self.params[3] as f64;
        for (((l, r), o_l), o_r) in il.iter().zip(ir).zip(ol).zip(or) {
            *o_l = self.channel(0, *l, stages, input, mode, output, wet);
            *o_r = self.channel(1, *r, stages, input, mode, output, wet);
        }
    }
}
