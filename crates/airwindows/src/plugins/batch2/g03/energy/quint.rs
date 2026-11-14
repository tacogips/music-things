//! Ordered quint section from Energy processReplacing.
use super::Energy;
impl Energy {
    pub(super) fn quint(&mut self, input: [f64; 2], correction: &mut [f64; 2], intensity: f64) {
        if self.countC < 1 || self.countC > 5 {
            self.countC = 1;
        }
        match self.countC {
            1 => {
                self.quintFactorL = self.PrevDL - input[0];
                self.quintAL += self.quintFactorL;
                self.quintEL -= self.quintFactorL;
                self.quintFactorL = self.quintAL * intensity;

                self.quintFactorR = self.PrevDR - input[1];
                self.quintAR += self.quintFactorR;
                self.quintER -= self.quintFactorR;
                self.quintFactorR = self.quintAR * intensity;
            }
            2 => {
                self.quintFactorL = self.PrevDL - input[0];
                self.quintBL += self.quintFactorL;
                self.quintAL -= self.quintFactorL;
                self.quintFactorL = self.quintBL * intensity;

                self.quintFactorR = self.PrevDR - input[1];
                self.quintBR += self.quintFactorR;
                self.quintAR -= self.quintFactorR;
                self.quintFactorR = self.quintBR * intensity;
            }
            3 => {
                self.quintFactorL = self.PrevDL - input[0];
                self.quintCL += self.quintFactorL;
                self.quintBL -= self.quintFactorL;
                self.quintFactorL = self.quintCL * intensity;

                self.quintFactorR = self.PrevDR - input[1];
                self.quintCR += self.quintFactorR;
                self.quintBR -= self.quintFactorR;
                self.quintFactorR = self.quintCR * intensity;
            }
            4 => {
                self.quintFactorL = self.PrevDL - input[0];
                self.quintDL += self.quintFactorL;
                self.quintCL -= self.quintFactorL;
                self.quintFactorL = self.quintDL * intensity;

                self.quintFactorR = self.PrevDR - input[1];
                self.quintDR += self.quintFactorR;
                self.quintCR -= self.quintFactorR;
                self.quintFactorR = self.quintDR * intensity;
            }
            5 => {
                self.quintFactorL = self.PrevDL - input[0];
                self.quintEL += self.quintFactorL;
                self.quintDL -= self.quintFactorL;
                self.quintFactorL = self.quintEL * intensity;

                self.quintFactorR = self.PrevDR - input[1];
                self.quintER += self.quintFactorR;
                self.quintDR -= self.quintFactorR;
                self.quintFactorR = self.quintER * intensity;
            }
            _ => unreachable!(),
        }
        self.quintAL /= 2.0;
        self.quintBL /= 2.0;
        self.quintCL /= 2.0;
        self.quintDL /= 2.0;
        self.quintEL /= 2.0;
        correction[0] = correction[0] + self.quintFactorL;

        self.quintAR /= 2.0;
        self.quintBR /= 2.0;
        self.quintCR /= 2.0;
        self.quintDR /= 2.0;
        self.quintER /= 2.0;
        correction[1] = correction[1] + self.quintFactorR;
        self.countC += 1;
    }
}
