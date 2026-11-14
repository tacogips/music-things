//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Vibrato, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Speed",
        label: "",
        default: 0.3,
    },
    ParamInfo {
        name: "Depth",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "FMSpeed",
        label: "",
        default: 0.4,
    },
    ParamInfo {
        name: "FMDepth",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Inv/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Vibrato",
    upstream_name: "Vibrato",
    description: "Vibrato lets you vibrato, chorus, flange, and make odd FM noises.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Vibrato::new(sample_rate, seed))
}
struct Vibrato {
    params: [f32; 5],
    seed: u32,
    buffers: [Box<[f64]>; 2],
    sweep: f64,
    sweep_b: f64,
    count: i32,
    prev: [f64; 2],
    even: [f64; 2],
    odd: [f64; 2],
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl Vibrato {
    fn new(_sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.3, 0.0, 0.4, 0.0, 1.0],
            seed,
            buffers: [
                vec![0.0; 16386].into_boxed_slice(),
                vec![0.0; 16386].into_boxed_slice(),
            ],
            sweep: std::f64::consts::FRAC_PI_2,
            sweep_b: std::f64::consts::FRAC_PI_2,
            count: 0,
            prev: [0.0; 2],
            even: [0.0; 2],
            odd: [0.0; 2],
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Vibrato {
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
            0..=3 => float2string(self.params[i]),
            4 => float2string((self.params[4] as f64 * 2.0 - 1.0) as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.buffers[0].fill(0.0);
        self.buffers[1].fill(0.0);
        self.sweep = std::f64::consts::FRAC_PI_2;
        self.sweep_b = self.sweep;
        self.count = 0;
        self.prev = [0.0; 2];
        self.even = [0.0; 2];
        self.odd = [0.0; 2];
        self.flip = false;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let speed = (0.1 + a as f64).powf(6.0);
        let depth = ((b as f64).powf(3.0) / speed.sqrt()) * 4.0;
        let speed_b = (0.1 + c as f64).powf(6.0);
        let depth_b = (d as f64).powf(3.0) / speed_b.sqrt();
        let tupi = std::f64::consts::PI * 2.0;
        let wet = e as f64 * 2.0 - 1.0;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            if input[0].abs() < 1.18e-23 {
                input[0] = self.fpd_l as f64 * 1.18e-17;
            }
            if input[1].abs() < 1.18e-23 {
                input[1] = self.fpd_r as f64 * 1.18e-17;
            }
            let dry = input;
            for (ch, sample) in input.iter_mut().enumerate() {
                let mut factor = self.prev[ch] - *sample;
                if self.flip {
                    self.even[ch] += factor;
                    self.odd[ch] -= factor;
                    factor = self.even[ch];
                } else {
                    self.odd[ch] += factor;
                    self.even[ch] -= factor;
                    factor = self.odd[ch];
                }
                self.odd[ch] = (self.odd[ch] - ((self.odd[ch] - self.even[ch]) / 256.0)) / 1.0001;
                self.even[ch] = (self.even[ch] - ((self.even[ch] - self.odd[ch]) / 256.0)) / 1.0001;
                self.prev[ch] = *sample;
                *sample += factor;
            }
            self.flip = !self.flip;
            if self.count < 1 || self.count > 8192 {
                self.count = 8192;
            }
            let write = self.count as usize;
            let offset = depth + depth * self.sweep.sin();
            let index = write + offset.floor() as usize;
            for (ch, sample) in input.iter_mut().enumerate() {
                let buf = &mut self.buffers[ch];
                buf[write] = *sample;
                buf[write + 8192] = *sample;
                *sample = buf[index] * (1.0 - (offset - offset.floor()));
                *sample += buf[index + 1];
                *sample += buf[index + 2] * (offset - offset.floor());
                *sample -=
                    ((buf[index] - buf[index + 1]) - (buf[index + 1] - buf[index + 2])) / 50.0;
                *sample *= 0.5;
            }
            self.sweep += speed + speed_b * self.sweep_b.sin() * depth_b;
            self.sweep_b += speed_b;
            if self.sweep > tupi {
                self.sweep -= tupi;
            }
            if self.sweep < 0.0 {
                self.sweep += tupi;
            }
            if self.sweep_b > tupi {
                self.sweep_b -= tupi;
            }
            self.count -= 1;
            if wet != 1.0 {
                for ch in 0..2 {
                    input[ch] = input[ch] * wet + dry[ch] * (1.0 - wet.abs());
                }
            }
            *out_l = fpd_dither(input[0], &mut self.fpd_l) as f32;
            *out_r = fpd_dither(input[1], &mut self.fpd_r) as f32;
        }
    }
}
