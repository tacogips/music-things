//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Luxor at b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
#![allow(clippy::excessive_precision, clippy::approx_constant)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Hardns",
        label: "",
        default: 0.70,
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
];
const INFO: PluginInfo = PluginInfo {
    name: "Luxor",
    upstream_name: "Luxor",
    description: "Luxor is a re-release of another old Character plugin.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Luxor::new(rate, seed))
}
struct Luxor {
    rate: f32,
    seed: u32,
    params: [f32; 4],
    history: [[f64; 34]; 2],
    last: [f64; 2],
    fpd: [u32; 2],
}
impl Luxor {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            rate,
            seed,
            params: [0.70, 0.3333333, 0.3333333, 1.0],
            history: [[0.0; 34]; 2],
            last: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn convolution(mut input: f64, h: &mut [f64; 34], drive: f64) -> f64 {
    h.copy_within(0..33, 1);
    h[0] = input * drive;
    input -= h[1] * (0.20641602693167951 - (0.00078952185394898 * h[1].abs()));
    input += h[2] * (0.07601816702459827 + (0.00022786334179951 * h[2].abs()));
    input -= h[3] * (0.03929765560019285 - (0.00054517993246352 * h[3].abs()));
    input -= h[4] * (0.00298333157711103 - (0.00033083756545638 * h[4].abs()));
    input += h[5] * (0.00724006282304610 + (0.00045483683460812 * h[5].abs()));
    input -= h[6] * (0.03073108963506036 - (0.00038190060537423 * h[6].abs()));
    input += h[7] * (0.02332434692533051 + (0.00040347288688932 * h[7].abs()));
    input -= h[8] * (0.03792606869061214 - (0.00039673687335892 * h[8].abs()));
    input += h[9] * (0.02437059376675688 + (0.00037221210539535 * h[9].abs()));
    input -= h[10] * (0.03416764311979521 - (0.00040314850796953 * h[10].abs()));
    input += h[11] * (0.01761669868102127 + (0.00035989484330131 * h[11].abs()));
    input -= h[12] * (0.02538237753523052 - (0.00040149119125394 * h[12].abs()));
    input += h[13] * (0.00770737340728377 + (0.00035462118723555 * h[13].abs()));
    input -= h[14] * (0.01580706228482803 - (0.00037563141307594 * h[14].abs()));
    input -= h[15] * (0.00055119240005586 - (0.00035409299268971 * h[15].abs()));
    input -= h[16] * (0.00818552143438768 - (0.00036507661042180 * h[16].abs()));
    input -= h[17] * (0.00661842703548304 - (0.00034550528559056 * h[17].abs()));
    input -= h[18] * (0.00362447476272098 - (0.00035553012761240 * h[18].abs()));
    input -= h[19] * (0.00957098027225745 - (0.00034091691045338 * h[19].abs()));
    input -= h[20] * (0.00193621774016660 - (0.00034554529131668 * h[20].abs()));
    input -= h[21] * (0.01005433027357935 - (0.00033878223153845 * h[21].abs()));
    input -= h[22] * (0.00221712428802004 - (0.00033481410137711 * h[22].abs()));
    input -= h[23] * (0.00911255639207995 - (0.00033263425232666 * h[23].abs()));
    input -= h[24] * (0.00339667169034909 - (0.00032634428038430 * h[24].abs()));
    input -= h[25] * (0.00774096948249924 - (0.00032599868802996 * h[25].abs()));
    input -= h[26] * (0.00463907626773794 - (0.00032131993173361 * h[26].abs()));
    input -= h[27] * (0.00658222997260378 - (0.00032014977430211 * h[27].abs()));
    input -= h[28] * (0.00550347079924993 - (0.00031557153256653 * h[28].abs()));
    input -= h[29] * (0.00588754981375325 - (0.00032041307242303 * h[29].abs()));
    input -= h[30] * (0.00590293898419892 - (0.00030457857428714 * h[30].abs()));
    input -= h[31] * (0.00558952010441800 - (0.00030448053548086 * h[31].abs()));
    input -= h[32] * (0.00598183557634295 - (0.00030715064323181 * h[32].abs()));
    input -= h[33] * (0.00555223929714115 - (0.00030319367948553 * h[33].abs()));
    input
}
impl AirwindowsEffect for Luxor {
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
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let threshold = a as f64;
        let breakup = (1.0 - threshold / 2.0) * 3.14159265358979;
        let hardness = if threshold < 1.0 {
            1.0 / (1.0 - threshold)
        } else {
            999999999999999999999.0
        };
        let mut sqdrive = b as f64 * 3.0;
        if sqdrive > 1.0 {
            sqdrive *= sqdrive;
        }
        sqdrive = sqdrive.sqrt();
        let mut indrive = c as f64 * 3.0;
        if indrive > 1.0 {
            indrive *= indrive;
        }
        indrive *= 1.0 + 0.1935 * sqdrive;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut outputs = [0.0; 2];
            for (ch, s) in [l, r].into_iter().enumerate() {
                let mut input = s as f64;
                if input.abs() < 1.18e-23 {
                    input = self.fpd[ch] as f64 * 1.18e-17;
                }
                input *= indrive;
                if sqdrive > 0.0 {
                    input = convolution(input, &mut self.history[ch], sqdrive);
                }
                if input.abs() > threshold {
                    let bridge =
                        ((input.abs() - threshold) * hardness).min(breakup).sin() / hardness;
                    input = if input > 0.0 {
                        bridge + threshold
                    } else {
                        -(bridge + threshold)
                    };
                }
                let randy = self.fpd[ch] as f64 / u32::MAX as f64 * 0.031;
                input = (input * (1.0 - randy) + self.last[ch] * randy) * d as f64;
                self.last[ch] = input;
                outputs[ch] = fpd_dither(input, &mut self.fpd[ch]) as f32;
            }
            *outl = outputs[0];
            *outr = outputs[1];
        }
    }
}
