//! State for the float path of PocketVerbs.
use crate::dither::fpd_from_seed;
use crate::rand::{replay_fpd_seed_loop, AwRand};

#[allow(non_snake_case)]
pub(super) struct State {
    pub(super) dAR: [f64; 7],
    pub(super) dBR: [f64; 7],
    pub(super) dCR: [f64; 7],
    pub(super) dDR: [f64; 7],
    pub(super) dER: [f64; 7],
    pub(super) dFR: [f64; 7],
    pub(super) dGR: [f64; 7],
    pub(super) dHR: [f64; 7],
    pub(super) dIR: [f64; 7],
    pub(super) dJR: [f64; 7],
    pub(super) dKR: [f64; 7],
    pub(super) dLR: [f64; 7],
    pub(super) dMR: [f64; 7],
    pub(super) dNR: [f64; 7],
    pub(super) dOR: [f64; 7],
    pub(super) dPR: [f64; 7],
    pub(super) dQR: [f64; 7],
    pub(super) dRR: [f64; 7],
    pub(super) dSR: [f64; 7],
    pub(super) dTR: [f64; 7],
    pub(super) dUR: [f64; 7],
    pub(super) dVR: [f64; 7],
    pub(super) dWR: [f64; 7],
    pub(super) dXR: [f64; 7],
    pub(super) dYR: [f64; 7],
    pub(super) dZR: [f64; 7],
    pub(super) aAR: Box<[f64]>,
    pub(super) aBR: Box<[f64]>,
    pub(super) aCR: Box<[f64]>,
    pub(super) aDR: Box<[f64]>,
    pub(super) aER: Box<[f64]>,
    pub(super) aFR: Box<[f64]>,
    pub(super) aGR: Box<[f64]>,
    pub(super) aHR: Box<[f64]>,
    pub(super) aIR: Box<[f64]>,
    pub(super) aJR: Box<[f64]>,
    pub(super) aKR: Box<[f64]>,
    pub(super) aLR: Box<[f64]>,
    pub(super) aMR: Box<[f64]>,
    pub(super) aNR: Box<[f64]>,
    pub(super) aOR: Box<[f64]>,
    pub(super) aPR: Box<[f64]>,
    pub(super) aQR: Box<[f64]>,
    pub(super) aRR: Box<[f64]>,
    pub(super) aSR: Box<[f64]>,
    pub(super) aTR: Box<[f64]>,
    pub(super) aUR: Box<[f64]>,
    pub(super) aVR: Box<[f64]>,
    pub(super) aWR: Box<[f64]>,
    pub(super) aXR: Box<[f64]>,
    pub(super) aYR: Box<[f64]>,
    pub(super) aZR: Box<[f64]>,
    pub(super) oAR: Box<[f64]>,
    pub(super) oBR: Box<[f64]>,
    pub(super) oCR: Box<[f64]>,
    pub(super) oDR: Box<[f64]>,
    pub(super) oER: Box<[f64]>,
    pub(super) oFR: Box<[f64]>,
    pub(super) oGR: Box<[f64]>,
    pub(super) oHR: Box<[f64]>,
    pub(super) oIR: Box<[f64]>,
    pub(super) oJR: Box<[f64]>,
    pub(super) oKR: Box<[f64]>,
    pub(super) oLR: Box<[f64]>,
    pub(super) oMR: Box<[f64]>,
    pub(super) oNR: Box<[f64]>,
    pub(super) oOR: Box<[f64]>,
    pub(super) oPR: Box<[f64]>,
    pub(super) oQR: Box<[f64]>,
    pub(super) oRR: Box<[f64]>,
    pub(super) oSR: Box<[f64]>,
    pub(super) oTR: Box<[f64]>,
    pub(super) oUR: Box<[f64]>,
    pub(super) oVR: Box<[f64]>,
    pub(super) oWR: Box<[f64]>,
    pub(super) oXR: Box<[f64]>,
    pub(super) oYR: Box<[f64]>,
    pub(super) oZR: Box<[f64]>,
    pub(super) dAL: [f64; 7],
    pub(super) dBL: [f64; 7],
    pub(super) dCL: [f64; 7],
    pub(super) dDL: [f64; 7],
    pub(super) dEL: [f64; 7],
    pub(super) dFL: [f64; 7],
    pub(super) dGL: [f64; 7],
    pub(super) dHL: [f64; 7],
    pub(super) dIL: [f64; 7],
    pub(super) dJL: [f64; 7],
    pub(super) dKL: [f64; 7],
    pub(super) dLL: [f64; 7],
    pub(super) dML: [f64; 7],
    pub(super) dNL: [f64; 7],
    pub(super) dOL: [f64; 7],
    pub(super) dPL: [f64; 7],
    pub(super) dQL: [f64; 7],
    pub(super) dRL: [f64; 7],
    pub(super) dSL: [f64; 7],
    pub(super) dTL: [f64; 7],
    pub(super) dUL: [f64; 7],
    pub(super) dVL: [f64; 7],
    pub(super) dWL: [f64; 7],
    pub(super) dXL: [f64; 7],
    pub(super) dYL: [f64; 7],
    pub(super) dZL: [f64; 7],
    pub(super) aAL: Box<[f64]>,
    pub(super) aBL: Box<[f64]>,
    pub(super) aCL: Box<[f64]>,
    pub(super) aDL: Box<[f64]>,
    pub(super) aEL: Box<[f64]>,
    pub(super) aFL: Box<[f64]>,
    pub(super) aGL: Box<[f64]>,
    pub(super) aHL: Box<[f64]>,
    pub(super) aIL: Box<[f64]>,
    pub(super) aJL: Box<[f64]>,
    pub(super) aKL: Box<[f64]>,
    pub(super) aLL: Box<[f64]>,
    pub(super) aML: Box<[f64]>,
    pub(super) aNL: Box<[f64]>,
    pub(super) aOL: Box<[f64]>,
    pub(super) aPL: Box<[f64]>,
    pub(super) aQL: Box<[f64]>,
    pub(super) aRL: Box<[f64]>,
    pub(super) aSL: Box<[f64]>,
    pub(super) aTL: Box<[f64]>,
    pub(super) aUL: Box<[f64]>,
    pub(super) aVL: Box<[f64]>,
    pub(super) aWL: Box<[f64]>,
    pub(super) aXL: Box<[f64]>,
    pub(super) aYL: Box<[f64]>,
    pub(super) aZL: Box<[f64]>,
    pub(super) oAL: Box<[f64]>,
    pub(super) oBL: Box<[f64]>,
    pub(super) oCL: Box<[f64]>,
    pub(super) oDL: Box<[f64]>,
    pub(super) oEL: Box<[f64]>,
    pub(super) oFL: Box<[f64]>,
    pub(super) oGL: Box<[f64]>,
    pub(super) oHL: Box<[f64]>,
    pub(super) oIL: Box<[f64]>,
    pub(super) oJL: Box<[f64]>,
    pub(super) oKL: Box<[f64]>,
    pub(super) oLL: Box<[f64]>,
    pub(super) oML: Box<[f64]>,
    pub(super) oNL: Box<[f64]>,
    pub(super) oOL: Box<[f64]>,
    pub(super) oPL: Box<[f64]>,
    pub(super) oQL: Box<[f64]>,
    pub(super) oRL: Box<[f64]>,
    pub(super) oSL: Box<[f64]>,
    pub(super) oTL: Box<[f64]>,
    pub(super) oUL: Box<[f64]>,
    pub(super) oVL: Box<[f64]>,
    pub(super) oWL: Box<[f64]>,
    pub(super) oXL: Box<[f64]>,
    pub(super) oYL: Box<[f64]>,
    pub(super) oZL: Box<[f64]>,
    pub(super) outAL: i32,
    pub(super) alpAL: i32,
    pub(super) outBL: i32,
    pub(super) alpBL: i32,
    pub(super) outCL: i32,
    pub(super) alpCL: i32,
    pub(super) outDL: i32,
    pub(super) alpDL: i32,
    pub(super) outEL: i32,
    pub(super) alpEL: i32,
    pub(super) outFL: i32,
    pub(super) alpFL: i32,
    pub(super) outGL: i32,
    pub(super) alpGL: i32,
    pub(super) outHL: i32,
    pub(super) alpHL: i32,
    pub(super) outIL: i32,
    pub(super) alpIL: i32,
    pub(super) outJL: i32,
    pub(super) alpJL: i32,
    pub(super) outKL: i32,
    pub(super) alpKL: i32,
    pub(super) outLL: i32,
    pub(super) alpLL: i32,
    pub(super) outML: i32,
    pub(super) alpML: i32,
    pub(super) outNL: i32,
    pub(super) alpNL: i32,
    pub(super) outOL: i32,
    pub(super) alpOL: i32,
    pub(super) outPL: i32,
    pub(super) alpPL: i32,
    pub(super) outQL: i32,
    pub(super) alpQL: i32,
    pub(super) outRL: i32,
    pub(super) alpRL: i32,
    pub(super) outSL: i32,
    pub(super) alpSL: i32,
    pub(super) outTL: i32,
    pub(super) alpTL: i32,
    pub(super) outUL: i32,
    pub(super) alpUL: i32,
    pub(super) outVL: i32,
    pub(super) alpVL: i32,
    pub(super) outWL: i32,
    pub(super) alpWL: i32,
    pub(super) outXL: i32,
    pub(super) alpXL: i32,
    pub(super) outYL: i32,
    pub(super) alpYL: i32,
    pub(super) outZL: i32,
    pub(super) alpZL: i32,
    pub(super) outAR: i32,
    pub(super) alpAR: i32,
    pub(super) maxdelayA: i32,
    pub(super) delayA: i32,
    pub(super) outBR: i32,
    pub(super) alpBR: i32,
    pub(super) maxdelayB: i32,
    pub(super) delayB: i32,
    pub(super) outCR: i32,
    pub(super) alpCR: i32,
    pub(super) maxdelayC: i32,
    pub(super) delayC: i32,
    pub(super) outDR: i32,
    pub(super) alpDR: i32,
    pub(super) maxdelayD: i32,
    pub(super) delayD: i32,
    pub(super) outER: i32,
    pub(super) alpER: i32,
    pub(super) maxdelayE: i32,
    pub(super) delayE: i32,
    pub(super) outFR: i32,
    pub(super) alpFR: i32,
    pub(super) maxdelayF: i32,
    pub(super) delayF: i32,
    pub(super) outGR: i32,
    pub(super) alpGR: i32,
    pub(super) maxdelayG: i32,
    pub(super) delayG: i32,
    pub(super) outHR: i32,
    pub(super) alpHR: i32,
    pub(super) maxdelayH: i32,
    pub(super) delayH: i32,
    pub(super) outIR: i32,
    pub(super) alpIR: i32,
    pub(super) maxdelayI: i32,
    pub(super) delayI: i32,
    pub(super) outJR: i32,
    pub(super) alpJR: i32,
    pub(super) maxdelayJ: i32,
    pub(super) delayJ: i32,
    pub(super) outKR: i32,
    pub(super) alpKR: i32,
    pub(super) maxdelayK: i32,
    pub(super) delayK: i32,
    pub(super) outLR: i32,
    pub(super) alpLR: i32,
    pub(super) maxdelayL: i32,
    pub(super) delayL: i32,
    pub(super) outMR: i32,
    pub(super) alpMR: i32,
    pub(super) maxdelayM: i32,
    pub(super) delayM: i32,
    pub(super) outNR: i32,
    pub(super) alpNR: i32,
    pub(super) maxdelayN: i32,
    pub(super) delayN: i32,
    pub(super) outOR: i32,
    pub(super) alpOR: i32,
    pub(super) maxdelayO: i32,
    pub(super) delayO: i32,
    pub(super) outPR: i32,
    pub(super) alpPR: i32,
    pub(super) maxdelayP: i32,
    pub(super) delayP: i32,
    pub(super) outQR: i32,
    pub(super) alpQR: i32,
    pub(super) maxdelayQ: i32,
    pub(super) delayQ: i32,
    pub(super) outRR: i32,
    pub(super) alpRR: i32,
    pub(super) maxdelayR: i32,
    pub(super) delayR: i32,
    pub(super) outSR: i32,
    pub(super) alpSR: i32,
    pub(super) maxdelayS: i32,
    pub(super) delayS: i32,
    pub(super) outTR: i32,
    pub(super) alpTR: i32,
    pub(super) maxdelayT: i32,
    pub(super) delayT: i32,
    pub(super) outUR: i32,
    pub(super) alpUR: i32,
    pub(super) maxdelayU: i32,
    pub(super) delayU: i32,
    pub(super) outVR: i32,
    pub(super) alpVR: i32,
    pub(super) maxdelayV: i32,
    pub(super) delayV: i32,
    pub(super) outWR: i32,
    pub(super) alpWR: i32,
    pub(super) maxdelayW: i32,
    pub(super) delayW: i32,
    pub(super) outXR: i32,
    pub(super) alpXR: i32,
    pub(super) maxdelayX: i32,
    pub(super) delayX: i32,
    pub(super) outYR: i32,
    pub(super) alpYR: i32,
    pub(super) maxdelayY: i32,
    pub(super) delayY: i32,
    pub(super) outZR: i32,
    pub(super) alpZR: i32,
    pub(super) maxdelayZ: i32,
    pub(super) delayZ: i32,
    pub(super) savedRoomsize: f64,
    pub(super) countdown: i32,
    pub(super) peakL: f64,
    pub(super) peakR: f64,
    pub(super) fpdL: u32,
    pub(super) fpdR: u32,
}
#[allow(non_snake_case)]
impl State {
    pub(super) fn new(seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            dAR: [0.0; 7],
            dBR: [0.0; 7],
            dCR: [0.0; 7],
            dDR: [0.0; 7],
            dER: [0.0; 7],
            dFR: [0.0; 7],
            dGR: [0.0; 7],
            dHR: [0.0; 7],
            dIR: [0.0; 7],
            dJR: [0.0; 7],
            dKR: [0.0; 7],
            dLR: [0.0; 7],
            dMR: [0.0; 7],
            dNR: [0.0; 7],
            dOR: [0.0; 7],
            dPR: [0.0; 7],
            dQR: [0.0; 7],
            dRR: [0.0; 7],
            dSR: [0.0; 7],
            dTR: [0.0; 7],
            dUR: [0.0; 7],
            dVR: [0.0; 7],
            dWR: [0.0; 7],
            dXR: [0.0; 7],
            dYR: [0.0; 7],
            dZR: [0.0; 7],
            aAR: vec![0.0; 15150].into_boxed_slice(),
            aBR: vec![0.0; 14618].into_boxed_slice(),
            aCR: vec![0.0; 14358].into_boxed_slice(),
            aDR: vec![0.0; 13818].into_boxed_slice(),
            aER: vec![0.0; 13562].into_boxed_slice(),
            aFR: vec![0.0; 13046].into_boxed_slice(),
            aGR: vec![0.0; 11966].into_boxed_slice(),
            aHR: vec![0.0; 11130].into_boxed_slice(),
            aIR: vec![0.0; 10598].into_boxed_slice(),
            aJR: vec![0.0; 9810].into_boxed_slice(),
            aKR: vec![0.0; 9522].into_boxed_slice(),
            aLR: vec![0.0; 8982].into_boxed_slice(),
            aMR: vec![0.0; 8786].into_boxed_slice(),
            aNR: vec![0.0; 8462].into_boxed_slice(),
            aOR: vec![0.0; 8310].into_boxed_slice(),
            aPR: vec![0.0; 7982].into_boxed_slice(),
            aQR: vec![0.0; 7322].into_boxed_slice(),
            aRR: vec![0.0; 6818].into_boxed_slice(),
            aSR: vec![0.0; 6506].into_boxed_slice(),
            aTR: vec![0.0; 6002].into_boxed_slice(),
            aUR: vec![0.0; 5838].into_boxed_slice(),
            aVR: vec![0.0; 5502].into_boxed_slice(),
            aWR: vec![0.0; 5010].into_boxed_slice(),
            aXR: vec![0.0; 4850].into_boxed_slice(),
            aYR: vec![0.0; 4296].into_boxed_slice(),
            aZR: vec![0.0; 4180].into_boxed_slice(),
            oAR: vec![0.0; 15150].into_boxed_slice(),
            oBR: vec![0.0; 14618].into_boxed_slice(),
            oCR: vec![0.0; 14358].into_boxed_slice(),
            oDR: vec![0.0; 13818].into_boxed_slice(),
            oER: vec![0.0; 13562].into_boxed_slice(),
            oFR: vec![0.0; 13046].into_boxed_slice(),
            oGR: vec![0.0; 11966].into_boxed_slice(),
            oHR: vec![0.0; 11130].into_boxed_slice(),
            oIR: vec![0.0; 10598].into_boxed_slice(),
            oJR: vec![0.0; 9810].into_boxed_slice(),
            oKR: vec![0.0; 9522].into_boxed_slice(),
            oLR: vec![0.0; 8982].into_boxed_slice(),
            oMR: vec![0.0; 8786].into_boxed_slice(),
            oNR: vec![0.0; 8462].into_boxed_slice(),
            oOR: vec![0.0; 8310].into_boxed_slice(),
            oPR: vec![0.0; 7982].into_boxed_slice(),
            oQR: vec![0.0; 7322].into_boxed_slice(),
            oRR: vec![0.0; 6818].into_boxed_slice(),
            oSR: vec![0.0; 6506].into_boxed_slice(),
            oTR: vec![0.0; 6002].into_boxed_slice(),
            oUR: vec![0.0; 5838].into_boxed_slice(),
            oVR: vec![0.0; 5502].into_boxed_slice(),
            oWR: vec![0.0; 5010].into_boxed_slice(),
            oXR: vec![0.0; 4850].into_boxed_slice(),
            oYR: vec![0.0; 4296].into_boxed_slice(),
            oZR: vec![0.0; 4180].into_boxed_slice(),
            dAL: [0.0; 7],
            dBL: [0.0; 7],
            dCL: [0.0; 7],
            dDL: [0.0; 7],
            dEL: [0.0; 7],
            dFL: [0.0; 7],
            dGL: [0.0; 7],
            dHL: [0.0; 7],
            dIL: [0.0; 7],
            dJL: [0.0; 7],
            dKL: [0.0; 7],
            dLL: [0.0; 7],
            dML: [0.0; 7],
            dNL: [0.0; 7],
            dOL: [0.0; 7],
            dPL: [0.0; 7],
            dQL: [0.0; 7],
            dRL: [0.0; 7],
            dSL: [0.0; 7],
            dTL: [0.0; 7],
            dUL: [0.0; 7],
            dVL: [0.0; 7],
            dWL: [0.0; 7],
            dXL: [0.0; 7],
            dYL: [0.0; 7],
            dZL: [0.0; 7],
            aAL: vec![0.0; 15150].into_boxed_slice(),
            aBL: vec![0.0; 14618].into_boxed_slice(),
            aCL: vec![0.0; 14358].into_boxed_slice(),
            aDL: vec![0.0; 13818].into_boxed_slice(),
            aEL: vec![0.0; 13562].into_boxed_slice(),
            aFL: vec![0.0; 13046].into_boxed_slice(),
            aGL: vec![0.0; 11966].into_boxed_slice(),
            aHL: vec![0.0; 11130].into_boxed_slice(),
            aIL: vec![0.0; 10598].into_boxed_slice(),
            aJL: vec![0.0; 9810].into_boxed_slice(),
            aKL: vec![0.0; 9522].into_boxed_slice(),
            aLL: vec![0.0; 8982].into_boxed_slice(),
            aML: vec![0.0; 8786].into_boxed_slice(),
            aNL: vec![0.0; 8462].into_boxed_slice(),
            aOL: vec![0.0; 8310].into_boxed_slice(),
            aPL: vec![0.0; 7982].into_boxed_slice(),
            aQL: vec![0.0; 7322].into_boxed_slice(),
            aRL: vec![0.0; 6818].into_boxed_slice(),
            aSL: vec![0.0; 6506].into_boxed_slice(),
            aTL: vec![0.0; 6002].into_boxed_slice(),
            aUL: vec![0.0; 5838].into_boxed_slice(),
            aVL: vec![0.0; 5502].into_boxed_slice(),
            aWL: vec![0.0; 5010].into_boxed_slice(),
            aXL: vec![0.0; 4850].into_boxed_slice(),
            aYL: vec![0.0; 4296].into_boxed_slice(),
            aZL: vec![0.0; 4180].into_boxed_slice(),
            oAL: vec![0.0; 15150].into_boxed_slice(),
            oBL: vec![0.0; 14618].into_boxed_slice(),
            oCL: vec![0.0; 14358].into_boxed_slice(),
            oDL: vec![0.0; 13818].into_boxed_slice(),
            oEL: vec![0.0; 13562].into_boxed_slice(),
            oFL: vec![0.0; 13046].into_boxed_slice(),
            oGL: vec![0.0; 11966].into_boxed_slice(),
            oHL: vec![0.0; 11130].into_boxed_slice(),
            oIL: vec![0.0; 10598].into_boxed_slice(),
            oJL: vec![0.0; 9810].into_boxed_slice(),
            oKL: vec![0.0; 9522].into_boxed_slice(),
            oLL: vec![0.0; 8982].into_boxed_slice(),
            oML: vec![0.0; 8786].into_boxed_slice(),
            oNL: vec![0.0; 8462].into_boxed_slice(),
            oOL: vec![0.0; 8310].into_boxed_slice(),
            oPL: vec![0.0; 7982].into_boxed_slice(),
            oQL: vec![0.0; 7322].into_boxed_slice(),
            oRL: vec![0.0; 6818].into_boxed_slice(),
            oSL: vec![0.0; 6506].into_boxed_slice(),
            oTL: vec![0.0; 6002].into_boxed_slice(),
            oUL: vec![0.0; 5838].into_boxed_slice(),
            oVL: vec![0.0; 5502].into_boxed_slice(),
            oWL: vec![0.0; 5010].into_boxed_slice(),
            oXL: vec![0.0; 4850].into_boxed_slice(),
            oYL: vec![0.0; 4296].into_boxed_slice(),
            oZL: vec![0.0; 4180].into_boxed_slice(),
            outAL: 1,
            alpAL: 1,
            outBL: 1,
            alpBL: 1,
            outCL: 1,
            alpCL: 1,
            outDL: 1,
            alpDL: 1,
            outEL: 1,
            alpEL: 1,
            outFL: 1,
            alpFL: 1,
            outGL: 1,
            alpGL: 1,
            outHL: 1,
            alpHL: 1,
            outIL: 1,
            alpIL: 1,
            outJL: 1,
            alpJL: 1,
            outKL: 1,
            alpKL: 1,
            outLL: 1,
            alpLL: 1,
            outML: 1,
            alpML: 1,
            outNL: 1,
            alpNL: 1,
            outOL: 1,
            alpOL: 1,
            outPL: 1,
            alpPL: 1,
            outQL: 1,
            alpQL: 1,
            outRL: 1,
            alpRL: 1,
            outSL: 1,
            alpSL: 1,
            outTL: 1,
            alpTL: 1,
            outUL: 1,
            alpUL: 1,
            outVL: 1,
            alpVL: 1,
            outWL: 1,
            alpWL: 1,
            outXL: 1,
            alpXL: 1,
            outYL: 1,
            alpYL: 1,
            outZL: 1,
            alpZL: 1,
            outAR: 1,
            alpAR: 1,
            maxdelayA: 7573,
            delayA: 4,
            outBR: 1,
            alpBR: 1,
            maxdelayB: 7307,
            delayB: 4,
            outCR: 1,
            alpCR: 1,
            maxdelayC: 7177,
            delayC: 4,
            outDR: 1,
            alpDR: 1,
            maxdelayD: 6907,
            delayD: 4,
            outER: 1,
            alpER: 1,
            maxdelayE: 6779,
            delayE: 4,
            outFR: 1,
            alpFR: 1,
            maxdelayF: 6521,
            delayF: 4,
            outGR: 1,
            alpGR: 1,
            maxdelayG: 5981,
            delayG: 4,
            outHR: 1,
            alpHR: 1,
            maxdelayH: 5563,
            delayH: 4,
            outIR: 1,
            alpIR: 1,
            maxdelayI: 5297,
            delayI: 4,
            outJR: 1,
            alpJR: 1,
            maxdelayJ: 4903,
            delayJ: 4,
            outKR: 1,
            alpKR: 1,
            maxdelayK: 4759,
            delayK: 4,
            outLR: 1,
            alpLR: 1,
            maxdelayL: 4489,
            delayL: 4,
            outMR: 1,
            alpMR: 1,
            maxdelayM: 4391,
            delayM: 4,
            outNR: 1,
            alpNR: 1,
            maxdelayN: 4229,
            delayN: 4,
            outOR: 1,
            alpOR: 1,
            maxdelayO: 4153,
            delayO: 4,
            outPR: 1,
            alpPR: 1,
            maxdelayP: 3989,
            delayP: 4,
            outQR: 1,
            alpQR: 1,
            maxdelayQ: 3659,
            delayQ: 4,
            outRR: 1,
            alpRR: 1,
            maxdelayR: 3407,
            delayR: 4,
            outSR: 1,
            alpSR: 1,
            maxdelayS: 3251,
            delayS: 4,
            outTR: 1,
            alpTR: 1,
            maxdelayT: 2999,
            delayT: 4,
            outUR: 1,
            alpUR: 1,
            maxdelayU: 2917,
            delayU: 4,
            outVR: 1,
            alpVR: 1,
            maxdelayV: 2749,
            delayV: 4,
            outWR: 1,
            alpWR: 1,
            maxdelayW: 2503,
            delayW: 4,
            outXR: 1,
            alpXR: 1,
            maxdelayX: 2423,
            delayX: 4,
            outYR: 1,
            alpYR: 1,
            maxdelayY: 2146,
            delayY: 4,
            outZR: 1,
            alpZR: 1,
            maxdelayZ: 2088,
            delayZ: 4,
            savedRoomsize: -1.0,
            countdown: -1,
            peakL: 1.0,
            peakR: 1.0,
            fpdL: fpd_from_seed(seed, 0),
            fpdR: fpd_from_seed(seed, 1),
        }
    }

    pub(super) fn reset(&mut self, seed: u32) {
        self.dAR.fill(0.0);
        self.dBR.fill(0.0);
        self.dCR.fill(0.0);
        self.dDR.fill(0.0);
        self.dER.fill(0.0);
        self.dFR.fill(0.0);
        self.dGR.fill(0.0);
        self.dHR.fill(0.0);
        self.dIR.fill(0.0);
        self.dJR.fill(0.0);
        self.dKR.fill(0.0);
        self.dLR.fill(0.0);
        self.dMR.fill(0.0);
        self.dNR.fill(0.0);
        self.dOR.fill(0.0);
        self.dPR.fill(0.0);
        self.dQR.fill(0.0);
        self.dRR.fill(0.0);
        self.dSR.fill(0.0);
        self.dTR.fill(0.0);
        self.dUR.fill(0.0);
        self.dVR.fill(0.0);
        self.dWR.fill(0.0);
        self.dXR.fill(0.0);
        self.dYR.fill(0.0);
        self.dZR.fill(0.0);
        self.aAR.fill(0.0);
        self.aBR.fill(0.0);
        self.aCR.fill(0.0);
        self.aDR.fill(0.0);
        self.aER.fill(0.0);
        self.aFR.fill(0.0);
        self.aGR.fill(0.0);
        self.aHR.fill(0.0);
        self.aIR.fill(0.0);
        self.aJR.fill(0.0);
        self.aKR.fill(0.0);
        self.aLR.fill(0.0);
        self.aMR.fill(0.0);
        self.aNR.fill(0.0);
        self.aOR.fill(0.0);
        self.aPR.fill(0.0);
        self.aQR.fill(0.0);
        self.aRR.fill(0.0);
        self.aSR.fill(0.0);
        self.aTR.fill(0.0);
        self.aUR.fill(0.0);
        self.aVR.fill(0.0);
        self.aWR.fill(0.0);
        self.aXR.fill(0.0);
        self.aYR.fill(0.0);
        self.aZR.fill(0.0);
        self.oAR.fill(0.0);
        self.oBR.fill(0.0);
        self.oCR.fill(0.0);
        self.oDR.fill(0.0);
        self.oER.fill(0.0);
        self.oFR.fill(0.0);
        self.oGR.fill(0.0);
        self.oHR.fill(0.0);
        self.oIR.fill(0.0);
        self.oJR.fill(0.0);
        self.oKR.fill(0.0);
        self.oLR.fill(0.0);
        self.oMR.fill(0.0);
        self.oNR.fill(0.0);
        self.oOR.fill(0.0);
        self.oPR.fill(0.0);
        self.oQR.fill(0.0);
        self.oRR.fill(0.0);
        self.oSR.fill(0.0);
        self.oTR.fill(0.0);
        self.oUR.fill(0.0);
        self.oVR.fill(0.0);
        self.oWR.fill(0.0);
        self.oXR.fill(0.0);
        self.oYR.fill(0.0);
        self.oZR.fill(0.0);
        self.dAL.fill(0.0);
        self.dBL.fill(0.0);
        self.dCL.fill(0.0);
        self.dDL.fill(0.0);
        self.dEL.fill(0.0);
        self.dFL.fill(0.0);
        self.dGL.fill(0.0);
        self.dHL.fill(0.0);
        self.dIL.fill(0.0);
        self.dJL.fill(0.0);
        self.dKL.fill(0.0);
        self.dLL.fill(0.0);
        self.dML.fill(0.0);
        self.dNL.fill(0.0);
        self.dOL.fill(0.0);
        self.dPL.fill(0.0);
        self.dQL.fill(0.0);
        self.dRL.fill(0.0);
        self.dSL.fill(0.0);
        self.dTL.fill(0.0);
        self.dUL.fill(0.0);
        self.dVL.fill(0.0);
        self.dWL.fill(0.0);
        self.dXL.fill(0.0);
        self.dYL.fill(0.0);
        self.dZL.fill(0.0);
        self.aAL.fill(0.0);
        self.aBL.fill(0.0);
        self.aCL.fill(0.0);
        self.aDL.fill(0.0);
        self.aEL.fill(0.0);
        self.aFL.fill(0.0);
        self.aGL.fill(0.0);
        self.aHL.fill(0.0);
        self.aIL.fill(0.0);
        self.aJL.fill(0.0);
        self.aKL.fill(0.0);
        self.aLL.fill(0.0);
        self.aML.fill(0.0);
        self.aNL.fill(0.0);
        self.aOL.fill(0.0);
        self.aPL.fill(0.0);
        self.aQL.fill(0.0);
        self.aRL.fill(0.0);
        self.aSL.fill(0.0);
        self.aTL.fill(0.0);
        self.aUL.fill(0.0);
        self.aVL.fill(0.0);
        self.aWL.fill(0.0);
        self.aXL.fill(0.0);
        self.aYL.fill(0.0);
        self.aZL.fill(0.0);
        self.oAL.fill(0.0);
        self.oBL.fill(0.0);
        self.oCL.fill(0.0);
        self.oDL.fill(0.0);
        self.oEL.fill(0.0);
        self.oFL.fill(0.0);
        self.oGL.fill(0.0);
        self.oHL.fill(0.0);
        self.oIL.fill(0.0);
        self.oJL.fill(0.0);
        self.oKL.fill(0.0);
        self.oLL.fill(0.0);
        self.oML.fill(0.0);
        self.oNL.fill(0.0);
        self.oOL.fill(0.0);
        self.oPL.fill(0.0);
        self.oQL.fill(0.0);
        self.oRL.fill(0.0);
        self.oSL.fill(0.0);
        self.oTL.fill(0.0);
        self.oUL.fill(0.0);
        self.oVL.fill(0.0);
        self.oWL.fill(0.0);
        self.oXL.fill(0.0);
        self.oYL.fill(0.0);
        self.oZL.fill(0.0);
        self.outAL = 1;
        self.alpAL = 1;
        self.outBL = 1;
        self.alpBL = 1;
        self.outCL = 1;
        self.alpCL = 1;
        self.outDL = 1;
        self.alpDL = 1;
        self.outEL = 1;
        self.alpEL = 1;
        self.outFL = 1;
        self.alpFL = 1;
        self.outGL = 1;
        self.alpGL = 1;
        self.outHL = 1;
        self.alpHL = 1;
        self.outIL = 1;
        self.alpIL = 1;
        self.outJL = 1;
        self.alpJL = 1;
        self.outKL = 1;
        self.alpKL = 1;
        self.outLL = 1;
        self.alpLL = 1;
        self.outML = 1;
        self.alpML = 1;
        self.outNL = 1;
        self.alpNL = 1;
        self.outOL = 1;
        self.alpOL = 1;
        self.outPL = 1;
        self.alpPL = 1;
        self.outQL = 1;
        self.alpQL = 1;
        self.outRL = 1;
        self.alpRL = 1;
        self.outSL = 1;
        self.alpSL = 1;
        self.outTL = 1;
        self.alpTL = 1;
        self.outUL = 1;
        self.alpUL = 1;
        self.outVL = 1;
        self.alpVL = 1;
        self.outWL = 1;
        self.alpWL = 1;
        self.outXL = 1;
        self.alpXL = 1;
        self.outYL = 1;
        self.alpYL = 1;
        self.outZL = 1;
        self.alpZL = 1;
        self.outAR = 1;
        self.alpAR = 1;
        self.maxdelayA = 7573;
        self.delayA = 4;
        self.outBR = 1;
        self.alpBR = 1;
        self.maxdelayB = 7307;
        self.delayB = 4;
        self.outCR = 1;
        self.alpCR = 1;
        self.maxdelayC = 7177;
        self.delayC = 4;
        self.outDR = 1;
        self.alpDR = 1;
        self.maxdelayD = 6907;
        self.delayD = 4;
        self.outER = 1;
        self.alpER = 1;
        self.maxdelayE = 6779;
        self.delayE = 4;
        self.outFR = 1;
        self.alpFR = 1;
        self.maxdelayF = 6521;
        self.delayF = 4;
        self.outGR = 1;
        self.alpGR = 1;
        self.maxdelayG = 5981;
        self.delayG = 4;
        self.outHR = 1;
        self.alpHR = 1;
        self.maxdelayH = 5563;
        self.delayH = 4;
        self.outIR = 1;
        self.alpIR = 1;
        self.maxdelayI = 5297;
        self.delayI = 4;
        self.outJR = 1;
        self.alpJR = 1;
        self.maxdelayJ = 4903;
        self.delayJ = 4;
        self.outKR = 1;
        self.alpKR = 1;
        self.maxdelayK = 4759;
        self.delayK = 4;
        self.outLR = 1;
        self.alpLR = 1;
        self.maxdelayL = 4489;
        self.delayL = 4;
        self.outMR = 1;
        self.alpMR = 1;
        self.maxdelayM = 4391;
        self.delayM = 4;
        self.outNR = 1;
        self.alpNR = 1;
        self.maxdelayN = 4229;
        self.delayN = 4;
        self.outOR = 1;
        self.alpOR = 1;
        self.maxdelayO = 4153;
        self.delayO = 4;
        self.outPR = 1;
        self.alpPR = 1;
        self.maxdelayP = 3989;
        self.delayP = 4;
        self.outQR = 1;
        self.alpQR = 1;
        self.maxdelayQ = 3659;
        self.delayQ = 4;
        self.outRR = 1;
        self.alpRR = 1;
        self.maxdelayR = 3407;
        self.delayR = 4;
        self.outSR = 1;
        self.alpSR = 1;
        self.maxdelayS = 3251;
        self.delayS = 4;
        self.outTR = 1;
        self.alpTR = 1;
        self.maxdelayT = 2999;
        self.delayT = 4;
        self.outUR = 1;
        self.alpUR = 1;
        self.maxdelayU = 2917;
        self.delayU = 4;
        self.outVR = 1;
        self.alpVR = 1;
        self.maxdelayV = 2749;
        self.delayV = 4;
        self.outWR = 1;
        self.alpWR = 1;
        self.maxdelayW = 2503;
        self.delayW = 4;
        self.outXR = 1;
        self.alpXR = 1;
        self.maxdelayX = 2423;
        self.delayX = 4;
        self.outYR = 1;
        self.alpYR = 1;
        self.maxdelayY = 2146;
        self.delayY = 4;
        self.outZR = 1;
        self.alpZR = 1;
        self.maxdelayZ = 2088;
        self.delayZ = 4;
        self.savedRoomsize = -1.0;
        self.countdown = -1;
        self.peakL = 1.0;
        self.peakR = 1.0;
        self.fpdL = fpd_from_seed(seed, 0);
        self.fpdR = fpd_from_seed(seed, 1);
    }
}
