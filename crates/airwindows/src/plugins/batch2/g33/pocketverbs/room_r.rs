//! Room right channel; statement order follows upstream.
use super::state::State;

#[allow(unused_parens)]
pub(super) fn process(
    state: &mut State,
    mut input_sample: f64,
    dry_sample: f64,
    const_allpass: f64,
    dryness: f64,
    wetness: f64,
) -> f64 {
    let mut allpasstemp: i32;
    allpasstemp = state.alpAR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayA {
        allpasstemp = state.delayA;
    }
    input_sample -= state.aAR[allpasstemp as usize] * const_allpass;
    state.aAR[state.alpAR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpAR -= 1;
    if state.alpAR < 0 || state.alpAR > state.delayA {
        state.alpAR = state.delayA;
    }
    input_sample += (state.aAR[state.alpAR as usize]);
    state.dAR[2] = state.dAR[1];
    state.dAR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpBR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayB {
        allpasstemp = state.delayB;
    }
    input_sample -= state.aBR[allpasstemp as usize] * const_allpass;
    state.aBR[state.alpBR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpBR -= 1;
    if state.alpBR < 0 || state.alpBR > state.delayB {
        state.alpBR = state.delayB;
    }
    input_sample += (state.aBR[state.alpBR as usize]);
    state.dBR[2] = state.dBR[1];
    state.dBR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpCR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayC {
        allpasstemp = state.delayC;
    }
    input_sample -= state.aCR[allpasstemp as usize] * const_allpass;
    state.aCR[state.alpCR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpCR -= 1;
    if state.alpCR < 0 || state.alpCR > state.delayC {
        state.alpCR = state.delayC;
    }
    input_sample += (state.aCR[state.alpCR as usize]);
    state.dCR[2] = state.dCR[1];
    state.dCR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpDR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayD {
        allpasstemp = state.delayD;
    }
    input_sample -= state.aDR[allpasstemp as usize] * const_allpass;
    state.aDR[state.alpDR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpDR -= 1;
    if state.alpDR < 0 || state.alpDR > state.delayD {
        state.alpDR = state.delayD;
    }
    input_sample += (state.aDR[state.alpDR as usize]);
    state.dDR[2] = state.dDR[1];
    state.dDR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpER - 1;
    if allpasstemp < 0 || allpasstemp > state.delayE {
        allpasstemp = state.delayE;
    }
    input_sample -= state.aER[allpasstemp as usize] * const_allpass;
    state.aER[state.alpER as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpER -= 1;
    if state.alpER < 0 || state.alpER > state.delayE {
        state.alpER = state.delayE;
    }
    input_sample += (state.aER[state.alpER as usize]);
    state.dER[2] = state.dER[1];
    state.dER[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpFR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayF {
        allpasstemp = state.delayF;
    }
    input_sample -= state.aFR[allpasstemp as usize] * const_allpass;
    state.aFR[state.alpFR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpFR -= 1;
    if state.alpFR < 0 || state.alpFR > state.delayF {
        state.alpFR = state.delayF;
    }
    input_sample += (state.aFR[state.alpFR as usize]);
    state.dFR[2] = state.dFR[1];
    state.dFR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpGR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayG {
        allpasstemp = state.delayG;
    }
    input_sample -= state.aGR[allpasstemp as usize] * const_allpass;
    state.aGR[state.alpGR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpGR -= 1;
    if state.alpGR < 0 || state.alpGR > state.delayG {
        state.alpGR = state.delayG;
    }
    input_sample += (state.aGR[state.alpGR as usize]);
    state.dGR[2] = state.dGR[1];
    state.dGR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpHR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayH {
        allpasstemp = state.delayH;
    }
    input_sample -= state.aHR[allpasstemp as usize] * const_allpass;
    state.aHR[state.alpHR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpHR -= 1;
    if state.alpHR < 0 || state.alpHR > state.delayH {
        state.alpHR = state.delayH;
    }
    input_sample += (state.aHR[state.alpHR as usize]);
    state.dHR[2] = state.dHR[1];
    state.dHR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpIR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayI {
        allpasstemp = state.delayI;
    }
    input_sample -= state.aIR[allpasstemp as usize] * const_allpass;
    state.aIR[state.alpIR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpIR -= 1;
    if state.alpIR < 0 || state.alpIR > state.delayI {
        state.alpIR = state.delayI;
    }
    input_sample += (state.aIR[state.alpIR as usize]);
    state.dIR[2] = state.dIR[1];
    state.dIR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpJR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayJ {
        allpasstemp = state.delayJ;
    }
    input_sample -= state.aJR[allpasstemp as usize] * const_allpass;
    state.aJR[state.alpJR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpJR -= 1;
    if state.alpJR < 0 || state.alpJR > state.delayJ {
        state.alpJR = state.delayJ;
    }
    input_sample += (state.aJR[state.alpJR as usize]);
    state.dJR[2] = state.dJR[1];
    state.dJR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpKR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayK {
        allpasstemp = state.delayK;
    }
    input_sample -= state.aKR[allpasstemp as usize] * const_allpass;
    state.aKR[state.alpKR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpKR -= 1;
    if state.alpKR < 0 || state.alpKR > state.delayK {
        state.alpKR = state.delayK;
    }
    input_sample += (state.aKR[state.alpKR as usize]);
    state.dKR[2] = state.dKR[1];
    state.dKR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpLR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayL {
        allpasstemp = state.delayL;
    }
    input_sample -= state.aLR[allpasstemp as usize] * const_allpass;
    state.aLR[state.alpLR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpLR -= 1;
    if state.alpLR < 0 || state.alpLR > state.delayL {
        state.alpLR = state.delayL;
    }
    input_sample += (state.aLR[state.alpLR as usize]);
    state.dLR[2] = state.dLR[1];
    state.dLR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpMR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayM {
        allpasstemp = state.delayM;
    }
    input_sample -= state.aMR[allpasstemp as usize] * const_allpass;
    state.aMR[state.alpMR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpMR -= 1;
    if state.alpMR < 0 || state.alpMR > state.delayM {
        state.alpMR = state.delayM;
    }
    input_sample += (state.aMR[state.alpMR as usize]);
    state.dMR[2] = state.dMR[1];
    state.dMR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpNR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayN {
        allpasstemp = state.delayN;
    }
    input_sample -= state.aNR[allpasstemp as usize] * const_allpass;
    state.aNR[state.alpNR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpNR -= 1;
    if state.alpNR < 0 || state.alpNR > state.delayN {
        state.alpNR = state.delayN;
    }
    input_sample += (state.aNR[state.alpNR as usize]);
    state.dNR[2] = state.dNR[1];
    state.dNR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpOR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayO {
        allpasstemp = state.delayO;
    }
    input_sample -= state.aOR[allpasstemp as usize] * const_allpass;
    state.aOR[state.alpOR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpOR -= 1;
    if state.alpOR < 0 || state.alpOR > state.delayO {
        state.alpOR = state.delayO;
    }
    input_sample += (state.aOR[state.alpOR as usize]);
    state.dOR[2] = state.dOR[1];
    state.dOR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpPR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayP {
        allpasstemp = state.delayP;
    }
    input_sample -= state.aPR[allpasstemp as usize] * const_allpass;
    state.aPR[state.alpPR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpPR -= 1;
    if state.alpPR < 0 || state.alpPR > state.delayP {
        state.alpPR = state.delayP;
    }
    input_sample += (state.aPR[state.alpPR as usize]);
    state.dPR[2] = state.dPR[1];
    state.dPR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpQR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayQ {
        allpasstemp = state.delayQ;
    }
    input_sample -= state.aQR[allpasstemp as usize] * const_allpass;
    state.aQR[state.alpQR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpQR -= 1;
    if state.alpQR < 0 || state.alpQR > state.delayQ {
        state.alpQR = state.delayQ;
    }
    input_sample += (state.aQR[state.alpQR as usize]);
    state.dQR[2] = state.dQR[1];
    state.dQR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpRR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayR {
        allpasstemp = state.delayR;
    }
    input_sample -= state.aRR[allpasstemp as usize] * const_allpass;
    state.aRR[state.alpRR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpRR -= 1;
    if state.alpRR < 0 || state.alpRR > state.delayR {
        state.alpRR = state.delayR;
    }
    input_sample += (state.aRR[state.alpRR as usize]);
    state.dRR[2] = state.dRR[1];
    state.dRR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpSR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayS {
        allpasstemp = state.delayS;
    }
    input_sample -= state.aSR[allpasstemp as usize] * const_allpass;
    state.aSR[state.alpSR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpSR -= 1;
    if state.alpSR < 0 || state.alpSR > state.delayS {
        state.alpSR = state.delayS;
    }
    input_sample += (state.aSR[state.alpSR as usize]);
    state.dSR[2] = state.dSR[1];
    state.dSR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpTR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayT {
        allpasstemp = state.delayT;
    }
    input_sample -= state.aTR[allpasstemp as usize] * const_allpass;
    state.aTR[state.alpTR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpTR -= 1;
    if state.alpTR < 0 || state.alpTR > state.delayT {
        state.alpTR = state.delayT;
    }
    input_sample += (state.aTR[state.alpTR as usize]);
    state.dTR[2] = state.dTR[1];
    state.dTR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpUR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayU {
        allpasstemp = state.delayU;
    }
    input_sample -= state.aUR[allpasstemp as usize] * const_allpass;
    state.aUR[state.alpUR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpUR -= 1;
    if state.alpUR < 0 || state.alpUR > state.delayU {
        state.alpUR = state.delayU;
    }
    input_sample += (state.aUR[state.alpUR as usize]);
    state.dUR[2] = state.dUR[1];
    state.dUR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpVR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayV {
        allpasstemp = state.delayV;
    }
    input_sample -= state.aVR[allpasstemp as usize] * const_allpass;
    state.aVR[state.alpVR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpVR -= 1;
    if state.alpVR < 0 || state.alpVR > state.delayV {
        state.alpVR = state.delayV;
    }
    input_sample += (state.aVR[state.alpVR as usize]);
    state.dVR[2] = state.dVR[1];
    state.dVR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpWR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayW {
        allpasstemp = state.delayW;
    }
    input_sample -= state.aWR[allpasstemp as usize] * const_allpass;
    state.aWR[state.alpWR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpWR -= 1;
    if state.alpWR < 0 || state.alpWR > state.delayW {
        state.alpWR = state.delayW;
    }
    input_sample += (state.aWR[state.alpWR as usize]);
    state.dWR[2] = state.dWR[1];
    state.dWR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpXR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayX {
        allpasstemp = state.delayX;
    }
    input_sample -= state.aXR[allpasstemp as usize] * const_allpass;
    state.aXR[state.alpXR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpXR -= 1;
    if state.alpXR < 0 || state.alpXR > state.delayX {
        state.alpXR = state.delayX;
    }
    input_sample += (state.aXR[state.alpXR as usize]);
    state.dXR[2] = state.dXR[1];
    state.dXR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpYR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayY {
        allpasstemp = state.delayY;
    }
    input_sample -= state.aYR[allpasstemp as usize] * const_allpass;
    state.aYR[state.alpYR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpYR -= 1;
    if state.alpYR < 0 || state.alpYR > state.delayY {
        state.alpYR = state.delayY;
    }
    input_sample += (state.aYR[state.alpYR as usize]);
    state.dYR[2] = state.dYR[1];
    state.dYR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.alpZR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayZ {
        allpasstemp = state.delayZ;
    }
    input_sample -= state.aZR[allpasstemp as usize] * const_allpass;
    state.aZR[state.alpZR as usize] = input_sample;
    input_sample *= const_allpass;
    state.alpZR -= 1;
    if state.alpZR < 0 || state.alpZR > state.delayZ {
        state.alpZR = state.delayZ;
    }
    input_sample += (state.aZR[state.alpZR as usize]);
    state.dZR[2] = state.dZR[1];
    state.dZR[1] = input_sample;
    input_sample = dry_sample;
    allpasstemp = state.outAR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayA {
        allpasstemp = state.delayA;
    }
    input_sample -= state.oAR[allpasstemp as usize] * const_allpass;
    state.oAR[state.outAR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outAR -= 1;
    if state.outAR < 0 || state.outAR > state.delayA {
        state.outAR = state.delayA;
    }
    input_sample += (state.oAR[state.outAR as usize]);
    state.dAR[5] = state.dAR[4];
    state.dAR[4] = input_sample;
    input_sample = (state.dAR[1] + state.dAR[2]) / 2.0;
    allpasstemp = state.outBR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayB {
        allpasstemp = state.delayB;
    }
    input_sample -= state.oBR[allpasstemp as usize] * const_allpass;
    state.oBR[state.outBR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outBR -= 1;
    if state.outBR < 0 || state.outBR > state.delayB {
        state.outBR = state.delayB;
    }
    input_sample += (state.oBR[state.outBR as usize]);
    state.dBR[5] = state.dBR[4];
    state.dBR[4] = input_sample;
    input_sample = (state.dBR[1] + state.dBR[2]) / 2.0;
    allpasstemp = state.outCR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayC {
        allpasstemp = state.delayC;
    }
    input_sample -= state.oCR[allpasstemp as usize] * const_allpass;
    state.oCR[state.outCR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outCR -= 1;
    if state.outCR < 0 || state.outCR > state.delayC {
        state.outCR = state.delayC;
    }
    input_sample += (state.oCR[state.outCR as usize]);
    state.dCR[5] = state.dCR[4];
    state.dCR[4] = input_sample;
    input_sample = (state.dCR[1] + state.dCR[2]) / 2.0;
    allpasstemp = state.outDR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayD {
        allpasstemp = state.delayD;
    }
    input_sample -= state.oDR[allpasstemp as usize] * const_allpass;
    state.oDR[state.outDR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outDR -= 1;
    if state.outDR < 0 || state.outDR > state.delayD {
        state.outDR = state.delayD;
    }
    input_sample += (state.oDR[state.outDR as usize]);
    state.dDR[5] = state.dDR[4];
    state.dDR[4] = input_sample;
    input_sample = (state.dDR[1] + state.dDR[2]) / 2.0;
    allpasstemp = state.outER - 1;
    if allpasstemp < 0 || allpasstemp > state.delayE {
        allpasstemp = state.delayE;
    }
    input_sample -= state.oER[allpasstemp as usize] * const_allpass;
    state.oER[state.outER as usize] = input_sample;
    input_sample *= const_allpass;
    state.outER -= 1;
    if state.outER < 0 || state.outER > state.delayE {
        state.outER = state.delayE;
    }
    input_sample += (state.oER[state.outER as usize]);
    state.dER[5] = state.dER[4];
    state.dER[4] = input_sample;
    input_sample = (state.dER[1] + state.dER[2]) / 2.0;
    allpasstemp = state.outFR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayF {
        allpasstemp = state.delayF;
    }
    input_sample -= state.oFR[allpasstemp as usize] * const_allpass;
    state.oFR[state.outFR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outFR -= 1;
    if state.outFR < 0 || state.outFR > state.delayF {
        state.outFR = state.delayF;
    }
    input_sample += (state.oFR[state.outFR as usize]);
    state.dFR[5] = state.dFR[4];
    state.dFR[4] = input_sample;
    input_sample = (state.dFR[1] + state.dFR[2]) / 2.0;
    allpasstemp = state.outGR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayG {
        allpasstemp = state.delayG;
    }
    input_sample -= state.oGR[allpasstemp as usize] * const_allpass;
    state.oGR[state.outGR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outGR -= 1;
    if state.outGR < 0 || state.outGR > state.delayG {
        state.outGR = state.delayG;
    }
    input_sample += (state.oGR[state.outGR as usize]);
    state.dGR[5] = state.dGR[4];
    state.dGR[4] = input_sample;
    input_sample = (state.dGR[1] + state.dGR[2]) / 2.0;
    allpasstemp = state.outHR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayH {
        allpasstemp = state.delayH;
    }
    input_sample -= state.oHR[allpasstemp as usize] * const_allpass;
    state.oHR[state.outHR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outHR -= 1;
    if state.outHR < 0 || state.outHR > state.delayH {
        state.outHR = state.delayH;
    }
    input_sample += (state.oHR[state.outHR as usize]);
    state.dHR[5] = state.dHR[4];
    state.dHR[4] = input_sample;
    input_sample = (state.dHR[1] + state.dHR[2]) / 2.0;
    allpasstemp = state.outIR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayI {
        allpasstemp = state.delayI;
    }
    input_sample -= state.oIR[allpasstemp as usize] * const_allpass;
    state.oIR[state.outIR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outIR -= 1;
    if state.outIR < 0 || state.outIR > state.delayI {
        state.outIR = state.delayI;
    }
    input_sample += (state.oIR[state.outIR as usize]);
    state.dIR[5] = state.dIR[4];
    state.dIR[4] = input_sample;
    input_sample = (state.dIR[1] + state.dIR[2]) / 2.0;
    allpasstemp = state.outJR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayJ {
        allpasstemp = state.delayJ;
    }
    input_sample -= state.oJR[allpasstemp as usize] * const_allpass;
    state.oJR[state.outJR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outJR -= 1;
    if state.outJR < 0 || state.outJR > state.delayJ {
        state.outJR = state.delayJ;
    }
    input_sample += (state.oJR[state.outJR as usize]);
    state.dJR[5] = state.dJR[4];
    state.dJR[4] = input_sample;
    input_sample = (state.dJR[1] + state.dJR[2]) / 2.0;
    allpasstemp = state.outKR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayK {
        allpasstemp = state.delayK;
    }
    input_sample -= state.oKR[allpasstemp as usize] * const_allpass;
    state.oKR[state.outKR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outKR -= 1;
    if state.outKR < 0 || state.outKR > state.delayK {
        state.outKR = state.delayK;
    }
    input_sample += (state.oKR[state.outKR as usize]);
    state.dKR[5] = state.dKR[4];
    state.dKR[4] = input_sample;
    input_sample = (state.dKR[1] + state.dKR[2]) / 2.0;
    allpasstemp = state.outLR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayL {
        allpasstemp = state.delayL;
    }
    input_sample -= state.oLR[allpasstemp as usize] * const_allpass;
    state.oLR[state.outLR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outLR -= 1;
    if state.outLR < 0 || state.outLR > state.delayL {
        state.outLR = state.delayL;
    }
    input_sample += (state.oLR[state.outLR as usize]);
    state.dLR[5] = state.dLR[4];
    state.dLR[4] = input_sample;
    input_sample = (state.dLR[1] + state.dLR[2]) / 2.0;
    allpasstemp = state.outMR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayM {
        allpasstemp = state.delayM;
    }
    input_sample -= state.oMR[allpasstemp as usize] * const_allpass;
    state.oMR[state.outMR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outMR -= 1;
    if state.outMR < 0 || state.outMR > state.delayM {
        state.outMR = state.delayM;
    }
    input_sample += (state.oMR[state.outMR as usize]);
    state.dMR[5] = state.dMR[4];
    state.dMR[4] = input_sample;
    input_sample = (state.dMR[1] + state.dMR[2]) / 2.0;
    allpasstemp = state.outNR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayN {
        allpasstemp = state.delayN;
    }
    input_sample -= state.oNR[allpasstemp as usize] * const_allpass;
    state.oNR[state.outNR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outNR -= 1;
    if state.outNR < 0 || state.outNR > state.delayN {
        state.outNR = state.delayN;
    }
    input_sample += (state.oNR[state.outNR as usize]);
    state.dNR[5] = state.dNR[4];
    state.dNR[4] = input_sample;
    input_sample = (state.dNR[1] + state.dNR[2]) / 2.0;
    allpasstemp = state.outOR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayO {
        allpasstemp = state.delayO;
    }
    input_sample -= state.oOR[allpasstemp as usize] * const_allpass;
    state.oOR[state.outOR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outOR -= 1;
    if state.outOR < 0 || state.outOR > state.delayO {
        state.outOR = state.delayO;
    }
    input_sample += (state.oOR[state.outOR as usize]);
    state.dOR[5] = state.dOR[4];
    state.dOR[4] = input_sample;
    input_sample = (state.dOR[1] + state.dOR[2]) / 2.0;
    allpasstemp = state.outPR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayP {
        allpasstemp = state.delayP;
    }
    input_sample -= state.oPR[allpasstemp as usize] * const_allpass;
    state.oPR[state.outPR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outPR -= 1;
    if state.outPR < 0 || state.outPR > state.delayP {
        state.outPR = state.delayP;
    }
    input_sample += (state.oPR[state.outPR as usize]);
    state.dPR[5] = state.dPR[4];
    state.dPR[4] = input_sample;
    input_sample = (state.dPR[1] + state.dPR[2]) / 2.0;
    allpasstemp = state.outQR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayQ {
        allpasstemp = state.delayQ;
    }
    input_sample -= state.oQR[allpasstemp as usize] * const_allpass;
    state.oQR[state.outQR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outQR -= 1;
    if state.outQR < 0 || state.outQR > state.delayQ {
        state.outQR = state.delayQ;
    }
    input_sample += (state.oQR[state.outQR as usize]);
    state.dQR[5] = state.dQR[4];
    state.dQR[4] = input_sample;
    input_sample = (state.dQR[1] + state.dQR[2]) / 2.0;
    allpasstemp = state.outRR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayR {
        allpasstemp = state.delayR;
    }
    input_sample -= state.oRR[allpasstemp as usize] * const_allpass;
    state.oRR[state.outRR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outRR -= 1;
    if state.outRR < 0 || state.outRR > state.delayR {
        state.outRR = state.delayR;
    }
    input_sample += (state.oRR[state.outRR as usize]);
    state.dRR[5] = state.dRR[4];
    state.dRR[4] = input_sample;
    input_sample = (state.dRR[1] + state.dRR[2]) / 2.0;
    allpasstemp = state.outSR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayS {
        allpasstemp = state.delayS;
    }
    input_sample -= state.oSR[allpasstemp as usize] * const_allpass;
    state.oSR[state.outSR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outSR -= 1;
    if state.outSR < 0 || state.outSR > state.delayS {
        state.outSR = state.delayS;
    }
    input_sample += (state.oSR[state.outSR as usize]);
    state.dSR[5] = state.dSR[4];
    state.dSR[4] = input_sample;
    input_sample = (state.dSR[1] + state.dSR[2]) / 2.0;
    allpasstemp = state.outTR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayT {
        allpasstemp = state.delayT;
    }
    input_sample -= state.oTR[allpasstemp as usize] * const_allpass;
    state.oTR[state.outTR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outTR -= 1;
    if state.outTR < 0 || state.outTR > state.delayT {
        state.outTR = state.delayT;
    }
    input_sample += (state.oTR[state.outTR as usize]);
    state.dTR[5] = state.dTR[4];
    state.dTR[4] = input_sample;
    input_sample = (state.dTR[1] + state.dTR[2]) / 2.0;
    allpasstemp = state.outUR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayU {
        allpasstemp = state.delayU;
    }
    input_sample -= state.oUR[allpasstemp as usize] * const_allpass;
    state.oUR[state.outUR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outUR -= 1;
    if state.outUR < 0 || state.outUR > state.delayU {
        state.outUR = state.delayU;
    }
    input_sample += (state.oUR[state.outUR as usize]);
    state.dUR[5] = state.dUR[4];
    state.dUR[4] = input_sample;
    input_sample = (state.dUR[1] + state.dUR[2]) / 2.0;
    allpasstemp = state.outVR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayV {
        allpasstemp = state.delayV;
    }
    input_sample -= state.oVR[allpasstemp as usize] * const_allpass;
    state.oVR[state.outVR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outVR -= 1;
    if state.outVR < 0 || state.outVR > state.delayV {
        state.outVR = state.delayV;
    }
    input_sample += (state.oVR[state.outVR as usize]);
    state.dVR[5] = state.dVR[4];
    state.dVR[4] = input_sample;
    input_sample = (state.dVR[1] + state.dVR[2]) / 2.0;
    allpasstemp = state.outWR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayW {
        allpasstemp = state.delayW;
    }
    input_sample -= state.oWR[allpasstemp as usize] * const_allpass;
    state.oWR[state.outWR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outWR -= 1;
    if state.outWR < 0 || state.outWR > state.delayW {
        state.outWR = state.delayW;
    }
    input_sample += (state.oWR[state.outWR as usize]);
    state.dWR[5] = state.dWR[4];
    state.dWR[4] = input_sample;
    input_sample = (state.dWR[1] + state.dWR[2]) / 2.0;
    allpasstemp = state.outXR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayX {
        allpasstemp = state.delayX;
    }
    input_sample -= state.oXR[allpasstemp as usize] * const_allpass;
    state.oXR[state.outXR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outXR -= 1;
    if state.outXR < 0 || state.outXR > state.delayX {
        state.outXR = state.delayX;
    }
    input_sample += (state.oXR[state.outXR as usize]);
    state.dXR[5] = state.dXR[4];
    state.dXR[4] = input_sample;
    input_sample = (state.dXR[1] + state.dXR[2]) / 2.0;
    allpasstemp = state.outYR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayY {
        allpasstemp = state.delayY;
    }
    input_sample -= state.oYR[allpasstemp as usize] * const_allpass;
    state.oYR[state.outYR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outYR -= 1;
    if state.outYR < 0 || state.outYR > state.delayY {
        state.outYR = state.delayY;
    }
    input_sample += (state.oYR[state.outYR as usize]);
    state.dYR[5] = state.dYR[4];
    state.dYR[4] = input_sample;
    input_sample = (state.dYR[1] + state.dYR[2]) / 2.0;
    allpasstemp = state.outZR - 1;
    if allpasstemp < 0 || allpasstemp > state.delayZ {
        allpasstemp = state.delayZ;
    }
    input_sample -= state.oZR[allpasstemp as usize] * const_allpass;
    state.oZR[state.outZR as usize] = input_sample;
    input_sample *= const_allpass;
    state.outZR -= 1;
    if state.outZR < 0 || state.outZR > state.delayZ {
        state.outZR = state.delayZ;
    }
    input_sample += (state.oZR[state.outZR as usize]);
    state.dZR[5] = state.dZR[4];
    state.dZR[4] = input_sample;
    input_sample = (state.dBR[4] * dryness);
    input_sample += (state.dCR[4] * dryness);
    input_sample += state.dDR[4];
    input_sample += state.dER[4];
    input_sample += state.dFR[4];
    input_sample += state.dGR[4];
    input_sample += state.dHR[4];
    input_sample += state.dIR[4];
    input_sample += state.dJR[4];
    input_sample += state.dKR[4];
    input_sample += state.dLR[4];
    input_sample += state.dMR[4];
    input_sample += state.dNR[4];
    input_sample += state.dOR[4];
    input_sample += state.dPR[4];
    input_sample += state.dQR[4];
    input_sample += state.dRR[4];
    input_sample += state.dSR[4];
    input_sample += state.dTR[4];
    input_sample += state.dUR[4];
    input_sample += state.dVR[4];
    input_sample += state.dWR[4];
    input_sample += state.dXR[4];
    input_sample += state.dYR[4];
    input_sample += (state.dZR[4] * wetness);
    input_sample += (state.dBR[5] * dryness);
    input_sample += (state.dCR[5] * dryness);
    input_sample += state.dDR[5];
    input_sample += state.dER[5];
    input_sample += state.dFR[5];
    input_sample += state.dGR[5];
    input_sample += state.dHR[5];
    input_sample += state.dIR[5];
    input_sample += state.dJR[5];
    input_sample += state.dKR[5];
    input_sample += state.dLR[5];
    input_sample += state.dMR[5];
    input_sample += state.dNR[5];
    input_sample += state.dOR[5];
    input_sample += state.dPR[5];
    input_sample += state.dQR[5];
    input_sample += state.dRR[5];
    input_sample += state.dSR[5];
    input_sample += state.dTR[5];
    input_sample += state.dUR[5];
    input_sample += state.dVR[5];
    input_sample += state.dWR[5];
    input_sample += state.dXR[5];
    input_sample += state.dYR[5];
    input_sample += (state.dZR[5] * wetness);
    input_sample /= (26.0 + (wetness * 4.0));
    input_sample
}
