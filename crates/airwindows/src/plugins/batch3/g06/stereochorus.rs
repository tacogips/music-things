//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/StereoChorus, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Speed",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Depth",
        label: "",
        default: 0.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "StereoChorus",
    upstream_name: "StereoChorus",
    description: "StereoChorus is a nice basic stereo chorus.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(StereoChorus::new(sample_rate, seed))
}
struct StereoChorus {
    params: [f32; 2],
    sample_rate: f32,
    seed: u32,
    buffers: [Box<[i32]>; 2],
    sweep: [f64; 2],
    count: i32,
    prev: [f64; 2],
    even: [f64; 2],
    odd: [f64; 2],
    flip: bool,
    refs: [[f64; 7]; 2],
    cycle: usize,
    fpd: [u32; 2],
}
impl StereoChorus {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            params: [0.0; 2],
            sample_rate,
            seed,
            buffers: std::array::from_fn(|_| vec![0; 65536].into_boxed_slice()),
            sweep: [std::f64::consts::PI / 2.7, std::f64::consts::PI],
            count: 0,
            prev: [0.0; 2],
            even: [0.0; 2],
            odd: [0.0; 2],
            flip: false,
            refs: [[0.0; 7]; 2],
            cycle: 0,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
        }
    }
}
fn interpolate(r: &mut [f64; 7], v: f64, end: usize) {
    match end {
        4 => {
            r[0] = r[4];
            r[2] = (r[0] + v) / 2.0;
            r[1] = (r[0] + r[2]) / 2.0;
            r[3] = (r[2] + v) / 2.0;
            r[4] = v;
        }
        3 => {
            r[0] = r[3];
            r[2] = (r[0] + r[0] + v) / 3.0;
            r[1] = (r[0] + v + v) / 3.0;
            r[3] = v;
        }
        2 => {
            r[0] = r[2];
            r[1] = (r[0] + v) / 2.0;
            r[2] = v;
        }
        _ => r[0] = v,
    }
}
impl AirwindowsEffect for StereoChorus {
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
        self.params
            .get(i)
            .map_or_else(String::new, |p| float2string(*p))
    }
    fn reset(&mut self) {
        for b in &mut self.buffers {
            b.fill(0);
        }
        self.sweep = [std::f64::consts::PI / 2.7, std::f64::consts::PI];
        self.count = 0;
        self.prev = [0.0; 2];
        self.even = [0.0; 2];
        self.odd = [0.0; 2];
        self.flip = false;
        self.refs = [[0.0; 7]; 2];
        self.cycle = 0;
        self.fpd = [fpd_from_seed(self.seed, 0), fpd_from_seed(self.seed, 1)];
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let end = (scale as usize).clamp(1, 4);
        self.cycle = self.cycle.min(end - 1);
        let speed = (0.32 + (self.params[0] / 6.0) as f64).powf(10.0);
        let depth = (self.params[1] / 60.0) as f64 / speed;
        let tupi = std::f64::consts::PI * 2.0;
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut input = [l as f64, r as f64];
            for (ch, v) in input.iter_mut().enumerate() {
                if v.abs() < 1.18e-23 {
                    *v = self.fpd[ch] as f64 * 1.18e-17;
                }
            }
            self.cycle += 1;
            if self.cycle == end {
                for (ch, v) in input.iter_mut().enumerate() {
                    let mut factor = self.prev[ch] - *v;
                    if self.flip {
                        self.even[ch] += factor;
                        self.odd[ch] -= factor;
                        factor = self.even[ch];
                    } else {
                        self.odd[ch] += factor;
                        self.even[ch] -= factor;
                        factor = self.odd[ch];
                    }
                    self.odd[ch] =
                        (self.odd[ch] - ((self.odd[ch] - self.even[ch]) / 256.0)) / 1.0001;
                    self.even[ch] =
                        (self.even[ch] - ((self.even[ch] - self.odd[ch]) / 256.0)) / 1.0001;
                    self.prev[ch] = *v;
                    *v += factor;
                }
                self.flip = !self.flip;
                if self.count < 1 || self.count > 32760 {
                    self.count = 32760;
                }
                for (ch, v) in input.iter_mut().enumerate() {
                    let base = self.count as usize;
                    let sample = (*v * 8388352.0) as i32;
                    self.buffers[ch][base] = sample;
                    self.buffers[ch][base + 32760] = sample;
                    let offset = depth + (depth * self.sweep[ch].sin());
                    let floor = offset.floor();
                    let idx = base + floor as usize;
                    let b = &self.buffers[ch];
                    let mut temp = (b[idx] as f64 * (1.0 - (offset - floor))) as i32;
                    temp += b[idx + 1];
                    temp += (b[idx + 2] as f64 * (offset - floor)) as i32;
                    temp -= ((b[idx] - b[idx + 1]) - (b[idx + 1] - b[idx + 2])) / 50;
                    self.sweep[ch] += speed;
                    if self.sweep[ch] > tupi {
                        self.sweep[ch] -= tupi;
                    }
                    *v = temp as f64 / 16776704.0;
                    interpolate(&mut self.refs[ch], *v, end);
                }
                self.count -= 1;
                self.cycle = 0;
            }
            *outl = fpd_dither(self.refs[0][self.cycle], &mut self.fpd[0]) as f32;
            *outr = fpd_dither(self.refs[1][self.cycle], &mut self.fpd[1]) as f32;
        }
    }
}
