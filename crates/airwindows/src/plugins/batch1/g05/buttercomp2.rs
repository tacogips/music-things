//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ButterComp2/ButterComp2.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm, measured max-abs 5.960464477539e-8; per-instance noise counters reset while upstream process statics persist.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: [ParamInfo; 3] = [
    ParamInfo {
        name: "Compress",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "ButterComp2",
    upstream_name: "ButterComp2",
    description: "ButterComp2 is improved ButterComp with an output control and sound upgrades.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ButterComp2::new(rate, seed))
}
struct Channel {
    control: [[f64; 2]; 2],
    target: [f64; 2],
    last: f64,
    noise: i32,
    fpd: u32,
}
impl Channel {
    fn new(noise: i32, fpd: u32) -> Self {
        Self {
            control: [[1.0; 2]; 2],
            target: [1.0; 2],
            last: 0.0,
            noise,
            fpd,
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn process(
        &mut self,
        input: f32,
        flip: bool,
        gain: f64,
        comp: f64,
        scale: f64,
        outputgain: f64,
        output: f64,
        wet: f64,
    ) -> f32 {
        let mut sample = input as f64;
        self.noise = self.noise % 1700021 + 1;
        let mut residue = self.noise.wrapping_mul(self.noise);
        for divisor in [170003, 17011, 1709, 173] {
            residue %= divisor;
            residue = residue.wrapping_mul(residue);
        }
        residue %= 17;
        let mut air = residue as f64;
        air *= 0.00000001;
        air *= 0.00000001;
        sample += air;
        if sample < 1.2e-38 && -sample < 1.2e-38 {
            sample -= air;
        }
        let dry = sample;
        sample *= gain;
        let mut divisor = comp / (1.0 + self.last.abs());
        divisor /= scale;
        let remainder = divisor;
        divisor = 1.0 - divisor;
        let mut pos = sample + 1.0;
        if pos < 0.0 {
            pos = 0.0;
        }
        let mut outpos = pos / 2.0;
        if outpos > 1.0 {
            outpos = 1.0;
        }
        pos *= pos;
        self.target[0] *= divisor;
        self.target[0] += pos * remainder;
        let calcpos = (1.0 / self.target[0]).powf(2.0);
        let mut neg = -sample + 1.0;
        if neg < 0.0 {
            neg = 0.0;
        }
        let mut outneg = neg / 2.0;
        if outneg > 1.0 {
            outneg = 1.0;
        }
        neg *= neg;
        self.target[1] *= divisor;
        self.target[1] += neg * remainder;
        let calcneg = (1.0 / self.target[1]).powf(2.0);
        let which = usize::from(!flip);
        let side = usize::from(sample <= 0.0);
        self.control[which][side] *= divisor;
        self.control[which][side] += (if side == 0 { calcpos } else { calcneg }) * remainder;
        let multiplier = self.control[which][0] * outpos + self.control[which][1] * outneg;
        sample *= multiplier;
        sample /= outputgain;
        if output != 1.0 {
            sample *= output;
        }
        if wet != 1.0 {
            sample = sample * wet + dry * (1.0 - wet);
        }
        self.last = sample;
        fpd_dither(sample, &mut self.fpd) as f32
    }
}
struct ButterComp2 {
    params: [f32; 3],
    rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    flip: bool,
}
impl ButterComp2 {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.5, 1.0],
            rate,
            seed,
            left: Channel::new(0, fpd_from_seed(seed, 0)),
            right: Channel::new(850010, fpd_from_seed(seed, 1)),
            flip: false,
        }
    }
}
impl AirwindowsEffect for ButterComp2 {
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
        match self.params.get(index) {
            Some(v) => float2string(if index == 1 {
                (*v as f64 * 2.0) as f32
            } else {
                *v
            }),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, left: &[f32], right: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.rate as f64;
        let gain = 10.0_f64.powf((self.params[0] as f64 * 14.0) / 20.0);
        let comp = 0.012 * (self.params[0] as f64 / 135.0);
        let output = self.params[1] as f64 * 2.0;
        let wet = self.params[2] as f64;
        let mut outputgain = gain;
        outputgain -= 1.0;
        outputgain /= 1.5;
        outputgain += 1.0;
        let len = left
            .len()
            .min(right.len())
            .min(out_l.len())
            .min(out_r.len());
        for i in 0..len {
            out_l[i] = self.left.process(
                left[i], self.flip, gain, comp, scale, outputgain, output, wet,
            );
            out_r[i] = self.right.process(
                right[i], self.flip, gain, comp, scale, outputgain, output, wet,
            );
            self.flip = !self.flip;
        }
    }
}
