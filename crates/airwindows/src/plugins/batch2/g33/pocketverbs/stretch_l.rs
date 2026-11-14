//! Stretch left channel; statement order follows upstream.
use super::state::State;

#[allow(unused_parens)]
pub(super) fn process(state: &mut State, mut input_sample: f64, const_allpass: f64) -> f64 {
    let mut allpasstemp: i32;
    allpasstemp = state.alpAL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayA {
        allpasstemp = state.delayA;
    }
    input_sample -= state.aAL[allpasstemp as usize] * const_allpass;
    state.aAL[state.alpAL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpAL -= 1;
    if state.alpAL < 0 || state.alpAL > state.delayA {
        state.alpAL = state.delayA;
    }
    input_sample += (state.aAL[state.alpAL as usize]);
    state.dAL[2] = state.dAL[1];
    state.dAL[1] = input_sample;
    input_sample = (state.dAL[1] + state.dAL[2]) / 2.0;
    allpasstemp = state.alpBL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayB {
        allpasstemp = state.delayB;
    }
    input_sample -= state.aBL[allpasstemp as usize] * const_allpass;
    state.aBL[state.alpBL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpBL -= 1;
    if state.alpBL < 0 || state.alpBL > state.delayB {
        state.alpBL = state.delayB;
    }
    input_sample += (state.aBL[state.alpBL as usize]);
    state.dBL[2] = state.dBL[1];
    state.dBL[1] = input_sample;
    input_sample = (state.dBL[1] + state.dBL[2]) / 2.0;
    allpasstemp = state.alpCL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayC {
        allpasstemp = state.delayC;
    }
    input_sample -= state.aCL[allpasstemp as usize] * const_allpass;
    state.aCL[state.alpCL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpCL -= 1;
    if state.alpCL < 0 || state.alpCL > state.delayC {
        state.alpCL = state.delayC;
    }
    input_sample += (state.aCL[state.alpCL as usize]);
    state.dCL[2] = state.dCL[1];
    state.dCL[1] = input_sample;
    input_sample = (state.dCL[1] + state.dCL[2]) / 2.0;
    allpasstemp = state.alpDL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayD {
        allpasstemp = state.delayD;
    }
    input_sample -= state.aDL[allpasstemp as usize] * const_allpass;
    state.aDL[state.alpDL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpDL -= 1;
    if state.alpDL < 0 || state.alpDL > state.delayD {
        state.alpDL = state.delayD;
    }
    input_sample += (state.aDL[state.alpDL as usize]);
    state.dDL[2] = state.dDL[1];
    state.dDL[1] = input_sample;
    input_sample = (state.dDL[1] + state.dDL[2]) / 2.0;
    allpasstemp = state.alpEL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayE {
        allpasstemp = state.delayE;
    }
    input_sample -= state.aEL[allpasstemp as usize] * const_allpass;
    state.aEL[state.alpEL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpEL -= 1;
    if state.alpEL < 0 || state.alpEL > state.delayE {
        state.alpEL = state.delayE;
    }
    input_sample += (state.aEL[state.alpEL as usize]);
    state.dEL[2] = state.dEL[1];
    state.dEL[1] = input_sample;
    input_sample = (state.dEL[1] + state.dEL[2]) / 2.0;
    allpasstemp = state.alpFL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayF {
        allpasstemp = state.delayF;
    }
    input_sample -= state.aFL[allpasstemp as usize] * const_allpass;
    state.aFL[state.alpFL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpFL -= 1;
    if state.alpFL < 0 || state.alpFL > state.delayF {
        state.alpFL = state.delayF;
    }
    input_sample += (state.aFL[state.alpFL as usize]);
    state.dFL[2] = state.dFL[1];
    state.dFL[1] = input_sample;
    input_sample = (state.dFL[1] + state.dFL[2]) / 2.0;
    allpasstemp = state.alpGL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayG {
        allpasstemp = state.delayG;
    }
    input_sample -= state.aGL[allpasstemp as usize] * const_allpass;
    state.aGL[state.alpGL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpGL -= 1;
    if state.alpGL < 0 || state.alpGL > state.delayG {
        state.alpGL = state.delayG;
    }
    input_sample += (state.aGL[state.alpGL as usize]);
    state.dGL[2] = state.dGL[1];
    state.dGL[1] = input_sample;
    input_sample = (state.dGL[1] + state.dGL[2]) / 2.0;
    allpasstemp = state.alpHL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayH {
        allpasstemp = state.delayH;
    }
    input_sample -= state.aHL[allpasstemp as usize] * const_allpass;
    state.aHL[state.alpHL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpHL -= 1;
    if state.alpHL < 0 || state.alpHL > state.delayH {
        state.alpHL = state.delayH;
    }
    input_sample += (state.aHL[state.alpHL as usize]);
    state.dHL[2] = state.dHL[1];
    state.dHL[1] = input_sample;
    input_sample = (state.dHL[1] + state.dHL[2]) / 2.0;
    allpasstemp = state.alpIL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayI {
        allpasstemp = state.delayI;
    }
    input_sample -= state.aIL[allpasstemp as usize] * const_allpass;
    state.aIL[state.alpIL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpIL -= 1;
    if state.alpIL < 0 || state.alpIL > state.delayI {
        state.alpIL = state.delayI;
    }
    input_sample += (state.aIL[state.alpIL as usize]);
    state.dIL[2] = state.dIL[1];
    state.dIL[1] = input_sample;
    input_sample = (state.dIL[1] + state.dIL[2]) / 2.0;
    allpasstemp = state.alpJL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayJ {
        allpasstemp = state.delayJ;
    }
    input_sample -= state.aJL[allpasstemp as usize] * const_allpass;
    state.aJL[state.alpJL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpJL -= 1;
    if state.alpJL < 0 || state.alpJL > state.delayJ {
        state.alpJL = state.delayJ;
    }
    input_sample += (state.aJL[state.alpJL as usize]);
    state.dJL[2] = state.dJL[1];
    state.dJL[1] = input_sample;
    input_sample = (state.dJL[1] + state.dJL[2]) / 2.0;
    allpasstemp = state.alpKL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayK {
        allpasstemp = state.delayK;
    }
    input_sample -= state.aKL[allpasstemp as usize] * const_allpass;
    state.aKL[state.alpKL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpKL -= 1;
    if state.alpKL < 0 || state.alpKL > state.delayK {
        state.alpKL = state.delayK;
    }
    input_sample += (state.aKL[state.alpKL as usize]);
    state.dKL[2] = state.dKL[1];
    state.dKL[1] = input_sample;
    input_sample = (state.dKL[1] + state.dKL[2]) / 2.0;
    allpasstemp = state.alpLL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayL {
        allpasstemp = state.delayL;
    }
    input_sample -= state.aLL[allpasstemp as usize] * const_allpass;
    state.aLL[state.alpLL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpLL -= 1;
    if state.alpLL < 0 || state.alpLL > state.delayL {
        state.alpLL = state.delayL;
    }
    input_sample += (state.aLL[state.alpLL as usize]);
    state.dLL[2] = state.dLL[1];
    state.dLL[1] = input_sample;
    input_sample = (state.dLL[1] + state.dLL[2]) / 2.0;
    allpasstemp = state.alpML - 1;
    if allpasstemp < 0 || allpasstemp > state.delayM {
        allpasstemp = state.delayM;
    }
    input_sample -= state.aML[allpasstemp as usize] * const_allpass;
    state.aML[state.alpML as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpML -= 1;
    if state.alpML < 0 || state.alpML > state.delayM {
        state.alpML = state.delayM;
    }
    input_sample += (state.aML[state.alpML as usize]);
    state.dML[2] = state.dML[1];
    state.dML[1] = input_sample;
    input_sample = (state.dML[1] + state.dML[2]) / 2.0;
    allpasstemp = state.alpNL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayN {
        allpasstemp = state.delayN;
    }
    input_sample -= state.aNL[allpasstemp as usize] * const_allpass;
    state.aNL[state.alpNL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpNL -= 1;
    if state.alpNL < 0 || state.alpNL > state.delayN {
        state.alpNL = state.delayN;
    }
    input_sample += (state.aNL[state.alpNL as usize]);
    state.dNL[2] = state.dNL[1];
    state.dNL[1] = input_sample;
    input_sample = (state.dNL[1] + state.dNL[2]) / 2.0;
    allpasstemp = state.alpOL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayO {
        allpasstemp = state.delayO;
    }
    input_sample -= state.aOL[allpasstemp as usize] * const_allpass;
    state.aOL[state.alpOL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpOL -= 1;
    if state.alpOL < 0 || state.alpOL > state.delayO {
        state.alpOL = state.delayO;
    }
    input_sample += (state.aOL[state.alpOL as usize]);
    state.dOL[2] = state.dOL[1];
    state.dOL[1] = input_sample;
    input_sample = (state.dOL[1] + state.dOL[2]) / 2.0;
    allpasstemp = state.alpPL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayP {
        allpasstemp = state.delayP;
    }
    input_sample -= state.aPL[allpasstemp as usize] * const_allpass;
    state.aPL[state.alpPL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpPL -= 1;
    if state.alpPL < 0 || state.alpPL > state.delayP {
        state.alpPL = state.delayP;
    }
    input_sample += (state.aPL[state.alpPL as usize]);
    state.dPL[2] = state.dPL[1];
    state.dPL[1] = input_sample;
    input_sample = (state.dPL[1] + state.dPL[2]) / 2.0;
    allpasstemp = state.alpQL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayQ {
        allpasstemp = state.delayQ;
    }
    input_sample -= state.aQL[allpasstemp as usize] * const_allpass;
    state.aQL[state.alpQL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpQL -= 1;
    if state.alpQL < 0 || state.alpQL > state.delayQ {
        state.alpQL = state.delayQ;
    }
    input_sample += (state.aQL[state.alpQL as usize]);
    state.dQL[2] = state.dQL[1];
    state.dQL[1] = input_sample;
    input_sample = (state.dQL[1] + state.dQL[2]) / 2.0;
    allpasstemp = state.alpRL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayR {
        allpasstemp = state.delayR;
    }
    input_sample -= state.aRL[allpasstemp as usize] * const_allpass;
    state.aRL[state.alpRL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpRL -= 1;
    if state.alpRL < 0 || state.alpRL > state.delayR {
        state.alpRL = state.delayR;
    }
    input_sample += (state.aRL[state.alpRL as usize]);
    state.dRL[2] = state.dRL[1];
    state.dRL[1] = input_sample;
    input_sample = (state.dRL[1] + state.dRL[2]) / 2.0;
    allpasstemp = state.alpSL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayS {
        allpasstemp = state.delayS;
    }
    input_sample -= state.aSL[allpasstemp as usize] * const_allpass;
    state.aSL[state.alpSL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpSL -= 1;
    if state.alpSL < 0 || state.alpSL > state.delayS {
        state.alpSL = state.delayS;
    }
    input_sample += (state.aSL[state.alpSL as usize]);
    state.dSL[2] = state.dSL[1];
    state.dSL[1] = input_sample;
    input_sample = (state.dSL[1] + state.dSL[2]) / 2.0;
    allpasstemp = state.alpTL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayT {
        allpasstemp = state.delayT;
    }
    input_sample -= state.aTL[allpasstemp as usize] * const_allpass;
    state.aTL[state.alpTL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpTL -= 1;
    if state.alpTL < 0 || state.alpTL > state.delayT {
        state.alpTL = state.delayT;
    }
    input_sample += (state.aTL[state.alpTL as usize]);
    state.dTL[2] = state.dTL[1];
    state.dTL[1] = input_sample;
    input_sample = (state.dTL[1] + state.dTL[2]) / 2.0;
    allpasstemp = state.alpUL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayU {
        allpasstemp = state.delayU;
    }
    input_sample -= state.aUL[allpasstemp as usize] * const_allpass;
    state.aUL[state.alpUL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpUL -= 1;
    if state.alpUL < 0 || state.alpUL > state.delayU {
        state.alpUL = state.delayU;
    }
    input_sample += (state.aUL[state.alpUL as usize]);
    state.dUL[2] = state.dUL[1];
    state.dUL[1] = input_sample;
    input_sample = (state.dUL[1] + state.dUL[2]) / 2.0;
    allpasstemp = state.alpVL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayV {
        allpasstemp = state.delayV;
    }
    input_sample -= state.aVL[allpasstemp as usize] * const_allpass;
    state.aVL[state.alpVL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpVL -= 1;
    if state.alpVL < 0 || state.alpVL > state.delayV {
        state.alpVL = state.delayV;
    }
    input_sample += (state.aVL[state.alpVL as usize]);
    state.dVL[2] = state.dVL[1];
    state.dVL[1] = input_sample;
    input_sample = (state.dVL[1] + state.dVL[2]) / 2.0;
    allpasstemp = state.alpWL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayW {
        allpasstemp = state.delayW;
    }
    input_sample -= state.aWL[allpasstemp as usize] * const_allpass;
    state.aWL[state.alpWL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpWL -= 1;
    if state.alpWL < 0 || state.alpWL > state.delayW {
        state.alpWL = state.delayW;
    }
    input_sample += (state.aWL[state.alpWL as usize]);
    state.dWL[2] = state.dWL[1];
    state.dWL[1] = input_sample;
    input_sample = (state.dWL[1] + state.dWL[2]) / 2.0;
    allpasstemp = state.alpXL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayX {
        allpasstemp = state.delayX;
    }
    input_sample -= state.aXL[allpasstemp as usize] * const_allpass;
    state.aXL[state.alpXL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpXL -= 1;
    if state.alpXL < 0 || state.alpXL > state.delayX {
        state.alpXL = state.delayX;
    }
    input_sample += (state.aXL[state.alpXL as usize]);
    state.dXL[2] = state.dXL[1];
    state.dXL[1] = input_sample;
    input_sample = (state.dXL[1] + state.dXL[2]) / 2.0;
    allpasstemp = state.alpYL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayY {
        allpasstemp = state.delayY;
    }
    input_sample -= state.aYL[allpasstemp as usize] * const_allpass;
    state.aYL[state.alpYL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpYL -= 1;
    if state.alpYL < 0 || state.alpYL > state.delayY {
        state.alpYL = state.delayY;
    }
    input_sample += (state.aYL[state.alpYL as usize]);
    state.dYL[2] = state.dYL[1];
    state.dYL[1] = input_sample;
    input_sample = (state.dYL[1] + state.dYL[2]) / 2.0;
    allpasstemp = state.alpZL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayZ {
        allpasstemp = state.delayZ;
    }
    input_sample -= state.aZL[allpasstemp as usize] * const_allpass;
    state.aZL[state.alpZL as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpZL -= 1;
    if state.alpZL < 0 || state.alpZL > state.delayZ {
        state.alpZL = state.delayZ;
    }
    input_sample += (state.aZL[state.alpZL as usize]);
    state.dZL[2] = state.dZL[1];
    state.dZL[1] = input_sample;
    input_sample = (state.dZL[1] + state.dZL[2]) / 2.0;
    allpasstemp = state.outAL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayA {
        allpasstemp = state.delayA;
    }
    input_sample -= state.oAL[allpasstemp as usize] * const_allpass;
    state.oAL[state.outAL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outAL -= 1;
    if state.outAL < 0 || state.outAL > state.delayA {
        state.outAL = state.delayA;
    }
    input_sample += (state.oAL[state.outAL as usize]);
    state.dAL[5] = state.dAL[4];
    state.dAL[4] = input_sample;
    input_sample = (state.dAL[4] + state.dAL[5]) / 2.0;
    allpasstemp = state.outBL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayB {
        allpasstemp = state.delayB;
    }
    input_sample -= state.oBL[allpasstemp as usize] * const_allpass;
    state.oBL[state.outBL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outBL -= 1;
    if state.outBL < 0 || state.outBL > state.delayB {
        state.outBL = state.delayB;
    }
    input_sample += (state.oBL[state.outBL as usize]);
    state.dBL[5] = state.dBL[4];
    state.dBL[4] = input_sample;
    input_sample = (state.dBL[4] + state.dBL[5]) / 2.0;
    allpasstemp = state.outCL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayC {
        allpasstemp = state.delayC;
    }
    input_sample -= state.oCL[allpasstemp as usize] * const_allpass;
    state.oCL[state.outCL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outCL -= 1;
    if state.outCL < 0 || state.outCL > state.delayC {
        state.outCL = state.delayC;
    }
    input_sample += (state.oCL[state.outCL as usize]);
    state.dCL[5] = state.dCL[4];
    state.dCL[4] = input_sample;
    input_sample = (state.dCL[4] + state.dCL[5]) / 2.0;
    allpasstemp = state.outDL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayD {
        allpasstemp = state.delayD;
    }
    input_sample -= state.oDL[allpasstemp as usize] * const_allpass;
    state.oDL[state.outDL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outDL -= 1;
    if state.outDL < 0 || state.outDL > state.delayD {
        state.outDL = state.delayD;
    }
    input_sample += (state.oDL[state.outDL as usize]);
    state.dDL[5] = state.dDL[4];
    state.dDL[4] = input_sample;
    input_sample = (state.dDL[4] + state.dDL[5]) / 2.0;
    allpasstemp = state.outEL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayE {
        allpasstemp = state.delayE;
    }
    input_sample -= state.oEL[allpasstemp as usize] * const_allpass;
    state.oEL[state.outEL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outEL -= 1;
    if state.outEL < 0 || state.outEL > state.delayE {
        state.outEL = state.delayE;
    }
    input_sample += (state.oEL[state.outEL as usize]);
    state.dEL[5] = state.dEL[4];
    state.dEL[4] = input_sample;
    input_sample = (state.dEL[4] + state.dEL[5]) / 2.0;
    allpasstemp = state.outFL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayF {
        allpasstemp = state.delayF;
    }
    input_sample -= state.oFL[allpasstemp as usize] * const_allpass;
    state.oFL[state.outFL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outFL -= 1;
    if state.outFL < 0 || state.outFL > state.delayF {
        state.outFL = state.delayF;
    }
    input_sample += (state.oFL[state.outFL as usize]);
    state.dFL[5] = state.dFL[4];
    state.dFL[4] = input_sample;
    input_sample = (state.dFL[4] + state.dFL[5]) / 2.0;
    allpasstemp = state.outGL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayG {
        allpasstemp = state.delayG;
    }
    input_sample -= state.oGL[allpasstemp as usize] * const_allpass;
    state.oGL[state.outGL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outGL -= 1;
    if state.outGL < 0 || state.outGL > state.delayG {
        state.outGL = state.delayG;
    }
    input_sample += (state.oGL[state.outGL as usize]);
    state.dGL[5] = state.dGL[4];
    state.dGL[4] = input_sample;
    input_sample = (state.dGL[4] + state.dGL[5]) / 2.0;
    allpasstemp = state.outHL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayH {
        allpasstemp = state.delayH;
    }
    input_sample -= state.oHL[allpasstemp as usize] * const_allpass;
    state.oHL[state.outHL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outHL -= 1;
    if state.outHL < 0 || state.outHL > state.delayH {
        state.outHL = state.delayH;
    }
    input_sample += (state.oHL[state.outHL as usize]);
    state.dHL[5] = state.dHL[4];
    state.dHL[4] = input_sample;
    input_sample = (state.dHL[4] + state.dHL[5]) / 2.0;
    allpasstemp = state.outIL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayI {
        allpasstemp = state.delayI;
    }
    input_sample -= state.oIL[allpasstemp as usize] * const_allpass;
    state.oIL[state.outIL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outIL -= 1;
    if state.outIL < 0 || state.outIL > state.delayI {
        state.outIL = state.delayI;
    }
    input_sample += (state.oIL[state.outIL as usize]);
    state.dIL[5] = state.dIL[4];
    state.dIL[4] = input_sample;
    input_sample = (state.dIL[4] + state.dIL[5]) / 2.0;
    allpasstemp = state.outJL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayJ {
        allpasstemp = state.delayJ;
    }
    input_sample -= state.oJL[allpasstemp as usize] * const_allpass;
    state.oJL[state.outJL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outJL -= 1;
    if state.outJL < 0 || state.outJL > state.delayJ {
        state.outJL = state.delayJ;
    }
    input_sample += (state.oJL[state.outJL as usize]);
    state.dJL[5] = state.dJL[4];
    state.dJL[4] = input_sample;
    input_sample = (state.dJL[4] + state.dJL[5]) / 2.0;
    allpasstemp = state.outKL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayK {
        allpasstemp = state.delayK;
    }
    input_sample -= state.oKL[allpasstemp as usize] * const_allpass;
    state.oKL[state.outKL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outKL -= 1;
    if state.outKL < 0 || state.outKL > state.delayK {
        state.outKL = state.delayK;
    }
    input_sample += (state.oKL[state.outKL as usize]);
    state.dKL[5] = state.dKL[4];
    state.dKL[4] = input_sample;
    input_sample = (state.dKL[4] + state.dKL[5]) / 2.0;
    allpasstemp = state.outLL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayL {
        allpasstemp = state.delayL;
    }
    input_sample -= state.oLL[allpasstemp as usize] * const_allpass;
    state.oLL[state.outLL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outLL -= 1;
    if state.outLL < 0 || state.outLL > state.delayL {
        state.outLL = state.delayL;
    }
    input_sample += (state.oLL[state.outLL as usize]);
    state.dLL[5] = state.dLL[4];
    state.dLL[4] = input_sample;
    input_sample = (state.dLL[4] + state.dLL[5]) / 2.0;
    allpasstemp = state.outML - 1;
    if allpasstemp < 0 || allpasstemp > state.delayM {
        allpasstemp = state.delayM;
    }
    input_sample -= state.oML[allpasstemp as usize] * const_allpass;
    state.oML[state.outML as usize] = input_sample;
    input_sample *= const_allpass;
    state.outML -= 1;
    if state.outML < 0 || state.outML > state.delayM {
        state.outML = state.delayM;
    }
    input_sample += (state.oML[state.outML as usize]);
    state.dML[5] = state.dML[4];
    state.dML[4] = input_sample;
    input_sample = (state.dML[4] + state.dML[5]) / 2.0;
    allpasstemp = state.outNL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayN {
        allpasstemp = state.delayN;
    }
    input_sample -= state.oNL[allpasstemp as usize] * const_allpass;
    state.oNL[state.outNL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outNL -= 1;
    if state.outNL < 0 || state.outNL > state.delayN {
        state.outNL = state.delayN;
    }
    input_sample += (state.oNL[state.outNL as usize]);
    state.dNL[5] = state.dNL[4];
    state.dNL[4] = input_sample;
    input_sample = (state.dNL[4] + state.dNL[5]) / 2.0;
    allpasstemp = state.outOL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayO {
        allpasstemp = state.delayO;
    }
    input_sample -= state.oOL[allpasstemp as usize] * const_allpass;
    state.oOL[state.outOL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outOL -= 1;
    if state.outOL < 0 || state.outOL > state.delayO {
        state.outOL = state.delayO;
    }
    input_sample += (state.oOL[state.outOL as usize]);
    state.dOL[5] = state.dOL[4];
    state.dOL[4] = input_sample;
    input_sample = (state.dOL[4] + state.dOL[5]) / 2.0;
    allpasstemp = state.outPL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayP {
        allpasstemp = state.delayP;
    }
    input_sample -= state.oPL[allpasstemp as usize] * const_allpass;
    state.oPL[state.outPL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outPL -= 1;
    if state.outPL < 0 || state.outPL > state.delayP {
        state.outPL = state.delayP;
    }
    input_sample += (state.oPL[state.outPL as usize]);
    state.dPL[5] = state.dPL[4];
    state.dPL[4] = input_sample;
    input_sample = (state.dPL[4] + state.dPL[5]) / 2.0;
    allpasstemp = state.outQL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayQ {
        allpasstemp = state.delayQ;
    }
    input_sample -= state.oQL[allpasstemp as usize] * const_allpass;
    state.oQL[state.outQL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outQL -= 1;
    if state.outQL < 0 || state.outQL > state.delayQ {
        state.outQL = state.delayQ;
    }
    input_sample += (state.oQL[state.outQL as usize]);
    state.dQL[5] = state.dQL[4];
    state.dQL[4] = input_sample;
    input_sample = (state.dQL[4] + state.dQL[5]) / 2.0;
    allpasstemp = state.outRL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayR {
        allpasstemp = state.delayR;
    }
    input_sample -= state.oRL[allpasstemp as usize] * const_allpass;
    state.oRL[state.outRL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outRL -= 1;
    if state.outRL < 0 || state.outRL > state.delayR {
        state.outRL = state.delayR;
    }
    input_sample += (state.oRL[state.outRL as usize]);
    state.dRL[5] = state.dRL[4];
    state.dRL[4] = input_sample;
    input_sample = (state.dRL[4] + state.dRL[5]) / 2.0;
    allpasstemp = state.outSL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayS {
        allpasstemp = state.delayS;
    }
    input_sample -= state.oSL[allpasstemp as usize] * const_allpass;
    state.oSL[state.outSL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outSL -= 1;
    if state.outSL < 0 || state.outSL > state.delayS {
        state.outSL = state.delayS;
    }
    input_sample += (state.oSL[state.outSL as usize]);
    state.dSL[5] = state.dSL[4];
    state.dSL[4] = input_sample;
    input_sample = (state.dSL[4] + state.dSL[5]) / 2.0;
    allpasstemp = state.outTL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayT {
        allpasstemp = state.delayT;
    }
    input_sample -= state.oTL[allpasstemp as usize] * const_allpass;
    state.oTL[state.outTL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outTL -= 1;
    if state.outTL < 0 || state.outTL > state.delayT {
        state.outTL = state.delayT;
    }
    input_sample += (state.oTL[state.outTL as usize]);
    state.dTL[5] = state.dTL[4];
    state.dTL[4] = input_sample;
    input_sample = (state.dTL[4] + state.dTL[5]) / 2.0;
    allpasstemp = state.outUL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayU {
        allpasstemp = state.delayU;
    }
    input_sample -= state.oUL[allpasstemp as usize] * const_allpass;
    state.oUL[state.outUL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outUL -= 1;
    if state.outUL < 0 || state.outUL > state.delayU {
        state.outUL = state.delayU;
    }
    input_sample += (state.oUL[state.outUL as usize]);
    state.dUL[5] = state.dUL[4];
    state.dUL[4] = input_sample;
    input_sample = (state.dUL[4] + state.dUL[5]) / 2.0;
    allpasstemp = state.outVL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayV {
        allpasstemp = state.delayV;
    }
    input_sample -= state.oVL[allpasstemp as usize] * const_allpass;
    state.oVL[state.outVL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outVL -= 1;
    if state.outVL < 0 || state.outVL > state.delayV {
        state.outVL = state.delayV;
    }
    input_sample += (state.oVL[state.outVL as usize]);
    state.dVL[5] = state.dVL[4];
    state.dVL[4] = input_sample;
    input_sample = (state.dVL[4] + state.dVL[5]) / 2.0;
    allpasstemp = state.outWL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayW {
        allpasstemp = state.delayW;
    }
    input_sample -= state.oWL[allpasstemp as usize] * const_allpass;
    state.oWL[state.outWL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outWL -= 1;
    if state.outWL < 0 || state.outWL > state.delayW {
        state.outWL = state.delayW;
    }
    input_sample += (state.oWL[state.outWL as usize]);
    state.dWL[5] = state.dWL[4];
    state.dWL[4] = input_sample;
    input_sample = (state.dWL[4] + state.dWL[5]) / 2.0;
    allpasstemp = state.outXL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayX {
        allpasstemp = state.delayX;
    }
    input_sample -= state.oXL[allpasstemp as usize] * const_allpass;
    state.oXL[state.outXL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outXL -= 1;
    if state.outXL < 0 || state.outXL > state.delayX {
        state.outXL = state.delayX;
    }
    input_sample += (state.oXL[state.outXL as usize]);
    state.dXL[5] = state.dXL[4];
    state.dXL[4] = input_sample;
    input_sample = (state.dXL[4] + state.dXL[5]) / 2.0;
    allpasstemp = state.outYL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayY {
        allpasstemp = state.delayY;
    }
    input_sample -= state.oYL[allpasstemp as usize] * const_allpass;
    state.oYL[state.outYL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outYL -= 1;
    if state.outYL < 0 || state.outYL > state.delayY {
        state.outYL = state.delayY;
    }
    input_sample += (state.oYL[state.outYL as usize]);
    state.dYL[5] = state.dYL[4];
    state.dYL[4] = input_sample;
    input_sample = (state.dYL[4] + state.dYL[5]) / 2.0;
    allpasstemp = state.outZL - 1;
    if allpasstemp < 0 || allpasstemp > state.delayZ {
        allpasstemp = state.delayZ;
    }
    input_sample -= state.oZL[allpasstemp as usize] * const_allpass;
    state.oZL[state.outZL as usize] = input_sample;
    input_sample *= const_allpass;
    state.outZL -= 1;
    if state.outZL < 0 || state.outZL > state.delayZ {
        state.outZL = state.delayZ;
    }
    input_sample += (state.oZL[state.outZL as usize]);
    state.dZL[5] = state.dZL[4];
    state.dZL[4] = input_sample;
    input_sample = (state.dZL[4] + state.dZL[5]) / 2.0;
    input_sample
}
