//! Ordered sept section from Energy processReplacing.
use super::Energy;
impl Energy {
    pub(super) fn sept(&mut self, input: [f64; 2], correction: &mut [f64; 2], intensity: f64) {
        if self.countE < 1 || self.countE > 7 {
            self.countE = 1;
        }
        match self.countE {
            1 => {
                self.septFactorL = self.PrevFL - input[0];
                self.septAL += self.septFactorL;
                self.septGL -= self.septFactorL;
                self.septFactorL = self.septAL * intensity;

                self.septFactorR = self.PrevFR - input[1];
                self.septAR += self.septFactorR;
                self.septGR -= self.septFactorR;
                self.septFactorR = self.septAR * intensity;
            }
            2 => {
                self.septFactorL = self.PrevFL - input[0];
                self.septBL += self.septFactorL;
                self.septAL -= self.septFactorL;
                self.septFactorL = self.septBL * intensity;

                self.septFactorR = self.PrevFR - input[1];
                self.septBR += self.septFactorR;
                self.septAR -= self.septFactorR;
                self.septFactorR = self.septBR * intensity;
            }
            3 => {
                self.septFactorL = self.PrevFL - input[0];
                self.septCL += self.septFactorL;
                self.septBL -= self.septFactorL;
                self.septFactorL = self.septCL * intensity;

                self.septFactorR = self.PrevFR - input[1];
                self.septCR += self.septFactorR;
                self.septBR -= self.septFactorR;
                self.septFactorR = self.septCR * intensity;
            }
            4 => {
                self.septFactorL = self.PrevFL - input[0];
                self.septDL += self.septFactorL;
                self.septCL -= self.septFactorL;
                self.septFactorL = self.septDL * intensity;

                self.septFactorR = self.PrevFR - input[1];
                self.septDR += self.septFactorR;
                self.septCR -= self.septFactorR;
                self.septFactorR = self.septDR * intensity;
            }
            5 => {
                self.septFactorL = self.PrevFL - input[0];
                self.septEL += self.septFactorL;
                self.septDL -= self.septFactorL;
                self.septFactorL = self.septEL * intensity;

                self.septFactorR = self.PrevFR - input[1];
                self.septER += self.septFactorR;
                self.septDR -= self.septFactorR;
                self.septFactorR = self.septER * intensity;
            }
            6 => {
                self.septFactorL = self.PrevFL - input[0];
                self.septFL += self.septFactorL;
                self.septEL -= self.septFactorL;
                self.septFactorL = self.septFL * intensity;

                self.septFactorR = self.PrevFR - input[1];
                self.septFR += self.septFactorR;
                self.septER -= self.septFactorR;
                self.septFactorR = self.septFR * intensity;
            }
            7 => {
                self.septFactorL = self.PrevFL - input[0];
                self.septGL += self.septFactorL;
                self.septFL -= self.septFactorL;
                self.septFactorL = self.septGL * intensity;

                self.septFactorR = self.PrevFR - input[1];
                self.septGR += self.septFactorR;
                self.septFR -= self.septFactorR;
                self.septFactorR = self.septGR * intensity;
            }
            _ => unreachable!(),
        }
        self.septAL /= 2.0;
        self.septBL /= 2.0;
        self.septCL /= 2.0;
        self.septDL /= 2.0;
        self.septEL /= 2.0;
        self.septFL /= 2.0;
        self.septGL /= 2.0;
        correction[0] = correction[0] + self.septFactorL;

        self.septAR /= 2.0;
        self.septBR /= 2.0;
        self.septCR /= 2.0;
        self.septDR /= 2.0;
        self.septER /= 2.0;
        self.septFR /= 2.0;
        self.septGR /= 2.0;
        correction[1] = correction[1] + self.septFactorR;
        self.countE += 1;
    }
}
