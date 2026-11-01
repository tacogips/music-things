//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/CStrip, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Treble",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Mid",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Bass",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Lowpass",
        label: "Khz",
        default: 1.0,
    },
    ParamInfo {
        name: "TrebFrq",
        label: "Khz",
        default: 0.4,
    },
    ParamInfo {
        name: "BassFrq",
        label: "hz",
        default: 0.4,
    },
    ParamInfo {
        name: "Hipass",
        label: "hz",
        default: 0.0,
    },
    ParamInfo {
        name: "Gate",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Compres",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "CompSpd",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "TimeLag",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "OutGain",
        label: "dB",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "CStrip",
    upstream_name: "CStrip",
    description: "CStrip is an Airwindows channel strip.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(CStrip::new(rate, seed))
}
#[derive(Clone, Copy)]
struct Channel {
    last: f64,
    last2: f64,
    high: [f64; 6],
    low: [f64; 6],
    triplet: [f64; 3],
    hp: [[f64; 2]; 4],
    hp_tail: [f64; 2],
    lp: [[f64; 2]; 4],
    lp_tail: [f64; 3],
    negative: bool,
    crossing: i32,
    roller: f64,
    gate: f64,
    control: [[f64; 2]; 2],
    target: [f64; 2],
    avg: [f64; 2],
}
impl Channel {
    fn new() -> Self {
        Self {
            last: 0.0,
            last2: 0.0,
            high: [0.0; 6],
            low: [0.0; 6],
            triplet: [0.0; 3],
            hp: [[0.0; 2]; 4],
            hp_tail: [0.0; 2],
            lp: [[0.0; 2]; 4],
            lp_tail: [0.0; 3],
            negative: false,
            crossing: 0,
            roller: 0.0,
            gate: 0.0,
            control: [[1.0; 2]; 2],
            target: [1.0; 2],
            avg: [0.0; 2],
        }
    }
}
struct CStrip {
    params: [f32; 12],
    rate: f32,
    seed: u32,
    ch: [Channel; 2],
    delay: [Box<[f64]>; 2],
    count: usize,
    flip: bool,
    flipthree: usize,
    fpd: [u32; 2],
}
impl CStrip {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.5, 0.5, 1.0, 0.4, 0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5],
            rate,
            seed,
            ch: [Channel::new(); 2],
            delay: [
                vec![0.0; 4099].into_boxed_slice(),
                vec![0.0; 4099].into_boxed_slice(),
            ],
            count: 0,
            flip: false,
            flipthree: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for CStrip {
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
        if i >= 12 {
            return String::new();
        }
        let p = self.params[i];
        let x = match i {
            0..=2 => (p as f64 * 24.0 - 12.0) as f32,
            3..=4 => ((p * p) as f64 * 15.0 + 1.0) as f32,
            5..=6 => ((p * p) as f64 * 1570.0 + 30.0) as f32,
            11 => (p as f64 * 36.0 - 18.0) as f32,
            _ => p,
        };
        float2string(x)
    }
    fn reset(&mut self) {
        self.ch = [Channel::new(); 2];
        self.delay[0].fill(0.0);
        self.delay[1].fill(0.0);
        self.count = 0;
        self.flip = false;
        self.flipthree = 0;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    #[allow(
        clippy::approx_constant,
        clippy::excessive_precision,
        clippy::needless_range_loop
    )]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let p = self.params;
        let rate = self.rate as f64;
        let compscale = (1.0 / 44100.0) * rate;
        let mut density = [0.0; 3];
        let mut engage_eq = false;
        for i in 0..3 {
            density[i] = p[i] as f64 * 12.0 - 6.0;
            if density[i] != 0.0 {
                engage_eq = true;
            }
            density[i] = 10.0_f64.powf(density[i] / 20.0) - 1.0;
        }
        let lp0 = (p[3] * p[3]) as f64 * 15.0 + 1.0;
        let iir_c = (lp0 * 0.0188 + 0.7).min(1.0);
        let iir_a = ((p[4] * p[4]) as f64 * 15.0 + 1.0) * 1000.0 / rate;
        let iir_b = ((p[5] * p[5]) as f64 * 1570.0 + 30.0) * 10.0 / rate;
        let hp0 = (p[6] * p[6]) as f64 * 1570.0 + 30.0;
        let iir_d = hp0 / rate;
        let threshold = (p[7] as f64).powf(4.0) / 3.0 + 0.00018;
        let offthreshold = threshold * 1.1;
        let offset = (p[10] as f64).powf(5.0) * 700.0;
        let near = offset.abs().floor() as usize;
        let farlevel = offset.abs() - near as f64;
        let nearlevel = 1.0 - farlevel;
        let inputgain = (p[8] as f64).powf(4.0) * 35.0 + 1.0;
        let mut compout = inputgain;
        compout -= 1.0;
        compout /= 1.2;
        compout += 1.0;
        let mut divisor = (0.008 * (p[9] as f64).powf(2.0)) + 0.0004;
        divisor /= compscale;
        let remainder = divisor;
        divisor = 1.0 - divisor;
        let outputgain = 10.0_f64.powf(((p[11] as f64 * 36.0) - 18.0) / 20.0);
        for i in 0..il.len().min(ir.len()).min(ol.len()).min(or.len()) {
            let input = [il[i], ir[i]];
            let mut out = [0.0_f32; 2];
            let mut x = [input[0] as f64, input[1] as f64];
            for c in 0..2 {
                if x[c].abs() < 1.18e-23 {
                    x[c] = self.fpd[c] as f64 * 1.18e-17;
                }
                self.ch[c].last2 = self.ch[c].last;
                self.ch[c].last = x[c];
            }
            if threshold > 0.00018 {
                for c in 0..2 {
                    let s = &mut self.ch[c];
                    if x[c] > 0.0 {
                        if s.negative {
                            s.crossing = (220.9_f64 * 0.3) as i32;
                        }
                        s.negative = false;
                    } else {
                        s.crossing += 1;
                        s.negative = true;
                    }
                    if s.crossing as f64 > 220.9 {
                        s.crossing = 220;
                    }
                    if s.gate == 0.0 {
                        if x[c].abs() > threshold {
                            if s.roller == 0.0 {
                                s.roller = s.crossing as f64;
                            } else {
                                s.roller -= 0.028331119964586;
                            }
                        } else {
                            s.roller -= 0.028331119964586;
                        }
                    } else if x[c].abs() > offthreshold {
                        if s.roller < s.crossing as f64 {
                            s.roller = s.crossing as f64;
                        } else {
                            s.roller -= 0.028331119964586;
                        }
                    } else {
                        s.roller -= 0.028331119964586;
                    }
                    if s.roller < 0.0 {
                        s.roller = 0.0;
                    }
                    if s.roller < 1.0 {
                        s.gate = s.roller;
                        let bridge = 1.0 - x[c].abs().cos();
                        if x[c] > 0.0 {
                            x[c] = x[c] * s.gate + bridge * (1.0 - s.gate);
                        } else {
                            x[c] = x[c] * s.gate - bridge * (1.0 - s.gate);
                        }
                        if s.gate == 0.0 {
                            x[c] = 0.0;
                        }
                    } else {
                        s.gate = 1.0;
                    }
                }
            }
            self.flip = !self.flip;
            self.flipthree += 1;
            if self.flipthree > 3 {
                self.flipthree = 1;
            }
            let branch = if self.flip { 0 } else { 1 };
            if hp0 > 30.01 {
                for c in 0..2 {
                    let s = &mut self.ch[c];
                    for j in 0..4 {
                        s.hp[j][branch] = s.hp[j][branch] * (1.0 - iir_d) + x[c] * iir_d;
                        x[c] -= s.hp[j][branch];
                    }
                    for j in 0..2 {
                        s.hp_tail[j] = s.hp_tail[j] * (1.0 - iir_d) + x[c] * iir_d;
                        x[c] -= s.hp_tail[j];
                    }
                }
            }
            if inputgain > 1.0 {
                for c in 0..2 {
                    let s = &mut self.ch[c];
                    x[c] *= inputgain;
                    let mut pos = x[c] * 0.618033988749894848204586
                        + s.avg[0] * (1.0 - 0.618033988749894848204586)
                        + 1.0;
                    s.avg[0] = x[c];
                    if pos < 0.0 {
                        pos = 0.0;
                    }
                    let outpos = (pos / 2.0).min(1.0);
                    pos *= pos;
                    s.target[0] *= divisor;
                    s.target[0] += pos * remainder;
                    let calcpos = (1.0 / s.target[0]).powf(2.0);
                    let mut neg = -x[c] * 0.618033988749894848204586
                        + s.avg[1] * (1.0 - 0.618033988749894848204586)
                        + 1.0;
                    s.avg[1] = -x[c];
                    if neg < 0.0 {
                        neg = 0.0;
                    }
                    let outneg = (neg / 2.0).min(1.0);
                    neg *= neg;
                    s.target[1] *= divisor;
                    s.target[1] += neg * remainder;
                    let calcneg = (1.0 / s.target[1]).powf(2.0);
                    let side = if x[c] > 0.0 { 0 } else { 1 };
                    let calc = if side == 0 { calcpos } else { calcneg };
                    s.control[branch][side] *= divisor;
                    s.control[branch][side] += calc * remainder;
                    let total = s.control[branch][0] * outpos + s.control[branch][1] * outneg;
                    x[c] *= total;
                    x[c] /= compout;
                }
            }
            if engage_eq {
                for c in 0..2 {
                    let s = &mut self.ch[c];
                    let t = self.flipthree - 1;
                    let prev = (t + 2) % 3;
                    let factor = s.last2 - x[c];
                    s.triplet[t] += factor;
                    s.triplet[prev] -= factor;
                    let tf = s.triplet[t] * -density[0];
                    let idx = 2 + t;
                    s.high[idx] = s.high[idx] * (1.0 - iir_a) + x[c] * iir_a;
                    let mut high = x[c] - s.high[idx];
                    s.low[idx] = s.low[idx] * (1.0 - iir_b) + x[c] * iir_b;
                    let mut bass = s.low[idx];
                    for v in &mut s.triplet {
                        *v /= 2.0;
                    }
                    high += tf;
                    s.high[branch] = s.high[branch] * (1.0 - iir_a) + high * iir_a;
                    high -= s.high[branch];
                    s.low[branch] = s.low[branch] * (1.0 - iir_b) + bass * iir_b;
                    bass = s.low[branch];
                    s.high[5] = s.high[5] * (1.0 - iir_a) + high * iir_a;
                    high -= s.high[5];
                    s.low[5] = s.low[5] * (1.0 - iir_b) + bass * iir_b;
                    bass = s.low[5];
                    let mid = (x[c] - bass) - high;
                    let mut bands = [high, mid, bass];
                    for k in 0..3 {
                        bands[k] *= density[k] + 1.0;
                        let mut bridge = bands[k].abs() * 1.57079633;
                        if bridge > 1.57079633 {
                            bridge = 1.57079633;
                        }
                        if density[k] > 0.0 {
                            bridge = bridge.sin();
                        } else {
                            bridge = 1.0 - bridge.cos();
                        }
                        if bands[k] > 0.0 {
                            bands[k] =
                                bands[k] * (1.0 - density[k].abs()) + bridge * density[k].abs();
                        } else {
                            bands[k] =
                                bands[k] * (1.0 - density[k].abs()) - bridge * density[k].abs();
                        }
                    }
                    x[c] = bands[1];
                    x[c] += bands[0];
                    x[c] += bands[2];
                }
            }
            if offset > 0.0 {
                if self.count < 1 || self.count > 2048 {
                    self.count = 2048;
                }
                for c in 0..2 {
                    self.delay[c][self.count] = x[c];
                    self.delay[c][self.count + 2048] = x[c];
                    x[c] = self.delay[c][self.count + near] * nearlevel;
                    x[c] += self.delay[c][self.count + near + 1] * farlevel;
                }
                self.count -= 1;
            }
            if lp0 < 15.99 {
                for c in 0..2 {
                    let s = &mut self.ch[c];
                    for j in 0..4 {
                        s.lp[j][branch] = s.lp[j][branch] * (1.0 - iir_c) + x[c] * iir_c;
                        x[c] = s.lp[j][branch];
                    }
                    s.lp_tail[branch] = s.lp_tail[branch] * (1.0 - iir_c) + x[c] * iir_c;
                    x[c] = s.lp_tail[branch];
                    s.lp_tail[2] = s.lp_tail[2] * (1.0 - iir_c) + x[c] * iir_c;
                    x[c] = s.lp_tail[2] * (1.0 - iir_c) + x[c] * iir_c;
                }
            }
            for c in 0..2 {
                if outputgain != 1.0 {
                    x[c] *= outputgain;
                }
                out[c] = fpd_dither(x[c], &mut self.fpd[c]) as f32;
            }
            ol[i] = out[0];
            or[i] = out[1];
        }
    }
}
