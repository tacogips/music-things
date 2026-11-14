//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Texturize, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Bright",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Punchy",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Texturize",
    upstream_name: "Texturize",
    description: "Texturize is a hidden-noise plugin for adding sonic texture to things.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Texturize::new(sample_rate, seed))
}
#[derive(Clone, Copy)]
struct Channel {
    polarity: bool,
    last: f64,
    iir: f64,
    noise_a: f64,
    noise_c: f64,
    flip: bool,
}
impl Channel {
    fn new() -> Self {
        Self {
            polarity: false,
            last: 0.0,
            iir: 0.0,
            noise_a: 0.0,
            noise_c: 0.0,
            flip: true,
        }
    }
    fn process(
        &mut self,
        input: f64,
        fpd: u32,
        slew_amount: f64,
        dyn_amount: f64,
        wet: f64,
    ) -> f64 {
        let dry = input;
        if input < 0.0 {
            if self.polarity {
                self.flip = self.noise_a < 0.0;
            }
            self.polarity = false;
        } else {
            self.polarity = true;
        }
        if self.flip {
            self.noise_a += fpd as f64 / u32::MAX as f64;
        } else {
            self.noise_a -= fpd as f64 / u32::MAX as f64;
        }
        self.flip = !self.flip;
        let mut sample = input.clamp(-1.0, 1.0);
        if dyn_amount < 0.4999 {
            sample = (sample * dyn_amount * 2.0) + (sample.sin() * (1.0 - (dyn_amount * 2.0)));
        }
        if dyn_amount > 0.5001 {
            sample = (sample.asin() * ((dyn_amount * 2.0) - 1.0))
                + (sample * (1.0 - ((dyn_amount * 2.0) - 1.0)));
        }
        let mut noise_b = (self.noise_a * (0.2 - (dyn_amount * 0.125)) * sample.abs()).sin();
        let mut slew = (sample - self.last).abs() * slew_amount;
        self.last = sample * (0.86 - (dyn_amount * 0.125));
        if slew > 1.0 {
            slew = 1.0;
        }
        let mut intensity = slew;
        intensity *= 2.472;
        intensity *= intensity;
        if intensity > 1.0 {
            intensity = 1.0;
        }
        self.iir = (self.iir * (1.0 - intensity)) + (noise_b * intensity);
        noise_b = self.iir;
        noise_b = (noise_b * slew) + (self.noise_c * (1.0 - slew));
        self.noise_c = noise_b;
        (self.noise_c * wet) + (dry * (1.0 - wet))
    }
}
struct Texturize {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    channels: [Channel; 2],
    fpd_l: u32,
    fpd_r: u32,
}
impl Texturize {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5; 3],
            sample_rate,
            seed,
            channels: [Channel::new(); 2],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Texturize {
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
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], outr: &mut [f32]) {
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let slew = ((self.params[0] as f64).powf(2.0) * 4.0 + 0.71) / overallscale;
        let dynamic = (self.params[1] as f64).powf(2.0);
        let wet = (self.params[2] as f64).powf(5.0);
        for (((&l, &r), lo), ro) in il.iter().zip(ir).zip(ol).zip(outr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            l = self.channels[0].process(l, self.fpd_l, slew, dynamic, wet);
            r = self.channels[1].process(r, self.fpd_r, slew, dynamic, wet);
            *lo = fpd_dither(l, &mut self.fpd_l) as f32;
            *ro = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
