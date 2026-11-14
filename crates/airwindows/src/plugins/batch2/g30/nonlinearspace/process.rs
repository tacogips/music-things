//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license.
//! Float processing path from NonlinearSpaceProc.cpp.
#![allow(non_snake_case)]
use super::NonlinearSpace;
use crate::dither::fpd_dither;

impl NonlinearSpace {
    #[rustfmt::skip]
    pub(super) fn process_block(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let s = &mut self.state;
        let [a, b, c, d, e, f] = self.params.map(|x| x as f64);
        let scale = match (a * 6.999) as usize { 0 => 16.0/44.1, 1 => 32.0/44.1, 2 => 1.0, 3 => 48.0/44.1, 4 => 64.0/44.1, 5 => 2.0, _ => 96.0/44.1 };
        s.nonlin *= 0.001;
        let basefeedback = 0.45 + s.nonlin * ((e * 2.0) - 1.0).powf(3.0);
        s.nonlin = 0.0;
        let tankfeedback = (basefeedback + b.powf(2.0) * 0.05).clamp(0.4, 0.5);
        let mut iirAmountC = 1.0 - (1.0-c).powf(2.0);
        iirAmountC += iirAmountC / scale;
        iirAmountC /= 2.0;
        if iirAmountC > 1.1 { iirAmountC = 1.1; }
        let iirAmount = (((1.0-d.powf(2.0))*0.09)/scale+0.001).clamp(0.001,1.0);
        let wetness = f;
        let dryness = 1.0-wetness;
        let roomsize = scale*0.203;
        let lean = 0.125;
        let invlean = 1.0-lean;
        let pspeed = 0.145;
        let outcouple = 0.5-tankfeedback;
        let constallpass = 0.618_033_988_749_894_9_f64;
        let predelay = 0.222*scale;
        s.delayA = (s.maxdelayA as f64 * roomsize) as i32;
        s.delayB = (s.maxdelayB as f64 * roomsize) as i32;
        s.delayC = (s.maxdelayC as f64 * roomsize) as i32;
        s.delayD = (s.maxdelayD as f64 * roomsize) as i32;
        s.delayE = (s.maxdelayE as f64 * roomsize) as i32;
        s.delayF = (s.maxdelayF as f64 * roomsize) as i32;
        s.delayG = (s.maxdelayG as f64 * roomsize) as i32;
        s.delayH = (s.maxdelayH as f64 * roomsize) as i32;
        s.delayI = (s.maxdelayI as f64 * roomsize) as i32;
        s.delayJ = (s.maxdelayJ as f64 * roomsize) as i32;
        s.delayK = (s.maxdelayK as f64 * roomsize) as i32;
        s.delayL = (s.maxdelayL as f64 * roomsize) as i32;
        s.delayM = (s.maxdelayM as f64 * roomsize) as i32;
        s.delayN = (s.maxdelayN as f64 * roomsize) as i32;
        s.delayO = (s.maxdelayO as f64 * roomsize) as i32;
        s.delayP = (s.maxdelayP as f64 * roomsize) as i32;
        s.delayQ = (s.maxdelayQ as f64 * roomsize) as i32;
        s.delayR = (s.maxdelayR as f64 * roomsize) as i32;
        s.delayS = (s.maxdelayS as f64 * roomsize) as i32;
        s.delayT = (s.maxdelayT as f64 * roomsize) as i32;
        s.delayU = (s.maxdelayU as f64 * roomsize) as i32;
        s.delayV = (s.maxdelayV as f64 * roomsize) as i32;
        s.delayW = (s.maxdelayW as f64 * roomsize) as i32;
        s.delayX = (s.maxdelayX as f64 * roomsize) as i32;
        s.delayY = (s.maxdelayY as f64 * roomsize) as i32;
        s.delayZ = (s.maxdelayZ as f64 * roomsize) as i32;
        s.delayMid = (s.maxdelayMid as f64 * roomsize) as i32;
        s.delaySide = (s.maxdelaySide as f64 * roomsize) as i32;
        s.delayLeft = (s.maxdelayLeft as f64 * roomsize) as i32;
        s.delayRight = (s.maxdelayRight as f64 * roomsize) as i32;
        s.delaypre = (s.maxdelaypre as f64 * predelay) as i32;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut inputSampleL = l as f64;
            let mut inputSampleR = r as f64;
            if inputSampleL.abs()<1.18e-23 { inputSampleL = s.fpdL as f64*1.18e-17; }
            if inputSampleR.abs()<1.18e-23 { inputSampleR = s.fpdR as f64*1.18e-17; }
            let mut drySampleL = inputSampleL;
            let mut drySampleR = inputSampleR;
            s.dpreL[s.onepre as usize] = inputSampleL;
            s.dpreR[s.onepre as usize] = inputSampleR;
            s.onepre -= 1; if s.onepre < 0 || s.onepre > s.delaypre { s.onepre = s.delaypre; }
            inputSampleL = s.dpreL[s.onepre as usize];
            inputSampleR = s.dpreR[s.onepre as usize];
            s.interpolA += s.pitchshiftA*pspeed;
            s.interpolB += s.pitchshiftB*pspeed;
            s.interpolC += s.pitchshiftC*pspeed;
            s.interpolD += s.pitchshiftD*pspeed;
            s.interpolE += s.pitchshiftE*pspeed;
            s.interpolF += s.pitchshiftF*pspeed;
            s.interpolG += s.pitchshiftG*pspeed;
            s.interpolH += s.pitchshiftH*pspeed;
            s.interpolI += s.pitchshiftI*pspeed;
            s.interpolJ += s.pitchshiftJ*pspeed;
            s.interpolK += s.pitchshiftK*pspeed;
            s.interpolL += s.pitchshiftL*pspeed;
            s.interpolM += s.pitchshiftM*pspeed;
            s.interpolN += s.pitchshiftN*pspeed;
            s.interpolO += s.pitchshiftO*pspeed;
            s.interpolP += s.pitchshiftP*pspeed;
            s.interpolQ += s.pitchshiftQ*pspeed;
            s.interpolR += s.pitchshiftR*pspeed;
            s.interpolS += s.pitchshiftS*pspeed;
            s.interpolT += s.pitchshiftT*pspeed;
            s.interpolU += s.pitchshiftU*pspeed;
            s.interpolV += s.pitchshiftV*pspeed;
            s.interpolW += s.pitchshiftW*pspeed;
            s.interpolX += s.pitchshiftX*pspeed;
            s.interpolY += s.pitchshiftY*pspeed;
            s.interpolZ += s.pitchshiftZ*pspeed;
            if s.interpolA > 1.0 { s.pitchshiftA = -s.pitchshiftA.abs(); s.interpolA += s.pitchshiftA*pspeed; }
            if s.interpolB > 1.0 { s.pitchshiftB = -s.pitchshiftB.abs(); s.interpolB += s.pitchshiftB*pspeed; }
            if s.interpolC > 1.0 { s.pitchshiftC = -s.pitchshiftC.abs(); s.interpolC += s.pitchshiftC*pspeed; }
            if s.interpolD > 1.0 { s.pitchshiftD = -s.pitchshiftD.abs(); s.interpolD += s.pitchshiftD*pspeed; }
            if s.interpolE > 1.0 { s.pitchshiftE = -s.pitchshiftE.abs(); s.interpolE += s.pitchshiftE*pspeed; }
            if s.interpolF > 1.0 { s.pitchshiftF = -s.pitchshiftF.abs(); s.interpolF += s.pitchshiftF*pspeed; }
            if s.interpolG > 1.0 { s.pitchshiftG = -s.pitchshiftG.abs(); s.interpolG += s.pitchshiftG*pspeed; }
            if s.interpolH > 1.0 { s.pitchshiftH = -s.pitchshiftH.abs(); s.interpolH += s.pitchshiftH*pspeed; }
            if s.interpolI > 1.0 { s.pitchshiftI = -s.pitchshiftI.abs(); s.interpolI += s.pitchshiftI*pspeed; }
            if s.interpolJ > 1.0 { s.pitchshiftJ = -s.pitchshiftJ.abs(); s.interpolJ += s.pitchshiftJ*pspeed; }
            if s.interpolK > 1.0 { s.pitchshiftK = -s.pitchshiftK.abs(); s.interpolK += s.pitchshiftK*pspeed; }
            if s.interpolL > 1.0 { s.pitchshiftL = -s.pitchshiftL.abs(); s.interpolL += s.pitchshiftL*pspeed; }
            if s.interpolM > 1.0 { s.pitchshiftM = -s.pitchshiftM.abs(); s.interpolM += s.pitchshiftM*pspeed; }
            if s.interpolN > 1.0 { s.pitchshiftN = -s.pitchshiftN.abs(); s.interpolN += s.pitchshiftN*pspeed; }
            if s.interpolO > 1.0 { s.pitchshiftO = -s.pitchshiftO.abs(); s.interpolO += s.pitchshiftO*pspeed; }
            if s.interpolP > 1.0 { s.pitchshiftP = -s.pitchshiftP.abs(); s.interpolP += s.pitchshiftP*pspeed; }
            if s.interpolQ > 1.0 { s.pitchshiftQ = -s.pitchshiftQ.abs(); s.interpolQ += s.pitchshiftQ*pspeed; }
            if s.interpolR > 1.0 { s.pitchshiftR = -s.pitchshiftR.abs(); s.interpolR += s.pitchshiftR*pspeed; }
            if s.interpolS > 1.0 { s.pitchshiftS = -s.pitchshiftS.abs(); s.interpolS += s.pitchshiftS*pspeed; }
            if s.interpolT > 1.0 { s.pitchshiftT = -s.pitchshiftT.abs(); s.interpolT += s.pitchshiftT*pspeed; }
            if s.interpolU > 1.0 { s.pitchshiftU = -s.pitchshiftU.abs(); s.interpolU += s.pitchshiftU*pspeed; }
            if s.interpolV > 1.0 { s.pitchshiftV = -s.pitchshiftV.abs(); s.interpolV += s.pitchshiftV*pspeed; }
            if s.interpolW > 1.0 { s.pitchshiftW = -s.pitchshiftW.abs(); s.interpolW += s.pitchshiftW*pspeed; }
            if s.interpolX > 1.0 { s.pitchshiftX = -s.pitchshiftX.abs(); s.interpolX += s.pitchshiftX*pspeed; }
            if s.interpolY > 1.0 { s.pitchshiftY = -s.pitchshiftY.abs(); s.interpolY += s.pitchshiftY*pspeed; }
            if s.interpolZ > 1.0 { s.pitchshiftZ = -s.pitchshiftZ.abs(); s.interpolZ += s.pitchshiftZ*pspeed; }
            if s.interpolA < 0.0 { s.pitchshiftA = s.pitchshiftA.abs(); s.interpolA += s.pitchshiftA*pspeed; }
            if s.interpolB < 0.0 { s.pitchshiftB = s.pitchshiftB.abs(); s.interpolB += s.pitchshiftB*pspeed; }
            if s.interpolC < 0.0 { s.pitchshiftC = s.pitchshiftC.abs(); s.interpolC += s.pitchshiftC*pspeed; }
            if s.interpolD < 0.0 { s.pitchshiftD = s.pitchshiftD.abs(); s.interpolD += s.pitchshiftD*pspeed; }
            if s.interpolE < 0.0 { s.pitchshiftE = s.pitchshiftE.abs(); s.interpolE += s.pitchshiftE*pspeed; }
            if s.interpolF < 0.0 { s.pitchshiftF = s.pitchshiftF.abs(); s.interpolF += s.pitchshiftF*pspeed; }
            if s.interpolG < 0.0 { s.pitchshiftG = s.pitchshiftG.abs(); s.interpolG += s.pitchshiftG*pspeed; }
            if s.interpolH < 0.0 { s.pitchshiftH = s.pitchshiftH.abs(); s.interpolH += s.pitchshiftH*pspeed; }
            if s.interpolI < 0.0 { s.pitchshiftI = s.pitchshiftI.abs(); s.interpolI += s.pitchshiftI*pspeed; }
            if s.interpolJ < 0.0 { s.pitchshiftJ = s.pitchshiftJ.abs(); s.interpolJ += s.pitchshiftJ*pspeed; }
            if s.interpolK < 0.0 { s.pitchshiftK = s.pitchshiftK.abs(); s.interpolK += s.pitchshiftK*pspeed; }
            if s.interpolL < 0.0 { s.pitchshiftL = s.pitchshiftL.abs(); s.interpolL += s.pitchshiftL*pspeed; }
            if s.interpolM < 0.0 { s.pitchshiftM = s.pitchshiftM.abs(); s.interpolM += s.pitchshiftM*pspeed; }
            if s.interpolN < 0.0 { s.pitchshiftN = s.pitchshiftN.abs(); s.interpolN += s.pitchshiftN*pspeed; }
            if s.interpolO < 0.0 { s.pitchshiftO = s.pitchshiftO.abs(); s.interpolO += s.pitchshiftO*pspeed; }
            if s.interpolP < 0.0 { s.pitchshiftP = s.pitchshiftP.abs(); s.interpolP += s.pitchshiftP*pspeed; }
            if s.interpolQ < 0.0 { s.pitchshiftQ = s.pitchshiftQ.abs(); s.interpolQ += s.pitchshiftQ*pspeed; }
            if s.interpolR < 0.0 { s.pitchshiftR = s.pitchshiftR.abs(); s.interpolR += s.pitchshiftR*pspeed; }
            if s.interpolS < 0.0 { s.pitchshiftS = s.pitchshiftS.abs(); s.interpolS += s.pitchshiftS*pspeed; }
            if s.interpolT < 0.0 { s.pitchshiftT = s.pitchshiftT.abs(); s.interpolT += s.pitchshiftT*pspeed; }
            if s.interpolU < 0.0 { s.pitchshiftU = s.pitchshiftU.abs(); s.interpolU += s.pitchshiftU*pspeed; }
            if s.interpolV < 0.0 { s.pitchshiftV = s.pitchshiftV.abs(); s.interpolV += s.pitchshiftV*pspeed; }
            if s.interpolW < 0.0 { s.pitchshiftW = s.pitchshiftW.abs(); s.interpolW += s.pitchshiftW*pspeed; }
            if s.interpolX < 0.0 { s.pitchshiftX = s.pitchshiftX.abs(); s.interpolX += s.pitchshiftX*pspeed; }
            if s.interpolY < 0.0 { s.pitchshiftY = s.pitchshiftY.abs(); s.interpolY += s.pitchshiftY*pspeed; }
            if s.interpolZ < 0.0 { s.pitchshiftZ = s.pitchshiftZ.abs(); s.interpolZ += s.pitchshiftZ*pspeed; }
            s.verboutR = s.verboutR.clamp(-1.0,1.0);
            s.verboutL = s.verboutL.clamp(-1.0,1.0);
            inputSampleL += s.verboutR;
            inputSampleR += s.verboutL;
            s.verboutL = 0.0; s.verboutR = 0.0;
            let mut mid = inputSampleL + inputSampleR;
            let mut side = inputSampleL - inputSampleR;
            let mut allpasstemp = s.oneMid-1;
            if allpasstemp < 0 || allpasstemp > s.delayMid { allpasstemp = s.delayMid; }
            mid -= s.dMid[allpasstemp as usize]*constallpass;
            s.dMid[s.oneMid as usize] = mid;
            mid *= constallpass;
            s.oneMid -= 1; if s.oneMid < 0 || s.oneMid > s.delayMid { s.oneMid = s.delayMid; }
            mid += s.dMid[s.oneMid as usize];
            s.nonlin += s.dMid[s.oneMid as usize].abs();
            let mut allpasstemp = s.oneSide-1;
            if allpasstemp < 0 || allpasstemp > s.delaySide { allpasstemp = s.delaySide; }
            side -= s.dSide[allpasstemp as usize]*constallpass;
            s.dSide[s.oneSide as usize] = side;
            side *= constallpass;
            s.oneSide -= 1; if s.oneSide < 0 || s.oneSide > s.delaySide { s.oneSide = s.delaySide; }
            side += s.dSide[s.oneSide as usize];
            s.nonlin += s.dSide[s.oneSide as usize].abs();
            let mut allpasstemp = s.oneLeft-1;
            if allpasstemp < 0 || allpasstemp > s.delayLeft { allpasstemp = s.delayLeft; }
            inputSampleL -= s.dLeft[allpasstemp as usize]*constallpass;
            s.dLeft[s.oneLeft as usize] = s.verboutL;
            inputSampleL *= constallpass;
            s.oneLeft -= 1; if s.oneLeft < 0 || s.oneLeft > s.delayLeft { s.oneLeft = s.delayLeft; }
            inputSampleL += s.dLeft[s.oneLeft as usize];
            s.nonlin += s.dLeft[s.oneLeft as usize].abs();
            let mut allpasstemp = s.oneRight-1;
            if allpasstemp < 0 || allpasstemp > s.delayRight { allpasstemp = s.delayRight; }
            inputSampleR -= s.dRight[allpasstemp as usize]*constallpass;
            s.dRight[s.oneRight as usize] = s.verboutR;
            inputSampleR *= constallpass;
            s.oneRight -= 1; if s.oneRight < 0 || s.oneRight > s.delayRight { s.oneRight = s.delayRight; }
            inputSampleR += s.dRight[s.oneRight as usize];
            s.nonlin += s.dRight[s.oneRight as usize].abs();
            inputSampleL += (mid+side)/2.0;
            inputSampleR += (mid-side)/2.0;
            let mut temp = s.dA[s.oneA as usize]*s.interpolA;
            temp += s.dA[s.treA as usize]*(1.0-s.interpolA);
            temp += s.dA[s.twoA as usize];
            s.dA[s.treA as usize] = temp*tankfeedback;
            s.dA[s.treA as usize] += inputSampleL;
            s.oneA -= 1; if s.oneA < 0 || s.oneA > s.delayA { s.oneA = s.delayA; }
            s.twoA -= 1; if s.twoA < 0 || s.twoA > s.delayA { s.twoA = s.delayA; }
            s.treA -= 1; if s.treA < 0 || s.treA > s.delayA { s.treA = s.delayA; }
            temp = s.dA[s.oneA as usize]*s.interpolA;
            temp += s.dA[s.treA as usize]*(1.0-s.interpolA);
            temp *= invlean + lean*s.dA[s.twoA as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dC[s.oneC as usize]*s.interpolC;
            temp += s.dC[s.treC as usize]*(1.0-s.interpolC);
            temp += s.dC[s.twoC as usize];
            s.dC[s.treC as usize] = temp*tankfeedback;
            s.dC[s.treC as usize] += inputSampleL;
            s.oneC -= 1; if s.oneC < 0 || s.oneC > s.delayC { s.oneC = s.delayC; }
            s.twoC -= 1; if s.twoC < 0 || s.twoC > s.delayC { s.twoC = s.delayC; }
            s.treC -= 1; if s.treC < 0 || s.treC > s.delayC { s.treC = s.delayC; }
            temp = s.dC[s.oneC as usize]*s.interpolC;
            temp += s.dC[s.treC as usize]*(1.0-s.interpolC);
            temp *= invlean + lean*s.dC[s.twoC as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dE[s.oneE as usize]*s.interpolE;
            temp += s.dE[s.treE as usize]*(1.0-s.interpolE);
            temp += s.dE[s.twoE as usize];
            s.dE[s.treE as usize] = temp*tankfeedback;
            s.dE[s.treE as usize] += inputSampleL;
            s.oneE -= 1; if s.oneE < 0 || s.oneE > s.delayE { s.oneE = s.delayE; }
            s.twoE -= 1; if s.twoE < 0 || s.twoE > s.delayE { s.twoE = s.delayE; }
            s.treE -= 1; if s.treE < 0 || s.treE > s.delayE { s.treE = s.delayE; }
            temp = s.dE[s.oneE as usize]*s.interpolE;
            temp += s.dE[s.treE as usize]*(1.0-s.interpolE);
            temp *= invlean + lean*s.dE[s.twoE as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dG[s.oneG as usize]*s.interpolG;
            temp += s.dG[s.treG as usize]*(1.0-s.interpolG);
            temp += s.dG[s.twoG as usize];
            s.dG[s.treG as usize] = temp*tankfeedback;
            s.dG[s.treG as usize] += inputSampleL;
            s.oneG -= 1; if s.oneG < 0 || s.oneG > s.delayG { s.oneG = s.delayG; }
            s.twoG -= 1; if s.twoG < 0 || s.twoG > s.delayG { s.twoG = s.delayG; }
            s.treG -= 1; if s.treG < 0 || s.treG > s.delayG { s.treG = s.delayG; }
            temp = s.dG[s.oneG as usize]*s.interpolG;
            temp += s.dG[s.treG as usize]*(1.0-s.interpolG);
            temp *= invlean + lean*s.dG[s.twoG as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dI[s.oneI as usize]*s.interpolI;
            temp += s.dI[s.treI as usize]*(1.0-s.interpolI);
            temp += s.dI[s.twoI as usize];
            s.dI[s.treI as usize] = temp*tankfeedback;
            s.dI[s.treI as usize] += inputSampleL;
            s.oneI -= 1; if s.oneI < 0 || s.oneI > s.delayI { s.oneI = s.delayI; }
            s.twoI -= 1; if s.twoI < 0 || s.twoI > s.delayI { s.twoI = s.delayI; }
            s.treI -= 1; if s.treI < 0 || s.treI > s.delayI { s.treI = s.delayI; }
            temp = s.dI[s.oneI as usize]*s.interpolI;
            temp += s.dI[s.treI as usize]*(1.0-s.interpolI);
            temp *= invlean + lean*s.dI[s.twoI as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dK[s.oneK as usize]*s.interpolK;
            temp += s.dK[s.treK as usize]*(1.0-s.interpolK);
            temp += s.dK[s.twoK as usize];
            s.dK[s.treK as usize] = temp*tankfeedback;
            s.dK[s.treK as usize] += inputSampleL;
            s.oneK -= 1; if s.oneK < 0 || s.oneK > s.delayK { s.oneK = s.delayK; }
            s.twoK -= 1; if s.twoK < 0 || s.twoK > s.delayK { s.twoK = s.delayK; }
            s.treK -= 1; if s.treK < 0 || s.treK > s.delayK { s.treK = s.delayK; }
            temp = s.dK[s.oneK as usize]*s.interpolK;
            temp += s.dK[s.treK as usize]*(1.0-s.interpolK);
            temp *= invlean + lean*s.dK[s.twoK as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dM[s.oneM as usize]*s.interpolM;
            temp += s.dM[s.treM as usize]*(1.0-s.interpolM);
            temp += s.dM[s.twoM as usize];
            s.dM[s.treM as usize] = temp*tankfeedback;
            s.dM[s.treM as usize] += inputSampleL;
            s.oneM -= 1; if s.oneM < 0 || s.oneM > s.delayM { s.oneM = s.delayM; }
            s.twoM -= 1; if s.twoM < 0 || s.twoM > s.delayM { s.twoM = s.delayM; }
            s.treM -= 1; if s.treM < 0 || s.treM > s.delayM { s.treM = s.delayM; }
            temp = s.dM[s.oneM as usize]*s.interpolM;
            temp += s.dM[s.treM as usize]*(1.0-s.interpolM);
            temp *= invlean + lean*s.dM[s.twoM as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dO[s.oneO as usize]*s.interpolO;
            temp += s.dO[s.treO as usize]*(1.0-s.interpolO);
            temp += s.dO[s.twoO as usize];
            s.dO[s.treO as usize] = temp*tankfeedback;
            s.dO[s.treO as usize] += inputSampleL;
            s.oneO -= 1; if s.oneO < 0 || s.oneO > s.delayO { s.oneO = s.delayO; }
            s.twoO -= 1; if s.twoO < 0 || s.twoO > s.delayO { s.twoO = s.delayO; }
            s.treO -= 1; if s.treO < 0 || s.treO > s.delayO { s.treO = s.delayO; }
            temp = s.dO[s.oneO as usize]*s.interpolO;
            temp += s.dO[s.treO as usize]*(1.0-s.interpolO);
            temp *= invlean + lean*s.dO[s.twoO as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dQ[s.oneQ as usize]*s.interpolQ;
            temp += s.dQ[s.treQ as usize]*(1.0-s.interpolQ);
            temp += s.dQ[s.twoQ as usize];
            s.dQ[s.treQ as usize] = temp*tankfeedback;
            s.dQ[s.treQ as usize] += inputSampleL;
            s.oneQ -= 1; if s.oneQ < 0 || s.oneQ > s.delayQ { s.oneQ = s.delayQ; }
            s.twoQ -= 1; if s.twoQ < 0 || s.twoQ > s.delayQ { s.twoQ = s.delayQ; }
            s.treQ -= 1; if s.treQ < 0 || s.treQ > s.delayQ { s.treQ = s.delayQ; }
            temp = s.dQ[s.oneQ as usize]*s.interpolQ;
            temp += s.dQ[s.treQ as usize]*(1.0-s.interpolQ);
            temp *= invlean + lean*s.dQ[s.twoQ as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dS[s.oneS as usize]*s.interpolS;
            temp += s.dS[s.treS as usize]*(1.0-s.interpolS);
            temp += s.dS[s.twoS as usize];
            s.dS[s.treS as usize] = temp*tankfeedback;
            s.dS[s.treS as usize] += inputSampleL;
            s.oneS -= 1; if s.oneS < 0 || s.oneS > s.delayS { s.oneS = s.delayS; }
            s.twoS -= 1; if s.twoS < 0 || s.twoS > s.delayS { s.twoS = s.delayS; }
            s.treS -= 1; if s.treS < 0 || s.treS > s.delayS { s.treS = s.delayS; }
            temp = s.dS[s.oneS as usize]*s.interpolS;
            temp += s.dS[s.treS as usize]*(1.0-s.interpolS);
            temp *= invlean + lean*s.dS[s.twoS as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dU[s.oneU as usize]*s.interpolU;
            temp += s.dU[s.treU as usize]*(1.0-s.interpolU);
            temp += s.dU[s.twoU as usize];
            s.dU[s.treU as usize] = temp*tankfeedback;
            s.dU[s.treU as usize] += inputSampleL;
            s.oneU -= 1; if s.oneU < 0 || s.oneU > s.delayU { s.oneU = s.delayU; }
            s.twoU -= 1; if s.twoU < 0 || s.twoU > s.delayU { s.twoU = s.delayU; }
            s.treU -= 1; if s.treU < 0 || s.treU > s.delayU { s.treU = s.delayU; }
            temp = s.dU[s.oneU as usize]*s.interpolU;
            temp += s.dU[s.treU as usize]*(1.0-s.interpolU);
            temp *= invlean + lean*s.dU[s.twoU as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dW[s.oneW as usize]*s.interpolW;
            temp += s.dW[s.treW as usize]*(1.0-s.interpolW);
            temp += s.dW[s.twoW as usize];
            s.dW[s.treW as usize] = temp*tankfeedback;
            s.dW[s.treW as usize] += inputSampleL;
            s.oneW -= 1; if s.oneW < 0 || s.oneW > s.delayW { s.oneW = s.delayW; }
            s.twoW -= 1; if s.twoW < 0 || s.twoW > s.delayW { s.twoW = s.delayW; }
            s.treW -= 1; if s.treW < 0 || s.treW > s.delayW { s.treW = s.delayW; }
            temp = s.dW[s.oneW as usize]*s.interpolW;
            temp += s.dW[s.treW as usize]*(1.0-s.interpolW);
            temp *= invlean + lean*s.dW[s.twoW as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dY[s.oneY as usize]*s.interpolY;
            temp += s.dY[s.treY as usize]*(1.0-s.interpolY);
            temp += s.dY[s.twoY as usize];
            s.dY[s.treY as usize] = temp*tankfeedback;
            s.dY[s.treY as usize] += inputSampleL;
            s.oneY -= 1; if s.oneY < 0 || s.oneY > s.delayY { s.oneY = s.delayY; }
            s.twoY -= 1; if s.twoY < 0 || s.twoY > s.delayY { s.twoY = s.delayY; }
            s.treY -= 1; if s.treY < 0 || s.treY > s.delayY { s.treY = s.delayY; }
            temp = s.dY[s.oneY as usize]*s.interpolY;
            temp += s.dY[s.treY as usize]*(1.0-s.interpolY);
            temp *= invlean + lean*s.dY[s.twoY as usize].abs();
            s.verboutL += temp;
            let mut temp = s.dB[s.oneB as usize]*s.interpolB;
            temp += s.dB[s.treB as usize]*(1.0-s.interpolB);
            temp += s.dB[s.twoB as usize];
            s.dB[s.treB as usize] = temp*tankfeedback;
            s.dB[s.treB as usize] += inputSampleR;
            s.oneB -= 1; if s.oneB < 0 || s.oneB > s.delayB { s.oneB = s.delayB; }
            s.twoB -= 1; if s.twoB < 0 || s.twoB > s.delayB { s.twoB = s.delayB; }
            s.treB -= 1; if s.treB < 0 || s.treB > s.delayB { s.treB = s.delayB; }
            temp = s.dB[s.oneB as usize]*s.interpolB;
            temp += s.dB[s.treB as usize]*(1.0-s.interpolB);
            temp *= invlean + lean*s.dB[s.twoB as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dD[s.oneD as usize]*s.interpolD;
            temp += s.dD[s.treD as usize]*(1.0-s.interpolD);
            temp += s.dD[s.twoD as usize];
            s.dD[s.treD as usize] = temp*tankfeedback;
            s.dD[s.treD as usize] += inputSampleR;
            s.oneD -= 1; if s.oneD < 0 || s.oneD > s.delayD { s.oneD = s.delayD; }
            s.twoD -= 1; if s.twoD < 0 || s.twoD > s.delayD { s.twoD = s.delayD; }
            s.treD -= 1; if s.treD < 0 || s.treD > s.delayD { s.treD = s.delayD; }
            temp = s.dD[s.oneD as usize]*s.interpolD;
            temp += s.dD[s.treD as usize]*(1.0-s.interpolD);
            temp *= invlean + lean*s.dD[s.twoD as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dF[s.oneF as usize]*s.interpolF;
            temp += s.dF[s.treF as usize]*(1.0-s.interpolF);
            temp += s.dF[s.twoF as usize];
            s.dF[s.treF as usize] = temp*tankfeedback;
            s.dF[s.treF as usize] += inputSampleR;
            s.oneF -= 1; if s.oneF < 0 || s.oneF > s.delayF { s.oneF = s.delayF; }
            s.twoF -= 1; if s.twoF < 0 || s.twoF > s.delayF { s.twoF = s.delayF; }
            s.treF -= 1; if s.treF < 0 || s.treF > s.delayF { s.treF = s.delayF; }
            temp = s.dF[s.oneF as usize]*s.interpolF;
            temp += s.dF[s.treF as usize]*(1.0-s.interpolF);
            temp *= invlean + lean*s.dF[s.twoF as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dH[s.oneH as usize]*s.interpolH;
            temp += s.dH[s.treH as usize]*(1.0-s.interpolH);
            temp += s.dH[s.twoH as usize];
            s.dH[s.treH as usize] = temp*tankfeedback;
            s.dH[s.treH as usize] += inputSampleR;
            s.oneH -= 1; if s.oneH < 0 || s.oneH > s.delayH { s.oneH = s.delayH; }
            s.twoH -= 1; if s.twoH < 0 || s.twoH > s.delayH { s.twoH = s.delayH; }
            s.treH -= 1; if s.treH < 0 || s.treH > s.delayH { s.treH = s.delayH; }
            temp = s.dH[s.oneH as usize]*s.interpolH;
            temp += s.dH[s.treH as usize]*(1.0-s.interpolH);
            temp *= invlean + lean*s.dH[s.twoH as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dJ[s.oneJ as usize]*s.interpolJ;
            temp += s.dJ[s.treJ as usize]*(1.0-s.interpolJ);
            temp += s.dJ[s.twoJ as usize];
            s.dJ[s.treJ as usize] = temp*tankfeedback;
            s.dJ[s.treJ as usize] += inputSampleR;
            s.oneJ -= 1; if s.oneJ < 0 || s.oneJ > s.delayJ { s.oneJ = s.delayJ; }
            s.twoJ -= 1; if s.twoJ < 0 || s.twoJ > s.delayJ { s.twoJ = s.delayJ; }
            s.treJ -= 1; if s.treJ < 0 || s.treJ > s.delayJ { s.treJ = s.delayJ; }
            temp = s.dJ[s.oneJ as usize]*s.interpolJ;
            temp += s.dJ[s.treJ as usize]*(1.0-s.interpolJ);
            temp *= invlean + lean*s.dJ[s.twoJ as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dL[s.oneL as usize]*s.interpolL;
            temp += s.dL[s.treL as usize]*(1.0-s.interpolL);
            temp += s.dL[s.twoL as usize];
            s.dL[s.treL as usize] = temp*tankfeedback;
            s.dL[s.treL as usize] += inputSampleR;
            s.oneL -= 1; if s.oneL < 0 || s.oneL > s.delayL { s.oneL = s.delayL; }
            s.twoL -= 1; if s.twoL < 0 || s.twoL > s.delayL { s.twoL = s.delayL; }
            s.treL -= 1; if s.treL < 0 || s.treL > s.delayL { s.treL = s.delayL; }
            temp = s.dL[s.oneL as usize]*s.interpolL;
            temp += s.dL[s.treL as usize]*(1.0-s.interpolL);
            temp *= invlean + lean*s.dL[s.twoL as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dN[s.oneN as usize]*s.interpolN;
            temp += s.dN[s.treN as usize]*(1.0-s.interpolN);
            temp += s.dN[s.twoN as usize];
            s.dN[s.treN as usize] = temp*tankfeedback;
            s.dN[s.treN as usize] += inputSampleR;
            s.oneN -= 1; if s.oneN < 0 || s.oneN > s.delayN { s.oneN = s.delayN; }
            s.twoN -= 1; if s.twoN < 0 || s.twoN > s.delayN { s.twoN = s.delayN; }
            s.treN -= 1; if s.treN < 0 || s.treN > s.delayN { s.treN = s.delayN; }
            temp = s.dN[s.oneN as usize]*s.interpolN;
            temp += s.dN[s.treN as usize]*(1.0-s.interpolN);
            temp *= invlean + lean*s.dN[s.twoN as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dP[s.oneP as usize]*s.interpolP;
            temp += s.dP[s.treP as usize]*(1.0-s.interpolP);
            temp += s.dP[s.twoP as usize];
            s.dP[s.treP as usize] = temp*tankfeedback;
            s.dP[s.treP as usize] += inputSampleR;
            s.oneP -= 1; if s.oneP < 0 || s.oneP > s.delayP { s.oneP = s.delayP; }
            s.twoP -= 1; if s.twoP < 0 || s.twoP > s.delayP { s.twoP = s.delayP; }
            s.treP -= 1; if s.treP < 0 || s.treP > s.delayP { s.treP = s.delayP; }
            temp = s.dP[s.oneP as usize]*s.interpolP;
            temp += s.dP[s.treP as usize]*(1.0-s.interpolP);
            temp *= invlean + lean*s.dP[s.twoP as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dR[s.oneR as usize]*s.interpolR;
            temp += s.dR[s.treR as usize]*(1.0-s.interpolR);
            temp += s.dR[s.twoR as usize];
            s.dR[s.treR as usize] = temp*tankfeedback;
            s.dR[s.treR as usize] += inputSampleR;
            s.oneR -= 1; if s.oneR < 0 || s.oneR > s.delayR { s.oneR = s.delayR; }
            s.twoR -= 1; if s.twoR < 0 || s.twoR > s.delayR { s.twoR = s.delayR; }
            s.treR -= 1; if s.treR < 0 || s.treR > s.delayR { s.treR = s.delayR; }
            temp = s.dR[s.oneR as usize]*s.interpolR;
            temp += s.dR[s.treR as usize]*(1.0-s.interpolR);
            temp *= invlean + lean*s.dR[s.twoR as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dT[s.oneT as usize]*s.interpolT;
            temp += s.dT[s.treT as usize]*(1.0-s.interpolT);
            temp += s.dT[s.twoT as usize];
            s.dT[s.treT as usize] = temp*tankfeedback;
            s.dT[s.treT as usize] += inputSampleR;
            s.oneT -= 1; if s.oneT < 0 || s.oneT > s.delayT { s.oneT = s.delayT; }
            s.twoT -= 1; if s.twoT < 0 || s.twoT > s.delayT { s.twoT = s.delayT; }
            s.treT -= 1; if s.treT < 0 || s.treT > s.delayT { s.treT = s.delayT; }
            temp = s.dT[s.oneT as usize]*s.interpolT;
            temp += s.dT[s.treT as usize]*(1.0-s.interpolT);
            temp *= invlean + lean*s.dT[s.twoT as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dV[s.oneV as usize]*s.interpolV;
            temp += s.dV[s.treV as usize]*(1.0-s.interpolV);
            temp += s.dV[s.twoV as usize];
            s.dV[s.treV as usize] = temp*tankfeedback;
            s.dV[s.treV as usize] += inputSampleR;
            s.oneV -= 1; if s.oneV < 0 || s.oneV > s.delayV { s.oneV = s.delayV; }
            s.twoV -= 1; if s.twoV < 0 || s.twoV > s.delayV { s.twoV = s.delayV; }
            s.treV -= 1; if s.treV < 0 || s.treV > s.delayV { s.treV = s.delayV; }
            temp = s.dV[s.oneV as usize]*s.interpolV;
            temp += s.dV[s.treV as usize]*(1.0-s.interpolV);
            temp *= invlean + lean*s.dV[s.twoV as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dX[s.oneX as usize]*s.interpolX;
            temp += s.dX[s.treX as usize]*(1.0-s.interpolX);
            temp += s.dX[s.twoX as usize];
            s.dX[s.treX as usize] = temp*tankfeedback;
            s.dX[s.treX as usize] += inputSampleR;
            s.oneX -= 1; if s.oneX < 0 || s.oneX > s.delayX { s.oneX = s.delayX; }
            s.twoX -= 1; if s.twoX < 0 || s.twoX > s.delayX { s.twoX = s.delayX; }
            s.treX -= 1; if s.treX < 0 || s.treX > s.delayX { s.treX = s.delayX; }
            temp = s.dX[s.oneX as usize]*s.interpolX;
            temp += s.dX[s.treX as usize]*(1.0-s.interpolX);
            temp *= invlean + lean*s.dX[s.twoX as usize].abs();
            s.verboutR += temp;
            let mut temp = s.dZ[s.oneZ as usize]*s.interpolZ;
            temp += s.dZ[s.treZ as usize]*(1.0-s.interpolZ);
            temp += s.dZ[s.twoZ as usize];
            s.dZ[s.treZ as usize] = temp*tankfeedback;
            s.dZ[s.treZ as usize] += inputSampleR;
            s.oneZ -= 1; if s.oneZ < 0 || s.oneZ > s.delayZ { s.oneZ = s.delayZ; }
            s.twoZ -= 1; if s.twoZ < 0 || s.twoZ > s.delayZ { s.twoZ = s.delayZ; }
            s.treZ -= 1; if s.treZ < 0 || s.treZ > s.delayZ { s.treZ = s.delayZ; }
            temp = s.dZ[s.oneZ as usize]*s.interpolZ;
            temp += s.dZ[s.treZ as usize]*(1.0-s.interpolZ);
            temp *= invlean + lean*s.dZ[s.twoZ as usize].abs();
            s.verboutR += temp;
            s.verboutL /= 8.0; s.verboutR /= 8.0;
            s.iirSampleL = s.iirSampleL*(1.0-iirAmount)+s.verboutL*iirAmount;
            s.verboutL -= s.iirSampleL;
            s.iirSampleR = s.iirSampleR*(1.0-iirAmount)+s.verboutR*iirAmount;
            s.verboutR -= s.iirSampleR;
            s.verboutL *= invlean + lean*s.verboutL.abs();
            s.verboutR *= invlean + lean*s.verboutR.abs();
            inputSampleL = s.verboutL; inputSampleR = s.verboutR;
            if s.flip {
                s.lowpassSampleAA = s.lowpassSampleAA*(1.0-iirAmountC)+inputSampleL*iirAmountC; inputSampleL = s.lowpassSampleAA;
                s.lowpassSampleBA = s.lowpassSampleBA*(1.0-iirAmountC)+inputSampleL*iirAmountC; inputSampleL = s.lowpassSampleBA;
                s.lowpassSampleCA = s.lowpassSampleCA*(1.0-iirAmountC)+inputSampleL*iirAmountC; inputSampleL = s.lowpassSampleCA;
                s.lowpassSampleDA = s.lowpassSampleDA*(1.0-iirAmountC)+inputSampleL*iirAmountC; inputSampleL = s.lowpassSampleDA;
                s.lowpassSampleE = s.lowpassSampleE*(1.0-iirAmountC)+inputSampleL*iirAmountC; inputSampleL = s.lowpassSampleE;
            } else {
                s.lowpassSampleAB = s.lowpassSampleAB*(1.0-iirAmountC)+inputSampleL*iirAmountC; inputSampleL = s.lowpassSampleAB;
                s.lowpassSampleBB = s.lowpassSampleBB*(1.0-iirAmountC)+inputSampleL*iirAmountC; inputSampleL = s.lowpassSampleBB;
                s.lowpassSampleCB = s.lowpassSampleCB*(1.0-iirAmountC)+inputSampleL*iirAmountC; inputSampleL = s.lowpassSampleCB;
                s.lowpassSampleDB = s.lowpassSampleDB*(1.0-iirAmountC)+inputSampleL*iirAmountC; inputSampleL = s.lowpassSampleDB;
                s.lowpassSampleF = s.lowpassSampleF*(1.0-iirAmountC)+inputSampleL*iirAmountC; inputSampleL = s.lowpassSampleF;
            }
            s.lowpassSampleG = s.lowpassSampleG*(1.0-iirAmountC)+inputSampleL*iirAmountC;
            inputSampleL = s.lowpassSampleG*(1.0-iirAmountC)+inputSampleL*iirAmountC;
            if s.flip {
                s.rowpassSampleAA = s.rowpassSampleAA*(1.0-iirAmountC)+inputSampleR*iirAmountC; inputSampleR = s.rowpassSampleAA;
                s.rowpassSampleBA = s.rowpassSampleBA*(1.0-iirAmountC)+inputSampleR*iirAmountC; inputSampleR = s.rowpassSampleBA;
                s.rowpassSampleCA = s.rowpassSampleCA*(1.0-iirAmountC)+inputSampleR*iirAmountC; inputSampleR = s.rowpassSampleCA;
                s.rowpassSampleDA = s.rowpassSampleDA*(1.0-iirAmountC)+inputSampleR*iirAmountC; inputSampleR = s.rowpassSampleDA;
                s.rowpassSampleE = s.rowpassSampleE*(1.0-iirAmountC)+inputSampleR*iirAmountC; inputSampleR = s.rowpassSampleE;
            } else {
                s.rowpassSampleAB = s.rowpassSampleAB*(1.0-iirAmountC)+inputSampleR*iirAmountC; inputSampleR = s.rowpassSampleAB;
                s.rowpassSampleBB = s.rowpassSampleBB*(1.0-iirAmountC)+inputSampleR*iirAmountC; inputSampleR = s.rowpassSampleBB;
                s.rowpassSampleCB = s.rowpassSampleCB*(1.0-iirAmountC)+inputSampleR*iirAmountC; inputSampleR = s.rowpassSampleCB;
                s.rowpassSampleDB = s.rowpassSampleDB*(1.0-iirAmountC)+inputSampleR*iirAmountC; inputSampleR = s.rowpassSampleDB;
                s.rowpassSampleF = s.rowpassSampleF*(1.0-iirAmountC)+inputSampleR*iirAmountC; inputSampleR = s.rowpassSampleF;
            }
            s.rowpassSampleG = s.rowpassSampleG*(1.0-iirAmountC)+inputSampleR*iirAmountC;
            inputSampleR = s.rowpassSampleG*(1.0-iirAmountC)+inputSampleR*iirAmountC;
            s.iirCCSampleL = s.iirCCSampleL*(1.0-iirAmount)+s.verboutL*iirAmount;
            s.verboutL -= s.iirCCSampleL;
            s.iirCCSampleR = s.iirCCSampleR*(1.0-iirAmount)+s.verboutR*iirAmount;
            s.verboutR -= s.iirCCSampleR;
            s.verboutL *= invlean+lean*s.verboutL.abs();
            s.verboutR *= invlean+lean*s.verboutR.abs();
            s.verboutL = inputSampleL*outcouple; s.verboutR = inputSampleR*outcouple;
            s.nonlin += s.verboutL.abs(); s.nonlin += s.verboutR.abs();
            drySampleL *= dryness; drySampleR *= dryness;
            inputSampleL *= wetness; inputSampleR *= wetness;
            inputSampleL += drySampleL; inputSampleR += drySampleR;
            *out_l = fpd_dither(inputSampleL, &mut s.fpdL) as f32;
            *out_r = fpd_dither(inputSampleR, &mut s.fpdR) as f32;
            s.flip = !s.flip;
        }
    }
}
