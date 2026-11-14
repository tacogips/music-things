//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/BuildATPDF/BuildATPDF.h
//! Revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Exact, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_from_seed, xorshift};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};

const PARAMS: [ParamInfo; 10] = {
    let names = [
        "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth",
        "Tenth",
    ];
    let mut params = [ParamInfo {
        name: "",
        label: " ",
        default: 0.5,
    }; 10];
    let mut i = 0;
    while i < 10 {
        params[i].name = names[i];
        i += 1;
    }
    params
};
const INFO: PluginInfo = PluginInfo {
    name: "BuildATPDF",
    upstream_name: "BuildATPDF",
    description: "BuildATPDF is a dither-making toolkit.",
    params: &PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(BuildATPDF::new(sample_rate, seed))
}
struct BuildATPDF {
    params: [f32; 10],
    sample_rate: f32,
    seed: u32,
    left: [f64; 11],
    right: [f64; 11],
    fpd_l: u32,
    fpd_r: u32,
}
impl BuildATPDF {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.5; 10],
            sample_rate,
            seed,
            left: [0.0; 11],
            right: [0.0; 11],
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
    fn channel(input: f32, buffer: &mut [f64; 11], fpd: &mut u32, factors: &[f64; 10]) -> f32 {
        let mut sample = input as f64;
        if sample.abs() < 1.18e-23 {
            sample = *fpd as f64 * 1.18e-17;
        }
        sample *= 8388608.0;
        buffer.copy_within(0..9, 1);
        buffer[0] = *fpd as f64 / u32::MAX as f64;
        let mut dither = buffer[0] * factors[0];
        for i in 1..10 {
            dither += buffer[i] * factors[i];
        }
        sample += dither;
        sample = sample.floor();
        sample /= 8388608.0;
        xorshift(fpd);
        sample as f32
    }
}
impl AirwindowsEffect for BuildATPDF {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, index: usize) -> f32 {
        self.params.get(index).copied().unwrap_or(0.0)
    }
    fn set_param(&mut self, index: usize, value: f32) {
        if let (Some(param), Some(value)) = (self.params.get_mut(index), sanitize_param(value)) {
            *param = value;
        }
    }
    fn param_display(&self, index: usize) -> String {
        self.params
            .get(index)
            .map_or_else(String::new, |p| float2string(*p * 2.0 - 1.0))
    }
    fn reset(&mut self) {
        let params = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = params;
    }
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let factors = self.params.map(|p| (p * 2.0 - 1.0) as f64);
        let len = in_l.len().min(in_r.len()).min(out_l.len()).min(out_r.len());
        for i in 0..len {
            out_l[i] = Self::channel(in_l[i], &mut self.left, &mut self.fpd_l, &factors);
            out_r[i] = Self::channel(in_r[i], &mut self.right, &mut self.fpd_r, &factors);
        }
    }
}
