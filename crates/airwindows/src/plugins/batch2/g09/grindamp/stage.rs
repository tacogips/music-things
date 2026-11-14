//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/GrindAmp, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
#![allow(
    non_snake_case,
    unused_parens,
    clippy::approx_constant,
    clippy::excessive_precision,
    clippy::assign_op_pattern,
    clippy::manual_clamp
)]
pub(super) struct Stage {
    smoothAL: f64,
    smoothBL: f64,
    smoothCL: f64,
    smoothDL: f64,
    smoothEL: f64,
    smoothFL: f64,
    smoothGL: f64,
    smoothHL: f64,
    smoothIL: f64,
    smoothJL: f64,
    smoothKL: f64,
    secondAL: f64,
    secondBL: f64,
    secondCL: f64,
    secondDL: f64,
    secondEL: f64,
    secondFL: f64,
    secondGL: f64,
    secondHL: f64,
    secondIL: f64,
    secondJL: f64,
    secondKL: f64,
    thirdAL: f64,
    thirdBL: f64,
    thirdCL: f64,
    thirdDL: f64,
    thirdEL: f64,
    thirdFL: f64,
    thirdGL: f64,
    thirdHL: f64,
    thirdIL: f64,
    thirdJL: f64,
    thirdKL: f64,
    iirSampleAL: f64,
    iirSampleBL: f64,
    iirSampleCL: f64,
    iirSampleDL: f64,
    iirSampleEL: f64,
    iirSampleFL: f64,
    iirSampleGL: f64,
    iirSampleHL: f64,
    iirSampleIL: f64,
    iirLowpassL: f64,
    iirSubL: f64,
    storeSampleL: f64,
    smoothAR: f64,
    smoothBR: f64,
    smoothCR: f64,
    smoothDR: f64,
    smoothER: f64,
    smoothFR: f64,
    smoothGR: f64,
    smoothHR: f64,
    smoothIR: f64,
    smoothJR: f64,
    smoothKR: f64,
    secondAR: f64,
    secondBR: f64,
    secondCR: f64,
    secondDR: f64,
    secondER: f64,
    secondFR: f64,
    secondGR: f64,
    secondHR: f64,
    secondIR: f64,
    secondJR: f64,
    secondKR: f64,
    thirdAR: f64,
    thirdBR: f64,
    thirdCR: f64,
    thirdDR: f64,
    thirdER: f64,
    thirdFR: f64,
    thirdGR: f64,
    thirdHR: f64,
    thirdIR: f64,
    thirdJR: f64,
    thirdKR: f64,
    iirSampleAR: f64,
    iirSampleBR: f64,
    iirSampleCR: f64,
    iirSampleDR: f64,
    iirSampleER: f64,
    iirSampleFR: f64,
    iirSampleGR: f64,
    iirSampleHR: f64,
    iirSampleIR: f64,
    iirLowpassR: f64,
    iirSubR: f64,
    storeSampleR: f64,
    fix: [[f64; 11]; 6],
}
impl Stage {
    pub(super) fn new() -> Self {
        Self {
            smoothAL: 0.0,
            smoothBL: 0.0,
            smoothCL: 0.0,
            smoothDL: 0.0,
            smoothEL: 0.0,
            smoothFL: 0.0,
            smoothGL: 0.0,
            smoothHL: 0.0,
            smoothIL: 0.0,
            smoothJL: 0.0,
            smoothKL: 0.0,
            secondAL: 0.0,
            secondBL: 0.0,
            secondCL: 0.0,
            secondDL: 0.0,
            secondEL: 0.0,
            secondFL: 0.0,
            secondGL: 0.0,
            secondHL: 0.0,
            secondIL: 0.0,
            secondJL: 0.0,
            secondKL: 0.0,
            thirdAL: 0.0,
            thirdBL: 0.0,
            thirdCL: 0.0,
            thirdDL: 0.0,
            thirdEL: 0.0,
            thirdFL: 0.0,
            thirdGL: 0.0,
            thirdHL: 0.0,
            thirdIL: 0.0,
            thirdJL: 0.0,
            thirdKL: 0.0,
            iirSampleAL: 0.0,
            iirSampleBL: 0.0,
            iirSampleCL: 0.0,
            iirSampleDL: 0.0,
            iirSampleEL: 0.0,
            iirSampleFL: 0.0,
            iirSampleGL: 0.0,
            iirSampleHL: 0.0,
            iirSampleIL: 0.0,
            iirLowpassL: 0.0,
            iirSubL: 0.0,
            storeSampleL: 0.0,
            smoothAR: 0.0,
            smoothBR: 0.0,
            smoothCR: 0.0,
            smoothDR: 0.0,
            smoothER: 0.0,
            smoothFR: 0.0,
            smoothGR: 0.0,
            smoothHR: 0.0,
            smoothIR: 0.0,
            smoothJR: 0.0,
            smoothKR: 0.0,
            secondAR: 0.0,
            secondBR: 0.0,
            secondCR: 0.0,
            secondDR: 0.0,
            secondER: 0.0,
            secondFR: 0.0,
            secondGR: 0.0,
            secondHR: 0.0,
            secondIR: 0.0,
            secondJR: 0.0,
            secondKR: 0.0,
            thirdAR: 0.0,
            thirdBR: 0.0,
            thirdCR: 0.0,
            thirdDR: 0.0,
            thirdER: 0.0,
            thirdFR: 0.0,
            thirdGR: 0.0,
            thirdHR: 0.0,
            thirdIR: 0.0,
            thirdJR: 0.0,
            thirdKR: 0.0,
            iirSampleAR: 0.0,
            iirSampleBR: 0.0,
            iirSampleCR: 0.0,
            iirSampleDR: 0.0,
            iirSampleER: 0.0,
            iirSampleFR: 0.0,
            iirSampleGR: 0.0,
            iirSampleHR: 0.0,
            iirSampleIR: 0.0,
            iirLowpassR: 0.0,
            iirSubR: 0.0,
            storeSampleR: 0.0,
            fix: [[0.0; 11]; 6],
        }
    }
    pub(super) fn setup(&mut self, rate: f64, tone: f64) {
        let cutoff = ((18000.0 + tone * 1000.0) / rate).clamp(0.001, 0.49);
        for (fix, resonance) in self.fix.iter_mut().zip([
            4.46570214, 1.51387132, 0.93979296, 0.70710678, 0.52972649, 0.50316379,
        ]) {
            fix[0] = cutoff;
            fix[1] = resonance;
            let k = (std::f64::consts::PI * fix[0]).tan();
            let norm = 1.0 / (1.0 + k / fix[1] + k * k);
            fix[2] = k * k * norm;
            fix[3] = 2.0 * fix[2];
            fix[4] = fix[2];
            fix[5] = 2.0 * (k * k - 1.0) * norm;
            fix[6] = (1.0 - k / fix[1] + k * k) * norm;
        }
    }
    #[allow(clippy::too_many_arguments)]
    pub(super) fn process(
        &mut self,
        mut inputSampleL: f64,
        mut inputSampleR: f64,
        drySampleL: f64,
        drySampleR: f64,
        inputlevel: f64,
        EQ: f64,
        BEQ: f64,
        toneEQ: f64,
        bassdrive: f64,
        outputlevel: f64,
        wet: f64,
        fpdL: u32,
        fpdR: u32,
    ) -> (f64, f64) {
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

        inputSampleL *= inputlevel;
        self.iirSampleAL = (self.iirSampleAL * (1.0 - EQ)) + (inputSampleL * EQ);
        inputSampleL = inputSampleL - (self.iirSampleAL * 0.92);

        if inputSampleL > 1.0 {
            inputSampleL = 1.0;
        }
        if inputSampleL < -1.0 {
            inputSampleL = -1.0;
        }
        let mut bridgerectifier = (inputSampleL).abs();
        let mut inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothAL
            + (self.secondAL * inverse)
            + (self.thirdAL * bridgerectifier)
            + inputSampleL);
        self.thirdAL = self.secondAL;
        self.secondAL = self.smoothAL;
        self.smoothAL = inputSampleL;
        inputSampleL = bridgerectifier;
        let mut basscatchL = inputSampleL;

        inputSampleR *= inputlevel;
        self.iirSampleAR = (self.iirSampleAR * (1.0 - EQ)) + (inputSampleR * EQ);
        inputSampleR = inputSampleR - (self.iirSampleAR * 0.92);

        if inputSampleR > 1.0 {
            inputSampleR = 1.0;
        }
        if inputSampleR < -1.0 {
            inputSampleR = -1.0;
        }
        bridgerectifier = (inputSampleR).abs();
        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothAR
            + (self.secondAR * inverse)
            + (self.thirdAR * bridgerectifier)
            + inputSampleR);
        self.thirdAR = self.secondAR;
        self.secondAR = self.smoothAR;
        self.smoothAR = inputSampleR;
        inputSampleR = bridgerectifier;
        let mut basscatchR = inputSampleR;

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
        self.iirSampleBL = (self.iirSampleBL * (1.0 - EQ)) + (inputSampleL * EQ);
        inputSampleL = inputSampleL - (self.iirSampleBL * 0.79);

        if inputSampleL > 1.0 {
            inputSampleL = 1.0;
        }
        if inputSampleL < -1.0 {
            inputSampleL = -1.0;
        }

        bridgerectifier = (inputSampleL).abs();
        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothBL
            + (self.secondBL * inverse)
            + (self.thirdBL * bridgerectifier)
            + inputSampleL);
        self.thirdBL = self.secondBL;
        self.secondBL = self.smoothBL;
        self.smoothBL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleR *= inputlevel;
        self.iirSampleBR = (self.iirSampleBR * (1.0 - EQ)) + (inputSampleR * EQ);
        inputSampleR = inputSampleR - (self.iirSampleBR * 0.79);

        if inputSampleR > 1.0 {
            inputSampleR = 1.0;
        }
        if inputSampleR < -1.0 {
            inputSampleR = -1.0;
        }

        bridgerectifier = (inputSampleR).abs();
        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothBR
            + (self.secondBR * inverse)
            + (self.thirdBR * bridgerectifier)
            + inputSampleR);
        self.thirdBR = self.secondBR;
        self.secondBR = self.smoothBR;
        self.smoothBR = inputSampleR;
        inputSampleR = bridgerectifier;

        self.iirSampleCL = (self.iirSampleCL * (1.0 - BEQ)) + (basscatchL * BEQ);
        basscatchL = self.iirSampleCL * bassdrive;
        bridgerectifier = (basscatchL).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchL > 0.0 {
            basscatchL = bridgerectifier;
        } else {
            basscatchL = -bridgerectifier;
        }
        if inputSampleL > 1.0 {
            inputSampleL = 1.0;
        }
        if inputSampleL < -1.0 {
            inputSampleL = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothCL
            + (self.secondCL * inverse)
            + (self.thirdCL * bridgerectifier)
            + inputSampleL);
        self.thirdCL = self.secondCL;
        self.secondCL = self.smoothCL;
        self.smoothCL = inputSampleL;
        inputSampleL = bridgerectifier;

        self.iirSampleCR = (self.iirSampleCR * (1.0 - BEQ)) + (basscatchR * BEQ);
        basscatchR = self.iirSampleCR * bassdrive;
        bridgerectifier = (basscatchR).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchR > 0.0 {
            basscatchR = bridgerectifier;
        } else {
            basscatchR = -bridgerectifier;
        }
        if inputSampleR > 1.0 {
            inputSampleR = 1.0;
        }
        if inputSampleR < -1.0 {
            inputSampleR = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothCR
            + (self.secondCR * inverse)
            + (self.thirdCR * bridgerectifier)
            + inputSampleR);
        self.thirdCR = self.secondCR;
        self.secondCR = self.smoothCR;
        self.smoothCR = inputSampleR;
        inputSampleR = bridgerectifier;

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

        self.iirSampleDL = (self.iirSampleDL * (1.0 - BEQ)) + (basscatchL * BEQ);
        basscatchL = self.iirSampleDL * bassdrive;
        bridgerectifier = (basscatchL).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchL > 0.0 {
            basscatchL = bridgerectifier;
        } else {
            basscatchL = -bridgerectifier;
        }
        if inputSampleL > 1.0 {
            inputSampleL = 1.0;
        }
        if inputSampleL < -1.0 {
            inputSampleL = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothDL
            + (self.secondDL * inverse)
            + (self.thirdDL * bridgerectifier)
            + inputSampleL);
        self.thirdDL = self.secondDL;
        self.secondDL = self.smoothDL;
        self.smoothDL = inputSampleL;
        inputSampleL = bridgerectifier;

        self.iirSampleDR = (self.iirSampleDR * (1.0 - BEQ)) + (basscatchR * BEQ);
        basscatchR = self.iirSampleDR * bassdrive;
        bridgerectifier = (basscatchR).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchR > 0.0 {
            basscatchR = bridgerectifier;
        } else {
            basscatchR = -bridgerectifier;
        }
        if inputSampleR > 1.0 {
            inputSampleR = 1.0;
        }
        if inputSampleR < -1.0 {
            inputSampleR = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothDR
            + (self.secondDR * inverse)
            + (self.thirdDR * bridgerectifier)
            + inputSampleR);
        self.thirdDR = self.secondDR;
        self.secondDR = self.smoothDR;
        self.smoothDR = inputSampleR;
        inputSampleR = bridgerectifier;

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

        self.iirSampleEL = (self.iirSampleEL * (1.0 - BEQ)) + (basscatchL * BEQ);
        basscatchL = self.iirSampleEL * bassdrive;
        bridgerectifier = (basscatchL).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchL > 0.0 {
            basscatchL = bridgerectifier;
        } else {
            basscatchL = -bridgerectifier;
        }
        if inputSampleL > 1.0 {
            inputSampleL = 1.0;
        }
        if inputSampleL < -1.0 {
            inputSampleL = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothEL
            + (self.secondEL * inverse)
            + (self.thirdEL * bridgerectifier)
            + inputSampleL);
        self.thirdEL = self.secondEL;
        self.secondEL = self.smoothEL;
        self.smoothEL = inputSampleL;
        inputSampleL = bridgerectifier;

        self.iirSampleER = (self.iirSampleER * (1.0 - BEQ)) + (basscatchR * BEQ);
        basscatchR = self.iirSampleER * bassdrive;
        bridgerectifier = (basscatchR).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchR > 0.0 {
            basscatchR = bridgerectifier;
        } else {
            basscatchR = -bridgerectifier;
        }
        if inputSampleR > 1.0 {
            inputSampleR = 1.0;
        }
        if inputSampleR < -1.0 {
            inputSampleR = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothER
            + (self.secondER * inverse)
            + (self.thirdER * bridgerectifier)
            + inputSampleR);
        self.thirdER = self.secondER;
        self.secondER = self.smoothER;
        self.smoothER = inputSampleR;
        inputSampleR = bridgerectifier;

        self.iirSampleFL = (self.iirSampleFL * (1.0 - BEQ)) + (basscatchL * BEQ);
        basscatchL = self.iirSampleFL * bassdrive;
        bridgerectifier = (basscatchL).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchL > 0.0 {
            basscatchL = bridgerectifier;
        } else {
            basscatchL = -bridgerectifier;
        }
        if inputSampleL > 1.0 {
            inputSampleL = 1.0;
        }
        if inputSampleL < -1.0 {
            inputSampleL = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothFL
            + (self.secondFL * inverse)
            + (self.thirdFL * bridgerectifier)
            + inputSampleL);
        self.thirdFL = self.secondFL;
        self.secondFL = self.smoothFL;
        self.smoothFL = inputSampleL;
        inputSampleL = bridgerectifier;

        self.iirSampleFR = (self.iirSampleFR * (1.0 - BEQ)) + (basscatchR * BEQ);
        basscatchR = self.iirSampleFR * bassdrive;
        bridgerectifier = (basscatchR).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchR > 0.0 {
            basscatchR = bridgerectifier;
        } else {
            basscatchR = -bridgerectifier;
        }
        if inputSampleR > 1.0 {
            inputSampleR = 1.0;
        }
        if inputSampleR < -1.0 {
            inputSampleR = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothFR
            + (self.secondFR * inverse)
            + (self.thirdFR * bridgerectifier)
            + inputSampleR);
        self.thirdFR = self.secondFR;
        self.secondFR = self.smoothFR;
        self.smoothFR = inputSampleR;
        inputSampleR = bridgerectifier;

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

        self.iirSampleGL = (self.iirSampleGL * (1.0 - BEQ)) + (basscatchL * BEQ);
        basscatchL = self.iirSampleGL * bassdrive;
        bridgerectifier = (basscatchL).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchL > 0.0 {
            basscatchL = bridgerectifier;
        } else {
            basscatchL = -bridgerectifier;
        }
        if inputSampleL > 1.0 {
            inputSampleL = 1.0;
        }
        if inputSampleL < -1.0 {
            inputSampleL = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothGL
            + (self.secondGL * inverse)
            + (self.thirdGL * bridgerectifier)
            + inputSampleL);
        self.thirdGL = self.secondGL;
        self.secondGL = self.smoothGL;
        self.smoothGL = inputSampleL;
        inputSampleL = bridgerectifier;

        self.iirSampleGR = (self.iirSampleGR * (1.0 - BEQ)) + (basscatchR * BEQ);
        basscatchR = self.iirSampleGR * bassdrive;
        bridgerectifier = (basscatchR).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchR > 0.0 {
            basscatchR = bridgerectifier;
        } else {
            basscatchR = -bridgerectifier;
        }
        if inputSampleR > 1.0 {
            inputSampleR = 1.0;
        }
        if inputSampleR < -1.0 {
            inputSampleR = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothGR
            + (self.secondGR * inverse)
            + (self.thirdGR * bridgerectifier)
            + inputSampleR);
        self.thirdGR = self.secondGR;
        self.secondGR = self.smoothGR;
        self.smoothGR = inputSampleR;
        inputSampleR = bridgerectifier;

        self.iirSampleHL = (self.iirSampleHL * (1.0 - BEQ)) + (basscatchL * BEQ);
        basscatchL = self.iirSampleHL * bassdrive;
        bridgerectifier = (basscatchL).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchL > 0.0 {
            basscatchL = bridgerectifier;
        } else {
            basscatchL = -bridgerectifier;
        }
        if inputSampleL > 1.0 {
            inputSampleL = 1.0;
        }
        if inputSampleL < -1.0 {
            inputSampleL = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothHL
            + (self.secondHL * inverse)
            + (self.thirdHL * bridgerectifier)
            + inputSampleL);
        self.thirdHL = self.secondHL;
        self.secondHL = self.smoothHL;
        self.smoothHL = inputSampleL;
        inputSampleL = bridgerectifier;

        self.iirSampleHR = (self.iirSampleHR * (1.0 - BEQ)) + (basscatchR * BEQ);
        basscatchR = self.iirSampleHR * bassdrive;
        bridgerectifier = (basscatchR).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchR > 0.0 {
            basscatchR = bridgerectifier;
        } else {
            basscatchR = -bridgerectifier;
        }
        if inputSampleR > 1.0 {
            inputSampleR = 1.0;
        }
        if inputSampleR < -1.0 {
            inputSampleR = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothHR
            + (self.secondHR * inverse)
            + (self.thirdHR * bridgerectifier)
            + inputSampleR);
        self.thirdHR = self.secondHR;
        self.secondHR = self.smoothHR;
        self.smoothHR = inputSampleR;
        inputSampleR = bridgerectifier;

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

        self.iirSampleIL = (self.iirSampleIL * (1.0 - BEQ)) + (basscatchL * BEQ);
        basscatchL = self.iirSampleIL * bassdrive;
        bridgerectifier = (basscatchL).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchL > 0.0 {
            basscatchL = bridgerectifier;
        } else {
            basscatchL = -bridgerectifier;
        }
        if inputSampleL > 1.0 {
            inputSampleL = 1.0;
        }
        if inputSampleL < -1.0 {
            inputSampleL = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothIL
            + (self.secondIL * inverse)
            + (self.thirdIL * bridgerectifier)
            + inputSampleL);
        self.thirdIL = self.secondIL;
        self.secondIL = self.smoothIL;
        self.smoothIL = inputSampleL;
        inputSampleL = bridgerectifier;

        bridgerectifier = (inputSampleL).abs();
        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothJL
            + (self.secondJL * inverse)
            + (self.thirdJL * bridgerectifier)
            + inputSampleL);
        self.thirdJL = self.secondJL;
        self.secondJL = self.smoothJL;
        self.smoothJL = inputSampleL;
        inputSampleL = bridgerectifier;

        bridgerectifier = (inputSampleL).abs();
        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothKL
            + (self.secondKL * inverse)
            + (self.thirdKL * bridgerectifier)
            + inputSampleL);
        self.thirdKL = self.secondKL;
        self.secondKL = self.smoothKL;
        self.smoothKL = inputSampleL;
        inputSampleL = bridgerectifier;

        self.iirSampleIR = (self.iirSampleIR * (1.0 - BEQ)) + (basscatchR * BEQ);
        basscatchR = self.iirSampleIR * bassdrive;
        bridgerectifier = (basscatchR).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if basscatchR > 0.0 {
            basscatchR = bridgerectifier;
        } else {
            basscatchR = -bridgerectifier;
        }
        if inputSampleR > 1.0 {
            inputSampleR = 1.0;
        }
        if inputSampleR < -1.0 {
            inputSampleR = -1.0;
        }

        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothIR
            + (self.secondIR * inverse)
            + (self.thirdIR * bridgerectifier)
            + inputSampleR);
        self.thirdIR = self.secondIR;
        self.secondIR = self.smoothIR;
        self.smoothIR = inputSampleR;
        inputSampleR = bridgerectifier;

        bridgerectifier = (inputSampleR).abs();
        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothJR
            + (self.secondJR * inverse)
            + (self.thirdJR * bridgerectifier)
            + inputSampleR);
        self.thirdJR = self.secondJR;
        self.secondJR = self.smoothJR;
        self.smoothJR = inputSampleR;
        inputSampleR = bridgerectifier;

        bridgerectifier = (inputSampleR).abs();
        inverse = (bridgerectifier + 1.0) / 2.0;
        bridgerectifier = (self.smoothKR
            + (self.secondKR * inverse)
            + (self.thirdKR * bridgerectifier)
            + inputSampleR);
        self.thirdKR = self.secondKR;
        self.secondKR = self.smoothKR;
        self.smoothKR = inputSampleR;
        inputSampleR = bridgerectifier;

        basscatchL /= 2.0;
        inputSampleL = (inputSampleL * toneEQ) + basscatchL;

        basscatchR /= 2.0;
        inputSampleR = (inputSampleR * toneEQ) + basscatchR;

        bridgerectifier = (inputSampleL * outputlevel).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if inputSampleL > 0.0 {
            inputSampleL = bridgerectifier;
        } else {
            inputSampleL = -bridgerectifier;
        }
        inputSampleL += basscatchL;

        inputSampleL /= (1.0 + toneEQ);

        bridgerectifier = (inputSampleR * outputlevel).abs();
        if bridgerectifier > 1.57079633 {
            bridgerectifier = 1.57079633;
        }
        bridgerectifier = (bridgerectifier).sin();
        if inputSampleR > 0.0 {
            inputSampleR = bridgerectifier;
        } else {
            inputSampleR = -bridgerectifier;
        }
        inputSampleR += basscatchR;

        inputSampleR /= (1.0 + toneEQ);

        let mut randy = ((fpdL as f64 / u32::MAX as f64) * 0.061);
        inputSampleL = ((inputSampleL * (1.0 - randy)) + (self.storeSampleL * randy)) * outputlevel;
        self.storeSampleL = inputSampleL;

        randy = ((fpdR as f64 / u32::MAX as f64) * 0.061);
        inputSampleR = ((inputSampleR * (1.0 - randy)) + (self.storeSampleR * randy)) * outputlevel;
        self.storeSampleR = inputSampleR;

        if wet != 1.0 {
            inputSampleL = (inputSampleL * wet) + (drySampleL * (1.0 - wet));
            inputSampleR = (inputSampleR * wet) + (drySampleR * (1.0 - wet));
        }

        (inputSampleL, inputSampleR)
    }
}
