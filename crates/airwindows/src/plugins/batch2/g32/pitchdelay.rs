//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/PitchDelay, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Time",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "Regen",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Freq",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Reso",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Pitch",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Dry/Wet",
        label: "",
        default: 1.0,
    },
];
pub(crate) const INFO: PluginInfo = PluginInfo {
    name: "PitchDelay",
    upstream_name: "PitchDelay",
    description: "PitchDelay is TapeDelay2 but with pitch shift instead of flutter.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(PitchDelay::new(sample_rate, seed))
}
struct Lane {
    buffer: Box<[f64]>,
    previous: f64,
    regen: f64,
    delay: f64,
    sweep: f64,
    regen_filter: [f64; 9],
    out_filter: [f64; 9],
    refs: [f64; 10],
    fpd: u32,
}
impl Lane {
    fn new(seed: u32, lane: u32) -> Self {
        Self {
            buffer: vec![0.0; 88211].into_boxed_slice(),
            previous: 0.0,
            regen: 0.0,
            delay: 0.0,
            sweep: 0.0,
            regen_filter: [0.0; 9],
            out_filter: [0.0; 9],
            refs: [0.0; 10],
            fpd: fpd_from_seed(seed, lane),
        }
    }
    fn reset(&mut self, seed: u32, lane: u32) {
        self.buffer.fill(0.0);
        self.previous = 0.0;
        self.regen = 0.0;
        self.delay = 0.0;
        self.sweep = 0.0;
        self.regen_filter.fill(0.0);
        self.out_filter.fill(0.0);
        self.refs.fill(0.0);
        self.fpd = fpd_from_seed(seed, lane);
    }
    #[allow(clippy::excessive_precision)]
    fn configure(&mut self, freq: f64, resonance: f64) {
        self.regen_filter[0] = freq;
        self.regen_filter[1] = resonance;
        let k = (std::f64::consts::PI * freq).tan();
        let norm = 1.0 / (1.0 + k / resonance + k * k);
        self.regen_filter[2] = k / resonance * norm;
        self.regen_filter[4] = -self.regen_filter[2];
        self.regen_filter[5] = 2.0 * (k * k - 1.0) * norm;
        self.regen_filter[6] = (1.0 - k / resonance + k * k) * norm;
        self.out_filter[0] = freq;
        self.out_filter[1] = resonance * 1.618033988749894848204586;
        let k = (std::f64::consts::PI * self.out_filter[0]).tan();
        let norm = 1.0 / (1.0 + k / self.out_filter[1] + k * k);
        self.out_filter[2] = k / self.out_filter[1] * norm;
        self.out_filter[4] = -self.out_filter[2];
        self.out_filter[5] = 2.0 * (k * k - 1.0) * norm;
        self.out_filter[6] = (1.0 - k / self.out_filter[1] + k * k) * norm;
    }
    #[allow(clippy::approx_constant)]
    fn step(
        &mut self,
        input: f64,
        feedback: f64,
        speed: f64,
        vib_speed: f64,
        vib_depth: f64,
    ) -> f64 {
        let mut pos = self.delay.floor() as i32;
        let mut sample = input + (self.regen * feedback);
        let temp = (sample * self.regen_filter[2]) + self.regen_filter[7];
        self.regen_filter[7] = -(temp * self.regen_filter[5]) + self.regen_filter[8];
        self.regen_filter[8] = (sample * self.regen_filter[4]) - (temp * self.regen_filter[6]);
        sample = temp;
        self.delay -= speed;
        if self.delay < 0.0 {
            self.delay += 88200.0;
        }
        let increment = (sample - self.previous) / speed;
        self.buffer[pos as usize] = self.previous;
        while pos != self.delay.floor() as i32 {
            self.buffer[pos as usize] = self.previous;
            self.previous += increment;
            pos -= 1;
            if pos < 0 {
                pos += 88200;
            }
        }
        self.previous = sample;
        self.sweep += 0.0001 * vib_speed;
        if self.sweep < 0.0 {
            self.sweep += 6.283185307179586;
        }
        if self.sweep > 6.283185307179586 {
            self.sweep -= 6.283185307179586;
        }
        let mut offset = self.sweep + std::f64::consts::PI;
        if offset > 6.283185307179586 {
            offset -= 6.283185307179586;
        }
        let mut tap_a = self.delay - (self.sweep * vib_depth);
        if tap_a < 0.0 {
            tap_a += 88200.0;
        }
        let mut tap_b = self.delay - (offset * vib_depth);
        if tap_b < 0.0 {
            tap_b += 88200.0;
        }
        let amp_a = ((self.sweep + (std::f64::consts::PI * 1.5)).sin() + 1.0) * 0.25;
        let amp_b = ((offset + (std::f64::consts::PI * 1.5)).sin() + 1.0) * 0.25;
        sample = self.buffer[tap_a.floor() as usize] * amp_a;
        sample += self.buffer[tap_b.floor() as usize] * amp_b;
        self.regen = sample.sin();
        let temp = (sample * self.out_filter[2]) + self.out_filter[7];
        self.out_filter[7] = -(temp * self.out_filter[5]) + self.out_filter[8];
        self.out_filter[8] = (sample * self.out_filter[4]) - (temp * self.out_filter[6]);
        temp
    }
    fn interpolate(&mut self, sample: f64, cycle_end: usize) {
        match cycle_end {
            4 => {
                self.refs[0] = self.refs[4];
                self.refs[2] = (self.refs[0] + sample) / 2.0;
                self.refs[1] = (self.refs[0] + self.refs[2]) / 2.0;
                self.refs[3] = (self.refs[2] + sample) / 2.0;
                self.refs[4] = sample;
            }
            3 => {
                self.refs[0] = self.refs[3];
                self.refs[2] = (self.refs[0] + self.refs[0] + sample) / 3.0;
                self.refs[1] = (self.refs[0] + sample + sample) / 3.0;
                self.refs[3] = sample;
            }
            2 => {
                self.refs[0] = self.refs[2];
                self.refs[1] = (self.refs[0] + sample) / 2.0;
                self.refs[2] = sample;
            }
            _ => self.refs[0] = sample,
        }
    }
    fn output(&mut self, cycle: usize, cycle_end: usize) -> f64 {
        let mut sample = self.refs[cycle];
        for n in (2..=cycle_end).rev() {
            let index = n + 3;
            self.refs[8] = sample;
            sample = (sample + self.refs[index]) * 0.5;
            self.refs[index] = self.refs[8];
        }
        sample
    }
}
struct PitchDelay {
    params: [f32; 6],
    sample_rate: f32,
    seed: u32,
    left: Lane,
    right: Lane,
    cycle: usize,
}
impl PitchDelay {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [1.0, 0.0, 0.5, 0.0, 0.5, 1.0],
            sample_rate,
            seed,
            left: Lane::new(seed, 0),
            right: Lane::new(seed, 1),
            cycle: 0,
        }
    }
}
impl AirwindowsEffect for PitchDelay {
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
            .map_or_else(String::new, |v| float2string(*v))
    }
    fn reset(&mut self) {
        self.left.reset(self.seed, 0);
        self.right.reset(self.seed, 1);
        self.cycle = 0;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], orr: &mut [f32]) {
        let [a, b, c, d, e, f] = self.params;
        let cycle_end = ((1.0 / 44100.0 * self.sample_rate as f64).floor() as usize).clamp(1, 4);
        if self.cycle > cycle_end - 1 {
            self.cycle = cycle_end - 1;
        }
        let speed = (a as f64).powf(4.0) * 20.0 + 1.0;
        let feedback_l = b as f64 * (3.0 - (self.left.regen * 2.0).abs());
        let feedback_r = b as f64 * (3.0 - (self.right.regen * 2.0).abs());
        let freq = ((c as f64).powf(3.0) * 0.4) + 0.0001;
        let resonance = (d as f64).powf(2.0) + 0.01;
        self.left.configure(freq, resonance);
        self.right.configure(freq, resonance);
        let vib_speed = (e as f64 - 0.5) * 61.8;
        let vib_depth = (vib_speed.abs() * 20.0 * speed) + 1.0;
        let wet = (f as f64 * 2.0).clamp(0.0, 1.0);
        let dry = (2.0 - f as f64 * 2.0).clamp(0.0, 1.0);
        for (((&l, &r), outl), outr) in il.iter().zip(ir).zip(ol).zip(orr) {
            let mut l = l as f64;
            let mut r = r as f64;
            if l.abs() < 1.18e-23 {
                l = self.left.fpd as f64 * 1.18e-17;
            }
            if r.abs() < 1.18e-23 {
                r = self.right.fpd as f64 * 1.18e-17;
            }
            let dry_l = l;
            let dry_r = r;
            self.cycle += 1;
            if self.cycle == cycle_end {
                l = self.left.step(l, feedback_l, speed, vib_speed, vib_depth);
                r = self.right.step(r, feedback_r, speed, vib_speed, vib_depth);
                self.left.interpolate(l, cycle_end);
                self.right.interpolate(r, cycle_end);
                self.cycle = 0;
            }
            l = self.left.output(self.cycle, cycle_end);
            r = self.right.output(self.cycle, cycle_end);
            if wet < 1.0 {
                l *= wet;
                r *= wet;
            }
            let mut dry_l = dry_l;
            let mut dry_r = dry_r;
            if dry < 1.0 {
                dry_l *= dry;
                dry_r *= dry;
            }
            l += dry_l;
            r += dry_r;
            *outl = fpd_dither(l, &mut self.left.fpd) as f32;
            *outr = fpd_dither(r, &mut self.right.fpd) as f32;
        }
    }
}
