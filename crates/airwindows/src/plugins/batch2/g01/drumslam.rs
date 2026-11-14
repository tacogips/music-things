//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/DrumSlam, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Drive",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Output",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "DrumSlam",
    upstream_name: "DrumSlam",
    description: "DrumSlam is a heavy-processing tape modeler.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(DrumSlam::new(rate, seed))
}
struct DrumSlam {
    params: [f32; 3],
    rate: f32,
    seed: u32,
    iir: [[[f64; 4]; 2]; 2],
    last: [f64; 2],
    flip: bool,
    fpd: [u32; 2],
}
impl DrumSlam {
    fn new(rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0, 1.0, 1.0],
            rate,
            seed,
            iir: [[[0.0; 4]; 2]; 2],
            last: [0.0; 2],
            flip: true,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn shape_band(mut sample: f64, coefficient: f64, drive: f64) -> f64 {
    sample = sample.clamp(-1.0, 1.0);
    sample -= sample * (sample.abs() * coefficient) * (sample.abs() * coefficient);
    sample * drive
}
#[allow(clippy::approx_constant)] // Upstream uses deliberately rounded constants.
fn shape_mid(mut mid: f64, last: &mut f64, drive: f64) -> f64 {
    mid *= drive;
    let mut skew = mid - *last;
    *last = mid;
    let mut bridge = skew.abs().min(3.1415926).sin();
    skew = if skew > 0.0 {
        bridge * 3.1415926
    } else {
        -bridge * 3.1415926
    };
    skew *= mid;
    skew *= 1.557079633;
    bridge = mid.abs();
    bridge += skew;
    if bridge > 1.57079633 {
        bridge = 1.57079633;
    }
    bridge = bridge.sin();
    bridge *= drive;
    bridge += skew;
    if bridge > 1.57079633 {
        bridge = 1.57079633;
    }
    bridge = bridge.sin();
    if mid > 0.0 {
        bridge
    } else {
        -bridge
    }
}
impl AirwindowsEffect for DrumSlam {
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
        match self.params.get(i) {
            Some(v) => float2string(if i == 0 {
                (*v as f64 * 3.0 + 1.0) as f32
            } else {
                *v
            }),
            None => String::new(),
        }
    }
    fn reset(&mut self) {
        let p = self.params;
        *self = Self::new(self.rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let [a, b, c] = self.params;
        let scale = (1.0 / 44100.0) * self.rate as f64;
        let lo = 0.0819 / scale;
        let hi = 0.377933067 / scale;
        let drive = a as f64 * 3.0 + 1.0;
        let wet = c as f64;
        for (((&l, &r), ol), or) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                let dry = *sample;
                *sample *= drive;
                let state = &mut self.iir[usize::from(!self.flip)][ch];
                state[0] = state[0] * (1.0 - lo) + *sample * lo;
                state[1] = state[1] * (1.0 - lo) + state[0] * lo;
                state[2] = state[2] * (1.0 - hi) + *sample * hi;
                state[3] = state[3] * (1.0 - hi) + state[2] * hi;
                let low = shape_band(state[1], 0.448, drive);
                let mid = shape_mid(state[3] - state[1], &mut self.last[ch], drive);
                let high = shape_band(*sample - state[3], 0.599, drive);
                *sample = ((low + mid + high) / drive) * b as f64;
                if wet != 1.0 {
                    *sample = *sample * wet + dry * (1.0 - wet);
                }
                *sample = fpd_dither(*sample, &mut self.fpd[ch]);
            }
            self.flip = !self.flip;
            *ol = samples[0] as f32;
            *or = samples[1] as f32;
        }
    }
}
