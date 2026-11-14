//! BlockParty, upstream plugins/LinuxVST/src/BlockParty/BlockParty.h
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
        name: "Pound",
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
    name: "BlockParty",
    upstream_name: "BlockParty",
    description: "BlockParty is like a moderately saturated analog limiter.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(BlockParty::new(sample_rate, seed))
}
#[derive(Clone, Copy)]
struct Channel {
    speed: [f64; 5],
    coefficient: [f64; 5],
    last_b: f64,
    last_d: f64,
    merged: f64,
    threshold: f64,
    threshold_b: f64,
}
impl Default for Channel {
    fn default() -> Self {
        Self {
            speed: [10000.0; 5],
            coefficient: [1.0; 5],
            last_b: 1.0,
            last_d: 1.0,
            merged: 1.0,
            threshold: 1.0,
            threshold_b: 1.0,
        }
    }
}
struct BlockParty {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    channel: [Channel; 2],
    count: usize,
    flip: bool,
    fpd: [u32; 2],
}
impl BlockParty {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        let _ = replay_fpd_seed_loop(&mut rng);
        let _ = replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 1.0],
            sample_rate,
            seed,
            channel: [Channel::default(); 2],
            count: 0,
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn step(
        input: &mut f64,
        threshold: &mut f64,
        target: f64,
        voicing: f64,
        speed: &mut f64,
        coefficient: &mut f64,
        release: f64,
        fastest: f64,
    ) {
        if input.abs() > *threshold {
            if *input > 0.0 {
                *input = *input * voicing + target * (1.0 - voicing);
            } else {
                *input = *input * voicing - target * (1.0 - voicing);
            }
            *threshold = input.abs();
            let vary = target / input.abs();
            let attack = speed.abs().sqrt();
            *coefficient *= attack - 1.0;
            if vary < *threshold {
                *coefficient += target;
            } else {
                *coefficient += vary;
            }
            *coefficient /= attack;
        } else {
            *threshold = target;
            *coefficient *= *speed * *speed - 1.0;
            *coefficient += 1.0;
            *coefficient /= *speed * *speed;
        }
        let mut new_speed = *speed * (*speed - 1.0);
        new_speed = new_speed + (*input * release).abs() + fastest;
        *speed = new_speed / *speed;
    }
    fn process_channel(
        &mut self,
        c: usize,
        input: f32,
        target: f64,
        voicing: f64,
        scale: f64,
        wet: f64,
    ) -> f32 {
        let mut sample = f64::from(input);
        if sample.abs() < 1.18e-23 {
            sample = f64::from(self.fpd[c]) * 1.18e-17;
        }
        let dry = sample;
        let s = &mut self.channel[c];
        let mut makeup = 1.0 / s.threshold;
        let outmakeup = makeup.sqrt();
        makeup += outmakeup;
        makeup *= 0.5;
        let mut release = s.merged * 32768.0;
        release /= scale;
        let fastest = release.sqrt();
        let correction = s.merged;
        if makeup != 1.0 {
            sample *= makeup;
        }
        let i = self.count;
        Self::step(
            &mut sample,
            &mut s.threshold,
            target,
            voicing,
            &mut s.speed[i],
            &mut s.coefficient[i],
            release,
            fastest,
        );
        let mut last_a = s.coefficient[i].powf(2.0);
        s.merged = s.last_b;
        s.merged += last_a;
        last_a *= 1.0 - correction;
        last_a += s.coefficient[i] * correction;
        s.last_b = last_a;
        let j = if self.flip { 3 } else { 4 };
        Self::step(
            &mut sample,
            &mut s.threshold_b,
            target,
            voicing,
            &mut s.speed[j],
            &mut s.coefficient[j],
            release,
            fastest,
        );
        let mut last_c = s.coefficient[4].powf(2.0);
        s.merged += s.last_d;
        s.merged += last_c;
        last_c *= 1.0 - correction;
        last_c += s.coefficient[j] * correction;
        s.last_d = last_c;
        s.merged *= 0.25;
        sample *= s.merged;
        if outmakeup != 1.0 {
            sample *= outmakeup;
        }
        if wet < 1.0 {
            sample = sample * wet + dry * (1.0 - wet);
        }
        sample = sample.clamp(-0.999, 0.999);
        fpd_dither(sample, &mut self.fpd[c]) as f32
    }
}
impl AirwindowsEffect for BlockParty {
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
        self.params
            .get(index)
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        self.channel = [Channel::default(); 2];
        self.count = 0;
        self.flip = true;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    #[allow(clippy::excessive_precision)]
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= f64::from(self.sample_rate);
        let target = 1.01 - (1.0 - (1.0 - f64::from(self.params[0]) * 0.5).powf(4.0));
        let mut voicing = 0.618033988749894848204586;
        if scale > 0.0 {
            voicing /= scale;
        }
        voicing = voicing.clamp(0.0, 1.0);
        let wet = f64::from(self.params[1]);
        for (((l, r), ol), or) in in_l
            .iter()
            .zip(in_r)
            .zip(out_l.iter_mut())
            .zip(out_r.iter_mut())
        {
            *ol = self.process_channel(0, *l, target, voicing, scale, wet);
            *or = self.process_channel(1, *r, target, voicing, scale, wet);
            self.count = (self.count + 1) % 3;
            self.flip = !self.flip;
        }
    }
}
