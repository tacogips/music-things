//! BitShiftGain, upstream plugins/LinuxVST/src/BitShiftGain/BitShiftGain.h
//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream revision: b110fac3991fff7c4655cc2d25d6a3e221f9799c
//! Parity: Exact, measured max-abs 0.

use crate::{
    display::int2string, params::sanitize_param, registry::PluginEntry, AirwindowsEffect,
    ParamInfo, PluginInfo,
};

const PARAMS: &[ParamInfo] = &[ParamInfo {
    name: "BitShift",
    label: "bits",
    default: 0.5,
}];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "BitShiftGain",
    upstream_name: "BitShiftGain",
    description:
        "BitShiftGain is the ‘One Weird Trick’ perfect boost/pad, but in 6db increments ONLY.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(_sample_rate: f32, _seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(BitShiftGain { params: [0.5] })
}
struct BitShiftGain {
    params: [f32; 1],
}
impl AirwindowsEffect for BitShiftGain {
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
        if index == 0 {
            int2string((self.params[0] * 32.0 - 16.0) as i32)
        } else {
            String::new()
        }
    }
    fn reset(&mut self) {}
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]) {
        let bits = (self.params[0] * 32.0 - 16.0) as i32;
        let gain = f64::from_bits(((bits + 1023) as u64) << 52);
        for (((l, r), ol), or) in in_l
            .iter()
            .zip(in_r)
            .zip(out_l.iter_mut())
            .zip(out_r.iter_mut())
        {
            *ol = (f64::from(*l) * gain) as f32;
            *or = (f64::from(*r) * gain) as f32;
        }
    }
}
