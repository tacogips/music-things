//! Ordered duo section from Energy processReplacing.
use super::Energy;
impl Energy {
    pub(super) fn duo(&mut self, input: [f64; 2], correction: &mut [f64; 2], intensity: f64) {
        self.duoFactorL = self.PrevAL - input[0];
        self.duoFactorR = self.PrevAR - input[1];
        if (self.flip) {
            self.duoEvenL += self.duoFactorL;
            self.duoOddL -= self.duoFactorL;
            self.duoFactorL = self.duoEvenL * intensity;

            self.duoEvenR += self.duoFactorR;
            self.duoOddR -= self.duoFactorR;
            self.duoFactorR = self.duoEvenR * intensity;
        } else {
            self.duoOddL += self.duoFactorL;
            self.duoEvenL -= self.duoFactorL;
            self.duoFactorL = self.duoOddL * intensity;

            self.duoOddR += self.duoFactorR;
            self.duoEvenR -= self.duoFactorR;
            self.duoFactorR = self.duoOddR * intensity;
        }
        self.duoOddL = (self.duoOddL - ((self.duoOddL - self.duoEvenL) / 256.0)) / 2.0;
        self.duoEvenL = (self.duoEvenL - ((self.duoEvenL - self.duoOddL) / 256.0)) / 2.0;
        correction[0] = correction[0] + self.duoFactorL;

        self.duoOddR = (self.duoOddR - ((self.duoOddR - self.duoEvenR) / 256.0)) / 2.0;
        self.duoEvenR = (self.duoEvenR - ((self.duoEvenR - self.duoOddR) / 256.0)) / 2.0;
        correction[1] = correction[1] + self.duoFactorR;
        self.flip = !self.flip;
    }
}
