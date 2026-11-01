//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Cider/CiderProc.cpp
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.
#![allow(
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::too_many_arguments
)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Cider",
    upstream_name: "Cider",
    description: "Cider is a re-release of another old Character plugin.",
    params: &[
        ParamInfo {
            name: "Hardns",
            label: "",
            default: 0.7,
        },
        ParamInfo {
            name: "Persnlty",
            label: "",
            default: 0.3333333,
        },
        ParamInfo {
            name: "Drive",
            label: "",
            default: 0.3333333,
        },
        ParamInfo {
            name: "Output",
            label: "",
            default: 1.0,
        },
    ],
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Cider::new(rate, seed))
}
struct Cider {
    rate: f32,
    seed: u32,
    params: [f32; 4],
    b: [[f64; 34]; 2],
    last: [f64; 2],
    fpd: [u32; 2],
}
impl Cider {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            rate,
            seed,
            params: [0.7, 0.3333333, 0.3333333, 1.0],
            b: [[0.0; 34]; 2],
            last: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
    fn channel(
        &mut self,
        sample: f32,
        n: usize,
        threshold: f64,
        hardness: f64,
        breakup: f64,
        sqdrive: f64,
        indrive: f64,
        outlevel: f64,
    ) -> f32 {
        let mut input = sample as f64;
        if input.abs() < 1.18e-23 {
            input = self.fpd[n] as f64 * 1.18e-17;
        }
        input *= indrive;
        if sqdrive > 0.0 {
            let b = &mut self.b[n];
            for i in (1..34).rev() {
                b[i] = b[i - 1];
            }
            b[0] = input * sqdrive;
            input += b[1] * (0.61283288942201319 + (0.00024011410669522 * b[1].abs()));
            input -= b[2] * (0.24036380659761222 - (0.00020789518206241 * b[2].abs()));
            input += b[3] * (0.09104669761717916 + (0.00012829642741548 * b[3].abs()));
            input -= b[4] * (0.02378290768554025 - (0.00017673646470440 * b[4].abs()));
            input -= b[5] * (0.02832818490275965 - (0.00013536187747384 * b[5].abs()));
            input += b[6] * (0.03268797679215937 + (0.00015035126653359 * b[6].abs()));
            input -= b[7] * (0.04024464202655586 - (0.00015034923056735 * b[7].abs()));
            input += b[8] * (0.01864890074318696 + (0.00014513281680642 * b[8].abs()));
            input -= b[9] * (0.01632731954100322 - (0.00015509089075614 * b[9].abs()));
            input -= b[10] * (0.00318907090555589 - (0.00014784812076550 * b[10].abs()));
            input -= b[11] * (0.00208573465221869 - (0.00015350520779465 * b[11].abs()));
            input -= b[12] * (0.00907033901519614 - (0.00015442964157250 * b[12].abs()));
            input -= b[13] * (0.00199458794148013 - (0.00015595640046297 * b[13].abs()));
            input -= b[14] * (0.00705979153201755 - (0.00015730069418051 * b[14].abs()));
            input -= b[15] * (0.00429488975412722 - (0.00015743697943505 * b[15].abs()));
            input -= b[16] * (0.00497724878704936 - (0.00016014760011861 * b[16].abs()));
            input -= b[17] * (0.00506059305562353 - (0.00016194824072466 * b[17].abs()));
            input -= b[18] * (0.00483432223285621 - (0.00016329050124225 * b[18].abs()));
            input -= b[19] * (0.00495100420886005 - (0.00016297509798749 * b[19].abs()));
            input -= b[20] * (0.00489319520555115 - (0.00016472839684661 * b[20].abs()));
            input -= b[21] * (0.00489177657970308 - (0.00016791875866630 * b[21].abs()));
            input -= b[22] * (0.00487900894707044 - (0.00016755993898534 * b[22].abs()));
            input -= b[23] * (0.00486234009335561 - (0.00016968157345446 * b[23].abs()));
            input -= b[24] * (0.00485737490288736 - (0.00017180713324431 * b[24].abs()));
            input -= b[25] * (0.00484106070563455 - (0.00017251073661092 * b[25].abs()));
            input -= b[26] * (0.00483219429408410 - (0.00017321683790891 * b[26].abs()));
            input -= b[27] * (0.00482013597437550 - (0.00017392186866488 * b[27].abs()));
            input -= b[28] * (0.00480949628051497 - (0.00017569098775602 * b[28].abs()));
            input -= b[29] * (0.00479992055604049 - (0.00017746046369449 * b[29].abs()));
            input -= b[30] * (0.00478750757986987 - (0.00017745630047554 * b[30].abs()));
            input -= b[31] * (0.00477828651185740 - (0.00017958043287604 * b[31].abs()));
            input -= b[32] * (0.00476906544384494 - (0.00018170456527653 * b[32].abs()));
            input -= b[33] * (0.00475700712413634 - (0.00018099144598088 * b[33].abs()));
        }
        if input.abs() > threshold {
            let mut bridge = (input.abs() - threshold) * hardness;
            if bridge > breakup {
                bridge = breakup;
            }
            bridge = bridge.sin() / hardness;
            if input > 0.0 {
                input = bridge + threshold;
            } else {
                input = -(bridge + threshold);
            }
        }
        let randy = (self.fpd[n] as f64 / u32::MAX as f64) * 0.057;
        input = ((input * (1.0 - randy)) + (self.last[n] * randy)) * outlevel;
        self.last[n] = input;
        fpd_dither(input, &mut self.fpd[n]) as f32
    }
}
impl AirwindowsEffect for Cider {
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
            0 | 3 => float2string(self.params[i]),
            1 | 2 => float2string((self.params[i] as f64 * 3.0) as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let threshold = self.params[0] as f64;
        let breakup = (1.0 - (threshold / 2.0)) * 3.14159265358979;
        let mut sqdrive = self.params[1] as f64 * 3.0;
        if sqdrive > 1.0 {
            sqdrive *= sqdrive;
        }
        sqdrive = sqdrive.sqrt();
        let mut indrive = self.params[2] as f64 * 3.0;
        if indrive > 1.0 {
            indrive *= indrive;
        }
        indrive *= 1.0 - (0.216 * sqdrive);
        let outlevel = self.params[3] as f64;
        let hardness = if threshold < 1.0 {
            1.0 / (1.0 - threshold)
        } else {
            999999999999999999999.0
        };
        for (((&l, &r), ol), or) in l.iter().zip(r).zip(ol).zip(or) {
            *ol = self.channel(
                l, 0, threshold, hardness, breakup, sqdrive, indrive, outlevel,
            );
            *or = self.channel(
                r, 1, threshold, hardness, breakup, sqdrive, indrive, outlevel,
            );
        }
    }
}
