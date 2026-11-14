//! Ordered sext section from Energy processReplacing.
use super::Energy;
impl Energy {
    pub(super) fn sext(&mut self, input: [f64; 2], correction: &mut [f64; 2], intensity: f64) {
        if self.countD < 1 || self.countD > 6 {
            self.countD = 1;
        }
        match self.countD {
            1 => {
                self.sextFactorL = self.PrevEL - input[0];
                self.sextAL += self.sextFactorL;
                self.sextFL -= self.sextFactorL;
                self.sextFactorL = self.sextAL * intensity;

                self.sextFactorR = self.PrevER - input[1];
                self.sextAR += self.sextFactorR;
                self.sextFR -= self.sextFactorR;
                self.sextFactorR = self.sextAR * intensity;
            }
            2 => {
                self.sextFactorL = self.PrevEL - input[0];
                self.sextBL += self.sextFactorL;
                self.sextAL -= self.sextFactorL;
                self.sextFactorL = self.sextBL * intensity;

                self.sextFactorR = self.PrevER - input[1];
                self.sextBR += self.sextFactorR;
                self.sextAR -= self.sextFactorR;
                self.sextFactorR = self.sextBR * intensity;
            }
            3 => {
                self.sextFactorL = self.PrevEL - input[0];
                self.sextCL += self.sextFactorL;
                self.sextBL -= self.sextFactorL;
                self.sextFactorL = self.sextCL * intensity;

                self.sextFactorR = self.PrevER - input[1];
                self.sextCR += self.sextFactorR;
                self.sextBR -= self.sextFactorR;
                self.sextFactorR = self.sextCR * intensity;
            }
            4 => {
                self.sextFactorL = self.PrevEL - input[0];
                self.sextDL += self.sextFactorL;
                self.sextCL -= self.sextFactorL;
                self.sextFactorL = self.sextDL * intensity;

                self.sextFactorR = self.PrevER - input[1];
                self.sextDR += self.sextFactorR;
                self.sextCR -= self.sextFactorR;
                self.sextFactorR = self.sextDR * intensity;
            }
            5 => {
                self.sextFactorL = self.PrevEL - input[0];
                self.sextEL += self.sextFactorL;
                self.sextDL -= self.sextFactorL;
                self.sextFactorL = self.sextEL * intensity;

                self.sextFactorR = self.PrevER - input[1];
                self.sextER += self.sextFactorR;
                self.sextDR -= self.sextFactorR;
                self.sextFactorR = self.sextER * intensity;
            }
            6 => {
                self.sextFactorL = self.PrevEL - input[0];
                self.sextFL += self.sextFactorL;
                self.sextEL -= self.sextFactorL;
                self.sextFactorL = self.sextFL * intensity;

                self.sextFactorR = self.PrevER - input[1];
                self.sextFR += self.sextFactorR;
                self.sextER -= self.sextFactorR;
                self.sextFactorR = self.sextFR * intensity;
            }
            _ => unreachable!(),
        }
        self.sextAL /= 2.0;
        self.sextBL /= 2.0;
        self.sextCL /= 2.0;
        self.sextDL /= 2.0;
        self.sextEL /= 2.0;
        self.sextFL /= 2.0;
        correction[0] = correction[0] + self.sextFactorL;

        self.sextAR /= 2.0;
        self.sextBR /= 2.0;
        self.sextCR /= 2.0;
        self.sextDR /= 2.0;
        self.sextER /= 2.0;
        self.sextFR /= 2.0;
        correction[1] = correction[1] + self.sextFactorR;
        self.countD += 1;
    }
}
