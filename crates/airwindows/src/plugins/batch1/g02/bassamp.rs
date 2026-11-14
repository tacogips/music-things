//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/BassAmp/BassAmp.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Exact; measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "High",
        label: "",
        default: 0.75,
    },
    ParamInfo {
        name: "Dry",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Dub",
        label: "",
        default: 0.75,
    },
    ParamInfo {
        name: "Sub",
        label: "",
        default: 0.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "BassAmp",
    upstream_name: "BassAmp",
    description: "BassAmp is an old plugin with new tweaks, that gives some bass guitar tones.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(BassAmp::new(sample_rate, seed))
}
#[derive(Default)]
struct Channel {
    last: [f64; 6],
    diff: f64,
    drive: [f64; 6],
    head: [f64; 3],
    sub: [f64; 3],
    hp: [f64; 26],
    was_negative: bool,
    sub_octave: bool,
}
struct BassAmp {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    channels: [Channel; 2],
    flip: bool,
    bflip: usize,
}
impl BassAmp {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.75, 0.0, 0.75, 0.0],
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            channels: [Channel::default(), Channel::default()],
            flip: false,
            bflip: 0,
        }
    }
}
struct Coeff {
    drive: f64,
    output: f64,
    dry: f64,
    iir: f64,
    bass_gain: f64,
    head_freq: f64,
    bass_out: f64,
    sub_gain: f64,
    sub_freq: f64,
    sub_out: f64,
    hp: f64,
}
impl Channel {
    fn process(
        &mut self,
        input: f64,
        flip: bool,
        bflip: usize,
        randy: f64,
        c: &Coeff,
        cross_f: Option<f64>,
    ) -> f64 {
        let dry = input;
        let mut half = (input
            + self.last[0]
            + self.last[1] * -0.646
            + self.last[2] * 0.311
            + self.last[3] * -0.093
            + self.last[4] * 0.057
            + self.last[5] * -0.023)
            / 2.0;
        self.last.copy_within(0..5, 1);
        self.last[0] = input;
        let half_dry = half;
        let (processed_half, half_low) = self.drive_sample(half, flip, c.iir, c.drive, cross_f);
        half = processed_half;
        let (mut processed, low) = self.drive_sample(input, flip, c.iir, c.drive, None);
        let half_dry = dry * 0.114 + half_dry * 0.886;
        let half_diff = (half - half_dry) / 2.0;
        let last_diff = self.diff * 0.122;
        self.diff = (processed - dry) / 2.0;
        self.diff += half_diff;
        self.diff -= last_diff;
        processed = dry;
        processed += self.diff;
        let low = low + half_low;
        if (1..=3).contains(&bflip) {
            let i = bflip - 1;
            self.head[i] += low * c.bass_gain;
            self.head[i] -= self.head[i] * self.head[i] * self.head[i] * c.head_freq;
            self.head[i] = (if i == 0 { 1.0 - randy } else { randy * 0.5 }) * self.head[0]
                + (if i == 1 { 1.0 - randy } else { randy * 0.5 }) * self.head[1]
                + (if i == 2 { 1.0 - randy } else { randy * 0.5 }) * self.head[2];
        }
        let head = self.head[0] + self.head[1] + self.head[2];
        let mut sub = head.abs();
        if head > 0.0 {
            if self.was_negative {
                self.sub_octave = !self.sub_octave;
            }
            self.was_negative = false;
        } else {
            self.was_negative = true;
        }
        if !self.sub_octave {
            sub = -sub;
        }
        if (1..=3).contains(&bflip) {
            let i = bflip - 1;
            self.sub[i] += sub * c.sub_gain;
            self.sub[i] -= self.sub[i] * self.sub[i] * self.sub[i] * c.sub_freq;
            self.sub[i] = (if i == 0 { 1.0 - randy } else { randy * 0.5 }) * self.sub[0]
                + (if i == 1 { 1.0 - randy } else { randy * 0.5 }) * self.sub[1]
                + (if i == 2 { 1.0 - randy } else { randy * 0.5 }) * self.sub[2];
        }
        sub = self.sub[0] + self.sub[1] + self.sub[2];
        processed *= c.output;
        let mut correction = dry * c.dry;
        correction += head * c.bass_out;
        correction += sub * c.sub_out;
        processed += correction;
        let mut temp = processed;
        correction = 0.0;
        for (i, state) in self.hp.iter_mut().enumerate() {
            *state = (*state * (1.0 - c.hp)) + (temp * c.hp);
            temp -= *state;
            if i == 0 {
                correction = *state;
            } else {
                correction += *state;
            }
        }
        processed -= correction;
        processed
    }
    fn drive_sample(
        &mut self,
        mut value: f64,
        flip: bool,
        iir: f64,
        drive: f64,
        cross_f: Option<f64>,
    ) -> (f64, f64) {
        let subtract = value;
        let mut temp = value;
        let mut correction = 0.0;
        let indexes = if flip { [0, 2, 4] } else { [1, 3, 5] };
        for (n, i) in indexes.into_iter().enumerate() {
            self.drive[i] = (self.drive[i] * (1.0 - iir)) + (temp * iir);
            temp -= self.drive[i];
            if n == 0 {
                correction = self.drive[i];
            } else {
                correction += if n == 2 {
                    cross_f.unwrap_or(self.drive[i])
                } else {
                    self.drive[i]
                };
            }
        }
        value -= correction;
        let low = subtract - value;
        value = value.clamp(-1.0, 1.0);
        let mut remaining = drive;
        while remaining > 0.60 {
            remaining -= 0.60;
            value -= value * (value.abs() * 0.60) * (value.abs() * 0.60);
            value *= 1.0 + 0.60;
        }
        value -= value * (value.abs() * remaining) * (value.abs() * remaining);
        value *= 1.0 + remaining;
        (value, low)
    }
}
impl AirwindowsEffect for BassAmp {
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
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let high = self.params[0] as f64;
        let dry = self.params[1] as f64;
        let dub = self.params[2] as f64 * 1.3;
        let sub = self.params[3] as f64 / 2.0;
        let bass_gain = dub * 0.1;
        let sub_gain = sub * 0.1;
        let c = Coeff {
            drive: (high * 3.0).powf(2.0),
            output: high,
            dry,
            iir: 0.344 / scale,
            bass_gain,
            head_freq: (bass_gain + 0.0001) / scale,
            bass_out: dub * 0.2,
            sub_gain,
            sub_freq: (sub_gain + 0.0001) / scale,
            sub_out: sub * 0.3,
            hp: 0.0000014 / scale,
        };
        let len = in_l.len().min(in_r.len()).min(out_l.len()).min(out_r.len());
        for i in 0..len {
            let mut samples = [in_l[i] as f64, in_r[i] as f64];
            for (ch, s) in samples.iter_mut().enumerate() {
                if s.abs() < 1.18e-37 {
                    *s = self.fpd[ch] as f64 * 1.18e-37;
                }
            }
            let randy = (self.fpd[0] as f64 / u32::MAX as f64) * 0.0555;
            let right_f = self.channels[1].drive[5];
            for (ch, s) in samples.iter_mut().enumerate() {
                let cross_f = if ch == 0 && !self.flip {
                    Some(right_f)
                } else {
                    None
                };
                *s = self.channels[ch].process(*s, self.flip, self.bflip, randy, &c, cross_f);
                *s = fpd_dither(*s, &mut self.fpd[ch]);
            }
            self.flip = !self.flip;
            self.bflip += 1;
            if self.bflip < 1 || self.bflip > 3 {
                self.bflip = 1;
            }
            out_l[i] = samples[0] as f32;
            out_r[i] = samples[1] as f32;
        }
    }
}
