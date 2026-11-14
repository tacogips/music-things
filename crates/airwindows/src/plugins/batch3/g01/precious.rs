//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Precious, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
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
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "Precious",
    upstream_name: "Precious",
    description: "Precious is a re-release of another old Character plugin.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Precious::new(sample_rate, seed))
}
struct Precious {
    params: [f32; 4],
    _sample_rate: f32,
    seed: u32,
    b: [[f64; 35]; 2],
    last: [f64; 2],
    fpd: [u32; 2],
}
impl Precious {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.7, 0.3333333, 0.3333333, 1.0],
            _sample_rate: sample_rate,
            seed,
            b: [[0.0; 35]; 2],
            last: [0.0; 2],
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
#[allow(clippy::excessive_precision)]
fn convolve(mut input: f64, b: &mut [f64; 35], sqdrive: f64) -> f64 {
    b.copy_within(0..33, 1);
    b[0] = input * sqdrive;
    input += b[1] * (0.59188440274551890 - (0.00008361469668405 * b[1].abs()));
    input -= b[2] * (0.24439750948076133 + (0.00002651678396848 * b[2].abs()));
    input += b[3] * (0.14109876103205621 - (0.00000840487181372 * b[3].abs()));
    input -= b[4] * (0.10053507128157971 + (0.00001768100964598 * b[4].abs()));
    input += b[5] * (0.05859287880626238 - (0.00000361398065989 * b[5].abs()));
    input -= b[6] * (0.04337406889823660 + (0.00000735941182117 * b[6].abs()));
    input += b[7] * (0.01589900680531097 + (0.00000207347387987 * b[7].abs()));
    input -= b[8] * (0.01087234854973281 + (0.00000732123412029 * b[8].abs()));
    input -= b[9] * (0.00845782429679176 - (0.00000133058605071 * b[9].abs()));
    input += b[10] * (0.00662278586618295 - (0.00000424594730611 * b[10].abs()));
    input -= b[11] * (0.02000592193760155 + (0.00000632896879068 * b[11].abs()));
    input += b[12] * (0.01321157777167565 - (0.00001421171592570 * b[12].abs()));
    input -= b[13] * (0.02249955362988238 + (0.00000163937127317 * b[13].abs()));
    input += b[14] * (0.01196492077581504 - (0.00000535385220676 * b[14].abs()));
    input -= b[15] * (0.01905917427000097 + (0.00000121672882030 * b[15].abs()));
    input += b[16] * (0.00761909482108073 - (0.00000326242895115 * b[16].abs()));
    input -= b[17] * (0.01362744780256239 + (0.00000359274216003 * b[17].abs()));
    input += b[18] * (0.00200183122683721 - (0.00000089207452791 * b[18].abs()));
    input -= b[19] * (0.00833042637239315 + (0.00000946767677294 * b[19].abs()));
    input -= b[20] * (0.00258481175207224 - (0.00000087429351464 * b[20].abs()));
    input -= b[21] * (0.00459744479712244 - (0.00000049519758701 * b[21].abs()));
    input -= b[22] * (0.00534277030993820 + (0.00000397547847155 * b[22].abs()));
    input -= b[23] * (0.00272332919605675 + (0.00000040077229097 * b[23].abs()));
    input -= b[24] * (0.00637243782359372 - (0.00000139419072176 * b[24].abs()));
    input -= b[25] * (0.00233001590327504 + (0.00000420129915747 * b[25].abs()));
    input -= b[26] * (0.00623296727793041 + (0.00000019010664856 * b[26].abs()));
    input -= b[27] * (0.00276177096376805 + (0.00000580301901385 * b[27].abs()));
    input -= b[28] * (0.00559184754866264 + (0.00000080597287792 * b[28].abs()));
    input -= b[29] * (0.00343180144395919 - (0.00000243701142085 * b[29].abs()));
    input -= b[30] * (0.00493325428861701 + (0.00000300985740900 * b[30].abs()));
    input -= b[31] * (0.00396140827680823 - (0.00000051459681789 * b[31].abs()));
    input -= b[32] * (0.00448497879902493 + (0.00000744412841743 * b[32].abs()));
    input -= b[33] * (0.00425146888772076 - (0.00000082346016542 * b[33].abs()));
    input
}
impl AirwindowsEffect for Precious {
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
            0 | 3 => float2string(self.params[i]),
            1 | 2 => float2string((self.params[i] as f64 * 3.0) as f32),
            _ => String::new(),
        }
    }
    fn reset(&mut self) {
        self.b = [[0.0; 35]; 2];
        self.last = [0.0; 2];
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    #[allow(clippy::approx_constant)]
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let threshold = self.params[0] as f64;
        let breakup = (1.0 - (threshold / 2.0)) * 3.14159265358979;
        let mut sqdrive = self.params[1] as f64 * 3.0;
        if sqdrive > 1.0 {
            sqdrive *= sqdrive;
        }
        sqdrive = sqdrive.sqrt();
        let mut indrive = self.params[2] as f64 * 3.0;
        if indrive > 1.0 {
            indrive *= indrive;
        }
        indrive *= 1.0 - (0.2095 * sqdrive);
        let hardness = if threshold < 1.0 {
            1.0 / (1.0 - threshold)
        } else {
            999999999999999999999.0
        };
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut outputs = [0.0f32; 2];
            for (ch, sample) in [l, r].into_iter().enumerate() {
                let mut input = sample as f64;
                if input.abs() < 1.18e-23 {
                    input = self.fpd[ch] as f64 * 1.18e-17;
                }
                input *= indrive;
                if sqdrive > 0.0 {
                    input = convolve(input, &mut self.b[ch], sqdrive);
                }
                if input.abs() > threshold {
                    let mut bridge = (input.abs() - threshold) * hardness;
                    if bridge > breakup {
                        bridge = breakup;
                    }
                    bridge = bridge.sin() / hardness;
                    input = if input > 0.0 {
                        bridge + threshold
                    } else {
                        -(bridge + threshold)
                    };
                }
                let randy = (self.fpd[ch] as f64 / u32::MAX as f64) * 0.017;
                input = ((input * (1.0 - randy)) + (self.last[ch] * randy)) * self.params[3] as f64;
                self.last[ch] = input;
                outputs[ch] = fpd_dither(input, &mut self.fpd[ch]) as f32;
            }
            *out_l = outputs[0];
            *out_r = outputs[1];
        }
    }
}
