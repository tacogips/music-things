//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ToTape5, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Louder",
        label: "",
        default: 0.25,
    },
    ParamInfo {
        name: "Softer",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Fatter",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Flutter",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Output",
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
    name: "ToTape5",
    upstream_name: "ToTape5",
    description: "ToTape5 is Airwindows analog tape emulation.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ToTape5::new(rate, seed))
}
struct ToTape5 {
    params: [f32; 6],
    sample_rate: f32,
    seed: u32,
    gcount: i32,
    rateof: f64,
    sweep: f64,
    nextmax: f64,
    hcount: i32,
    flip: i32,
    d: [[f64; 1000]; 2],
    e: [[f64; 1000]; 2],
    mid: [[f64; 3]; 2],
    bump: [[f64; 3]; 2],
    min_bump: [f64; 2],
    iir: [[f64; 26]; 2],
    fpd: [u32; 2],
}
impl ToTape5 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.25, 0.0, 0.0, 0.0, 1.0, 1.0],
            sample_rate,
            seed,
            gcount: 0,
            rateof: 0.5,
            sweep: 0.0,
            nextmax: 0.5,
            hcount: 0,
            flip: 0,
            d: [[0.0; 1000]; 2],
            e: [[0.0; 1000]; 2],
            mid: [[0.0; 3]; 2],
            bump: [[0.0; 3]; 2],
            min_bump: [0.0; 2],
            iir: [[0.0; 26]; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for ToTape5 {
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
            .map_or_else(String::new, |&x| float2string(x))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    #[allow(
        clippy::excessive_precision,
        clippy::approx_constant,
        clippy::needless_range_loop
    )]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e, f] = self.params;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let fp_old = 0.618033988749894848204586;
        let inputgain = (a as f64 + 1.0).powf(3.0);
        let outputgain = e as f64;
        let wet = f as f64;
        let trim = 0.211324865405187117745425;
        let soften = (b as f64).powf(2.0);
        let temp_randy = 0.06 + soften / 10.0;
        let roll = (1.0 - soften * 0.45) / overallscale;
        let mut head = (c as f64).powf(2.0);
        let maxdelay = (((head + 0.3) * 2.2) * overallscale).floor() as i32;
        head *= head.abs();
        let head_freq = 0.044 / overallscale;
        let iir_amount = 0.000001 / overallscale;
        let alt = 1.0 - iir_amount;
        let boost = 0.0001 / overallscale;
        let depth = (d as f64).powf(2.0) * overallscale;
        let fluttertrim = 0.005 / overallscale;
        let sweeptrim = (0.0006 * depth) / overallscale;
        let newrate = 0.005 / overallscale;
        let oldrate = 1.0 - newrate;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            for ch in 0..2 {
                if input[ch].abs() < 1.18e-23 {
                    input[ch] = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            let dry = input;
            let flutterrandy = self.fpd[0] as f64 / u32::MAX as f64;
            let mut randy = flutterrandy * temp_randy;
            let invrandy = 1.0 - randy;
            randy /= 2.0;
            if self.gcount < 0 || self.gcount > 300 {
                self.gcount = 300;
            }
            let mut count = self.gcount as usize;
            for ch in 0..2 {
                self.d[ch][count] = input[ch];
                self.d[ch][count + 301] = input[ch];
            }
            self.gcount -= 1;
            if depth != 0.0 {
                let offset = (1.0 + self.sweep.sin()) * depth;
                count += offset.floor() as usize;
                for ch in 0..2 {
                    let frac = offset - offset.floor();
                    let mut v = self.d[ch][count] * (1.0 - frac);
                    v += self.d[ch][count + 1] * frac;
                    v -= (self.d[ch][count + 2] * frac) * trim;
                    input[ch] = v;
                }
                self.rateof = self.nextmax * newrate + self.rateof * oldrate;
                self.sweep += self.rateof * fluttertrim;
                self.sweep += self.sweep * sweeptrim;
                if self.sweep >= std::f64::consts::PI * 2.0 {
                    self.sweep = 0.0;
                    self.nextmax = 0.02 + flutterrandy * 0.98;
                }
            }
            if inputgain != 1.0 {
                input[0] *= inputgain;
                input[1] *= inputgain;
            }
            if self.flip < 1 || self.flip > 3 {
                self.flip = 1;
            }
            let index = (self.flip - 1) as usize;
            let mut subtract = [0.0; 2];
            for ch in 0..2 {
                self.mid[ch][index] = self.mid[ch][index] * (1.0 - roll) + input[ch] * roll;
                let weights = [
                    if index == 0 { invrandy } else { randy },
                    if index == 1 { invrandy } else { randy },
                    if index == 2 { invrandy } else { randy },
                ];
                self.mid[ch][index] = weights[0] * self.mid[ch][0]
                    + weights[1] * self.mid[ch][1]
                    + weights[2] * self.mid[ch][2];
                subtract[ch] = input[ch] - self.mid[ch][index];
                self.bump[ch][index] += input[ch] * 0.05;
                let v = self.bump[ch][index];
                self.bump[ch][index] -= v * v * v * head_freq;
                self.bump[ch][index] = weights[0] * self.bump[ch][0]
                    + weights[1] * self.bump[ch][1]
                    + weights[2] * self.bump[ch][2];
                let mut bridge = subtract[ch].abs() * 1.57079633;
                if bridge > 1.57079633 {
                    bridge = 1.57079633;
                }
                bridge = 1.0 - bridge.cos();
                if subtract[ch] > 0.0 {
                    subtract[ch] = bridge;
                }
                if subtract[ch] < 0.0 {
                    subtract[ch] = -bridge;
                }
                input[ch] -= subtract[ch];
                bridge = input[ch].abs();
                if bridge > 1.57079633 {
                    bridge = 1.57079633;
                }
                bridge = bridge.sin();
                if input[ch] > 0.0 {
                    input[ch] = bridge;
                }
                if input[ch] < 0.0 {
                    input[ch] = -bridge;
                }
            }
            self.flip += 1;
            let mut allpass = self.hcount - 1;
            if allpass < 0 || allpass > maxdelay {
                allpass = maxdelay;
            }
            let mut hb = [0.0; 2];
            for ch in 0..2 {
                hb[ch] = self.bump[ch][0] + self.bump[ch][1] + self.bump[ch][2];
                hb[ch] -= self.e[ch][allpass as usize] * fp_old;
                self.e[ch][self.hcount as usize] = hb[ch];
                input[ch] *= fp_old;
            }
            self.hcount -= 1;
            if self.hcount < 0 || self.hcount > maxdelay {
                self.hcount = maxdelay;
            }
            for ch in 0..2 {
                hb[ch] += self.e[ch][self.hcount as usize];
                subtract[ch] -= hb[ch] * (head + self.min_bump[ch]);
                let mut temp = input[ch];
                self.min_bump[ch] = self.min_bump[ch] * alt + input[ch].abs() * boost;
                if self.min_bump[ch] > 0.01 {
                    self.min_bump[ch] = 0.01;
                }
                for v in &mut self.iir[ch] {
                    *v = *v * alt + temp * iir_amount;
                    temp -= *v;
                    subtract[ch] += *v;
                }
                input[ch] -= subtract[ch];
                if outputgain != 1.0 {
                    input[ch] *= outputgain;
                }
                if wet != 1.0 {
                    input[ch] = input[ch] * wet + dry[ch] * (1.0 - wet);
                }
            }
            *outl = fpd_dither(input[0], &mut self.fpd[0]) as f32;
            *outr = fpd_dither(input[1], &mut self.fpd[1]) as f32;
        }
    }
}
