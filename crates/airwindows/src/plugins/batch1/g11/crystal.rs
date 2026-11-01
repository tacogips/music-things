//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Crystal, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm; measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Hardnss",
        label: "",
        default: 0.7,
    },
    ParamInfo {
        name: "Personl",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Drive",
        label: "",
        default: 0.333,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Crystal",
    upstream_name: "Crystal",
    description: "Crystal is a tone shaper and buss soft-clipper.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Crystal::new(sample_rate, seed))
}
struct Crystal {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    b: [[f64; 35]; 2],
    last: [f64; 2],
    fpd: [u32; 2],
}
impl Crystal {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.7, 1.0, 0.333, 1.0],
            sample_rate,
            seed,
            b: [[0.0; 35]; 2],
            last: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
impl AirwindowsEffect for Crystal {
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
            0 | 1 | 3 => float2string(self.params[i]),
            2 => float2string((self.params[2] as f64 * 3.0) as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    #[allow(clippy::approx_constant, clippy::excessive_precision)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let threshold = self.params[0] as f64;
        let breakup = (1.0 - threshold / 2.0) * 3.14159265358979;
        let hardness = if threshold < 1.0 {
            1.0 / (1.0 - threshold)
        } else {
            999999999999999999999.0
        };
        let mut sqdrive = self.params[1] as f64;
        if sqdrive > 1.0 {
            sqdrive *= sqdrive;
        }
        sqdrive = sqdrive.sqrt();
        let mut indrive = self.params[2] as f64 * 3.0;
        if indrive > 1.0 {
            indrive *= indrive;
        }
        indrive *= 1.0 - 0.1695 * sqdrive;
        let outlevel = self.params[3] as f64;
        const COEFF: [(f64, f64); 23] = [
            (0.38856694371895023, 0.14001177830115491),
            (0.17469488984546111, 0.05204541941091459),
            (0.11643521461774288, -0.01193121216518472),
            (0.08874416268268183, -0.05867502375036486),
            (0.07222999223073785, -0.08519974113692971),
            (0.06103207678880003, -0.09230674983449150),
            (0.05277389277465404, -0.08487342372497046),
            (0.04631144388636078, -0.06976851898821038),
            (0.04102721072495113, -0.05337974329110802),
            (0.03656047655964371, -0.03990914278458497),
            (0.03268677450573373, -0.03090433934018759),
            (0.02926012259262895, -0.02585223214266682),
            (0.02618257163789973, -0.02326667039588473),
            (0.02338568277879992, -0.02167067760829789),
            (0.02082142324645262, -0.02013392273267951),
            (0.01845525966656259, -0.01833038930966512),
            (0.01626113504980445, -0.01631893218593511),
            (0.01422084088669267, -0.01427828125219885),
            (0.01231993595709338, -0.01233991521342998),
            (0.01054774630451013, -0.01054774630542346),
            (0.00889548162355088, -0.00889548162263755),
            (0.00735749099304526, -0.00735749099395860),
            (0.00592812350468000, -0.00592812350376666),
        ];
        for i in 0..il.len().min(ir.len()).min(ol.len()).min(or.len()) {
            let input = [il[i], ir[i]];
            let mut output = [0.0_f32; 2];
            for c in 0..2 {
                let mut x = input[c] as f64;
                if x.abs() < 1.18e-23 {
                    x = self.fpd[c] as f64 * 1.18e-17;
                }
                x *= indrive;
                if sqdrive > 0.0 {
                    for j in (1..=23).rev() {
                        self.b[c][j] = self.b[c][j - 1];
                    }
                    self.b[c][0] = x * sqdrive;
                    for (j, &(a, b)) in COEFF.iter().enumerate() {
                        let v = self.b[c][j + 1];
                        let term = v * (a + b * v.abs());
                        if j % 2 == 0 {
                            x += term;
                        } else {
                            x -= term;
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
                let randy = (self.fpd[c] as f64 / u32::MAX as f64) * 0.022;
                let bridge = (x * (1.0 - randy) + self.last[c] * randy) * outlevel;
                self.last[c] = x;
                x = bridge;
                output[c] = fpd_dither(x, &mut self.fpd[c]) as f32;
            }
            ol[i] = output[0];
            or[i] = output[1];
        }
    }
}
