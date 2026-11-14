//! Ordered oct section from Energy processReplacing.
use super::Energy;
impl Energy {
    pub(super) fn oct(&mut self, input: [f64; 2], correction: &mut [f64; 2], intensity: f64) {
        if self.countF < 1 || self.countF > 8 {
            self.countF = 1;
        }
        match self.countF {
            1 => {
                self.octFactorL = self.PrevGL - input[0];
                self.octAL += self.octFactorL;
                self.octHL -= self.octFactorL;
                self.octFactorL = self.octAL * intensity;

                self.octFactorR = self.PrevGR - input[1];
                self.octAR += self.octFactorR;
                self.octHR -= self.octFactorR;
                self.octFactorR = self.octAR * intensity;
            }
            2 => {
                self.octFactorL = self.PrevGL - input[0];
                self.octBL += self.octFactorL;
                self.octAL -= self.octFactorL;
                self.octFactorL = self.octBL * intensity;

                self.octFactorR = self.PrevGR - input[1];
                self.octBR += self.octFactorR;
                self.octAR -= self.octFactorR;
                self.octFactorR = self.octBR * intensity;
            }
            3 => {
                self.octFactorL = self.PrevGL - input[0];
                self.octCL += self.octFactorL;
                self.octBL -= self.octFactorL;
                self.octFactorL = self.octCL * intensity;

                self.octFactorR = self.PrevGR - input[1];
                self.octCR += self.octFactorR;
                self.octBR -= self.octFactorR;
                self.octFactorR = self.octCR * intensity;
            }
            4 => {
                self.octFactorL = self.PrevGL - input[0];
                self.octDL += self.octFactorL;
                self.octCL -= self.octFactorL;
                self.octFactorL = self.octDL * intensity;

                self.octFactorR = self.PrevGR - input[1];
                self.octDR += self.octFactorR;
                self.octCR -= self.octFactorR;
                self.octFactorR = self.octDR * intensity;
            }
            5 => {
                self.octFactorL = self.PrevGL - input[0];
                self.octEL += self.octFactorL;
                self.octDL -= self.octFactorL;
                self.octFactorL = self.octEL * intensity;

                self.octFactorR = self.PrevGR - input[1];
                self.octER += self.octFactorR;
                self.octDR -= self.octFactorR;
                self.octFactorR = self.octER * intensity;
            }
            6 => {
                self.octFactorL = self.PrevGL - input[0];
                self.octFL += self.octFactorL;
                self.octEL -= self.octFactorL;
                self.octFactorL = self.octFL * intensity;

                self.octFactorR = self.PrevGR - input[1];
                self.octFR += self.octFactorR;
                self.octER -= self.octFactorR;
                self.octFactorR = self.octFR * intensity;
            }
            7 => {
                self.octFactorL = self.PrevGL - input[0];
                self.octGL += self.octFactorL;
                self.octFL -= self.octFactorL;
                self.octFactorL = self.octGL * intensity;

                self.octFactorR = self.PrevGR - input[1];
                self.octGR += self.octFactorR;
                self.octFR -= self.octFactorR;
                self.octFactorR = self.octGR * intensity;
            }
            8 => {
                self.octFactorL = self.PrevGL - input[0];
                self.octHL += self.octFactorL;
                self.octGL -= self.octFactorL;
                self.octFactorL = self.octHL * intensity;

                self.octFactorR = self.PrevGR - input[1];
                self.octHR += self.octFactorR;
                self.octGR -= self.octFactorR;
                self.octFactorR = self.octHR * intensity;
            }
            _ => unreachable!(),
        }
        self.octAL /= 2.0;
        self.octBL /= 2.0;
        self.octCL /= 2.0;
        self.octDL /= 2.0;
        self.octEL /= 2.0;
        self.octFL /= 2.0;
        self.octGL /= 2.0;
        self.octHL /= 2.0;
        correction[0] = correction[0] + self.octFactorL;

        self.octAR /= 2.0;
        self.octBR /= 2.0;
        self.octCR /= 2.0;
        self.octDR /= 2.0;
        self.octER /= 2.0;
        self.octFR /= 2.0;
        self.octGR /= 2.0;
        self.octHR /= 2.0;
        correction[1] = correction[1] + self.octFactorR;
        self.countF += 1;
    }
}
