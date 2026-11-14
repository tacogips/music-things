//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Righteous4, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "LTarget",
        label: "dB",
        default: 0.0,
    },
    ParamInfo {
        name: "BtDepth",
        label: "bit",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Righteous4",
    upstream_name: "Righteous4",
    description: "Righteous4 is a final output stage for targeting dynamic range.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Righteous4::new(sample_rate, seed))
}
const BENFORD: [f64; 13] = [
    1000.0, 301.0, 176.0, 125.0, 97.0, 79.0, 67.0, 58.0, 51.0, 46.0, 1000.0, 0.0, 0.0,
];
#[derive(Clone, Copy)]
struct Channel {
    filters: [f64; 26],
    byn: [f64; 13],
    noise: f64,
    last: f64,
    iir: f64,
    gw_prev: f64,
    gw_a: f64,
    gw_b: f64,
    fpd: u32,
}
impl Channel {
    fn new(seed: u32, ch: usize) -> Self {
        Self {
            filters: [0.0; 26],
            byn: BENFORD,
            noise: 0.0,
            last: 0.0,
            iir: 0.0,
            gw_prev: 0.0,
            gw_a: 0.0,
            gw_b: 0.0,
            fpd: fpd_from_seed(seed, ch as u32),
        }
    }
}
struct Righteous4 {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    channels: [Channel; 2],
}
impl Righteous4 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0; 2],
            sample_rate,
            seed,
            channels: [Channel::new(seed, 0), Channel::new(seed, 1)],
        }
    }
}
fn hotbin(value: f64) -> usize {
    let mut b = value;
    while b >= 1.0 {
        b /= 10.0;
    }
    for _ in 0..5 {
        if b < 1.0 {
            b *= 10.0;
        }
    }
    let bin = b.floor() as i32;
    if bin > 0 && bin < 10 {
        bin as usize
    } else {
        10
    }
}
fn score(bin: usize, byn: &mut [f64; 13]) -> f64 {
    if bin == 10 {
        return 0.0;
    }
    byn[bin] += 1.0;
    let mut total = 0.0;
    for i in 1..10 {
        total += BENFORD[i] - byn[i];
    }
    byn[bin] -= 1.0;
    total
}
fn naturalize(mut x: f64, depth: i32, dry: f64, channel: &mut Channel) -> f64 {
    x -= channel.noise;
    if depth == 2 {
        x *= 8388608.0;
    }
    if depth == 1 {
        x *= 32768.0;
    }
    let a = hotbin(x.floor());
    let total_a = score(a, &mut channel.byn);
    let b = hotbin(x.ceil());
    let total_b = score(b, &mut channel.byn);
    if total_a < total_b {
        channel.byn[a] += 1.0;
        x = x.floor();
    } else {
        channel.byn[b] += 1.0;
        x = x.ceil();
    }
    channel.byn[10] /= 2.0;
    if depth == 2 {
        x /= 8388608.0;
    }
    if depth == 1 {
        x /= 32768.0;
    }
    channel.noise += x - dry;
    x
}
impl AirwindowsEffect for Righteous4 {
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
            0 => float2string((self.params[0] as f64 * 24.0 - 28.0) as f32),
            1 => match (self.params[1] as f64 * 2.999) as i32 {
                0 => "16",
                1 => "24",
                2 => "32",
                _ => "",
            }
            .into(),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.channels = [Channel::new(self.seed, 0), Channel::new(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let old = 0.6180339887498948;
        let new = 1.0 - old;
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let mut back = 0.0002597;
        back /= scale;
        back = 1.0 - back;
        let mut target = (self.params[0] as f64 * 24.0) - 28.0;
        target += 17.0;
        target = 10.0_f64.powf(target / 20.0);
        if target == 0.0 {
            target = 1.0;
        }
        let depth = (self.params[1] as f64 * 2.999) as i32 + 1;
        let mut mid = 20.0 / 149940.0;
        mid /= scale;
        let mid_alt = 1.0 - mid;
        let mut gwa = 0.718;
        gwa -= scale * 0.05;
        let gwb = 1.0 - gwa;
        let soft = 0.2135;
        let hard = 1.0 - soft;
        let clip = 10.0_f64.powf(-0.0058888);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            for (ch, x) in samples.iter_mut().enumerate() {
                let c = &mut self.channels[ch];
                if x.abs() < 1.18e-23 {
                    *x = c.fpd as f64 * 1.18e-17;
                }
                *x /= target;
                c.iir *= back;
                let harmonic = (2.0 * *x * *x * c.iir).sin();
                let mut bridge = x.clamp(-1.2533141373155, 1.2533141373155);
                bridge =
                    (bridge * bridge.abs()).sin() / if bridge == 0.0 { 1.0 } else { bridge.abs() };
                if *x > bridge {
                    c.iir += (*x - bridge) * 0.0009;
                }
                if *x < -bridge {
                    c.iir += (*x + bridge) * 0.0009;
                }
                *x = bridge;
                let mut temp = *x;
                let mut correction = 0.0;
                for stage in &mut c.filters {
                    *stage = (*stage * mid_alt) + (temp * mid);
                    temp -= *stage;
                    correction += *stage;
                }
                correction *= harmonic.abs();
                correction -= harmonic * old;
                *x -= correction;
                if *x < 0.0 {
                    *x = (*x * new) - ((-*x).sin() * old);
                }
                *x *= target;
                c.gw_b = c.gw_a;
                c.gw_a = *x - c.gw_prev;
                temp = c.gw_a;
                temp *= gwa;
                temp += c.gw_b * gwb;
                correction = (*x - c.gw_prev) - temp;
                c.gw_prev = *x;
                *x -= correction;
                let dry = *x;
                if c.last >= clip {
                    if *x < clip {
                        c.last = (clip * hard) + (*x * soft);
                    } else {
                        c.last = clip;
                    }
                }
                if c.last <= -clip {
                    if *x > -clip {
                        c.last = (-clip * hard) + (*x * soft);
                    } else {
                        c.last = -clip;
                    }
                }
                if *x > clip {
                    if c.last < clip {
                        *x = (clip * hard) + (c.last * soft);
                    } else {
                        *x = clip;
                    }
                }
                if *x < -clip {
                    if c.last > -clip {
                        *x = (-clip * hard) + (c.last * soft);
                    } else {
                        *x = -clip;
                    }
                }
                c.last = dry;
                if depth == 3 {
                    *x = fpd_dither(*x, &mut c.fpd);
                } else {
                    *x = naturalize(*x, depth, dry, c);
                }
                if *x > clip {
                    *x = clip;
                }
                if *x < -clip {
                    *x = -clip;
                }
            }
            *out_l = samples[0] as f32;
            *out_r = samples[1] as f32;
        }
    }
}
