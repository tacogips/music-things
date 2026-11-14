//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Elation, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
#![allow(clippy::excessive_precision)]
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
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
    name: "Elation",
    upstream_name: "Elation",
    description: "Elation is a re-release of another old Character plugin.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Elation::new(sample_rate, seed))
}
struct Channel {
    history: [f64; 34],
    comp: [[f64; 2]; 2],
    previous: [f64; 2],
    last: f64,
}
impl Default for Channel {
    fn default() -> Self {
        Self {
            history: [0.0; 34],
            comp: [[1.0; 2]; 2],
            previous: [0.0; 2],
            last: 0.0,
        }
    }
}
impl Channel {
    fn compress(
        &mut self,
        mut input: f64,
        stage: usize,
        threshold: f64,
        recovery: f64,
        flip: bool,
    ) -> f64 {
        let ab = input.abs();
        let mut thresh = if ab > self.previous[stage] {
            self.previous[stage]
        } else {
            ab
        };
        if thresh > 2.0 {
            self.comp[stage][0] = (self.comp[stage][0] + 1.0 / thresh) / 2.0;
            self.comp[stage][1] = (self.comp[stage][1] + 1.0 / thresh) / 2.0;
        }
        let c = &mut self.comp[stage][if flip { 0 } else { 1 }];
        input *= *c;
        thresh *= *c;
        if thresh > threshold {
            *c = (*c + *c + threshold / thresh) / 3.0;
        } else if *c < 1.0 {
            *c += ((1.0 - *c) * recovery).powf(7.0);
        }
        self.previous[stage] = ab;
        input
    }
    fn convolve(&mut self, mut input: f64, drive: f64) -> f64 {
        self.history.copy_within(0..33, 1);
        self.history[0] = input * drive;
        input -=
            self.history[1] * (0.25867935358656502 - (0.00045755657070112 * self.history[1].abs()));
        input +=
            self.history[2] * (0.11509367290253694 - (0.00017494270657228 * self.history[2].abs()));
        input -=
            self.history[3] * (0.06709853575891785 - (0.00058913102597723 * self.history[3].abs()));
        input +=
            self.history[4] * (0.01871006356851681 - (0.00003387358004645 * self.history[4].abs()));
        input -=
            self.history[5] * (0.00794797957360465 - (0.00044224784691203 * self.history[5].abs()));
        input -=
            self.history[6] * (0.01956921817394220 - (0.00006718936750076 * self.history[6].abs()));
        input +=
            self.history[7] * (0.01682120257195205 + (0.00032857446292230 * self.history[7].abs()));
        input -=
            self.history[8] * (0.03401069039824205 - (0.00013634182872897 * self.history[8].abs()));
        input +=
            self.history[9] * (0.02369950268232634 + (0.00023112685751657 * self.history[9].abs()));
        input -= self.history[10]
            * (0.03477071178117132 - (0.00018029792231600 * self.history[10].abs()));
        input += self.history[11]
            * (0.02024369717958201 + (0.00017337813374202 * self.history[11].abs()));
        input -= self.history[12]
            * (0.02819087729102172 - (0.00021438538665420 * self.history[12].abs()));
        input += self.history[13]
            * (0.01147946743141303 + (0.00014424066034649 * self.history[13].abs()));
        input -= self.history[14]
            * (0.01894777011468867 - (0.00021549146262408 * self.history[14].abs()));
        input += self.history[15]
            * (0.00301370330346873 + (0.00013527460148394 * self.history[15].abs()));
        input -= self.history[16]
            * (0.01067147835815486 - (0.00020960689910868 * self.history[16].abs()));
        input -= self.history[17]
            * (0.00402715397506384 - (0.00014421582712470 * self.history[17].abs()));
        input -= self.history[18]
            * (0.00502221703392005 - (0.00019805767015024 * self.history[18].abs()));
        input -= self.history[19]
            * (0.00808788533308497 - (0.00016095444141931 * self.history[19].abs()));
        input -= self.history[20]
            * (0.00232696588842683 - (0.00018384470981829 * self.history[20].abs()));
        input -= self.history[21]
            * (0.00943950821324531 - (0.00017098987347593 * self.history[21].abs()));
        input -= self.history[22]
            * (0.00193709517200834 - (0.00018151995939591 * self.history[22].abs()));
        input -= self.history[23]
            * (0.00899713952612659 - (0.00017385835059948 * self.history[23].abs()));
        input -= self.history[24]
            * (0.00280584331659089 - (0.00017742164162470 * self.history[24].abs()));
        input -= self.history[25]
            * (0.00780381001954970 - (0.00018002500755708 * self.history[25].abs()));
        input -= self.history[26]
            * (0.00400370310490333 - (0.00017471691087957 * self.history[26].abs()));
        input -= self.history[27]
            * (0.00661527728186928 - (0.00018137323370347 * self.history[27].abs()));
        input -= self.history[28]
            * (0.00496545526864518 - (0.00017681872601767 * self.history[28].abs()));
        input -= self.history[29]
            * (0.00580728820997532 - (0.00018186220389790 * self.history[29].abs()));
        input -= self.history[30]
            * (0.00549309984725666 - (0.00017722985399075 * self.history[30].abs()));
        input -= self.history[31]
            * (0.00542194777529239 - (0.00018486900185338 * self.history[31].abs()));
        input -= self.history[32]
            * (0.00565992080998939 - (0.00018005824393118 * self.history[32].abs()));
        input -= self.history[33]
            * (0.00532121562846656 - (0.00018643189636216 * self.history[33].abs()));
        input
    }
}
struct Elation {
    params: [f32; 4],
    sample_rate: f32,
    seed: u32,
    left: Channel,
    right: Channel,
    flip: bool,
    fpd_l: u32,
    fpd_r: u32,
}
impl Elation {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 0.3333333, 0.3333333, 1.0],
            sample_rate,
            seed,
            left: Channel::default(),
            right: Channel::default(),
            flip: false,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Elation {
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
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d] = self.params;
        let wet = a as f64;
        let mut sqdrive = b as f64 * 3.0;
        if sqdrive > 1.0 {
            sqdrive *= sqdrive;
        }
        sqdrive = sqdrive.sqrt();
        let mut drive = c as f64 * 3.0;
        let threshold = (6.0 - drive) / 6.0;
        let recovery = drive / 32.0;
        if drive > 1.0 {
            drive *= drive;
        }
        drive *= 1.0 + 0.226 * sqdrive;
        let outlevel = d as f64;
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.fpd_l as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.fpd_r as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            l *= drive;
            r *= drive;
            l = self.left.compress(l, 0, threshold, recovery, self.flip);
            r = self.right.compress(r, 0, threshold, recovery, self.flip);
            if sqdrive > 0.0 {
                l = self.left.convolve(l, sqdrive);
                r = self.right.convolve(r, sqdrive);
            }
            l = self.left.compress(l, 1, threshold, recovery, self.flip);
            r = self.right.compress(r, 1, threshold, recovery, self.flip);
            self.flip = !self.flip;
            let random = self.fpd_l as f64 / u32::MAX as f64 * 0.054;
            let result = ((l * (1.0 - random) + self.left.last * random) * wet
                + dry_l * (1.0 - wet))
                * outlevel;
            self.left.last = l;
            l = result;
            let random = self.fpd_r as f64 / u32::MAX as f64 * 0.054;
            let result = ((r * (1.0 - random) + self.right.last * random) * wet
                + dry_r * (1.0 - wet))
                * outlevel;
            self.right.last = r;
            r = result;
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
