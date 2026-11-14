//! BitGlitter, upstream plugins/LinuxVST/src/BitGlitter/BitGlitter.h
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
        name: "Input",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Glitter",
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
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "BitGlitter",
    upstream_name: "BitGlitter",
    description: "BitGlitter is an old-sampler style digital lo-fi plugin.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(BitGlitter::new(sample_rate, seed))
}
#[derive(Clone, Copy, Default)]
struct State {
    ata_last: f64,
    held_a: f64,
    position_a: f64,
    held_b: f64,
    position_b: f64,
    last_output: f64,
}
struct BitGlitter {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    state: [State; 2],
    fpd: [u32; 2],
}
impl BitGlitter {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        let _ = replay_fpd_seed_loop(&mut rng);
        let _ = replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.0, 0.5, 1.0],
            sample_rate,
            seed,
            state: [State::default(); 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    fn quantize(mut output: f64, rez: f64) -> f64 {
        if output > 0.0 {
            let mut offset = output;
            while offset > 0.0 {
                offset -= rez;
            }
            output -= offset;
        }
        if output < 0.0 {
            let mut offset = output;
            while offset < 0.0 {
                offset += rez;
            }
            output -= offset;
        }
        output *= 1.0 - rez;
        if output.abs() < rez {
            output = 0.0;
        }
        output
    }
    #[allow(clippy::too_many_arguments)]
    fn channel(
        &mut self,
        c: usize,
        input: f32,
        ingain: f64,
        outgain: f64,
        wet: f64,
        rate_a: f64,
        rate_b: f64,
    ) -> f32 {
        let mut sample = f64::from(input);
        if sample.abs() < 1.18e-23 {
            sample = f64::from(self.fpd[c]) * 1.18e-17;
        }
        let dry = sample;
        sample *= ingain;
        sample = sample.clamp(-1.0, 1.0);
        sample *= 1.2533141373155;
        sample = (sample * sample.abs()).sin()
            / if sample.abs() == 0.0 {
                1.0
            } else {
                sample.abs()
            };
        let s = &mut self.state[c];
        let mut half = (sample + s.ata_last) / 2.0;
        s.ata_last = sample;
        s.position_a += rate_a;
        let mut raw = s.held_a;
        if s.position_a > 1.0 {
            s.position_a -= 1.0;
            s.held_a = 0.0 * s.position_a + sample * (1.0 - s.position_a);
            raw = raw * 0.5 + s.held_a * 0.5;
        }
        raw = Self::quantize(raw, 0.0016666666666667);
        s.position_b += rate_b;
        let mut interp = s.held_b;
        if s.position_b > 1.0 {
            s.position_b -= 1.0;
            s.held_b = 0.0 * s.position_b + half * (1.0 - s.position_b);
            interp = interp * 0.5 + s.held_b * 0.5;
        }
        half = Self::quantize(interp, 0.0026666666666667);
        sample = raw;
        sample += half;
        sample /= 2.0;
        let mut output = (sample * (1.0 - wet / 2.0)) + (s.last_output * (wet / 2.0));
        s.last_output = sample;
        output *= outgain;
        if wet < 1.0 {
            output = (dry * (1.0 - wet)) + (output * wet);
        }
        fpd_dither(output, &mut self.fpd[c]) as f32
    }
}
impl AirwindowsEffect for BitGlitter {
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
            0 | 2 => float2string((f64::from(self.params[index]) * 36.0 - 18.0) as f32),
            1 | 3 => float2string(self.params[index]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.state = [State::default(); 2];
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= f64::from(self.sample_rate);
        let factor = (f64::from(self.params[1]) + 1.0).powf(7.0) + 2.0;
        let divvy = (factor * scale) as i32;
        let rate_a = 1.0 / f64::from(divvy);
        let rate_b = 1.61803398875 / f64::from(divvy);
        let ingain = 10.0_f64.powf((f64::from(self.params[0]) * 36.0 - 18.0) / 14.0);
        let outgain = 10.0_f64.powf((f64::from(self.params[2]) * 36.0 - 18.0) / 14.0);
        let wet = f64::from(self.params[3]);
        for (((l, r), ol), or) in in_l
            .iter()
            .zip(in_r)
            .zip(out_l.iter_mut())
            .zip(out_r.iter_mut())
        {
            *ol = self.channel(0, *l, ingain, outgain, wet, rate_a, rate_b);
            *or = self.channel(1, *r, ingain, outgain, wet, rate_a, rate_b);
        }
    }
}
