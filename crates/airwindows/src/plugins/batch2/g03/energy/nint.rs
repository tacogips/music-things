//! Ordered nint section from Energy processReplacing.
use super::Energy;
impl Energy {
    pub(super) fn nint(&mut self, input: [f64; 2], correction: &mut [f64; 2], intensity: f64) {
        if self.countG < 1 || self.countG > 9 {
            self.countG = 1;
        }
        match self.countG {
            1 => {
                self.nintFactorL = self.PrevHL - input[0];
                self.nintAL += self.nintFactorL;
                self.nintIL -= self.nintFactorL;
                self.nintFactorL = self.nintAL * intensity;

                self.nintFactorR = self.PrevHR - input[1];
                self.nintAR += self.nintFactorR;
                self.nintIR -= self.nintFactorR;
                self.nintFactorR = self.nintAR * intensity;
            }
            2 => {
                self.nintFactorL = self.PrevHL - input[0];
                self.nintBL += self.nintFactorL;
                self.nintAL -= self.nintFactorL;
                self.nintFactorL = self.nintBL * intensity;

                self.nintFactorR = self.PrevHR - input[1];
                self.nintBR += self.nintFactorR;
                self.nintAR -= self.nintFactorR;
                self.nintFactorR = self.nintBR * intensity;
            }
            3 => {
                self.nintFactorL = self.PrevHL - input[0];
                self.nintCL += self.nintFactorL;
                self.nintBL -= self.nintFactorL;
                self.nintFactorL = self.nintCL * intensity;

                self.nintFactorR = self.PrevHR - input[1];
                self.nintCR += self.nintFactorR;
                self.nintBR -= self.nintFactorR;
                self.nintFactorR = self.nintCR * intensity;
            }
            4 => {
                self.nintFactorL = self.PrevHL - input[0];
                self.nintDL += self.nintFactorL;
                self.nintCL -= self.nintFactorL;
                self.nintFactorL = self.nintDL * intensity;

                self.nintFactorR = self.PrevHR - input[1];
                self.nintDR += self.nintFactorR;
                self.nintCR -= self.nintFactorR;
                self.nintFactorR = self.nintDR * intensity;
            }
            5 => {
                self.nintFactorL = self.PrevHL - input[0];
                self.nintEL += self.nintFactorL;
                self.nintDL -= self.nintFactorL;
                self.nintFactorL = self.nintEL * intensity;

                self.nintFactorR = self.PrevHR - input[1];
                self.nintER += self.nintFactorR;
                self.nintDR -= self.nintFactorR;
                self.nintFactorR = self.nintER * intensity;
            }
            6 => {
                self.nintFactorL = self.PrevHL - input[0];
                self.nintFL += self.nintFactorL;
                self.nintEL -= self.nintFactorL;
                self.nintFactorL = self.nintFL * intensity;

                self.nintFactorR = self.PrevHR - input[1];
                self.nintFR += self.nintFactorR;
                self.nintER -= self.nintFactorR;
                self.nintFactorR = self.nintFR * intensity;
            }
            7 => {
                self.nintFactorL = self.PrevHL - input[0];
                self.nintGL += self.nintFactorL;
                self.nintFL -= self.nintFactorL;
                self.nintFactorL = self.nintGL * intensity;

                self.nintFactorR = self.PrevHR - input[1];
                self.nintGR += self.nintFactorR;
                self.nintFR -= self.nintFactorR;
                self.nintFactorR = self.nintGR * intensity;
            }
            8 => {
                self.nintFactorL = self.PrevHL - input[0];
                self.nintHL += self.nintFactorL;
                self.nintGL -= self.nintFactorL;
                self.nintFactorL = self.nintHL * intensity;

                self.nintFactorR = self.PrevHR - input[1];
                self.nintHR += self.nintFactorR;
                self.nintGR -= self.nintFactorR;
                self.nintFactorR = self.nintHR * intensity;
            }
            9 => {
                self.nintFactorL = self.PrevHL - input[0];
                self.nintIL += self.nintFactorL;
                self.nintHL -= self.nintFactorL;
                self.nintFactorL = self.nintIL * intensity;

                self.nintFactorR = self.PrevHR - input[1];
                self.nintIR += self.nintFactorR;
                self.nintHR -= self.nintFactorR;
                self.nintFactorR = self.nintIR * intensity;
            }
            _ => unreachable!(),
        }
        self.nintAL /= 2.0;
        self.nintBL /= 2.0;
        self.nintCL /= 2.0;
        self.nintDL /= 2.0;
        self.nintEL /= 2.0;
        self.nintFL /= 2.0;
        self.nintGL /= 2.0;
        self.nintHL /= 2.0;
        self.nintIL /= 2.0;
        correction[0] = correction[0] + self.nintFactorL;

        self.nintAR /= 2.0;
        self.nintBR /= 2.0;
        self.nintCR /= 2.0;
        self.nintDR /= 2.0;
        self.nintER /= 2.0;
        self.nintFR /= 2.0;
        self.nintGR /= 2.0;
        self.nintHR /= 2.0;
        self.nintIR /= 2.0;
        correction[1] = correction[1] + self.nintFactorR;
        self.countG += 1;
    }
}
