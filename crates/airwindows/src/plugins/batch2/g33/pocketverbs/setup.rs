//! Buffer retuning performed once per process block.
use super::state::State;

fn retune(delay: &mut i32, max_delay: i32, roomsize: f64) {
    *delay = (max_delay as f64 * roomsize) as i32;
    let mut count = 2;
    while count <= 70 {
        if *delay != count && *delay % count == 0 {
            *delay += 1;
            count = 2;
        }
        count += 1;
    }
    if *delay > max_delay {
        *delay = max_delay;
    }
}

#[allow(non_snake_case)]
pub(super) fn process(state: &mut State, roomsize: f64) {
    if state.savedRoomsize != roomsize {
        state.savedRoomsize = roomsize;
        state.countdown = 26;
    }
    if state.countdown <= 0 {
        return;
    }
    match state.countdown {
        1 => {
            retune(&mut state.delayA, state.maxdelayA, roomsize);
            state.aAL[state.alpAL as usize..15149].fill(0.0);
            state.oAL[state.outAL as usize..15149].fill(0.0);
            retune(&mut state.delayA, state.maxdelayA, roomsize);
            state.aAR[state.alpAR as usize..15149].fill(0.0);
            state.oAR[state.outAR as usize..15149].fill(0.0);
        }
        2 => {
            retune(&mut state.delayB, state.maxdelayB, roomsize);
            state.aBL[state.alpBL as usize..14617].fill(0.0);
            state.oBL[state.outBL as usize..14617].fill(0.0);
            retune(&mut state.delayB, state.maxdelayB, roomsize);
            state.aBR[state.alpBR as usize..14617].fill(0.0);
            state.oBR[state.outBR as usize..14617].fill(0.0);
        }
        3 => {
            retune(&mut state.delayC, state.maxdelayC, roomsize);
            state.aCL[state.alpCL as usize..14357].fill(0.0);
            state.oCL[state.outCL as usize..14357].fill(0.0);
            retune(&mut state.delayC, state.maxdelayC, roomsize);
            state.aCR[state.alpCR as usize..14357].fill(0.0);
            state.oCR[state.outCR as usize..14357].fill(0.0);
        }
        4 => {
            retune(&mut state.delayD, state.maxdelayD, roomsize);
            state.aDL[state.alpDL as usize..13817].fill(0.0);
            state.oDL[state.outDL as usize..13817].fill(0.0);
            retune(&mut state.delayD, state.maxdelayD, roomsize);
            state.aDR[state.alpDR as usize..13817].fill(0.0);
            state.oDR[state.outDR as usize..13817].fill(0.0);
        }
        5 => {
            retune(&mut state.delayE, state.maxdelayE, roomsize);
            state.aEL[state.alpEL as usize..13561].fill(0.0);
            state.oEL[state.outEL as usize..13561].fill(0.0);
            retune(&mut state.delayE, state.maxdelayE, roomsize);
            state.aER[state.alpER as usize..13561].fill(0.0);
            state.oER[state.outER as usize..13561].fill(0.0);
        }
        6 => {
            retune(&mut state.delayF, state.maxdelayF, roomsize);
            state.aFL[state.alpFL as usize..13045].fill(0.0);
            state.oFL[state.outFL as usize..13045].fill(0.0);
            retune(&mut state.delayF, state.maxdelayF, roomsize);
            state.aFR[state.alpFR as usize..13045].fill(0.0);
            state.oFR[state.outFR as usize..13045].fill(0.0);
        }
        7 => {
            retune(&mut state.delayG, state.maxdelayG, roomsize);
            state.aGL[state.alpGL as usize..11965].fill(0.0);
            state.oGL[state.outGL as usize..11965].fill(0.0);
            retune(&mut state.delayG, state.maxdelayG, roomsize);
            state.aGR[state.alpGR as usize..11965].fill(0.0);
            state.oGR[state.outGR as usize..11965].fill(0.0);
        }
        8 => {
            retune(&mut state.delayH, state.maxdelayH, roomsize);
            state.aHL[state.alpHL as usize..11129].fill(0.0);
            state.oHL[state.outHL as usize..11129].fill(0.0);
            retune(&mut state.delayH, state.maxdelayH, roomsize);
            state.aHR[state.alpHR as usize..11129].fill(0.0);
            state.oHR[state.outHR as usize..11129].fill(0.0);
        }
        9 => {
            retune(&mut state.delayI, state.maxdelayI, roomsize);
            state.aIL[state.alpIL as usize..10597].fill(0.0);
            state.oIL[state.outIL as usize..10597].fill(0.0);
            retune(&mut state.delayI, state.maxdelayI, roomsize);
            state.aIR[state.alpIR as usize..10597].fill(0.0);
            state.oIR[state.outIR as usize..10597].fill(0.0);
        }
        10 => {
            retune(&mut state.delayJ, state.maxdelayJ, roomsize);
            state.aJL[state.alpJL as usize..9809].fill(0.0);
            state.oJL[state.outJL as usize..9809].fill(0.0);
            retune(&mut state.delayJ, state.maxdelayJ, roomsize);
            state.aJR[state.alpJR as usize..9809].fill(0.0);
            state.oJR[state.outJR as usize..9809].fill(0.0);
        }
        11 => {
            retune(&mut state.delayK, state.maxdelayK, roomsize);
            state.aKL[state.alpKL as usize..9521].fill(0.0);
            state.oKL[state.outKL as usize..9521].fill(0.0);
            retune(&mut state.delayK, state.maxdelayK, roomsize);
            state.aKR[state.alpKR as usize..9521].fill(0.0);
            state.oKR[state.outKR as usize..9521].fill(0.0);
        }
        12 => {
            retune(&mut state.delayL, state.maxdelayL, roomsize);
            state.aLL[state.alpLL as usize..8981].fill(0.0);
            state.oLL[state.outLL as usize..8981].fill(0.0);
            retune(&mut state.delayL, state.maxdelayL, roomsize);
            state.aLR[state.alpLR as usize..8981].fill(0.0);
            state.oLR[state.outLR as usize..8981].fill(0.0);
        }
        13 => {
            retune(&mut state.delayM, state.maxdelayM, roomsize);
            state.aML[state.alpML as usize..8785].fill(0.0);
            state.oML[state.outML as usize..8785].fill(0.0);
            retune(&mut state.delayM, state.maxdelayM, roomsize);
            state.aMR[state.alpMR as usize..8785].fill(0.0);
            state.oMR[state.outMR as usize..8785].fill(0.0);
        }
        14 => {
            retune(&mut state.delayN, state.maxdelayN, roomsize);
            state.aNL[state.alpNL as usize..8461].fill(0.0);
            state.oNL[state.outNL as usize..8461].fill(0.0);
            retune(&mut state.delayN, state.maxdelayN, roomsize);
            state.aNR[state.alpNR as usize..8461].fill(0.0);
            state.oNR[state.outNR as usize..8461].fill(0.0);
        }
        15 => {
            retune(&mut state.delayO, state.maxdelayO, roomsize);
            state.aOL[state.alpOL as usize..8309].fill(0.0);
            state.oOL[state.outOL as usize..8309].fill(0.0);
            retune(&mut state.delayO, state.maxdelayO, roomsize);
            state.aOR[state.alpOR as usize..8309].fill(0.0);
            state.oOR[state.outOR as usize..8309].fill(0.0);
        }
        16 => {
            retune(&mut state.delayP, state.maxdelayP, roomsize);
            state.aPL[state.alpPL as usize..7981].fill(0.0);
            state.oPL[state.outPL as usize..7981].fill(0.0);
            retune(&mut state.delayP, state.maxdelayP, roomsize);
            state.aPR[state.alpPR as usize..7981].fill(0.0);
            state.oPR[state.outPR as usize..7981].fill(0.0);
        }
        17 => {
            retune(&mut state.delayQ, state.maxdelayQ, roomsize);
            state.aQL[state.alpQL as usize..7321].fill(0.0);
            state.oQL[state.outQL as usize..7321].fill(0.0);
            retune(&mut state.delayQ, state.maxdelayQ, roomsize);
            state.aQR[state.alpQR as usize..7321].fill(0.0);
            state.oQR[state.outQR as usize..7321].fill(0.0);
        }
        18 => {
            retune(&mut state.delayR, state.maxdelayR, roomsize);
            state.aRL[state.alpRL as usize..6817].fill(0.0);
            state.oRL[state.outRL as usize..6817].fill(0.0);
            retune(&mut state.delayR, state.maxdelayR, roomsize);
            state.aRR[state.alpRR as usize..6817].fill(0.0);
            state.oRR[state.outRR as usize..6817].fill(0.0);
        }
        19 => {
            retune(&mut state.delayS, state.maxdelayS, roomsize);
            state.aSL[state.alpSL as usize..6505].fill(0.0);
            state.oSL[state.outSL as usize..6505].fill(0.0);
            retune(&mut state.delayS, state.maxdelayS, roomsize);
            state.aSR[state.alpSR as usize..6505].fill(0.0);
            state.oSR[state.outSR as usize..6505].fill(0.0);
        }
        20 => {
            retune(&mut state.delayT, state.maxdelayT, roomsize);
            state.aTL[state.alpTL as usize..6001].fill(0.0);
            state.oTL[state.outTL as usize..6001].fill(0.0);
            retune(&mut state.delayT, state.maxdelayT, roomsize);
            state.aTR[state.alpTR as usize..6001].fill(0.0);
            state.oTR[state.outTR as usize..6001].fill(0.0);
        }
        21 => {
            retune(&mut state.delayU, state.maxdelayU, roomsize);
            state.aUL[state.alpUL as usize..5837].fill(0.0);
            state.oUL[state.outUL as usize..5837].fill(0.0);
            retune(&mut state.delayU, state.maxdelayU, roomsize);
            state.aUR[state.alpUR as usize..5837].fill(0.0);
            state.oUR[state.outUR as usize..5837].fill(0.0);
        }
        22 => {
            retune(&mut state.delayV, state.maxdelayV, roomsize);
            state.aVL[state.alpVL as usize..5501].fill(0.0);
            state.oVL[state.outVL as usize..5501].fill(0.0);
            retune(&mut state.delayV, state.maxdelayV, roomsize);
            state.aVR[state.alpVR as usize..5501].fill(0.0);
            state.oVR[state.outVR as usize..5501].fill(0.0);
        }
        23 => {
            retune(&mut state.delayW, state.maxdelayW, roomsize);
            state.aWL[state.alpWL as usize..5009].fill(0.0);
            state.oWL[state.outWL as usize..5009].fill(0.0);
            retune(&mut state.delayW, state.maxdelayW, roomsize);
            state.aWR[state.alpWR as usize..5009].fill(0.0);
            state.oWR[state.outWR as usize..5009].fill(0.0);
        }
        24 => {
            retune(&mut state.delayX, state.maxdelayX, roomsize);
            state.aXL[state.alpXL as usize..4849].fill(0.0);
            state.oXL[state.outXL as usize..4849].fill(0.0);
            retune(&mut state.delayX, state.maxdelayX, roomsize);
            state.aXR[state.alpXR as usize..4849].fill(0.0);
            state.oXR[state.outXR as usize..4849].fill(0.0);
        }
        25 => {
            retune(&mut state.delayY, state.maxdelayY, roomsize);
            state.aYL[state.alpYL as usize..4295].fill(0.0);
            state.oYL[state.outYL as usize..4295].fill(0.0);
            retune(&mut state.delayY, state.maxdelayY, roomsize);
            state.aYR[state.alpYR as usize..4295].fill(0.0);
            state.oYR[state.outYR as usize..4295].fill(0.0);
        }
        26 => {
            retune(&mut state.delayZ, state.maxdelayZ, roomsize);
            state.aZL[state.alpZL as usize..4179].fill(0.0);
            state.oZL[state.outZL as usize..4179].fill(0.0);
            retune(&mut state.delayZ, state.maxdelayZ, roomsize);
            state.aZR[state.alpZR as usize..4179].fill(0.0);
            state.oZR[state.outZR as usize..4179].fill(0.0);
        }
        _ => {}
    }
    state.countdown -= 1;
}
