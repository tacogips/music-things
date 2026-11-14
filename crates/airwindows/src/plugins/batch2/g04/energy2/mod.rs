//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license.
//! Upstream: airwindows/plugins/LinuxVST/src/Energy2, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(non_snake_case)]
mod bands;
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Hiss",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Glitter",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Rat",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Fizz",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Scrape",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Chug",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Yowr",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Snarl",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "I/Dr/Wt",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Energy2",
    upstream_name: "Energy2",
    description: "Energy2 is electrifying fixed-frequency treble boosts for high sample rate.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Energy2::new(sample_rate, seed))
}
struct Energy2 {
    params: [f32; 9],
    sample_rate: f32,
    seed: u32,
    fpdL: u32,
    fpdR: u32,
    lastRefL: [f64; 10],
    lastRefR: [f64; 10],
    cycle: i32,
    duoEvenL: f64,
    duoOddL: f64,
    duoFactorL: f64,
    flip: bool,
    tripletAL: f64,
    tripletBL: f64,
    tripletCL: f64,
    tripletFactorL: f64,
    countA: i32,
    quadAL: f64,
    quadBL: f64,
    quadCL: f64,
    quadDL: f64,
    quadFactorL: f64,
    countB: i32,
    quintAL: f64,
    quintBL: f64,
    quintCL: f64,
    quintDL: f64,
    quintEL: f64,
    quintFactorL: f64,
    countC: i32,
    sextAL: f64,
    sextBL: f64,
    sextCL: f64,
    sextDL: f64,
    sextEL: f64,
    sextFL: f64,
    sextFactorL: f64,
    countD: i32,
    septAL: f64,
    septBL: f64,
    septCL: f64,
    septDL: f64,
    septEL: f64,
    septFL: f64,
    septGL: f64,
    septFactorL: f64,
    countE: i32,
    octAL: f64,
    octBL: f64,
    octCL: f64,
    octDL: f64,
    octEL: f64,
    octFL: f64,
    octGL: f64,
    octHL: f64,
    octFactorL: f64,
    countF: i32,
    nintAL: f64,
    nintBL: f64,
    nintCL: f64,
    nintDL: f64,
    nintEL: f64,
    nintFL: f64,
    nintGL: f64,
    nintHL: f64,
    nintIL: f64,
    nintFactorL: f64,
    countG: i32,
    PrevHL: f64,
    PrevGL: f64,
    PrevFL: f64,
    PrevEL: f64,
    PrevDL: f64,
    PrevCL: f64,
    PrevBL: f64,
    PrevAL: f64,
    duoEvenR: f64,
    duoOddR: f64,
    duoFactorR: f64,
    tripletAR: f64,
    tripletBR: f64,
    tripletCR: f64,
    tripletFactorR: f64,
    quadAR: f64,
    quadBR: f64,
    quadCR: f64,
    quadDR: f64,
    quadFactorR: f64,
    quintAR: f64,
    quintBR: f64,
    quintCR: f64,
    quintDR: f64,
    quintER: f64,
    quintFactorR: f64,
    sextAR: f64,
    sextBR: f64,
    sextCR: f64,
    sextDR: f64,
    sextER: f64,
    sextFR: f64,
    sextFactorR: f64,
    septAR: f64,
    septBR: f64,
    septCR: f64,
    septDR: f64,
    septER: f64,
    septFR: f64,
    septGR: f64,
    septFactorR: f64,
    octAR: f64,
    octBR: f64,
    octCR: f64,
    octDR: f64,
    octER: f64,
    octFR: f64,
    octGR: f64,
    octHR: f64,
    octFactorR: f64,
    nintAR: f64,
    nintBR: f64,
    nintCR: f64,
    nintDR: f64,
    nintER: f64,
    nintFR: f64,
    nintGR: f64,
    nintHR: f64,
    nintIR: f64,
    nintFactorR: f64,
    PrevHR: f64,
    PrevGR: f64,
    PrevFR: f64,
    PrevER: f64,
    PrevDR: f64,
    PrevCR: f64,
    PrevBR: f64,
    PrevAR: f64,
}
impl Energy2 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 1.0],
            sample_rate,
            seed,
            fpdL: fpd_from_seed(seed, 0),
            fpdR: fpd_from_seed(seed, 1),
            lastRefL: [0.0; 10],
            lastRefR: [0.0; 10],
            cycle: 0,
            duoEvenL: 0.0,
            duoOddL: 0.0,
            duoFactorL: 0.0,
            flip: false,
            tripletAL: 0.0,
            tripletBL: 0.0,
            tripletCL: 0.0,
            tripletFactorL: 0.0,
            countA: 1,
            quadAL: 0.0,
            quadBL: 0.0,
            quadCL: 0.0,
            quadDL: 0.0,
            quadFactorL: 0.0,
            countB: 1,
            quintAL: 0.0,
            quintBL: 0.0,
            quintCL: 0.0,
            quintDL: 0.0,
            quintEL: 0.0,
            quintFactorL: 0.0,
            countC: 1,
            sextAL: 0.0,
            sextBL: 0.0,
            sextCL: 0.0,
            sextDL: 0.0,
            sextEL: 0.0,
            sextFL: 0.0,
            sextFactorL: 0.0,
            countD: 1,
            septAL: 0.0,
            septBL: 0.0,
            septCL: 0.0,
            septDL: 0.0,
            septEL: 0.0,
            septFL: 0.0,
            septGL: 0.0,
            septFactorL: 0.0,
            countE: 1,
            octAL: 0.0,
            octBL: 0.0,
            octCL: 0.0,
            octDL: 0.0,
            octEL: 0.0,
            octFL: 0.0,
            octGL: 0.0,
            octHL: 0.0,
            octFactorL: 0.0,
            countF: 1,
            nintAL: 0.0,
            nintBL: 0.0,
            nintCL: 0.0,
            nintDL: 0.0,
            nintEL: 0.0,
            nintFL: 0.0,
            nintGL: 0.0,
            nintHL: 0.0,
            nintIL: 0.0,
            nintFactorL: 0.0,
            countG: 1,
            PrevHL: 0.0,
            PrevGL: 0.0,
            PrevFL: 0.0,
            PrevEL: 0.0,
            PrevDL: 0.0,
            PrevCL: 0.0,
            PrevBL: 0.0,
            PrevAL: 0.0,
            duoEvenR: 0.0,
            duoOddR: 0.0,
            duoFactorR: 0.0,
            tripletAR: 0.0,
            tripletBR: 0.0,
            tripletCR: 0.0,
            tripletFactorR: 0.0,
            quadAR: 0.0,
            quadBR: 0.0,
            quadCR: 0.0,
            quadDR: 0.0,
            quadFactorR: 0.0,
            quintAR: 0.0,
            quintBR: 0.0,
            quintCR: 0.0,
            quintDR: 0.0,
            quintER: 0.0,
            quintFactorR: 0.0,
            sextAR: 0.0,
            sextBR: 0.0,
            sextCR: 0.0,
            sextDR: 0.0,
            sextER: 0.0,
            sextFR: 0.0,
            sextFactorR: 0.0,
            septAR: 0.0,
            septBR: 0.0,
            septCR: 0.0,
            septDR: 0.0,
            septER: 0.0,
            septFR: 0.0,
            septGR: 0.0,
            septFactorR: 0.0,
            octAR: 0.0,
            octBR: 0.0,
            octCR: 0.0,
            octDR: 0.0,
            octER: 0.0,
            octFR: 0.0,
            octGR: 0.0,
            octHR: 0.0,
            octFactorR: 0.0,
            nintAR: 0.0,
            nintBR: 0.0,
            nintCR: 0.0,
            nintDR: 0.0,
            nintER: 0.0,
            nintFR: 0.0,
            nintGR: 0.0,
            nintHR: 0.0,
            nintIR: 0.0,
            nintFactorR: 0.0,
            PrevHR: 0.0,
            PrevGR: 0.0,
            PrevFR: 0.0,
            PrevER: 0.0,
            PrevDR: 0.0,
            PrevCR: 0.0,
            PrevBR: 0.0,
            PrevAR: 0.0,
        }
    }
}
impl AirwindowsEffect for Energy2 {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        self.params.get(i).map_or_else(String::new, |v| {
            float2string((*v as f64 * 2.0 - 1.0) as f32)
        })
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let cycleEnd = ((1.0 / 44100.0 * self.sample_rate as f64).floor() as i32).clamp(1, 4);
        if self.cycle > cycleEnd - 1 {
            self.cycle = cycleEnd - 1;
        }
        let mut intensity = [0.0; 8];
        for (i, v) in intensity.iter_mut().enumerate() {
            *v = -(self.params[i] as f64 * 2.0 - 1.0).powf(3.0);
        }
        let mix = self.params[8] as f64 * 2.0 - 1.0;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpdL as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpdR as f64 * 1.18e-17;
            }
            let dryL = l;
            let dryR = r;
            self.cycle += 1;
            if self.cycle == cycleEnd {
                let (cl, cr) = self.bands(l, r, &intensity);
                self.PrevHL = self.PrevGL;
                self.PrevGL = self.PrevFL;
                self.PrevFL = self.PrevEL;
                self.PrevEL = self.PrevDL;
                self.PrevDL = self.PrevCL;
                self.PrevCL = self.PrevBL;
                self.PrevBL = self.PrevAL;
                self.PrevAL = l;
                self.PrevHR = self.PrevGR;
                self.PrevGR = self.PrevFR;
                self.PrevFR = self.PrevER;
                self.PrevER = self.PrevDR;
                self.PrevDR = self.PrevCR;
                self.PrevCR = self.PrevBR;
                self.PrevBR = self.PrevAR;
                self.PrevAR = r;
                l = cl * mix;
                r = cr * mix;
                for (refs, input) in [(&mut self.lastRefL, l), (&mut self.lastRefR, r)] {
                    match cycleEnd {
                        4 => {
                            refs[0] = refs[4];
                            refs[2] = (refs[0] + input) / 2.0;
                            refs[1] = (refs[0] + refs[2]) / 2.0;
                            refs[3] = (refs[2] + input) / 2.0;
                            refs[4] = input;
                        }
                        3 => {
                            refs[0] = refs[3];
                            refs[2] = (refs[0] + refs[0] + input) / 3.0;
                            refs[1] = (refs[0] + input + input) / 3.0;
                            refs[3] = input;
                        }
                        2 => {
                            refs[0] = refs[2];
                            refs[1] = (refs[0] + input) / 2.0;
                            refs[2] = input;
                        }
                        _ => {
                            refs[0] = input;
                        }
                    }
                }
                self.cycle = 0;
            }
            l = self.lastRefL[self.cycle as usize];
            r = self.lastRefR[self.cycle as usize];
            for (refs, sample) in [(&mut self.lastRefL, &mut l), (&mut self.lastRefR, &mut r)] {
                if cycleEnd >= 4 {
                    refs[8] = *sample;
                    *sample = (*sample + refs[7]) * 0.5;
                    refs[7] = refs[8];
                }
                if cycleEnd >= 3 {
                    refs[8] = *sample;
                    *sample = (*sample + refs[6]) * 0.5;
                    refs[6] = refs[8];
                }
                if cycleEnd >= 2 {
                    refs[8] = *sample;
                    *sample = (*sample + refs[5]) * 0.5;
                    refs[5] = refs[8];
                }
            }
            *outl = fpd_dither(l + dryL, &mut self.fpdL) as f32;
            *outr = fpd_dither(r + dryR, &mut self.fpdR) as f32;
        }
    }
}
