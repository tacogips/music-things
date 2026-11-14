//! Copyright (c) 2016 airwindows, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/GuitarConditioner, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Exact, measured max-abs 0.
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, PluginInfo};
pub(crate) const INFO: PluginInfo = PluginInfo { name: "GuitarConditioner", upstream_name: "GuitarConditioner", description: "GuitarConditioner is like a Tube Screamer voicing without the squishiness and indistinctness.", params: &[] };
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(GuitarConditioner::new(sample_rate, seed))
}
struct GuitarConditioner {
    sample_rate: f32,
    seed: u32,
    fpd: [u32; 2],
    flip: bool,
    iir: [[[f64; 2]; 2]; 2],
    last: [[f64; 2]; 2],
}
impl GuitarConditioner {
    fn new(sample_rate: f32, seed: u32) -> Self {
        Self {
            sample_rate,
            seed,
            fpd: [fpd_from_seed(seed, 0), fpd_from_seed(seed, 1)],
            flip: true,
            iir: [[[0.0; 2]; 2]; 2],
            last: [[0.0; 2]; 2],
        }
    }
}
impl AirwindowsEffect for GuitarConditioner {
    fn info(&self) -> &'static PluginInfo {
        &INFO
    }
    fn get_param(&self, _: usize) -> f32 {
        0.0
    }
    fn set_param(&mut self, _: usize, _: f32) {}
    fn param_display(&self, _: usize) -> String {
        String::new()
    }
    fn reset(&mut self) {
        *self = Self::new(self.sample_rate, self.seed);
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let scale = self.sample_rate as f64 / 44100.0;
        let treble_iir = (0.287496 / scale) * 2.0;
        let bass_iir = (0.085184 / scale) * 2.0;
        let thresholds = [0.0081 / scale, 0.0256 / scale];
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(or) {
            let mut samples = [l as f64, r as f64];
            for (ch, sample) in samples.iter_mut().enumerate() {
                if sample.abs() < 1.18e-23 {
                    *sample = self.fpd[ch] as f64 * 1.18e-17;
                }
                let mut bands = [*sample + *sample, *sample];
                let offsets = [
                    ((1.0 - 0.3333333333) + ((1.0 - bands[0].abs()) * -0.3333333333))
                        .clamp(0.0, 1.0),
                    ((1.0 - 0.6666666666) + bands[1].abs() * 0.6666666666).clamp(0.0, 1.0),
                ];
                let lane = if self.flip { 0 } else { 1 };
                for band in 0..2 {
                    let coeff = offsets[band] * if band == 0 { treble_iir } else { bass_iir };
                    let state = &mut self.iir[ch][band][lane];
                    *state = *state * (1.0 - coeff) + bands[band] * coeff;
                    bands[band] -= *state;
                    let delta = bands[band] - self.last[ch][band];
                    if delta > thresholds[band] {
                        bands[band] = self.last[ch][band] + thresholds[band];
                    }
                    if -delta > thresholds[band] {
                        bands[band] = self.last[ch][band] - thresholds[band];
                    }
                    self.last[ch][band] = bands[band];
                }
                *sample = fpd_dither(bands[0] + bands[1], &mut self.fpd[ch]);
            }
            self.flip = !self.flip;
            *outl = samples[0] as f32;
            *outr = samples[1] as f32;
        }
    }
}
