//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/DubCenter, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "TGrind",
        label: "",
        default: 0.9,
    },
    ParamInfo {
        name: "Grd/Out",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "XOver",
        label: "",
        default: 0.74,
    },
    ParamInfo {
        name: "BsDrive",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "BsVoice",
        label: "",
        default: 0.95,
    },
    ParamInfo {
        name: "BassOut",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "SbDrive",
        label: "",
        default: 0.2,
    },
    ParamInfo {
        name: "SbVoice",
        label: "",
        default: 0.2,
    },
    ParamInfo {
        name: "SubOut",
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
    name: "DubCenter",
    upstream_name: "DubCenter",
    description: "DubCenter is a version of DubSub where the bass reinforcement is purely mono.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DubCenter::new(rate, seed))
}
struct DubCenter {
    params: [f32; 10],
    rate: f32,
    seed: u32,
    drive_iir: [[[f64; 3]; 2]; 2],
    flip: bool,
    bflip: usize,
    was_negative: bool,
    sub_octave: bool,
    head: [f64; 3],
    sub: [f64; 3],
    last_head: f64,
    last_sub: f64,
    iir: [f64; 26],
    gate: f64,
    fpd: [u32; 2],
}
impl DubCenter {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.9, 0.5, 0.74, 1.0, 0.95, 0.5, 0.2, 0.2, 0.5, 1.0],
            rate,
            seed,
            drive_iir: [[[0.0; 3]; 2]; 2],
            flip: false,
            bflip: 0,
            was_negative: false,
            sub_octave: false,
            head: [0.0; 3],
            sub: [0.0; 3],
            last_head: 0.0,
            last_sub: 0.0,
            iir: [0.0; 26],
            gate: 1.0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
pub(super) fn highpass(sample: &mut f64, state: &mut f64, amount: f64) {
    *state = *state * (1.0 - amount) + *sample * amount;
    *sample -= *state;
}
#[allow(clippy::too_many_arguments)]
pub(super) fn oscillator(
    state: &mut [f64; 3],
    index: usize,
    sample: f64,
    gain: f64,
    freq: f64,
    randy: f64,
    inv: f64,
    clamp: f64,
) -> f64 {
    if index == 0 {
        return 0.0;
    }
    let n = index - 1;
    state[n] += sample * gain;
    state[n] -= state[n] * state[n] * state[n] * freq;
    state[n] = (if n == 0 { inv } else { randy }) * state[0]
        + (if n == 1 { inv } else { randy }) * state[1]
        + (if n == 2 { inv } else { randy }) * state[2];
    if state[n] > 0.0 {
        state[n] -= clamp;
    }
    if state[n] < 0.0 {
        state[n] += clamp;
    }
    state[n]
}
impl AirwindowsEffect for DubCenter {
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
        match self.params.get(i) {
            Some(v) => float2string(if [1, 5, 8].contains(&i) {
                (*v as f64 * 2.0 - 1.0) as f32
            } else {
                *v
            }),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let p = self.params.map(f64::from);
        let scale = (1.0 / 44100.0) * self.rate as f64;
        let drive = (p[0] * 3.0).powi(2);
        let drive_out = p[1] * 2.0 - 1.0;
        let amount = (p[2] * 0.33 + 0.1) / scale;
        let head_freq = (p[4] * 0.1 + 0.0001) / scale;
        let sub_freq = (p[7] * 0.1 + 0.0001) / scale;
        let head_amount = head_freq / 44.1;
        let sub_amount = sub_freq / 44.1;
        for (((&l, &r), ol), or) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut x = [l as f64, r as f64];
            for (ch, s) in x.iter_mut().enumerate() {
                if s.abs() < 1.18e-23 {
                    *s = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            let dry = x;
            self.gate += ((x[0] + x[1]) * 5.0).abs();
            self.gate -= 0.001;
            self.gate = self.gate.clamp(0.0, 1.0);
            let clamp = (1.0 - self.gate) * 0.00001;
            let mut low = [0.0; 2];
            for (ch, sample) in x.iter_mut().enumerate() {
                let original = *sample;
                for state in &mut self.drive_iir[usize::from(!self.flip)][ch] {
                    highpass(sample, state, amount);
                }
                low[ch] = original - *sample;
                *sample = sample.clamp(-1.0, 1.0);
                let mut out = drive;
                while out > 0.60 {
                    out -= 0.60;
                    *sample -= *sample * (sample.abs() * 0.60) * (sample.abs() * 0.60);
                    *sample *= 1.60;
                }
                *sample -= *sample * (sample.abs() * out) * (sample.abs() * out);
                *sample *= 1.0 + out;
            }
            let mut low = (low[0] + low[1]) / 2.0;
            if low > 0.0 {
                if self.was_negative {
                    self.sub_octave = !self.sub_octave;
                }
                self.was_negative = false;
            } else {
                self.was_negative = true;
            }
            let mut randy = (self.fpd[0] as f64 / u32::MAX as f64) * 0.111;
            let inv = 1.0 - randy;
            randy /= 2.0;
            for state in &mut self.iir[..22] {
                highpass(&mut low, state, head_amount);
            }
            let mut head = oscillator(
                &mut self.head,
                self.bflip,
                low,
                p[3] * 0.1,
                head_freq,
                randy,
                inv,
                clamp,
            );
            highpass(&mut head, &mut self.iir[22], head_amount);
            highpass(&mut head, &mut self.iir[23], head_amount);
            let mut sub = head;
            highpass(&mut sub, &mut self.iir[24], sub_amount);
            sub = sub.abs();
            if !self.sub_octave {
                sub = -sub;
            }
            sub = oscillator(
                &mut self.sub,
                self.bflip,
                sub,
                p[6] * 0.1,
                sub_freq,
                randy,
                inv,
                clamp,
            );
            highpass(&mut sub, &mut self.iir[25], sub_amount);
            for (ch, sample) in x.iter_mut().enumerate() {
                *sample *= drive_out;
                *sample += (head + self.last_head) * (p[5] * 2.0 - 1.0);
                *sample += (sub + self.last_sub) * (p[8] * 2.0 - 1.0);
                if p[9] != 1.0 {
                    *sample = *sample * p[9] + dry[ch] * (1.0 - p[9]);
                }
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            self.last_head = head;
            self.last_sub = sub;
            self.flip = !self.flip;
            self.bflip += 1;
            if self.bflip > 3 {
                self.bflip = 1;
            }
            *ol = x[0] as f32;
            *or = x[1] as f32;
        }
    }
}
