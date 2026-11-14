//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/IronOxideClassic, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
#![allow(clippy::approx_constant)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Input Trim",
        label: "dB",
        default: 0.5,
    },
    ParamInfo {
        name: "Tape Speed",
        label: "ips",
        default: 0.562_341_33,
    },
    ParamInfo {
        name: "Output Trim",
        label: "dB",
        default: 0.5,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "IronOxideClassic",
    upstream_name: "IronOxideClassic",
    description:
        "IronOxideClassic is the purer, simpler, early form of Iron Oxide before all the features.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(IronOxideClassic::new(sample_rate, seed))
}
struct IronOxideClassic {
    params: [f32; 3],
    sample_rate: f32,
    seed: u32,
    input_iir: [[f64; 2]; 2],
    fast: [[f64; 2]; 2],
    slow: [[f64; 2]; 2],
    delay: [[f64; 264]; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl IronOxideClassic {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.562_341_33, 0.5],
            sample_rate,
            seed,
            input_iir: [[0.0; 2]; 2],
            fast: [[0.0; 2]; 2],
            slow: [[0.0; 2]; 2],
            delay: [[0.0; 264]; 2],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn taper(d: &[f64; 264]) -> f64 {
    let count = 3;
    let mut t = d[count + 127];
    for i in [113, 109, 107, 103, 101, 97, 89, 83] {
        t += d[count + i];
    }
    t /= 2.0;
    for i in [79, 73, 71, 67, 61, 59, 53, 47, 43, 41, 37, 31, 29] {
        t += d[count + i];
    }
    t /= 2.0;
    for i in [23, 19, 17, 13, 11] {
        t += d[count + i];
    }
    t /= 2.0;
    for i in [7, 5, 3] {
        t += d[count + i];
    }
    t /= 2.0;
    t += d[count + 2];
    t += d[count + 1];
    t
}
impl AirwindowsEffect for IronOxideClassic {
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
        let [a, b, c] = self.params;
        match i {
            0 => float2string((a as f64 * 36.0 - 18.0) as f32),
            1 => float2string((((b * b) * (b * b)) as f64 * 148.5 + 1.5) as f32),
            2 => float2string((c as f64 * 36.0 - 18.0) as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c] = self.params;
        let scale = self.sample_rate as f64 / 44100.0;
        let ingain = 10.0f64.powf((a as f64 * 36.0 - 18.0) / 20.0);
        let outgain = 10.0f64.powf((c as f64 * 36.0 - 18.0) / 20.0);
        let mut ips = (((b * b) * (b * b)) as f64 * 148.5 + 1.5) * 1.1;
        if !(1.0..=200.0).contains(&ips) {
            ips = 33.0;
        }
        let mut iir = ips / 430.0;
        let mut fast = ips / 15.0;
        let mut slow = 2.0 / (ips * ips);
        let mut low = 5.0 / ips;
        if scale == 0.0 {
            fast += 1.0;
            slow += 1.0;
        } else {
            iir /= scale;
            low *= scale;
            fast = 1.0 + fast / scale;
            slow = 1.0 + slow / scale;
        }
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut output = [l as f64, r as f64];
            let side = if self.flip { 0 } else { 1 };
            for (ch, x) in output.iter_mut().enumerate() {
                if x.abs() < 1.18e-23 {
                    *x = self.fpd[ch] as f64 * 1.18e-17;
                }
                self.input_iir[ch][side] = self.input_iir[ch][side] * (1.0 - iir) + *x * iir;
                *x -= self.input_iir[ch][side];
                if ingain != 1.0 {
                    *x *= ingain;
                }
                let rect = x.abs().min(1.57079633).sin();
                *x = if *x > 0.0 { rect } else { -rect };
                self.delay[ch][0] = *x;
                self.delay[ch][131] = *x;
                self.fast[ch][side] /= fast;
                self.slow[ch][side] /= slow;
                self.fast[ch][side] += self.delay[ch][0];
                self.slow[ch][side] += taper(&self.delay[ch]) / 128.0;
                *x = self.fast[ch][side] - self.slow[ch][side] / slow;
                *x /= fast;
                *x /= low;
                let rect = x.abs().min(1.57079633).sin();
                *x = if *x > 0.0 { rect } else { -rect };
                if outgain != 1.0 {
                    *x *= outgain;
                }
                *x = fpd_dither(*x, &mut self.fpd[ch]);
            }
            self.flip = !self.flip;
            *outl = output[0] as f32;
            *outr = output[1] as f32;
        }
    }
}
