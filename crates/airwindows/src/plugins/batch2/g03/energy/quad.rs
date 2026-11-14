//! Ordered quad section from Energy processReplacing.
use super::Energy;
impl Energy {
    pub(super) fn quad(&mut self, input: [f64; 2], correction: &mut [f64; 2], intensity: f64) {
        if self.countB < 1 || self.countB > 4 {
            self.countB = 1;
        }
        match self.countB {
            1 => {
                self.quadFactorL = self.PrevCL - input[0];
                self.quadAL += self.quadFactorL;
                self.quadDL -= self.quadFactorL;
                self.quadFactorL = self.quadAL * intensity;

                self.quadFactorR = self.PrevCR - input[1];
                self.quadAR += self.quadFactorR;
                self.quadDR -= self.quadFactorR;
                self.quadFactorR = self.quadAR * intensity;
            }
            2 => {
                self.quadFactorL = self.PrevCL - input[0];
                self.quadBL += self.quadFactorL;
                self.quadAL -= self.quadFactorL;
                self.quadFactorL = self.quadBL * intensity;

                self.quadFactorR = self.PrevCR - input[1];
                self.quadBR += self.quadFactorR;
                self.quadAR -= self.quadFactorR;
                self.quadFactorR = self.quadBR * intensity;
            }
            3 => {
                self.quadFactorL = self.PrevCL - input[0];
                self.quadCL += self.quadFactorL;
                self.quadBL -= self.quadFactorL;
                self.quadFactorL = self.quadCL * intensity;

                self.quadFactorR = self.PrevCR - input[1];
                self.quadCR += self.quadFactorR;
                self.quadBR -= self.quadFactorR;
                self.quadFactorR = self.quadCR * intensity;
            }
            4 => {
                self.quadFactorL = self.PrevCL - input[0];
                self.quadDL += self.quadFactorL;
                self.quadCL -= self.quadFactorL;
                self.quadFactorL = self.quadDL * intensity;

                self.quadFactorR = self.PrevCR - input[1];
                self.quadDR += self.quadFactorR;
                self.quadCR -= self.quadFactorR;
                self.quadFactorR = self.quadDR * intensity;
            }
            _ => unreachable!(),
        }
        self.quadAL /= 2.0;
        self.quadBL /= 2.0;
        self.quadCL /= 2.0;
        self.quadDL /= 2.0;
        correction[0] = correction[0] + self.quadFactorL;

        self.quadAR /= 2.0;
        self.quadBR /= 2.0;
        self.quadCR /= 2.0;
        self.quadDR /= 2.0;
        correction[1] = correction[1] + self.quadFactorR;
        self.countB += 1;
    }
}
