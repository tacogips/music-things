//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Neverland, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Hardns",
        label: "",
        default: 0.74,
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
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Neverland",
    upstream_name: "Neverland",
    description: "Neverland is a re-release of my old Neve-style color adder, exacly as it was.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Neverland::new(rate, seed))
}
struct Neverland {
    params: [f32; 4],
    rate: f32,
    seed: u32,
    left: [f64; 34],
    right: [f64; 34],
    last_l: f64,
    last_r: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl Neverland {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.74, 0.3333333, 0.3333333, 1.0],
            rate,
            seed,
            left: [0.0; 34],
            right: [0.0; 34],
            last_l: 0.0,
            last_r: 0.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
#[allow(clippy::excessive_precision)]
fn convolve(mut sample: f64, history: &mut [f64; 34], drive: f64) -> f64 {
    history.copy_within(0..33, 1);
    history[0] = sample * drive;
    sample += history[1] * (0.20641602693167951 - (0.00078952185394898 * history[1].abs()));
    sample -= history[2] * (0.07601816702459827 + (0.00022786334179951 * history[2].abs()));
    sample += history[3] * (0.03929765560019285 - (0.00054517993246352 * history[3].abs()));
    sample += history[4] * (0.00298333157711103 - (0.00033083756545638 * history[4].abs()));
    sample -= history[5] * (0.00724006282304610 + (0.00045483683460812 * history[5].abs()));
    sample += history[6] * (0.03073108963506036 - (0.00038190060537423 * history[6].abs()));
    sample -= history[7] * (0.02332434692533051 + (0.00040347288688932 * history[7].abs()));
    sample += history[8] * (0.03792606869061214 - (0.00039673687335892 * history[8].abs()));
    sample -= history[9] * (0.02437059376675688 + (0.00037221210539535 * history[9].abs()));
    sample += history[10] * (0.03416764311979521 - (0.00040314850796953 * history[10].abs()));
    sample -= history[11] * (0.01761669868102127 + (0.00035989484330131 * history[11].abs()));
    sample += history[12] * (0.02538237753523052 - (0.00040149119125394 * history[12].abs()));
    sample -= history[13] * (0.00770737340728377 + (0.00035462118723555 * history[13].abs()));
    sample += history[14] * (0.01580706228482803 - (0.00037563141307594 * history[14].abs()));
    sample += history[15] * (0.00055119240005586 - (0.00035409299268971 * history[15].abs()));
    sample += history[16] * (0.00818552143438768 - (0.00036507661042180 * history[16].abs()));
    sample += history[17] * (0.00661842703548304 - (0.00034550528559056 * history[17].abs()));
    sample += history[18] * (0.00362447476272098 - (0.00035553012761240 * history[18].abs()));
    sample += history[19] * (0.00957098027225745 - (0.00034091691045338 * history[19].abs()));
    sample += history[20] * (0.00193621774016660 - (0.00034554529131668 * history[20].abs()));
    sample += history[21] * (0.01005433027357935 - (0.00033878223153845 * history[21].abs()));
    sample += history[22] * (0.00221712428802004 - (0.00033481410137711 * history[22].abs()));
    sample += history[23] * (0.00911255639207995 - (0.00033263425232666 * history[23].abs()));
    sample += history[24] * (0.00339667169034909 - (0.00032634428038430 * history[24].abs()));
    sample += history[25] * (0.00774096948249924 - (0.00032599868802996 * history[25].abs()));
    sample += history[26] * (0.00463907626773794 - (0.00032131993173361 * history[26].abs()));
    sample += history[27] * (0.00658222997260378 - (0.00032014977430211 * history[27].abs()));
    sample += history[28] * (0.00550347079924993 - (0.00031557153256653 * history[28].abs()));
    sample += history[29] * (0.00588754981375325 - (0.00032041307242303 * history[29].abs()));
    sample += history[30] * (0.00590293898419892 - (0.00030457857428714 * history[30].abs()));
    sample += history[31] * (0.00558952010441800 - (0.00030448053548086 * history[31].abs()));
    sample += history[32] * (0.00598183557634295 - (0.00030715064323181 * history[32].abs()));
    sample += history[33] * (0.00555223929714115 - (0.00030319367948553 * history[33].abs()));
    sample
}
fn distort(sample: f64, threshold: f64, hardness: f64, breakup: f64) -> f64 {
    if sample.abs() > threshold {
        let bridge = ((sample.abs() - threshold) * hardness).min(breakup).sin() / hardness;
        if sample > 0.0 {
            bridge + threshold
        } else {
            -(bridge + threshold)
        }
    } else {
        sample
    }
}
impl AirwindowsEffect for Neverland {
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
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let threshold = a as f64;
        let breakup = (1.0 - threshold / 2.0) * 3.14159265358979;
        let mut sqdrive = (b as f64 * 3.0) * 0.788;
        if sqdrive > 1.0 {
            sqdrive *= sqdrive;
        }
        sqdrive = sqdrive.sqrt();
        let mut indrive = c as f64 * 3.0;
        if indrive > 1.0 {
            indrive *= indrive;
        }
        indrive /= 1.0 + 0.2 * sqdrive;
        let hardness = if threshold < 1.0 {
            1.0 / (1.0 - threshold)
        } else {
            999999999999999999999.0
        };
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            l = distort(l * indrive, threshold, hardness, breakup);
            r = distort(r * indrive, threshold, hardness, breakup);
            if sqdrive > 0.0 {
                l = convolve(l, &mut self.left, sqdrive);
                r = convolve(r, &mut self.right, sqdrive);
            }
            let rand_l = self.fpd_l as f64 / u32::MAX as f64 * 0.034;
            l = (l * (1.0 - rand_l) + self.last_l * rand_l) * d as f64;
            self.last_l = l;
            let rand_r = self.fpd_r as f64 / u32::MAX as f64 * 0.034;
            r = (r * (1.0 - rand_r) + self.last_r * rand_r) * d as f64;
            self.last_r = r;
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
