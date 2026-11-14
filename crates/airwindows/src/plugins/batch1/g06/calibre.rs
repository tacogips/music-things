/* Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
 * Upstream: airwindows/plugins/LinuxVST/src/Calibre/Calibre.h
 * Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
 * Parity: Libm; measured max-abs 0.0.
 */
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
static PARAMS: [ParamInfo; 4] = [
    ParamInfo {
        name: "Hardns",
        label: "",
        default: 0.74,
    },
    ParamInfo {
        name: "Persnlty",
        label: "",
        default: f32::from_bits(0x3eaaaaaa),
    },
    ParamInfo {
        name: "Drive",
        label: "",
        default: f32::from_bits(0x3eaaaaaa),
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
];
static INFO: PluginInfo = PluginInfo {
    name: "Calibre",
    upstream_name: "Calibre",
    description: "Calibre is a re-release of another old Character plugin.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Calibre::new(rate, seed))
}
const COEFF: [(f64, f64, bool, bool); 33] = [
    (0.23505923670562212, 0.00028312859289245, false, false),
    (0.08188436704577637, 0.00008817721351341, false, true),
    (0.05075798481700617, 0.00018817166632483, false, false),
    (0.00455811821873093, 0.00001922902995296, true, false),
    (0.00027610521433660, 0.00013252525469291, false, false),
    (0.03529246280346626, 0.00002772989223299, false, false),
    (0.01784111585586136, 0.00010230276997291, true, true),
    (0.04394950700298298, 0.00005910607126944, false, false),
    (0.01990770780547606, 0.00007640328340556, true, true),
    (0.04073629569741782, 0.00007712327117090, false, false),
    (0.01349648572795252, 0.00005959130575917, true, true),
    (0.03191590248003717, 0.00008418000575151, false, false),
    (0.00348795527924766, 0.00005489156318238, true, true),
    (0.02198496281481767, 0.00008471601187581, false, false),
    (0.00504771152505089, 0.00005525060587917, false, false),
    (0.01391075698598491, 0.00007929630732607, false, false),
    (0.01142762504081717, 0.00005967036737742, false, false),
    (0.00893541815021255, 0.00007535697758141, false, false),
    (0.01459704973464936, 0.00005969199602841, false, false),
    (0.00694755135226282, 0.00006930127097865, false, false),
    (0.01516695630808575, 0.00006365800069826, false, false),
    (0.00705917318113651, 0.00006497209096539, false, false),
    (0.01420501209177591, 0.00006555654576113, false, false),
    (0.00815905656808701, 0.00006105622534761, false, false),
    (0.01274326525552961, 0.00006542652857017, false, false),
    (0.00937146927845488, 0.00006051267868722, false, false),
    (0.01146573981165209, 0.00006381511607749, false, false),
    (0.01021294359409007, 0.00005930397856398, false, false),
    (0.01065217095323532, 0.00006371505438319, false, false),
    (0.01058751196699751, 0.00006042857480233, false, false),
    (0.01026557827762401, 0.00006007776163871, false, false),
    (0.01060929183604604, 0.00006114703012726, false, false),
    (0.01014533525058528, 0.00005963567932887, false, false),
];
struct Calibre {
    params: [f32; 4],
    rate: f32,
    seed: u32,
    fpd: [u32; 2],
    history: [[f64; 34]; 2],
    last: [f64; 2],
}
impl Calibre {
    fn new(rate: f32, seed: u32) -> Self {
        let mut r = AwRand::new(seed);
        replay_fpd_seed_loop(&mut r);
        replay_fpd_seed_loop(&mut r);
        Self {
            params: [
                0.74,
                f32::from_bits(0x3eaaaaaa),
                f32::from_bits(0x3eaaaaaa),
                1.0,
            ],
            rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            history: [[0.0; 34]; 2],
            last: [0.0; 2],
        }
    }
    #[allow(clippy::too_many_arguments)]
    fn sample(
        &mut self,
        x: f32,
        ch: usize,
        sqdrive: f64,
        indrive: f64,
        threshold: f64,
        hardness: f64,
        breakup: f64,
    ) -> f32 {
        let mut x = x as f64;
        if x.abs() < 1.18e-23 {
            x = self.fpd[ch] as f64 * 1.18e-17;
        }
        x *= indrive;
        if sqdrive > 0.0 {
            self.history[ch].copy_within(0..33, 1);
            self.history[ch][0] = x * sqdrive;
            for (i, &(a, b, plus, add)) in COEFF.iter().enumerate() {
                let h = self.history[ch][i + 1];
                let k = if plus {
                    a + b * h.abs()
                } else {
                    a - b * h.abs()
                };
                if add {
                    x += h * k
                } else {
                    x -= h * k
                }
            }
        }
        if x.abs() > threshold {
            let bridge = ((x.abs() - threshold) * hardness).min(breakup).sin() / hardness;
            x = if x > 0.0 {
                bridge + threshold
            } else {
                -(bridge + threshold)
            };
        }
        let r = (self.fpd[ch] as f64 / u32::MAX as f64) * 0.042;
        x = ((x * (1.0 - r)) + (self.last[ch] * r)) * self.params[3] as f64;
        self.last[ch] = x;
        fpd_dither(x, &mut self.fpd[ch]) as f32
    }
}
impl AirwindowsEffect for Calibre {
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
        self.params.get(i).map_or_else(String::new, |v| {
            crate::display::float2string(if i == 1 || i == 2 { *v * 3.0 } else { *v })
        })
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = params;
    }
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let threshold = self.params[0] as f64;
        let breakup = (1.0 - threshold / 2.0) * 3.14159265358979;
        let mut sqdrive = self.params[1] as f64 * 3.0;
        if sqdrive > 1.0 {
            sqdrive *= sqdrive;
        }
        sqdrive = sqdrive.sqrt();
        let mut indrive = self.params[2] as f64 * 3.0;
        if indrive > 1.0 {
            indrive *= indrive;
        }
        indrive *= 1.0 + 0.1935 * sqdrive;
        let hardness = if threshold < 1.0 {
            1.0 / (1.0 - threshold)
        } else {
            999999999999999999999.0
        };
        for (((l, r), a), b) in il.iter().zip(ir).zip(ol).zip(or) {
            *a = self.sample(*l, 0, sqdrive, indrive, threshold, hardness, breakup);
            *b = self.sample(*r, 1, sqdrive, indrive, threshold, hardness, breakup);
        }
    }
}
