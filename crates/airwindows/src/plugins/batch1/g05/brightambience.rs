//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/BrightAmbience/BrightAmbience.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const TAPS: [&[usize]; 33] = [
    &[
        313, 289, 271, 239, 223, 209, 191, 181, 151, 143, 121, 109, 73, 47, 41, 19, 11,
    ],
    &[
        643, 619, 613, 601, 593, 577, 557, 529, 521, 503, 491, 463, 457, 421, 407, 389, 379, 361,
        341, 331,
    ],
    &[
        1009, 991, 971, 961, 947, 923, 913, 899, 869, 857, 829, 811, 803, 793, 767, 727, 713, 691,
        671, 653,
    ],
    &[
        1373, 1357, 1343, 1331, 1319, 1291, 1279, 1271, 1249, 1241, 1217, 1187, 1147, 1121, 1109,
        1087, 1067, 1051, 1037, 1019,
    ],
    &[
        1777, 1747, 1711, 1693, 1679, 1657, 1639, 1633, 1609, 1579, 1549, 1523, 1513, 1501, 1483,
        1469, 1451, 1439, 1427, 1403,
    ],
    &[
        2153, 2129, 2117, 2083, 2069, 2047, 2039, 2027, 2011, 1969, 1951, 1919, 1909, 1891, 1871,
        1847, 1837, 1819, 1807, 1787,
    ],
    &[
        2533, 2509, 2497, 2479, 2459, 2441, 2431, 2419, 2377, 2353, 2341, 2329, 2299, 2293, 2267,
        2251, 2239, 2227, 2183, 2167,
    ],
    &[
        2963, 2957, 2911, 2897, 2879, 2861, 2837, 2801, 2777, 2767, 2731, 2707, 2683, 2671, 2663,
        2647, 2623, 2603, 2579, 2563,
    ],
    &[
        3299, 3289, 3281, 3253, 3229, 3221, 3191, 3167, 3151, 3139, 3121, 3113, 3103, 3083, 3067,
        3043, 3013, 3007, 2993, 2981,
    ],
    &[
        3677, 3667, 3653, 3631, 3613, 3593, 3581, 3559, 3553, 3523, 3503, 3469, 3457, 3443, 3431,
        3419, 3391, 3377, 3341, 3329,
    ],
    &[
        4033, 4009, 4001, 3979, 3961, 3937, 3919, 3893, 3877, 3851, 3833, 3821, 3799, 3791, 3769,
        3757, 3733, 3721, 3707, 3691,
    ],
    &[
        4399, 4367, 4357, 4343, 4331, 4307, 4289, 4279, 4243, 4223, 4213, 4187, 4181, 4159, 4147,
        4133, 4103, 4097, 4087, 4073,
    ],
    &[
        4813, 4799, 4777, 4757, 4733, 4703, 4679, 4657, 4643, 4631, 4601, 4591, 4577, 4547, 4517,
        4507, 4489, 4471, 4439, 4421,
    ],
    &[
        5179, 5147, 5113, 5101, 5087, 5081, 5059, 5053, 5021, 5009, 4979, 4961, 4939, 4931, 4903,
        4891, 4877, 4871, 4861, 4849,
    ],
    &[
        5611, 5599, 5587, 5569, 5549, 5521, 5503, 5483, 5461, 5429, 5401, 5377, 5353, 5333, 5323,
        5267, 5261, 5237, 5213, 5191,
    ],
    &[
        5959, 5947, 5933, 5921, 5911, 5897, 5879, 5849, 5833, 5813, 5807, 5773, 5743, 5737, 5729,
        5717, 5699, 5669, 5641, 5627,
    ],
    &[
        6353, 6329, 6317, 6301, 6281, 6263, 6253, 6233, 6197, 6163, 6137, 6127, 6109, 6077, 6067,
        6049, 6031, 6011, 6001, 5977,
    ],
    &[
        6707, 6677, 6653, 6641, 6623, 6607, 6581, 6563, 6553, 6539, 6511, 6499, 6487, 6469, 6457,
        6449, 6439, 6413, 6397, 6373,
    ],
    &[
        7069, 7061, 7039, 6997, 6983, 6971, 6953, 6941, 6919, 6913, 6907, 6859, 6847, 6829, 6823,
        6809, 6787, 6763, 6749, 6731,
    ],
    &[
        7481, 7447, 7429, 7409, 7397, 7369, 7363, 7319, 7291, 7277, 7267, 7249, 7243, 7237, 7187,
        7171, 7151, 7117, 7099, 7087,
    ],
    &[
        7879, 7871, 7841, 7831, 7813, 7799, 7783, 7747, 7733, 7709, 7673, 7661, 7633, 7627, 7607,
        7597, 7561, 7543, 7529, 7513,
    ],
    &[
        8287, 8273, 8233, 8221, 8209, 8201, 8173, 8161, 8143, 8117, 8107, 8083, 8069, 8059, 8023,
        8009, 7991, 7943, 7927, 7909,
    ],
    &[
        8669, 8657, 8647, 8623, 8599, 8581, 8569, 8563, 8539, 8521, 8507, 8497, 8473, 8437, 8411,
        8387, 8371, 8363, 8339, 8303,
    ],
    &[
        9089, 9071, 9043, 9019, 9007, 8971, 8963, 8929, 8917, 8893, 8863, 8849, 8837, 8803, 8779,
        8761, 8753, 8731, 8699, 8677,
    ],
    &[
        9433, 9413, 9403, 9391, 9371, 9349, 9323, 9311, 9301, 9277, 9257, 9239, 9227, 9221, 9169,
        9161, 9151, 9137, 9109, 9101,
    ],
    &[
        9851, 9817, 9773, 9763, 9721, 9703, 9697, 9673, 9641, 9631, 9619, 9607, 9581, 9559, 9547,
        9521, 9493, 9487, 9469, 9449,
    ],
    &[
        10321, 10301, 10261, 10253, 10231, 10211, 10189, 10159, 10139, 10099, 10093, 10079, 10039,
        9983, 9973, 9953, 9929, 9911, 9893, 9871,
    ],
    &[
        10669, 10657, 10649, 10637, 10621, 10609, 10603, 10597, 10571, 10561, 10553, 10499, 10481,
        10453, 10439, 10417, 10399, 10391, 10363, 10343,
    ],
    &[
        11099, 11071, 11047, 11033, 11021, 10993, 10973, 10943, 10933, 10919, 10897, 10879, 10867,
        10841, 10783, 10763, 10751, 10733, 10721, 10687,
    ],
    &[
        11467, 11441, 11413, 11399, 11363, 11353, 11329, 11323, 11303, 11293, 11281, 11251, 11231,
        11209, 11201, 11177, 11171, 11153, 11141, 11117,
    ],
    &[
        11897, 11873, 11843, 11819, 11791, 11773, 11731, 11723, 11701, 11687, 11659, 11623, 11611,
        11587, 11573, 11549, 11537, 11527, 11513, 11503,
    ],
    &[
        12281, 12247, 12239, 12203, 12197, 12191, 12163, 12143, 12127, 12109, 12083, 12077, 12059,
        12037, 11989, 11969, 11951, 11933, 11927, 11911,
    ],
    &[
        12679, 12671, 12661, 12613, 12587, 12577, 12563, 12557, 12521, 12499, 12493, 12457, 12427,
        12401, 12373, 12367, 12349, 12337, 12323, 12301,
    ],
];
const PARAMS: [ParamInfo; 3] = [
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Sustain",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Decay",
        label: "",
        default: 0.0,
    },
];
const INFO: PluginInfo = PluginInfo {
    name: "BrightAmbience",
    upstream_name: "BrightAmbience",
    description: "BrightAmbience is a plugin for very bright gated reverbs.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(BrightAmbience::new(rate, seed))
}
struct BrightAmbience {
    params: [f32; 3],
    _rate: f32,
    seed: u32,
    left: Box<[i32]>,
    right: Box<[i32]>,
    count: i32,
    fpd_l: u32,
    fpd_r: u32,
}
impl BrightAmbience {
    fn new(rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0; 3],
            _rate: rate,
            seed,
            left: vec![0; 25361].into_boxed_slice(),
            right: vec![0; 25361].into_boxed_slice(),
            count: 0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
    fn channel(
        input: f64,
        buffer: &mut [i32],
        count: usize,
        cpu: usize,
        decay: f64,
        wet: f64,
    ) -> f64 {
        let sample = (input * 8388352.0) as i32;
        buffer[count] = sample;
        buffer[count + 12680] = sample;
        let mut temp: i32 = 0;
        for group in (0..=cpu).rev() {
            temp = (temp as f64 / decay) as i32;
            for &offset in TAPS[group] {
                temp = temp.wrapping_add(buffer[count + offset]);
            }
        }
        (input * (1.0 - wet)) + ((temp as f64 / (8388352.0 * 4.0)) * wet)
    }
}
impl AirwindowsEffect for BrightAmbience {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(p), Some(v)) = (self.params.get_mut(index), sanitize_param(value)) {
            *p = v;
        }
    }
    fn param_display(&self, index: usize) -> String {
        self.params
            .get(index)
            .map_or_else(String::new, |p| float2string(*p))
    }
    fn reset(&mut self) {
        self.left.fill(0);
        self.right.fill(0);
        self.count = 0;
        self.fpd_l = fpd_from_seed(self.seed, 0);
        self.fpd_r = fpd_from_seed(self.seed, 1);
    }
    fn process(&mut self, l: &[f32], r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let wet = self.params[0] as f64;
        let cpu = (self.params[1] * 32.0) as usize;
        let decay = 1.0 + ((self.params[2] as f64).powf(2.0) / 2.0);
        let n = l.len().min(r.len()).min(out_l.len()).min(out_r.len());
        for i in 0..n {
            let mut left = l[i] as f64;
            let mut right = r[i] as f64;
            if left.abs() < 1.18e-23 {
                left = self.fpd_l as f64 * 1.18e-17;
            }
            if right.abs() < 1.18e-23 {
                right = self.fpd_r as f64 * 1.18e-17;
            }
            if self.count < 0 || self.count > 12680 {
                self.count = 12680;
            }
            let count = self.count as usize;
            left = Self::channel(left, &mut self.left, count, cpu, decay, wet);
            right = Self::channel(right, &mut self.right, count, cpu, decay, wet);
            self.count -= 1;
            out_l[i] = fpd_dither(left, &mut self.fpd_l) as f32;
            out_r[i] = fpd_dither(right, &mut self.fpd_r) as f32;
        }
    }
}
