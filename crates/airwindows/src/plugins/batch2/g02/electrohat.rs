//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/ElectroHat, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "HiHat",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Trim",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Bright",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 0.1,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "ElectroHat",
    upstream_name: "ElectroHat",
    description:
        "ElectroHat is a hi-hat tone generator that uses the original sound as a control voltage.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(ElectroHat::new(sample_rate, seed))
}
struct ElectroHat {
    params: [f32; 5],
    sample_rate: f32,
    seed: u32,
    stored_l: f64,
    stored_r: f64,
    last_l: f64,
    last_r: f64,
    tik: i32,
    lok: i32,
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl ElectroHat {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.5, 1.0, 0.1, 1.0],
            sample_rate,
            seed,
            stored_l: 0.0,
            stored_r: 0.0,
            last_l: 0.0,
            last_r: 0.0,
            tik: 3746926,
            lok: 0,
            flip: true,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for ElectroHat {
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
        match i {
            0 => ["Syn Hat", "Electro", "Dense", "606 St", "808 St", "909 St"]
                [(self.params[0] as f64 * 5.999) as usize]
                .into(),
            1..=4 => float2string(self.params[i]),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e] = self.params;
        let mut mode = (a as f64 * 5.999) as i32 + 1;
        let mut increment = b as f64;
        let mut brighten = c as f64;
        let output = d as f64;
        let wet = e as f64;
        if mode == 4 {
            mode = 1;
            increment = 0.411;
            brighten = 0.87;
        }
        if mode == 5 {
            mode = 2;
            increment = 0.111;
            brighten = 1.0;
        }
        if mode == 6 {
            mode = 2;
            increment = 0.299;
            brighten = 0.359;
        }
        increment *= 0.98;
        increment += 0.01;
        increment += (mode + 1) as f64;
        let fa = increment;
        let fb = fa * increment;
        let fc = fb * increment;
        let fd = fc * increment;
        let fe = fd * increment;
        let ff = fe * increment;
        let (pa, pb, pc, pd, pe, pf) = (
            fa as i32, fb as i32, fc as i32, fd as i32, fe as i32, ff as i32,
        );
        let pg = pf
            .wrapping_mul(pe)
            .wrapping_mul(pd)
            .wrapping_mul(pc)
            .wrapping_mul(pb);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let dry_l = l as f64;
            let dry_r = r as f64;
            let mut l = dry_l.abs() * output;
            let mut r = dry_r.abs() * output;
            if self.flip {
                self.tik += 1;
                self.tik %= pg;
                let mut tok = self.tik.wrapping_mul(self.tik) % pf;
                tok = tok.wrapping_mul(tok) % pe;
                tok = tok.wrapping_mul(tok) % pd;
                tok = tok.wrapping_mul(tok) % pc;
                tok = tok.wrapping_mul(tok) % pb;
                tok = tok.wrapping_mul(tok) % pa;
                l *= tok as f64;
                r *= tok as f64;
                if ((self.lok - tok).abs() < (self.lok + tok).abs() && (mode == 1 || mode == 3))
                    || ((self.lok - tok).abs() > (self.lok + tok).abs() && mode == 2)
                {
                    l *= -(tok as f64);
                    r *= -(tok as f64);
                }
                self.lok = tok;
                let temp = l;
                l -= self.last_l * brighten;
                self.last_l = temp;
                let temp = r;
                r -= self.last_r * brighten;
                self.last_r = temp;
            } else {
                l = self.last_l;
                r = self.last_r;
            }
            if self.sample_rate > 64000.0 {
                self.flip = !self.flip;
            } else {
                self.flip = true;
            }
            let temp = l;
            l += self.stored_l;
            self.stored_l = temp;
            let temp = r;
            r += self.stored_r;
            self.stored_r = temp;
            if wet != 1.0 {
                l = l * wet + dry_l * (1.0 - wet);
                r = r * wet + dry_r * (1.0 - wet);
            }
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
