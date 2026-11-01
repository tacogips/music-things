//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Apicolypse, b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm; measured max-abs 0.0.
// Preserve upstream constants and operation order.
#![allow(clippy::approx_constant, clippy::excessive_precision)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Hardns",
        label: "",
        default: 0.7,
    },
    ParamInfo {
        name: "Persnlty",
        label: "",
        default: 0.3333333,
    },
    ParamInfo {
        name: "Drive",
        label: "",
        default: 0.3333333,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
];
pub(super) const INFO: PluginInfo = PluginInfo {
    name: "Apicolypse",
    upstream_name: "Apicolypse",
    description: "Apicolypse is a re-release of my old API-style color adder, exacly as it was.",
    params: PARAMS,
};
pub(super) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Apicolypse::new(rate, seed))
}
struct Apicolypse {
    params: [f32; 4],
    rate: f32,
    seed: u32,
    b: [[f64; 34]; 2],
    last: [f64; 2],
    fpd: [u32; 2],
}
impl Apicolypse {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.7, 0.3333333, 0.3333333, 1.0],
            rate,
            seed,
            b: [[0.0; 34]; 2],
            last: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Apicolypse {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, i: usize) -> f32 {
        self.params.get(i).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, i: usize, v: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(i), sanitize_param(v)) {
            *p = v;
        }
    }
    fn param_display(&self, i: usize) -> String {
        if i >= 4 {
            return String::new();
        }
        let p = self.params[i] as f64;
        float2string((if i == 1 || i == 2 { p * 3.0 } else { p }) as f32)
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, l: &[f32], r: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let threshold = self.params[0] as f64;
        let breakup = (1.0 - threshold / 2.0) * 3.14159265358979;
        let hardness = if threshold < 1.0 {
            1.0 / (1.0 - threshold)
        } else {
            999999999999999999999.0
        };
        let mut sqdrive = self.params[1] as f64 * 3.0;
        if sqdrive > 1.0 {
            sqdrive *= sqdrive;
        }
        sqdrive = sqdrive.sqrt();
        let mut drive = self.params[2] as f64 * 3.0;
        if drive > 1.0 {
            drive *= drive;
        }
        drive *= 1.0 - 0.008 * sqdrive;
        let outlevel = self.params[3] as f64;
        for (((&l, &r), ol), orr) in l.iter().zip(r).zip(ol.iter_mut()).zip(orr.iter_mut()) {
            let mut samples = [l as f64, r as f64];
            for (c, v) in samples.iter_mut().enumerate() {
                if v.abs() < 1.18e-23 {
                    *v = self.fpd[c] as f64 * 1.18e-17;
                }
                *v *= drive;
                if sqdrive > 0.0 {
                    let b = &mut self.b[c];
                    b.copy_within(0..33, 1);
                    b[0] = *v * sqdrive;
                    let v_ref = &mut *v;
                    let mut v = *v_ref;
                    v += b[1] * (0.09299870608542582 - (0.00009582362368873 * b[1].abs()));
                    v -= b[2] * (0.11947847710741009 - (0.00004500891602770 * b[2].abs()));
                    v += b[3] * (0.09071606264761795 + (0.00005639498984741 * b[3].abs()));
                    v -= b[4] * (0.08561982770836980 - (0.00004964855606916 * b[4].abs()));
                    v += b[5] * (0.06440549220820363 + (0.00002428052139507 * b[5].abs()));
                    v -= b[6] * (0.05987991812840746 + (0.00000101867082290 * b[6].abs()));
                    v += b[7] * (0.03980233135839382 + (0.00003312430049041 * b[7].abs()));
                    v -= b[8] * (0.03648402630896925 - (0.00002116186381142 * b[8].abs()));
                    v += b[9] * (0.01826860869525248 + (0.00003115110025396 * b[9].abs()));
                    v -= b[10] * (0.01723968622495364 - (0.00002450634121718 * b[10].abs()));
                    v += b[11] * (0.00187588812316724 + (0.00002838206198968 * b[11].abs()));
                    v -= b[12] * (0.00381796423957237 - (0.00003155815499462 * b[12].abs()));
                    v -= b[13] * (0.00852092214496733 - (0.00001702651162392 * b[13].abs()));
                    v += b[14] * (0.00315560292270588 + (0.00002547861676047 * b[14].abs()));
                    v -= b[15] * (0.01258630914496868 - (0.00004555319243213 * b[15].abs()));
                    v += b[16] * (0.00536435648963575 + (0.00001812393657101 * b[16].abs()));
                    v -= b[17] * (0.01272975658159178 - (0.00004103775306121 * b[17].abs()));
                    v += b[18] * (0.00403818975172755 + (0.00003764615492871 * b[18].abs()));
                    v -= b[19] * (0.01042617366897483 - (0.00003605210426041 * b[19].abs()));
                    v += b[20] * (0.00126599583390057 + (0.00004305458668852 * b[20].abs()));
                    v -= b[21] * (0.00747876207688339 - (0.00003731207018977 * b[21].abs()));
                    v -= b[22] * (0.00149873689175324 - (0.00005086601800791 * b[22].abs()));
                    v -= b[23] * (0.00503221309488033 - (0.00003636086782783 * b[23].abs()));
                    v -= b[24] * (0.00342998224655821 - (0.00004103091180506 * b[24].abs()));
                    v -= b[25] * (0.00355585977903117 - (0.00003698982145400 * b[25].abs()));
                    v -= b[26] * (0.00437201792934817 - (0.00002720235666939 * b[26].abs()));
                    v -= b[27] * (0.00299217874451556 - (0.00004446954727956 * b[27].abs()));
                    v -= b[28] * (0.00457924652487249 - (0.00003859065778860 * b[28].abs()));
                    v -= b[29] * (0.00298182934892027 - (0.00002064710931733 * b[29].abs()));
                    v -= b[30] * (0.00438838441540584 - (0.00005223008424866 * b[30].abs()));
                    v -= b[31] * (0.00323984218794705 - (0.00003397987535887 * b[31].abs()));
                    v -= b[32] * (0.00407693981307314 - (0.00003935772436894 * b[32].abs()));
                    v -= b[33] * (0.00350435348467321 - (0.00005525463935338 * b[33].abs()));
                    *v_ref = v;
                }
                if v.abs() > threshold {
                    let bridge = ((v.abs() - threshold) * hardness).min(breakup).sin() / hardness;
                    *v = if *v > 0.0 {
                        bridge + threshold
                    } else {
                        -(bridge + threshold)
                    };
                }
                let randy = self.fpd[c] as f64 / u32::MAX as f64 * 0.033;
                *v = (*v * (1.0 - randy) + self.last[c] * randy) * outlevel;
                self.last[c] = *v;
                *v = fpd_dither(*v, &mut self.fpd[c]);
            }
            *ol = samples[0] as f32;
            *orr = samples[1] as f32;
        }
    }
}
