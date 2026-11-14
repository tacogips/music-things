//! Copyright (c) 2011 __MyCompanyName__, Airwindows uses the MIT license
//! Upstream: airwindows/plugins/LinuxVST/src/Pressure5, revision b110fac3991fff7c4655cc2d25d6a3e221f9799c.
//! Parity: Libm, measured max-abs 0.
use crate::display::float2string;
use crate::dither::{fpd_dither, fpd_from_seed};
use crate::params::sanitize_param;
use crate::rand::{replay_fpd_seed_loop, AwRand};
use crate::registry::PluginEntry;
use crate::{AirwindowsEffect, ParamInfo, PluginInfo};
const PARAMS: &[ParamInfo] = &[
    ParamInfo {
        name: "Pressre",
        label: "",
        default: 0.0,
    },
    ParamInfo {
        name: "Speed",
        label: "",
        default: 0.25,
    },
    ParamInfo {
        name: "Mewines",
        label: "",
        default: 1.0,
    },
    ParamInfo {
        name: "PawClaw",
        label: "",
        default: 0.5,
    },
    ParamInfo {
        name: "Output",
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
    name: "Pressure5",
    upstream_name: "Pressure5",
    description: "Pressure5 expands Pressure4 with after-boost and built-in ClipOnly2.",
    params: PARAMS,
};
pub(crate) const ENTRY: PluginEntry = PluginEntry {
    info: &INFO,
    construct,
};
fn construct(sample_rate: f32, seed: u32) -> Box<dyn AirwindowsEffect> {
    Box::new(Pressure5::new(sample_rate, seed))
}
#[derive(Default)]
struct Clip {
    last: f64,
    samples: [f64; 17],
    positive: bool,
    negative: bool,
}
impl Clip {
    fn process(&mut self, mut input: f64, spacing: usize) -> f64 {
        input = input.clamp(-4.0, 4.0);
        if self.positive {
            if input < self.last {
                self.last = 0.7058208 + input * 0.2609148;
            } else {
                self.last = 0.2491717 + self.last * 0.7390851;
            }
        }
        self.positive = false;
        if input > 0.9549925859 {
            self.positive = true;
            input = 0.7058208 + self.last * 0.2609148;
        }
        if self.negative {
            if input > self.last {
                self.last = -0.7058208 + input * 0.2609148;
            } else {
                self.last = -0.2491717 + self.last * 0.7390851;
            }
        }
        self.negative = false;
        if input < -0.9549925859 {
            self.negative = true;
            input = -0.7058208 + self.last * 0.2609148;
        }
        self.samples[spacing] = input;
        let result = self.last;
        for x in (1..=spacing).rev() {
            self.samples[x - 1] = self.samples[x];
        }
        self.last = self.samples[0];
        result
    }
}
fn filter(b: &mut [f64; 13], input: f64, offset: usize) -> f64 {
    let t = input * b[2] + b[offset];
    b[offset] = input * b[3] - t * b[5] + b[offset + 1];
    b[offset + 1] = input * b[4] - t * b[6];
    t
}
struct Pressure5 {
    params: [f32; 6],
    sample_rate: f32,
    seed: u32,
    speed: [f64; 2],
    coefficient: [f64; 2],
    flip: bool,
    a: [f64; 13],
    b: [f64; 13],
    left: Clip,
    right: Clip,
    slew: f64,
    fpd_l: u32,
    fpd_r: u32,
}
impl Pressure5 {
    fn new(sample_rate: f32, seed: u32) -> Self {
        let mut rng = AwRand::new(seed);
        replay_fpd_seed_loop(&mut rng);
        replay_fpd_seed_loop(&mut rng);
        Self {
            params: [0.0, 0.25, 1.0, 0.5, 0.5, 1.0],
            sample_rate,
            seed,
            speed: [10000.0; 2],
            coefficient: [1.0; 2],
            flip: false,
            a: [0.0; 13],
            b: [0.0; 13],
            left: Clip::default(),
            right: Clip::default(),
            slew: 0.0,
            fpd_l: fpd_from_seed(seed, 0),
            fpd_r: fpd_from_seed(seed, 1),
        }
    }
}
impl AirwindowsEffect for Pressure5 {
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
        let p = self.params;
        *self = Self::new(self.sample_rate, self.seed);
        self.params = p;
    }
    fn process(&mut self, il: &[f32], ir: &[f32], ol: &mut [f32], or: &mut [f32]) {
        let mut scale = 1.0;
        scale /= 44100.0;
        scale *= self.sample_rate as f64;
        let spacing = (scale.floor() as usize).clamp(1, 16);
        let threshold = 1.0 - self.params[0] as f64 * 0.95;
        let makeup = 1.0 / threshold;
        let mut release = (1.28 - self.params[1] as f64).powf(5.0) * 32768.0;
        let mut fastest = release.sqrt();
        release /= scale;
        fastest /= scale;
        let mew_ref = self.params[2] as f64;
        let paws = -(self.params[3] as f64 - 0.5) * 1.618_033_988_749_895;
        let gain = (self.params[4] as f64 * 2.0).powf(2.0);
        let wet = self.params[5] as f64;
        self.a[0] = 24000.0 / self.sample_rate as f64;
        #[allow(clippy::approx_constant)] // Preserve the upstream filter resonance.
        {
            self.a[1] = 0.7071;
        }
        let k = (std::f64::consts::PI * self.a[0]).tan();
        let norm = 1.0 / (1.0 + k / self.a[1] + k * k);
        self.a[2] = k * k * norm;
        self.a[3] = 2.0 * self.a[2];
        self.a[4] = self.a[2];
        self.a[5] = 2.0 * (k * k - 1.0) * norm;
        self.a[6] = (1.0 - k / self.a[1] + k * k) * norm;
        self.b[..7].copy_from_slice(&self.a[..7]);
        for (((&l, &r), out_l), out_r) in il.iter().zip(ir).zip(ol).zip(or) {
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
            l *= makeup;
            r *= makeup;
            if self.a[0] < 0.4999 {
                l = filter(&mut self.a, l, 7);
                r = filter(&mut self.a, r, 9);
            }
            let sense = l.abs().max(r.abs());
            let mut mew = (mew_ref + self.slew * paws).sin();
            let positive = mew >= 0.0;
            if !positive {
                mew = -mew;
            }
            let index = if self.flip { 0 } else { 1 };
            let speed = &mut self.speed[index];
            let coeff = &mut self.coefficient[index];
            if sense > threshold {
                let vary = threshold / sense;
                let attack = speed.abs().sqrt();
                *coeff *= attack - 1.0;
                *coeff += if vary < threshold { threshold } else { vary };
                *coeff /= attack;
            } else {
                *coeff *= (*speed * *speed) - 1.0;
                *coeff += 1.0;
                *coeff /= *speed * *speed;
            }
            let mut new_speed = *speed * (*speed - 1.0);
            new_speed += (sense * release).abs() + fastest;
            *speed = new_speed / *speed;
            let c = if positive {
                coeff.powf(2.0)
            } else {
                coeff.sqrt()
            };
            let c = c * mew + *coeff * (1.0 - mew);
            l *= c;
            r *= c;
            if gain != 1.0 {
                l *= gain;
                r *= gain;
            }
            self.flip = !self.flip;
            if self.b[0] < 0.49999 {
                l = filter(&mut self.b, l, 7);
                r = filter(&mut self.b, r, 9);
            }
            if wet != 1.0 {
                l = l * wet + dry_l * (1.0 - wet);
                r = r * wet + dry_r * (1.0 - wet);
            }
            self.slew = (l - self.left.last).abs().max((r - self.right.last).abs());
            l = self.left.process(l, spacing);
            r = self.right.process(r, spacing);
            *out_l = fpd_dither(l, &mut self.fpd_l) as f32;
            *out_r = fpd_dither(r, &mut self.fpd_r) as f32;
        }
    }
}
