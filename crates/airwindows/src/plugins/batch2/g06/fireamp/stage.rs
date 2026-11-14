//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
use super::FireAmp;
impl FireAmp {
    pub(super) fn stages(
        &mut self,
        mut inputSampleL: f64,
        mut inputSampleR: f64,
        startlevel: f64,
        EQ: f64,
        bassfactor: f64,
        toneEQ: f64,
        BEQ: f64,
    ) -> (f64, f64) {
        let mut outSample = (inputSampleL * self.fixA[2]) + self.fixA[7];
        self.fixA[7] = (inputSampleL * self.fixA[3]) - (outSample * self.fixA[5]) + self.fixA[8];
        self.fixA[8] = (inputSampleL * self.fixA[4]) - (outSample * self.fixA[6]);
        inputSampleL = outSample;
        outSample = (inputSampleR * self.fixA[2]) + self.fixA[9];
        self.fixA[9] = (inputSampleR * self.fixA[3]) - (outSample * self.fixA[5]) + self.fixA[10];
        self.fixA[10] = (inputSampleR * self.fixA[4]) - (outSample * self.fixA[6]);
        inputSampleR = outSample;

        if inputSampleL > 1.0 {
            inputSampleL = 1.0;
        }
        if inputSampleL < -1.0 {
            inputSampleL = -1.0;
        }
        let mut basscutL = 0.98;

        let mut inputlevelL = startlevel;
        inputSampleL *= inputlevelL;
        inputlevelL = ((inputlevelL * 7.0) + 1.0) / 8.0;
        self.iirSampleAL = (self.iirSampleAL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleAL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        let mut bridgerectifier = (self.smoothAL + inputSampleL);
        self.smoothAL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleL *= inputlevelL;
        inputlevelL = ((inputlevelL * 7.0) + 1.0) / 8.0;
        self.iirSampleBL = (self.iirSampleBL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleBL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        bridgerectifier = (self.smoothBL + inputSampleL);
        self.smoothBL = inputSampleL;
        inputSampleL = bridgerectifier;

        if inputSampleR > 1.0 {
            inputSampleR = 1.0;
        }
        if inputSampleR < -1.0 {
            inputSampleR = -1.0;
        }
        let mut basscutR = 0.98;

        let mut inputlevelR = startlevel;
        inputSampleR *= inputlevelR;
        inputlevelR = ((inputlevelR * 7.0) + 1.0) / 8.0;
        self.iirSampleAR = (self.iirSampleAR * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleAR * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothAR + inputSampleR);
        self.smoothAR = inputSampleR;
        inputSampleR = bridgerectifier;

        inputSampleR *= inputlevelR;
        inputlevelR = ((inputlevelR * 7.0) + 1.0) / 8.0;
        self.iirSampleBR = (self.iirSampleBR * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleBR * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothBR + inputSampleR);
        self.smoothBR = inputSampleR;
        inputSampleR = bridgerectifier;

        outSample = (inputSampleL * self.fixB[2]) + self.fixB[7];
        self.fixB[7] = (inputSampleL * self.fixB[3]) - (outSample * self.fixB[5]) + self.fixB[8];
        self.fixB[8] = (inputSampleL * self.fixB[4]) - (outSample * self.fixB[6]);
        inputSampleL = outSample;
        outSample = (inputSampleR * self.fixB[2]) + self.fixB[9];
        self.fixB[9] = (inputSampleR * self.fixB[3]) - (outSample * self.fixB[5]) + self.fixB[10];
        self.fixB[10] = (inputSampleR * self.fixB[4]) - (outSample * self.fixB[6]);
        inputSampleR = outSample;

        inputSampleL *= inputlevelL;
        inputlevelL = ((inputlevelL * 7.0) + 1.0) / 8.0;
        self.iirSampleCL = (self.iirSampleCL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleCL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        bridgerectifier = (self.smoothCL + inputSampleL);
        self.smoothCL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleL *= inputlevelL;
        inputlevelL = ((inputlevelL * 7.0) + 1.0) / 8.0;
        self.iirSampleDL = (self.iirSampleDL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleDL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        bridgerectifier = (self.smoothDL + inputSampleL);
        self.smoothDL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleR *= inputlevelR;
        inputlevelR = ((inputlevelR * 7.0) + 1.0) / 8.0;
        self.iirSampleCR = (self.iirSampleCR * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleCR * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothCR + inputSampleR);
        self.smoothCR = inputSampleR;
        inputSampleR = bridgerectifier;

        inputSampleR *= inputlevelR;
        inputlevelR = ((inputlevelR * 7.0) + 1.0) / 8.0;
        self.iirSampleDR = (self.iirSampleDR * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleDR * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothDR + inputSampleR);
        self.smoothDR = inputSampleR;
        inputSampleR = bridgerectifier;

        outSample = (inputSampleL * self.fixC[2]) + self.fixC[7];
        self.fixC[7] = (inputSampleL * self.fixC[3]) - (outSample * self.fixC[5]) + self.fixC[8];
        self.fixC[8] = (inputSampleL * self.fixC[4]) - (outSample * self.fixC[6]);
        inputSampleL = outSample;
        outSample = (inputSampleR * self.fixC[2]) + self.fixC[9];
        self.fixC[9] = (inputSampleR * self.fixC[3]) - (outSample * self.fixC[5]) + self.fixC[10];
        self.fixC[10] = (inputSampleR * self.fixC[4]) - (outSample * self.fixC[6]);
        inputSampleR = outSample;

        inputSampleL *= inputlevelL;
        inputlevelL = ((inputlevelL * 7.0) + 1.0) / 8.0;
        self.iirSampleEL = (self.iirSampleEL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleEL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        bridgerectifier = (self.smoothEL + inputSampleL);
        self.smoothEL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleL *= inputlevelL;
        inputlevelL = ((inputlevelL * 7.0) + 1.0) / 8.0;
        self.iirSampleFL = (self.iirSampleFL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleFL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        bridgerectifier = (self.smoothFL + inputSampleL);
        self.smoothFL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleR *= inputlevelR;
        inputlevelR = ((inputlevelR * 7.0) + 1.0) / 8.0;
        self.iirSampleER = (self.iirSampleER * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleER * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothER + inputSampleR);
        self.smoothER = inputSampleR;
        inputSampleR = bridgerectifier;

        inputSampleR *= inputlevelR;
        inputlevelR = ((inputlevelR * 7.0) + 1.0) / 8.0;
        self.iirSampleFR = (self.iirSampleFR * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleFR * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothFR + inputSampleR);
        self.smoothFR = inputSampleR;
        inputSampleR = bridgerectifier;

        outSample = (inputSampleL * self.fixD[2]) + self.fixD[7];
        self.fixD[7] = (inputSampleL * self.fixD[3]) - (outSample * self.fixD[5]) + self.fixD[8];
        self.fixD[8] = (inputSampleL * self.fixD[4]) - (outSample * self.fixD[6]);
        inputSampleL = outSample;
        outSample = (inputSampleR * self.fixD[2]) + self.fixD[9];
        self.fixD[9] = (inputSampleR * self.fixD[3]) - (outSample * self.fixD[5]) + self.fixD[10];
        self.fixD[10] = (inputSampleR * self.fixD[4]) - (outSample * self.fixD[6]);
        inputSampleR = outSample;

        inputSampleL *= inputlevelL;
        inputlevelL = ((inputlevelL * 7.0) + 1.0) / 8.0;
        self.iirSampleGL = (self.iirSampleGL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleGL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        bridgerectifier = (self.smoothGL + inputSampleL);
        self.smoothGL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleL *= inputlevelL;
        inputlevelL = ((inputlevelL * 7.0) + 1.0) / 8.0;
        self.iirSampleHL = (self.iirSampleHL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleHL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        bridgerectifier = (self.smoothHL + inputSampleL);
        self.smoothHL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleR *= inputlevelR;
        inputlevelR = ((inputlevelR * 7.0) + 1.0) / 8.0;
        self.iirSampleGR = (self.iirSampleGR * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleGR * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothGR + inputSampleR);
        self.smoothGR = inputSampleR;
        inputSampleR = bridgerectifier;

        inputSampleR *= inputlevelR;
        inputlevelR = ((inputlevelR * 7.0) + 1.0) / 8.0;
        self.iirSampleHR = (self.iirSampleHR * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleHR * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothHR + inputSampleR);
        self.smoothHR = inputSampleR;
        inputSampleR = bridgerectifier;

        outSample = (inputSampleL * self.fixE[2]) + self.fixE[7];
        self.fixE[7] = (inputSampleL * self.fixE[3]) - (outSample * self.fixE[5]) + self.fixE[8];
        self.fixE[8] = (inputSampleL * self.fixE[4]) - (outSample * self.fixE[6]);
        inputSampleL = outSample;
        outSample = (inputSampleR * self.fixE[2]) + self.fixE[9];
        self.fixE[9] = (inputSampleR * self.fixE[3]) - (outSample * self.fixE[5]) + self.fixE[10];
        self.fixE[10] = (inputSampleR * self.fixE[4]) - (outSample * self.fixE[6]);
        inputSampleR = outSample;

        inputSampleL *= inputlevelL;
        inputlevelL = ((inputlevelL * 7.0) + 1.0) / 8.0;
        self.iirSampleIL = (self.iirSampleIL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleIL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        bridgerectifier = (self.smoothIL + inputSampleL);
        self.smoothIL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleL *= inputlevelL;
        inputlevelL = ((inputlevelL * 7.0) + 1.0) / 8.0;
        self.iirSampleJL = (self.iirSampleJL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleJL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        bridgerectifier = (self.smoothJL + inputSampleL);
        self.smoothJL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleR *= inputlevelR;
        inputlevelR = ((inputlevelR * 7.0) + 1.0) / 8.0;
        self.iirSampleIR = (self.iirSampleIR * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleIR * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothIR + inputSampleR);
        self.smoothIR = inputSampleR;
        inputSampleR = bridgerectifier;

        inputSampleR *= inputlevelR;
        inputlevelR = ((inputlevelR * 7.0) + 1.0) / 8.0;
        self.iirSampleJR = (self.iirSampleJR * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleJR * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothJR + inputSampleR);
        self.smoothJR = inputSampleR;
        inputSampleR = bridgerectifier;

        outSample = (inputSampleL * self.fixF[2]) + self.fixF[7];
        self.fixF[7] = (inputSampleL * self.fixF[3]) - (outSample * self.fixF[5]) + self.fixF[8];
        self.fixF[8] = (inputSampleL * self.fixF[4]) - (outSample * self.fixF[6]);
        inputSampleL = outSample;
        outSample = (inputSampleR * self.fixF[2]) + self.fixF[9];
        self.fixF[9] = (inputSampleR * self.fixF[3]) - (outSample * self.fixF[5]) + self.fixF[10];
        self.fixF[10] = (inputSampleR * self.fixF[4]) - (outSample * self.fixF[6]);
        inputSampleR = outSample;

        inputSampleL *= inputlevelL;
        inputlevelL = ((inputlevelL * 7.0) + 1.0) / 8.0;
        self.iirSampleKL = (self.iirSampleKL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleKL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        bridgerectifier = (self.smoothKL + inputSampleL);
        self.smoothKL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleL *= inputlevelL;
        self.iirSampleLL = (self.iirSampleLL * (1.0 - EQ)) + (inputSampleL * EQ);
        basscutL *= bassfactor;
        inputSampleL = inputSampleL - (self.iirSampleLL * basscutL);

        inputSampleL -=
            (inputSampleL * (inputSampleL.abs() * 0.654) * (inputSampleL.abs() * 0.654));

        bridgerectifier = (self.smoothLL + inputSampleL);
        self.smoothLL = inputSampleL;
        inputSampleL = bridgerectifier;

        inputSampleR *= inputlevelR;
        inputlevelR = ((inputlevelR * 7.0) + 1.0) / 8.0;
        self.iirSampleKR = (self.iirSampleKR * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleKR * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothKR + inputSampleR);
        self.smoothKR = inputSampleR;
        inputSampleR = bridgerectifier;

        inputSampleR *= inputlevelR;
        self.iirSampleLR = (self.iirSampleLR * (1.0 - EQ)) + (inputSampleR * EQ);
        basscutR *= bassfactor;
        inputSampleR = inputSampleR - (self.iirSampleLR * basscutR);

        inputSampleR -=
            (inputSampleR * (inputSampleR.abs() * 0.654) * (inputSampleR.abs() * 0.654));

        bridgerectifier = (self.smoothLR + inputSampleR);
        self.smoothLR = inputSampleR;
        inputSampleR = bridgerectifier;

        self.iirLowpassL = (self.iirLowpassL * (1.0 - toneEQ)) + (inputSampleL * toneEQ);
        inputSampleL = self.iirLowpassL;

        self.iirLowpassR = (self.iirLowpassR * (1.0 - toneEQ)) + (inputSampleR * toneEQ);
        inputSampleR = self.iirLowpassR;

        self.iirSpkAL = (self.iirSpkAL * (1.0 - BEQ)) + (inputSampleL * BEQ);

        self.iirSpkAR = (self.iirSpkAR * (1.0 - BEQ)) + (inputSampleR * BEQ);

        (inputSampleL, inputSampleR)
    }
}
