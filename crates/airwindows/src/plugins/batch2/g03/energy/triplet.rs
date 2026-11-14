//! Ordered triplet section from Energy processReplacing.
use super::Energy;
impl Energy {
    pub(super) fn triplet(&mut self, input: [f64; 2], correction: &mut [f64; 2], intensity: f64) {
        if self.countA < 1 || self.countA > 3 {
            self.countA = 1;
        }
        match self.countA {
            1 => {
                self.tripletFactorL = self.PrevBL - input[0];
                self.tripletAL += self.tripletFactorL;
                self.tripletCL -= self.tripletFactorL;
                self.tripletFactorL = self.tripletAL * intensity;

                self.tripletFactorR = self.PrevBR - input[1];
                self.tripletAR += self.tripletFactorR;
                self.tripletCR -= self.tripletFactorR;
                self.tripletFactorR = self.tripletAR * intensity;
            }
            2 => {
                self.tripletFactorL = self.PrevBL - input[0];
                self.tripletBL += self.tripletFactorL;
                self.tripletAL -= self.tripletFactorL;
                self.tripletFactorL = self.tripletBL * intensity;

                self.tripletFactorR = self.PrevBR - input[1];
                self.tripletBR += self.tripletFactorR;
                self.tripletAR -= self.tripletFactorR;
                self.tripletFactorR = self.tripletBR * intensity;
            }
            3 => {
                self.tripletFactorL = self.PrevBL - input[0];
                self.tripletCL += self.tripletFactorL;
                self.tripletBL -= self.tripletFactorL;
                self.tripletFactorL = self.tripletCL * intensity;

                self.tripletFactorR = self.PrevBR - input[1];
                self.tripletCR += self.tripletFactorR;
                self.tripletBR -= self.tripletFactorR;
                self.tripletFactorR = self.tripletCR * intensity;
            }
            _ => unreachable!(),
        }
        self.tripletAL /= 2.0;
        self.tripletBL /= 2.0;
        self.tripletCL /= 2.0;
        correction[0] = correction[0] + self.tripletFactorL;

        self.tripletAR /= 2.0;
        self.tripletBR /= 2.0;
        self.tripletCR /= 2.0;
        correction[1] = correction[1] + self.tripletFactorR;
        self.countA += 1;
    }
}
