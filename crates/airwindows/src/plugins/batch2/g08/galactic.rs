//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Galactic, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Replace",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Brightness",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Detune",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Bigness",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Galactic",
    upstream_name: "Galactic",
    description: "Galactic is a super-reverb designed specially for pads and space ambient.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Galactic::new(sample_rate, seed))
}
const SIZES: [usize; 13] = [
    9700, 6000, 2320, 940, 15220, 8460, 4540, 3200, 6480, 3660, 1720, 680, 3111,
];
const LENGTHS: [f64; 12] = [
    4801.0, 2909.0, 1153.0, 461.0, 7607.0, 4217.0, 2269.0, 1597.0, 3407.0, 1823.0, 859.0, 331.0,
];
struct Galactic {
    params: [f32; 5],
    rate: f32,
    seed: u32,
    delays: [[Box<[f64]>; 13]; 2],
    count: [usize; 13],
    feedback: [[f64; 4]; 2],
    iir_a: [f64; 2],
    iir_b: [f64; 2],
    refs: [[f64; 7]; 2],
    cycle: usize,
    vib: f64,
    old_fpd: f64,
    fpd: [u32; 2],
}
impl Galactic {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.5, 1.0, 1.0],
            rate,
            seed,
            delays: std::array::from_fn(|_| {
                std::array::from_fn(|i| vec![0.0; SIZES[i]].into_boxed_slice())
            }),
            count: [1; 13],
            feedback: [[0.0; 4]; 2],
            iir_a: [0.0; 2],
            iir_b: [0.0; 2],
            refs: [[0.0; 7]; 2],
            cycle: 0,
            vib: 3.0,
            old_fpd: 429496.7295,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn mix(v: [f64; 4]) -> [f64; 4] {
    [
        v[0] - (v[1] + v[2] + v[3]),
        v[1] - (v[0] + v[2] + v[3]),
        v[2] - (v[0] + v[1] + v[3]),
        v[3] - (v[0] + v[1] + v[2]),
    ]
}
impl AirwindowsEffect for Galactic {
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
        for channel in &mut self.delays {
            for delay in channel {
                delay.fill(0.0);
            }
        }
        self.count = [1; 13];
        self.feedback = [[0.0; 4]; 2];
        self.iir_a = [0.0; 2];
        self.iir_b = [0.0; 2];
        self.refs = [[0.0; 7]; 2];
        self.cycle = 0;
        self.vib = 3.0;
        self.old_fpd = 429496.7295;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let scale = (1.0 / 44100.0) * self.rate as f64;
        let end = (scale.floor() as usize).clamp(1, 4);
        self.cycle = self.cycle.min(end - 1);
        let regen = 0.0625 + ((1.0 - a as f64) * 0.0625);
        let attenuate = (1.0 - regen / 0.125) * 1.333;
        let lowpass = (1.00001 - (1.0 - b as f64)).powf(2.0) / scale.sqrt();
        let drift = (c as f64).powf(3.0) * 0.001;
        let size = d as f64 * 1.77 + 0.1;
        let wet = 1.0 - (1.0 - e as f64).powf(3.0);
        let lengths = LENGTHS.map(|v| (v * size) as usize);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            for (ch, sample) in input.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            let dry = input;
            self.vib += self.old_fpd * drift;
            if self.vib > std::f64::consts::PI * 2.0 {
                self.vib = 0.0;
                self.old_fpd = 0.4294967295 + self.fpd[0] as f64 * 0.0000000000618;
            }
            for (ch, sample) in input.iter().enumerate() {
                self.delays[ch][12][self.count[12]] = *sample * attenuate;
            }
            self.count[12] += 1;
            if self.count[12] > 256 {
                self.count[12] = 0;
            }
            for (ch, sample) in input.iter_mut().enumerate() {
                let offset = ((self.vib
                    + if ch == 0 {
                        0.0
                    } else {
                        std::f64::consts::PI / 2.0
                    })
                .sin()
                    + 1.0)
                    * 127.0;
                let working = (self.count[12] as f64 + offset) as usize;
                let frac = offset - offset.floor();
                *sample = self.delays[ch][12][working % 257] * (1.0 - frac)
                    + self.delays[ch][12][(working + 1) % 257] * frac;
                self.iir_a[ch] = self.iir_a[ch] * (1.0 - lowpass) + *sample * lowpass;
                *sample = self.iir_a[ch];
            }
            self.cycle += 1;
            if self.cycle == end {
                let mut values = [[0.0; 4]; 2];
                for (ch, &sample) in input.iter().enumerate() {
                    for i in 0..4 {
                        self.delays[ch][i + 8][self.count[i + 8]] =
                            sample + self.feedback[1 - ch][i] * regen;
                    }
                }
                for (i, &length) in lengths.iter().enumerate().take(12).skip(8) {
                    self.count[i] += 1;
                    if self.count[i] > length {
                        self.count[i] = 0;
                    }
                }
                for (ch, value) in values.iter_mut().enumerate() {
                    for (i, v) in value.iter_mut().enumerate() {
                        *v = self.delays[ch][i + 8][self.count[i + 8]];
                    }
                }
                for base in [0, 4] {
                    for (ch, &value) in values.iter().enumerate() {
                        let m = mix(value);
                        for (i, &v) in m.iter().enumerate() {
                            self.delays[ch][base + i][self.count[base + i]] = v;
                        }
                    }
                    for (i, &length) in lengths.iter().enumerate().skip(base).take(4) {
                        self.count[i] += 1;
                        if self.count[i] > length {
                            self.count[i] = 0;
                        }
                    }
                    for (ch, value) in values.iter_mut().enumerate() {
                        for (i, v) in value.iter_mut().enumerate() {
                            *v = self.delays[ch][base + i][self.count[base + i]];
                        }
                    }
                }
                for (ch, &value) in values.iter().enumerate() {
                    self.feedback[ch] = mix(value);
                    let sample = (value[0] + value[1] + value[2] + value[3]) / 8.0;
                    let refs = &mut self.refs[ch];
                    match end {
                        4 => {
                            refs[0] = refs[4];
                            refs[2] = (refs[0] + sample) / 2.0;
                            refs[1] = (refs[0] + refs[2]) / 2.0;
                            refs[3] = (refs[2] + sample) / 2.0;
                            refs[4] = sample;
                        }
                        3 => {
                            refs[0] = refs[3];
                            refs[2] = (refs[0] + refs[0] + sample) / 3.0;
                            refs[1] = (refs[0] + sample + sample) / 3.0;
                            refs[3] = sample;
                        }
                        2 => {
                            refs[0] = refs[2];
                            refs[1] = (refs[0] + sample) / 2.0;
                            refs[2] = sample;
                        }
                        _ => refs[0] = sample,
                    }
                }
                self.cycle = 0;
            }
            for ch in 0..2 {
                input[ch] = self.refs[ch][self.cycle];
                self.iir_b[ch] = self.iir_b[ch] * (1.0 - lowpass) + input[ch] * lowpass;
                input[ch] = self.iir_b[ch];
                if wet < 1.0 {
                    input[ch] = input[ch] * wet + dry[ch] * (1.0 - wet);
                }
            }
            *out_l = fpd_dither(input[0], &mut self.fpd[0]) as f32;
            *out_r = fpd_dither(input[1], &mut self.fpd[1]) as f32;
        }
    }
}
