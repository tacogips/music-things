//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/BigAmp, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
#![allow(
    non_snake_case,
    unused_parens,
    clippy::approx_constant,
    clippy::assign_op_pattern,
    clippy::manual_clamp,
    clippy::excessive_precision
)]
mod cabinet;
use crate::display::float2string;
use crate::dither::fpd_from_seed;
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Gain",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Tone",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 0.8,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "BigAmp",
    upstream_name: "BigAmp",
    description: "BigAmp is a very flexible amplike thing with a taste for the bizarre.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(BigAmp::new(sample_rate, seed))
}
struct BigAmp {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    last_sample: [f64; 2],
    store_sample: [f64; 2],
    last_slew: [f64; 2],
    iir: [[f64; 10]; 2],
    odd: [[f64; 257]; 2],
    even: [[f64; 257]; 2],
    flip: bool,
    count: i32,
    cab: [[f64; 90]; 2],
    last_cab: [f64; 2],
    smooth_cab_a: [f64; 2],
    smooth_cab_b: [f64; 2],
    last_ref: [[f64; 10]; 2],
    cycle: usize,
    fix: [[f64; 11]; 6],
    fpd_l: u32,
    fpd_r: u32,
}
impl BigAmp {
    // Upstream processReplacing's six fixed ultrasonic low-pass sections.
    fn setup_fixed_filters(&mut self) {
        let cutoff = ((15000.0 + self.params[1] as f64 * 10000.0) / self.sample_rate as f64)
            .clamp(0.001, 0.49);
        for (filter, resonance) in self.fix.iter_mut().zip([
            4.46570214, 1.51387132, 0.93979296, 0.70710678, 0.52972649, 0.50316379,
        ]) {
            filter[0] = cutoff;
            filter[1] = resonance;
            let k = (std::f64::consts::PI * filter[0]).tan();
            let norm = 1.0 / (1.0 + k / filter[1] + k * k);
            filter[2] = k * k * norm;
            filter[3] = 2.0 * filter[2];
            filter[4] = filter[2];
            filter[5] = 2.0 * (k * k - 1.0) * norm;
            filter[6] = (1.0 - k / filter[1] + k * k) * norm;
        }
    }
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.5, 0.5, 0.8, 1.0],
            sample_rate,
            seed,
            last_sample: [0.0; 2],
            store_sample: [0.0; 2],
            last_slew: [0.0; 2],
            iir: [[0.0; 10]; 2],
            odd: [[0.0; 257]; 2],
            even: [[0.0; 257]; 2],
            flip: false,
            count: 0,
            cab: [[0.0; 90]; 2],
            last_cab: [0.0; 2],
            smooth_cab_a: [0.0; 2],
            smooth_cab_b: [0.0; 2],
            last_ref: [[0.0; 10]; 2],
            cycle: 0,
            fix: [[0.0; 11]; 6],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for BigAmp {
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
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        self.setup_fixed_filters();
        let bassfill = self.params[0] as f64;
        let basstrim = self.params[1] as f64;
        let outputlevel = self.params[2] as f64;
        let wet = self.params[3] as f64;
        let mut overallscale = 1.0;
        overallscale /= 44100.0;
        overallscale *= self.sample_rate as f64;
        let cycle_end = (overallscale.floor() as usize).clamp(1, 4);
        if self.cycle > cycle_end - 1 {
            self.cycle = cycle_end - 1;
        }
        let bleed = outputlevel / 16.0;
        let inputlevel = bassfill * 3.0;
        let samplerate = self.sample_rate as f64;
        let EQ = (basstrim / samplerate) * 22050.0;
        let BEQ = (bleed / samplerate) * 22050.0;
        let diagonal = ((0.000861678 * samplerate) as i32).min(127);
        let side = (diagonal as f64 / 1.4142135623730951) as i32;
        let down = (side + diagonal) / 2;
        for (((&left, &right), out_l), out_r) in
            il.iter().zip(ir).zip(ol.iter_mut()).zip(orr.iter_mut())
        {
            let mut inputSampleL = left as f64;
            let mut inputSampleR = right as f64;
            if inputSampleL.abs() < 1.18e-23 {
                inputSampleL = self.fpd_l as f64 * 1.18e-17;
            }
            if inputSampleR.abs() < 1.18e-23 {
                inputSampleR = self.fpd_r as f64 * 1.18e-17;
            }
            let drySampleL = inputSampleL;
            let drySampleR = inputSampleR;
            let mut outSample = (inputSampleL * self.fix[0][2]) + self.fix[0][7];
            self.fix[0][7] =
                (inputSampleL * self.fix[0][3]) - (outSample * self.fix[0][5]) + self.fix[0][8];
            self.fix[0][8] = (inputSampleL * self.fix[0][4]) - (outSample * self.fix[0][6]);
            inputSampleL = outSample;
            outSample = (inputSampleR * self.fix[0][2]) + self.fix[0][9];
            self.fix[0][9] =
                (inputSampleR * self.fix[0][3]) - (outSample * self.fix[0][5]) + self.fix[0][10];
            self.fix[0][10] = (inputSampleR * self.fix[0][4]) - (outSample * self.fix[0][6]);
            inputSampleR = outSample;

            let mut skewL = (inputSampleL - self.last_sample[0]);
            self.last_sample[0] = inputSampleL;

            let mut bridgerectifier = (skewL).abs();
            if bridgerectifier > 3.1415926 {
                bridgerectifier = 3.1415926;
            }

            bridgerectifier = (bridgerectifier).sin();
            if skewL > 0.0 {
                skewL = bridgerectifier;
            } else {
                skewL = -bridgerectifier;
            }

            skewL *= inputSampleL;
            skewL = (skewL + (skewL * basstrim)) / 2.0;
            inputSampleL *= basstrim;

            let mut skewR = (inputSampleR - self.last_sample[1]);
            self.last_sample[1] = inputSampleR;

            bridgerectifier = (skewR).abs();
            if bridgerectifier > 3.1415926 {
                bridgerectifier = 3.1415926;
            }

            bridgerectifier = (bridgerectifier).sin();
            if skewR > 0.0 {
                skewR = bridgerectifier;
            } else {
                skewR = -bridgerectifier;
            }

            skewR *= inputSampleR;
            skewR = (skewR + (skewR * basstrim)) / 2.0;
            inputSampleR *= basstrim;
            let mut basscut = basstrim;

            outSample = (inputSampleL * self.fix[1][2]) + self.fix[1][7];
            self.fix[1][7] =
                (inputSampleL * self.fix[1][3]) - (outSample * self.fix[1][5]) + self.fix[1][8];
            self.fix[1][8] = (inputSampleL * self.fix[1][4]) - (outSample * self.fix[1][6]);
            inputSampleL = outSample;
            outSample = (inputSampleR * self.fix[1][2]) + self.fix[1][9];
            self.fix[1][9] =
                (inputSampleR * self.fix[1][3]) - (outSample * self.fix[1][5]) + self.fix[1][10];
            self.fix[1][10] = (inputSampleR * self.fix[1][4]) - (outSample * self.fix[1][6]);
            inputSampleR = outSample;

            inputSampleL *= inputlevel;
            let mut offsetL = (1.0 - EQ) + ((inputSampleL).abs() * EQ);
            if offsetL < 0.0 {
                offsetL = 0.0;
            }
            if offsetL > 1.0 {
                offsetL = 1.0;
            }
            self.iir[0][0] =
                (self.iir[0][0] * (1.0 - (offsetL * EQ))) + (inputSampleL * (offsetL * EQ));
            inputSampleL = inputSampleL - (self.iir[0][0] * basscut);

            bridgerectifier = (inputSampleL).abs() + skewL;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = ((bridgerectifier).sin() * 1.57079633) + skewL;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = (bridgerectifier).sin() * 1.57079633;
            if inputSampleL > 0.0 {
                inputSampleL = (inputSampleL * (-0.57079633 + skewL))
                    + (bridgerectifier * (1.57079633 + skewL));
            } else {
                inputSampleL = (inputSampleL * (-0.57079633 + skewL))
                    - (bridgerectifier * (1.57079633 + skewL));
            }

            self.iir[0][1] =
                (self.iir[0][1] * (1.0 - (offsetL * EQ))) + (inputSampleL * (offsetL * EQ));
            inputSampleL = inputSampleL - (self.iir[0][1] * basscut);

            inputSampleR *= inputlevel;
            let mut offsetR = (1.0 - EQ) + ((inputSampleR).abs() * EQ);
            if offsetR < 0.0 {
                offsetR = 0.0;
            }
            if offsetR > 1.0 {
                offsetR = 1.0;
            }
            self.iir[1][0] =
                (self.iir[1][0] * (1.0 - (offsetR * EQ))) + (inputSampleR * (offsetR * EQ));
            inputSampleR = inputSampleR - (self.iir[1][0] * basscut);

            bridgerectifier = (inputSampleR).abs() + skewR;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = ((bridgerectifier).sin() * 1.57079633) + skewR;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = (bridgerectifier).sin() * 1.57079633;
            if inputSampleR > 0.0 {
                inputSampleR = (inputSampleR * (-0.57079633 + skewR))
                    + (bridgerectifier * (1.57079633 + skewR));
            } else {
                inputSampleR = (inputSampleR * (-0.57079633 + skewR))
                    - (bridgerectifier * (1.57079633 + skewR));
            }

            self.iir[1][1] =
                (self.iir[1][1] * (1.0 - (offsetR * EQ))) + (inputSampleR * (offsetR * EQ));
            inputSampleR = inputSampleR - (self.iir[1][1] * basscut);
            basscut /= 2.0;

            outSample = (inputSampleL * self.fix[2][2]) + self.fix[2][7];
            self.fix[2][7] =
                (inputSampleL * self.fix[2][3]) - (outSample * self.fix[2][5]) + self.fix[2][8];
            self.fix[2][8] = (inputSampleL * self.fix[2][4]) - (outSample * self.fix[2][6]);
            inputSampleL = outSample;
            outSample = (inputSampleR * self.fix[2][2]) + self.fix[2][9];
            self.fix[2][9] =
                (inputSampleR * self.fix[2][3]) - (outSample * self.fix[2][5]) + self.fix[2][10];
            self.fix[2][10] = (inputSampleR * self.fix[2][4]) - (outSample * self.fix[2][6]);
            inputSampleR = outSample;

            inputSampleL *= inputlevel;
            skewL /= 2.0;
            offsetL = (1.0 + offsetL) / 2.0;
            bridgerectifier = (inputSampleL).abs() + skewL;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = ((bridgerectifier).sin() * 1.57079633) + skewL;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = (bridgerectifier).sin() * 1.57079633;
            if inputSampleL > 0.0 {
                inputSampleL = (inputSampleL * (-0.57079633 + skewL))
                    + (bridgerectifier * (1.57079633 + skewL));
            } else {
                inputSampleL = (inputSampleL * (-0.57079633 + skewL))
                    - (bridgerectifier * (1.57079633 + skewL));
            }

            self.iir[0][2] =
                (self.iir[0][2] * (1.0 - (offsetL * EQ))) + (inputSampleL * (offsetL * EQ));
            inputSampleL = inputSampleL - (self.iir[0][2] * basscut);

            self.iir[0][3] =
                (self.iir[0][3] * (1.0 - (offsetL * EQ))) + (inputSampleL * (offsetL * EQ));
            inputSampleL = self.iir[0][3];

            inputSampleR *= inputlevel;
            offsetR = (1.0 + offsetR) / 2.0;
            bridgerectifier = (inputSampleR).abs() + skewR;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = ((bridgerectifier).sin() * 1.57079633) + skewR;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = (bridgerectifier).sin() * 1.57079633;
            if inputSampleR > 0.0 {
                inputSampleR = (inputSampleR * (-0.57079633 + skewR))
                    + (bridgerectifier * (1.57079633 + skewR));
            } else {
                inputSampleR = (inputSampleR * (-0.57079633 + skewR))
                    - (bridgerectifier * (1.57079633 + skewR));
            }

            self.iir[1][2] =
                (self.iir[1][2] * (1.0 - (offsetR * EQ))) + (inputSampleR * (offsetR * EQ));
            inputSampleR = inputSampleR - (self.iir[1][2] * basscut);
            basscut /= 2.0;

            self.iir[1][3] =
                (self.iir[1][3] * (1.0 - (offsetR * EQ))) + (inputSampleR * (offsetR * EQ));
            inputSampleR = self.iir[1][3];

            outSample = (inputSampleL * self.fix[3][2]) + self.fix[3][7];
            self.fix[3][7] =
                (inputSampleL * self.fix[3][3]) - (outSample * self.fix[3][5]) + self.fix[3][8];
            self.fix[3][8] = (inputSampleL * self.fix[3][4]) - (outSample * self.fix[3][6]);
            inputSampleL = outSample;
            outSample = (inputSampleR * self.fix[3][2]) + self.fix[3][9];
            self.fix[3][9] =
                (inputSampleR * self.fix[3][3]) - (outSample * self.fix[3][5]) + self.fix[3][10];
            self.fix[3][10] = (inputSampleR * self.fix[3][4]) - (outSample * self.fix[3][6]);
            inputSampleR = outSample;

            inputSampleL *= inputlevel;
            skewL /= 2.0;
            offsetL = (1.0 + offsetL) / 2.0;
            bridgerectifier = (inputSampleL).abs() + skewL;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = ((bridgerectifier).sin() * 1.57079633) + skewL;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = (bridgerectifier).sin() * 1.57079633;
            if inputSampleL > 0.0 {
                inputSampleL = (inputSampleL * (-0.57079633 + skewL))
                    + (bridgerectifier * (1.57079633 + skewL));
            } else {
                inputSampleL = (inputSampleL * (-0.57079633 + skewL))
                    - (bridgerectifier * (1.57079633 + skewL));
            }

            self.iir[0][4] =
                (self.iir[0][4] * (1.0 - (offsetL * EQ))) + (inputSampleL * (offsetL * EQ));
            inputSampleL = inputSampleL - (self.iir[0][4] * basscut);

            self.iir[0][5] =
                (self.iir[0][5] * (1.0 - (offsetL * EQ))) + (inputSampleL * (offsetL * EQ));
            inputSampleL = self.iir[0][5];

            inputSampleR *= inputlevel;
            skewR /= 2.0;
            offsetR = (1.0 + offsetR) / 2.0;
            bridgerectifier = (inputSampleR).abs() + skewR;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = ((bridgerectifier).sin() * 1.57079633) + skewR;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = (bridgerectifier).sin() * 1.57079633;
            if inputSampleR > 0.0 {
                inputSampleR = (inputSampleR * (-0.57079633 + skewR))
                    + (bridgerectifier * (1.57079633 + skewR));
            } else {
                inputSampleR = (inputSampleR * (-0.57079633 + skewR))
                    - (bridgerectifier * (1.57079633 + skewR));
            }

            self.iir[1][4] =
                (self.iir[1][4] * (1.0 - (offsetR * EQ))) + (inputSampleR * (offsetR * EQ));
            inputSampleR = inputSampleR - (self.iir[1][4] * basscut);

            self.iir[1][5] =
                (self.iir[1][5] * (1.0 - (offsetR * EQ))) + (inputSampleR * (offsetR * EQ));
            inputSampleR = self.iir[1][5];

            outSample = (inputSampleL * self.fix[4][2]) + self.fix[4][7];
            self.fix[4][7] =
                (inputSampleL * self.fix[4][3]) - (outSample * self.fix[4][5]) + self.fix[4][8];
            self.fix[4][8] = (inputSampleL * self.fix[4][4]) - (outSample * self.fix[4][6]);
            inputSampleL = outSample;
            outSample = (inputSampleR * self.fix[4][2]) + self.fix[4][9];
            self.fix[4][9] =
                (inputSampleR * self.fix[4][3]) - (outSample * self.fix[4][5]) + self.fix[4][10];
            self.fix[4][10] = (inputSampleR * self.fix[4][4]) - (outSample * self.fix[4][6]);
            inputSampleR = outSample;

            inputSampleL *= inputlevel;
            skewL /= 2.0;
            offsetL = (1.0 + offsetL) / 2.0;
            bridgerectifier = (inputSampleL).abs() + skewL;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = (bridgerectifier).sin() * 1.57079633;
            if inputSampleL > 0.0 {
                inputSampleL = (inputSampleL * (-0.57079633 + skewL))
                    + (bridgerectifier * (1.57079633 + skewL));
            } else {
                inputSampleL = (inputSampleL * (-0.57079633 + skewL))
                    - (bridgerectifier * (1.57079633 + skewL));
            }

            self.iir[0][6] =
                (self.iir[0][6] * (1.0 - (offsetL * EQ))) + (inputSampleL * (offsetL * EQ));
            inputSampleL = self.iir[0][6];

            self.iir[0][7] =
                (self.iir[0][7] * (1.0 - (offsetL * BEQ))) + (inputSampleL * (offsetL * BEQ));

            inputSampleR *= inputlevel;
            skewR /= 2.0;
            offsetR = (1.0 + offsetR) / 2.0;
            bridgerectifier = (inputSampleR).abs() + skewR;
            if bridgerectifier > 1.57079633 {
                bridgerectifier = 1.57079633;
            }
            bridgerectifier = (bridgerectifier).sin() * 1.57079633;
            if inputSampleR > 0.0 {
                inputSampleR = (inputSampleR * (-0.57079633 + skewR))
                    + (bridgerectifier * (1.57079633 + skewR));
            } else {
                inputSampleR = (inputSampleR * (-0.57079633 + skewR))
                    - (bridgerectifier * (1.57079633 + skewR));
            }

            self.iir[1][6] =
                (self.iir[1][6] * (1.0 - (offsetR * EQ))) + (inputSampleR * (offsetR * EQ));
            inputSampleR = self.iir[1][6];

            self.iir[1][7] =
                (self.iir[1][7] * (1.0 - (offsetR * BEQ))) + (inputSampleR * (offsetR * BEQ));

            if self.count < 0 || self.count > 128 {
                self.count = 128;
            }
            let resultBL;
            let resultBR;
            if self.flip {
                self.odd[0][(self.count + 128) as usize] = self.iir[0][7];
                self.odd[0][(self.count) as usize] = self.iir[0][7];
                resultBL = (self.odd[0][(self.count + down) as usize]
                    + self.odd[0][(self.count + side) as usize]
                    + self.odd[0][(self.count + diagonal) as usize]);
                self.odd[1][(self.count + 128) as usize] = self.iir[1][7];
                self.odd[1][(self.count) as usize] = self.iir[1][7];
                resultBR = (self.odd[1][(self.count + down) as usize]
                    + self.odd[1][(self.count + side) as usize]
                    + self.odd[1][(self.count + diagonal) as usize]);
            } else {
                self.even[0][(self.count + 128) as usize] = self.iir[0][7];
                self.even[0][(self.count) as usize] = self.iir[0][7];
                resultBL = (self.even[0][(self.count + down) as usize]
                    + self.even[0][(self.count + side) as usize]
                    + self.even[0][(self.count + diagonal) as usize]);
                self.even[1][(self.count + 128) as usize] = self.iir[1][7];
                self.even[1][(self.count) as usize] = self.iir[1][7];
                resultBR = (self.even[1][(self.count + down) as usize]
                    + self.even[1][(self.count + side) as usize]
                    + self.even[1][(self.count + diagonal) as usize]);
            }
            self.count -= 1;

            self.iir[0][8] =
                (self.iir[0][8] * (1.0 - (offsetL * BEQ))) + (resultBL * (offsetL * BEQ));
            inputSampleL += (self.iir[0][8] * bleed);

            self.iir[0][9] =
                (self.iir[0][9] * (1.0 - (offsetL * BEQ))) + (inputSampleL * (offsetL * BEQ));
            inputSampleL += (self.iir[0][9] * bassfill);
            inputSampleL = (inputSampleL * outputlevel).sin();
            let mut randy = (((self.fpd_l as f64) / u32::MAX as f64) * 0.04);
            inputSampleL =
                ((inputSampleL * (1.0 - randy)) + (self.store_sample[0] * randy)) * outputlevel;
            self.store_sample[0] = inputSampleL;

            self.iir[1][8] =
                (self.iir[1][8] * (1.0 - (offsetR * BEQ))) + (resultBR * (offsetR * BEQ));
            inputSampleR += (self.iir[1][8] * bleed);

            self.iir[1][9] =
                (self.iir[1][9] * (1.0 - (offsetR * BEQ))) + (inputSampleR * (offsetR * BEQ));
            inputSampleR += (self.iir[1][9] * bassfill);
            inputSampleR = (inputSampleR * outputlevel).sin();
            randy = (((self.fpd_r as f64) / u32::MAX as f64) * 0.04);
            inputSampleR =
                ((inputSampleR * (1.0 - randy)) + (self.store_sample[1] * randy)) * outputlevel;
            self.store_sample[1] = inputSampleR;

            outSample = (inputSampleL * self.fix[5][2]) + self.fix[5][7];
            self.fix[5][7] =
                (inputSampleL * self.fix[5][3]) - (outSample * self.fix[5][5]) + self.fix[5][8];
            self.fix[5][8] = (inputSampleL * self.fix[5][4]) - (outSample * self.fix[5][6]);
            inputSampleL = outSample;
            outSample = (inputSampleR * self.fix[5][2]) + self.fix[5][9];
            self.fix[5][9] =
                (inputSampleR * self.fix[5][3]) - (outSample * self.fix[5][5]) + self.fix[5][10];
            self.fix[5][10] = (inputSampleR * self.fix[5][4]) - (outSample * self.fix[5][6]);
            inputSampleR = outSample;

            self.flip = !self.flip;

            if (wet != 1.0) {
                inputSampleL = (inputSampleL * wet) + (drySampleL * (1.0 - wet));
                inputSampleR = (inputSampleR * wet) + (drySampleR * (1.0 - wet));
            }

            self.cycle += 1;
            if self.cycle == cycle_end {
                for channel in 0..2 {
                    let sample = if channel == 0 {
                        inputSampleL
                    } else {
                        inputSampleR
                    };
                    let dry = if channel == 0 { drySampleL } else { drySampleR };
                    let fpd = if channel == 0 { self.fpd_l } else { self.fpd_r };
                    let temp = (sample + self.smooth_cab_a[channel]) / 3.0;
                    self.smooth_cab_a[channel] = sample;
                    let sample = cabinet::convolve(&mut self.cab[channel], temp);
                    let temp = (sample + self.smooth_cab_b[channel]) / 3.0;
                    self.smooth_cab_b[channel] = sample;
                    let sample = temp / 4.0;
                    let randy = (fpd as f64 / u32::MAX as f64) * 0.05;
                    let result = ((((sample * (1.0 - randy)) + (self.last_cab[channel] * randy))
                        * wet)
                        + (dry * (1.0 - wet)))
                        * outputlevel;
                    self.last_cab[channel] = sample;
                    let refs = &mut self.last_ref[channel];
                    match cycle_end {
                        4 => {
                            refs[0] = refs[4];
                            refs[2] = (refs[0] + result) / 2.0;
                            refs[1] = (refs[0] + refs[2]) / 2.0;
                            refs[3] = (refs[2] + result) / 2.0;
                            refs[4] = result;
                        }
                        3 => {
                            refs[0] = refs[3];
                            refs[2] = (refs[0] + refs[0] + result) / 3.0;
                            refs[1] = (refs[0] + result + result) / 3.0;
                            refs[3] = result;
                        }
                        2 => {
                            refs[0] = refs[2];
                            refs[1] = (refs[0] + result) / 2.0;
                            refs[2] = result;
                        }
                        _ => refs[0] = result,
                    }
                }
                self.cycle = 0;
            }
            inputSampleL = self.last_ref[0][self.cycle];
            inputSampleR = self.last_ref[1][self.cycle];
            for stage in (2..=cycle_end).rev() {
                let idx = stage + 3;
                self.last_ref[0][8] = inputSampleL;
                inputSampleL = (inputSampleL + self.last_ref[0][idx]) * 0.5;
                self.last_ref[0][idx] = self.last_ref[0][8];
                self.last_ref[1][8] = inputSampleR;
                inputSampleR = (inputSampleR + self.last_ref[1][idx]) * 0.5;
                self.last_ref[1][idx] = self.last_ref[1][8];
            }
            *out_l = crate::dither::fpd_dither(inputSampleL, &mut self.fpd_l) as f32;
            *out_r = crate::dither::fpd_dither(inputSampleR, &mut self.fpd_r) as f32;
        }
    }
}
