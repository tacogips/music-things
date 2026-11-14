//! Ordered harmonic stages from Energy2's float processing path.
use super::Energy2;
impl Energy2 {
    pub(super) fn bands(
        &mut self,
        inputSampleL: f64,
        inputSampleR: f64,
        intensity: &[f64; 8],
    ) -> (f64, f64) {
        let [duoIntensity, tripletIntensity, quadIntensity, quintIntensity, sextIntensity, septIntensity, octIntensity, nintIntensity] =
            *intensity;
        let mut correctionL = 0.0;
        let mut correctionR = 0.0;
        if duoIntensity.abs() > 0.0001 {
            self.duoFactorL = self.PrevAL - inputSampleL;
            self.duoFactorR = self.PrevAR - inputSampleR;
            if self.flip {
                self.duoEvenL += self.duoFactorL;
                self.duoOddL -= self.duoFactorL;
                self.duoFactorL = self.duoEvenL * duoIntensity;

                self.duoEvenR += self.duoFactorR;
                self.duoOddR -= self.duoFactorR;
                self.duoFactorR = self.duoEvenR * duoIntensity;
            } else {
                self.duoOddL += self.duoFactorL;
                self.duoEvenL -= self.duoFactorL;
                self.duoFactorL = self.duoOddL * duoIntensity;

                self.duoOddR += self.duoFactorR;
                self.duoEvenR -= self.duoFactorR;
                self.duoFactorR = self.duoOddR * duoIntensity;
            }
            self.duoOddL = (self.duoOddL - ((self.duoOddL - self.duoEvenL) / 256.0)) / 2.0;
            self.duoEvenL = (self.duoEvenL - ((self.duoEvenL - self.duoOddL) / 256.0)) / 2.0;
            correctionL += self.duoFactorL;

            self.duoOddR = (self.duoOddR - ((self.duoOddR - self.duoEvenR) / 256.0)) / 2.0;
            self.duoEvenR = (self.duoEvenR - ((self.duoEvenR - self.duoOddR) / 256.0)) / 2.0;
            correctionR += self.duoFactorR;
            self.flip = !self.flip;
        }

        if tripletIntensity.abs() > 0.0001 {
            if self.countA < 1 || self.countA > 3 {
                self.countA = 1;
            }
            match self.countA {
                1 => {
                    self.tripletFactorL = self.PrevBL - inputSampleL;
                    self.tripletAL += self.tripletFactorL;
                    self.tripletCL -= self.tripletFactorL;
                    self.tripletFactorL = self.tripletAL * tripletIntensity;

                    self.tripletFactorR = self.PrevBR - inputSampleR;
                    self.tripletAR += self.tripletFactorR;
                    self.tripletCR -= self.tripletFactorR;
                    self.tripletFactorR = self.tripletAR * tripletIntensity;
                }
                2 => {
                    self.tripletFactorL = self.PrevBL - inputSampleL;
                    self.tripletBL += self.tripletFactorL;
                    self.tripletAL -= self.tripletFactorL;
                    self.tripletFactorL = self.tripletBL * tripletIntensity;

                    self.tripletFactorR = self.PrevBR - inputSampleR;
                    self.tripletBR += self.tripletFactorR;
                    self.tripletAR -= self.tripletFactorR;
                    self.tripletFactorR = self.tripletBR * tripletIntensity;
                }
                3 => {
                    self.tripletFactorL = self.PrevBL - inputSampleL;
                    self.tripletCL += self.tripletFactorL;
                    self.tripletBL -= self.tripletFactorL;
                    self.tripletFactorL = self.tripletCL * tripletIntensity;

                    self.tripletFactorR = self.PrevBR - inputSampleR;
                    self.tripletCR += self.tripletFactorR;
                    self.tripletBR -= self.tripletFactorR;
                    self.tripletFactorR = self.tripletCR * tripletIntensity;
                }
                _ => {}
            }
            self.tripletAL /= 2.0;
            self.tripletBL /= 2.0;
            self.tripletCL /= 2.0;
            correctionL += self.tripletFactorL;

            self.tripletAR /= 2.0;
            self.tripletBR /= 2.0;
            self.tripletCR /= 2.0;
            correctionR += self.tripletFactorR;
            self.countA += 1;
        }

        if quadIntensity.abs() > 0.0001 {
            if self.countB < 1 || self.countB > 4 {
                self.countB = 1;
            }
            match self.countB {
                1 => {
                    self.quadFactorL = self.PrevCL - inputSampleL;
                    self.quadAL += self.quadFactorL;
                    self.quadDL -= self.quadFactorL;
                    self.quadFactorL = self.quadAL * quadIntensity;

                    self.quadFactorR = self.PrevCR - inputSampleR;
                    self.quadAR += self.quadFactorR;
                    self.quadDR -= self.quadFactorR;
                    self.quadFactorR = self.quadAR * quadIntensity;
                }
                2 => {
                    self.quadFactorL = self.PrevCL - inputSampleL;
                    self.quadBL += self.quadFactorL;
                    self.quadAL -= self.quadFactorL;
                    self.quadFactorL = self.quadBL * quadIntensity;

                    self.quadFactorR = self.PrevCR - inputSampleR;
                    self.quadBR += self.quadFactorR;
                    self.quadAR -= self.quadFactorR;
                    self.quadFactorR = self.quadBR * quadIntensity;
                }
                3 => {
                    self.quadFactorL = self.PrevCL - inputSampleL;
                    self.quadCL += self.quadFactorL;
                    self.quadBL -= self.quadFactorL;
                    self.quadFactorL = self.quadCL * quadIntensity;

                    self.quadFactorR = self.PrevCR - inputSampleR;
                    self.quadCR += self.quadFactorR;
                    self.quadBR -= self.quadFactorR;
                    self.quadFactorR = self.quadCR * quadIntensity;
                }
                4 => {
                    self.quadFactorL = self.PrevCL - inputSampleL;
                    self.quadDL += self.quadFactorL;
                    self.quadCL -= self.quadFactorL;
                    self.quadFactorL = self.quadDL * quadIntensity;

                    self.quadFactorR = self.PrevCR - inputSampleR;
                    self.quadDR += self.quadFactorR;
                    self.quadCR -= self.quadFactorR;
                    self.quadFactorR = self.quadDR * quadIntensity;
                }
                _ => {}
            }
            self.quadAL /= 2.0;
            self.quadBL /= 2.0;
            self.quadCL /= 2.0;
            self.quadDL /= 2.0;
            correctionL += self.quadFactorL;

            self.quadAR /= 2.0;
            self.quadBR /= 2.0;
            self.quadCR /= 2.0;
            self.quadDR /= 2.0;
            correctionR += self.quadFactorR;
            self.countB += 1;
        }

        if quintIntensity.abs() > 0.0001 {
            if self.countC < 1 || self.countC > 5 {
                self.countC = 1;
            }
            match self.countC {
                1 => {
                    self.quintFactorL = self.PrevDL - inputSampleL;
                    self.quintAL += self.quintFactorL;
                    self.quintEL -= self.quintFactorL;
                    self.quintFactorL = self.quintAL * quintIntensity;

                    self.quintFactorR = self.PrevDR - inputSampleR;
                    self.quintAR += self.quintFactorR;
                    self.quintER -= self.quintFactorR;
                    self.quintFactorR = self.quintAR * quintIntensity;
                }
                2 => {
                    self.quintFactorL = self.PrevDL - inputSampleL;
                    self.quintBL += self.quintFactorL;
                    self.quintAL -= self.quintFactorL;
                    self.quintFactorL = self.quintBL * quintIntensity;

                    self.quintFactorR = self.PrevDR - inputSampleR;
                    self.quintBR += self.quintFactorR;
                    self.quintAR -= self.quintFactorR;
                    self.quintFactorR = self.quintBR * quintIntensity;
                }
                3 => {
                    self.quintFactorL = self.PrevDL - inputSampleL;
                    self.quintCL += self.quintFactorL;
                    self.quintBL -= self.quintFactorL;
                    self.quintFactorL = self.quintCL * quintIntensity;

                    self.quintFactorR = self.PrevDR - inputSampleR;
                    self.quintCR += self.quintFactorR;
                    self.quintBR -= self.quintFactorR;
                    self.quintFactorR = self.quintCR * quintIntensity;
                }
                4 => {
                    self.quintFactorL = self.PrevDL - inputSampleL;
                    self.quintDL += self.quintFactorL;
                    self.quintCL -= self.quintFactorL;
                    self.quintFactorL = self.quintDL * quintIntensity;

                    self.quintFactorR = self.PrevDR - inputSampleR;
                    self.quintDR += self.quintFactorR;
                    self.quintCR -= self.quintFactorR;
                    self.quintFactorR = self.quintDR * quintIntensity;
                }
                5 => {
                    self.quintFactorL = self.PrevDL - inputSampleL;
                    self.quintEL += self.quintFactorL;
                    self.quintDL -= self.quintFactorL;
                    self.quintFactorL = self.quintEL * quintIntensity;

                    self.quintFactorR = self.PrevDR - inputSampleR;
                    self.quintER += self.quintFactorR;
                    self.quintDR -= self.quintFactorR;
                    self.quintFactorR = self.quintER * quintIntensity;
                }
                _ => {}
            }
            self.quintAL /= 2.0;
            self.quintBL /= 2.0;
            self.quintCL /= 2.0;
            self.quintDL /= 2.0;
            self.quintEL /= 2.0;
            correctionL += self.quintFactorL;

            self.quintAR /= 2.0;
            self.quintBR /= 2.0;
            self.quintCR /= 2.0;
            self.quintDR /= 2.0;
            self.quintER /= 2.0;
            correctionR += self.quintFactorR;
            self.countC += 1;
        }

        if sextIntensity.abs() > 0.0001 {
            if self.countD < 1 || self.countD > 6 {
                self.countD = 1;
            }
            match self.countD {
                1 => {
                    self.sextFactorL = self.PrevEL - inputSampleL;
                    self.sextAL += self.sextFactorL;
                    self.sextFL -= self.sextFactorL;
                    self.sextFactorL = self.sextAL * sextIntensity;

                    self.sextFactorR = self.PrevER - inputSampleR;
                    self.sextAR += self.sextFactorR;
                    self.sextFR -= self.sextFactorR;
                    self.sextFactorR = self.sextAR * sextIntensity;
                }
                2 => {
                    self.sextFactorL = self.PrevEL - inputSampleL;
                    self.sextBL += self.sextFactorL;
                    self.sextAL -= self.sextFactorL;
                    self.sextFactorL = self.sextBL * sextIntensity;

                    self.sextFactorR = self.PrevER - inputSampleR;
                    self.sextBR += self.sextFactorR;
                    self.sextAR -= self.sextFactorR;
                    self.sextFactorR = self.sextBR * sextIntensity;
                }
                3 => {
                    self.sextFactorL = self.PrevEL - inputSampleL;
                    self.sextCL += self.sextFactorL;
                    self.sextBL -= self.sextFactorL;
                    self.sextFactorL = self.sextCL * sextIntensity;

                    self.sextFactorR = self.PrevER - inputSampleR;
                    self.sextCR += self.sextFactorR;
                    self.sextBR -= self.sextFactorR;
                    self.sextFactorR = self.sextCR * sextIntensity;
                }
                4 => {
                    self.sextFactorL = self.PrevEL - inputSampleL;
                    self.sextDL += self.sextFactorL;
                    self.sextCL -= self.sextFactorL;
                    self.sextFactorL = self.sextDL * sextIntensity;

                    self.sextFactorR = self.PrevER - inputSampleR;
                    self.sextDR += self.sextFactorR;
                    self.sextCR -= self.sextFactorR;
                    self.sextFactorR = self.sextDR * sextIntensity;
                }
                5 => {
                    self.sextFactorL = self.PrevEL - inputSampleL;
                    self.sextEL += self.sextFactorL;
                    self.sextDL -= self.sextFactorL;
                    self.sextFactorL = self.sextEL * sextIntensity;

                    self.sextFactorR = self.PrevER - inputSampleR;
                    self.sextER += self.sextFactorR;
                    self.sextDR -= self.sextFactorR;
                    self.sextFactorR = self.sextER * sextIntensity;
                }
                6 => {
                    self.sextFactorL = self.PrevEL - inputSampleL;
                    self.sextFL += self.sextFactorL;
                    self.sextEL -= self.sextFactorL;
                    self.sextFactorL = self.sextFL * sextIntensity;

                    self.sextFactorR = self.PrevER - inputSampleR;
                    self.sextFR += self.sextFactorR;
                    self.sextER -= self.sextFactorR;
                    self.sextFactorR = self.sextFR * sextIntensity;
                }
                _ => {}
            }
            self.sextAL /= 2.0;
            self.sextBL /= 2.0;
            self.sextCL /= 2.0;
            self.sextDL /= 2.0;
            self.sextEL /= 2.0;
            self.sextFL /= 2.0;
            correctionL += self.sextFactorL;

            self.sextAR /= 2.0;
            self.sextBR /= 2.0;
            self.sextCR /= 2.0;
            self.sextDR /= 2.0;
            self.sextER /= 2.0;
            self.sextFR /= 2.0;
            correctionR += self.sextFactorR;
            self.countD += 1;
        }

        if septIntensity.abs() > 0.0001 {
            if self.countE < 1 || self.countE > 7 {
                self.countE = 1;
            }
            match self.countE {
                1 => {
                    self.septFactorL = self.PrevFL - inputSampleL;
                    self.septAL += self.septFactorL;
                    self.septGL -= self.septFactorL;
                    self.septFactorL = self.septAL * septIntensity;

                    self.septFactorR = self.PrevFR - inputSampleR;
                    self.septAR += self.septFactorR;
                    self.septGR -= self.septFactorR;
                    self.septFactorR = self.septAR * septIntensity;
                }
                2 => {
                    self.septFactorL = self.PrevFL - inputSampleL;
                    self.septBL += self.septFactorL;
                    self.septAL -= self.septFactorL;
                    self.septFactorL = self.septBL * septIntensity;

                    self.septFactorR = self.PrevFR - inputSampleR;
                    self.septBR += self.septFactorR;
                    self.septAR -= self.septFactorR;
                    self.septFactorR = self.septBR * septIntensity;
                }
                3 => {
                    self.septFactorL = self.PrevFL - inputSampleL;
                    self.septCL += self.septFactorL;
                    self.septBL -= self.septFactorL;
                    self.septFactorL = self.septCL * septIntensity;

                    self.septFactorR = self.PrevFR - inputSampleR;
                    self.septCR += self.septFactorR;
                    self.septBR -= self.septFactorR;
                    self.septFactorR = self.septCR * septIntensity;
                }
                4 => {
                    self.septFactorL = self.PrevFL - inputSampleL;
                    self.septDL += self.septFactorL;
                    self.septCL -= self.septFactorL;
                    self.septFactorL = self.septDL * septIntensity;

                    self.septFactorR = self.PrevFR - inputSampleR;
                    self.septDR += self.septFactorR;
                    self.septCR -= self.septFactorR;
                    self.septFactorR = self.septDR * septIntensity;
                }
                5 => {
                    self.septFactorL = self.PrevFL - inputSampleL;
                    self.septEL += self.septFactorL;
                    self.septDL -= self.septFactorL;
                    self.septFactorL = self.septEL * septIntensity;

                    self.septFactorR = self.PrevFR - inputSampleR;
                    self.septER += self.septFactorR;
                    self.septDR -= self.septFactorR;
                    self.septFactorR = self.septER * septIntensity;
                }
                6 => {
                    self.septFactorL = self.PrevFL - inputSampleL;
                    self.septFL += self.septFactorL;
                    self.septEL -= self.septFactorL;
                    self.septFactorL = self.septFL * septIntensity;

                    self.septFactorR = self.PrevFR - inputSampleR;
                    self.septFR += self.septFactorR;
                    self.septER -= self.septFactorR;
                    self.septFactorR = self.septFR * septIntensity;
                }
                7 => {
                    self.septFactorL = self.PrevFL - inputSampleL;
                    self.septGL += self.septFactorL;
                    self.septFL -= self.septFactorL;
                    self.septFactorL = self.septGL * septIntensity;

                    self.septFactorR = self.PrevFR - inputSampleR;
                    self.septGR += self.septFactorR;
                    self.septFR -= self.septFactorR;
                    self.septFactorR = self.septGR * septIntensity;
                }
                _ => {}
            }
            self.septAL /= 2.0;
            self.septBL /= 2.0;
            self.septCL /= 2.0;
            self.septDL /= 2.0;
            self.septEL /= 2.0;
            self.septFL /= 2.0;
            self.septGL /= 2.0;
            correctionL += self.septFactorL;

            self.septAR /= 2.0;
            self.septBR /= 2.0;
            self.septCR /= 2.0;
            self.septDR /= 2.0;
            self.septER /= 2.0;
            self.septFR /= 2.0;
            self.septGR /= 2.0;
            correctionR += self.septFactorR;
            self.countE += 1;
        }

        if octIntensity.abs() > 0.0001 {
            if self.countF < 1 || self.countF > 8 {
                self.countF = 1;
            }
            match self.countF {
                1 => {
                    self.octFactorL = self.PrevGL - inputSampleL;
                    self.octAL += self.octFactorL;
                    self.octHL -= self.octFactorL;
                    self.octFactorL = self.octAL * octIntensity;

                    self.octFactorR = self.PrevGR - inputSampleR;
                    self.octAR += self.octFactorR;
                    self.octHR -= self.octFactorR;
                    self.octFactorR = self.octAR * octIntensity;
                }
                2 => {
                    self.octFactorL = self.PrevGL - inputSampleL;
                    self.octBL += self.octFactorL;
                    self.octAL -= self.octFactorL;
                    self.octFactorL = self.octBL * octIntensity;

                    self.octFactorR = self.PrevGR - inputSampleR;
                    self.octBR += self.octFactorR;
                    self.octAR -= self.octFactorR;
                    self.octFactorR = self.octBR * octIntensity;
                }
                3 => {
                    self.octFactorL = self.PrevGL - inputSampleL;
                    self.octCL += self.octFactorL;
                    self.octBL -= self.octFactorL;
                    self.octFactorL = self.octCL * octIntensity;

                    self.octFactorR = self.PrevGR - inputSampleR;
                    self.octCR += self.octFactorR;
                    self.octBR -= self.octFactorR;
                    self.octFactorR = self.octCR * octIntensity;
                }
                4 => {
                    self.octFactorL = self.PrevGL - inputSampleL;
                    self.octDL += self.octFactorL;
                    self.octCL -= self.octFactorL;
                    self.octFactorL = self.octDL * octIntensity;

                    self.octFactorR = self.PrevGR - inputSampleR;
                    self.octDR += self.octFactorR;
                    self.octCR -= self.octFactorR;
                    self.octFactorR = self.octDR * octIntensity;
                }
                5 => {
                    self.octFactorL = self.PrevGL - inputSampleL;
                    self.octEL += self.octFactorL;
                    self.octDL -= self.octFactorL;
                    self.octFactorL = self.octEL * octIntensity;

                    self.octFactorR = self.PrevGR - inputSampleR;
                    self.octER += self.octFactorR;
                    self.octDR -= self.octFactorR;
                    self.octFactorR = self.octER * octIntensity;
                }
                6 => {
                    self.octFactorL = self.PrevGL - inputSampleL;
                    self.octFL += self.octFactorL;
                    self.octEL -= self.octFactorL;
                    self.octFactorL = self.octFL * octIntensity;

                    self.octFactorR = self.PrevGR - inputSampleR;
                    self.octFR += self.octFactorR;
                    self.octER -= self.octFactorR;
                    self.octFactorR = self.octFR * octIntensity;
                }
                7 => {
                    self.octFactorL = self.PrevGL - inputSampleL;
                    self.octGL += self.octFactorL;
                    self.octFL -= self.octFactorL;
                    self.octFactorL = self.octGL * octIntensity;

                    self.octFactorR = self.PrevGR - inputSampleR;
                    self.octGR += self.octFactorR;
                    self.octFR -= self.octFactorR;
                    self.octFactorR = self.octGR * octIntensity;
                }
                8 => {
                    self.octFactorL = self.PrevGL - inputSampleL;
                    self.octHL += self.octFactorL;
                    self.octGL -= self.octFactorL;
                    self.octFactorL = self.octHL * octIntensity;

                    self.octFactorR = self.PrevGR - inputSampleR;
                    self.octHR += self.octFactorR;
                    self.octGR -= self.octFactorR;
                    self.octFactorR = self.octHR * octIntensity;
                }
                _ => {}
            }
            self.octAL /= 2.0;
            self.octBL /= 2.0;
            self.octCL /= 2.0;
            self.octDL /= 2.0;
            self.octEL /= 2.0;
            self.octFL /= 2.0;
            self.octGL /= 2.0;
            self.octHL /= 2.0;
            correctionL += self.octFactorL;

            self.octAR /= 2.0;
            self.octBR /= 2.0;
            self.octCR /= 2.0;
            self.octDR /= 2.0;
            self.octER /= 2.0;
            self.octFR /= 2.0;
            self.octGR /= 2.0;
            self.octHR /= 2.0;
            correctionR += self.octFactorR;
            self.countF += 1;
        }

        if nintIntensity.abs() > 0.0001 {
            if self.countG < 1 || self.countG > 9 {
                self.countG = 1;
            }
            match self.countG {
                1 => {
                    self.nintFactorL = self.PrevHL - inputSampleL;
                    self.nintAL += self.nintFactorL;
                    self.nintIL -= self.nintFactorL;
                    self.nintFactorL = self.nintAL * nintIntensity;

                    self.nintFactorR = self.PrevHR - inputSampleR;
                    self.nintAR += self.nintFactorR;
                    self.nintIR -= self.nintFactorR;
                    self.nintFactorR = self.nintAR * nintIntensity;
                }
                2 => {
                    self.nintFactorL = self.PrevHL - inputSampleL;
                    self.nintBL += self.nintFactorL;
                    self.nintAL -= self.nintFactorL;
                    self.nintFactorL = self.nintBL * nintIntensity;

                    self.nintFactorR = self.PrevHR - inputSampleR;
                    self.nintBR += self.nintFactorR;
                    self.nintAR -= self.nintFactorR;
                    self.nintFactorR = self.nintBR * nintIntensity;
                }
                3 => {
                    self.nintFactorL = self.PrevHL - inputSampleL;
                    self.nintCL += self.nintFactorL;
                    self.nintBL -= self.nintFactorL;
                    self.nintFactorL = self.nintCL * nintIntensity;

                    self.nintFactorR = self.PrevHR - inputSampleR;
                    self.nintCR += self.nintFactorR;
                    self.nintBR -= self.nintFactorR;
                    self.nintFactorR = self.nintCR * nintIntensity;
                }
                4 => {
                    self.nintFactorL = self.PrevHL - inputSampleL;
                    self.nintDL += self.nintFactorL;
                    self.nintCL -= self.nintFactorL;
                    self.nintFactorL = self.nintDL * nintIntensity;

                    self.nintFactorR = self.PrevHR - inputSampleR;
                    self.nintDR += self.nintFactorR;
                    self.nintCR -= self.nintFactorR;
                    self.nintFactorR = self.nintDR * nintIntensity;
                }
                5 => {
                    self.nintFactorL = self.PrevHL - inputSampleL;
                    self.nintEL += self.nintFactorL;
                    self.nintDL -= self.nintFactorL;
                    self.nintFactorL = self.nintEL * nintIntensity;

                    self.nintFactorR = self.PrevHR - inputSampleR;
                    self.nintER += self.nintFactorR;
                    self.nintDR -= self.nintFactorR;
                    self.nintFactorR = self.nintER * nintIntensity;
                }
                6 => {
                    self.nintFactorL = self.PrevHL - inputSampleL;
                    self.nintFL += self.nintFactorL;
                    self.nintEL -= self.nintFactorL;
                    self.nintFactorL = self.nintFL * nintIntensity;

                    self.nintFactorR = self.PrevHR - inputSampleR;
                    self.nintFR += self.nintFactorR;
                    self.nintER -= self.nintFactorR;
                    self.nintFactorR = self.nintFR * nintIntensity;
                }
                7 => {
                    self.nintFactorL = self.PrevHL - inputSampleL;
                    self.nintGL += self.nintFactorL;
                    self.nintFL -= self.nintFactorL;
                    self.nintFactorL = self.nintGL * nintIntensity;

                    self.nintFactorR = self.PrevHR - inputSampleR;
                    self.nintGR += self.nintFactorR;
                    self.nintFR -= self.nintFactorR;
                    self.nintFactorR = self.nintGR * nintIntensity;
                }
                8 => {
                    self.nintFactorL = self.PrevHL - inputSampleL;
                    self.nintHL += self.nintFactorL;
                    self.nintGL -= self.nintFactorL;
                    self.nintFactorL = self.nintHL * nintIntensity;

                    self.nintFactorR = self.PrevHR - inputSampleR;
                    self.nintHR += self.nintFactorR;
                    self.nintGR -= self.nintFactorR;
                    self.nintFactorR = self.nintHR * nintIntensity;
                }
                9 => {
                    self.nintFactorL = self.PrevHL - inputSampleL;
                    self.nintIL += self.nintFactorL;
                    self.nintHL -= self.nintFactorL;
                    self.nintFactorL = self.nintIL * nintIntensity;

                    self.nintFactorR = self.PrevHR - inputSampleR;
                    self.nintIR += self.nintFactorR;
                    self.nintHR -= self.nintFactorR;
                    self.nintFactorR = self.nintIR * nintIntensity;
                }
                _ => {}
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
            correctionL += self.nintFactorL;

            self.nintAR /= 2.0;
            self.nintBR /= 2.0;
            self.nintCR /= 2.0;
            self.nintDR /= 2.0;
            self.nintER /= 2.0;
            self.nintFR /= 2.0;
            self.nintGR /= 2.0;
            self.nintHR /= 2.0;
            self.nintIR /= 2.0;
            correctionR += self.nintFactorR;
            self.countG += 1;
        }

        (correctionL, correctionR)
    }
}
