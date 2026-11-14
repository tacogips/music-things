// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Envelope {
    pub(crate) delay: f32,
    pub(crate) attack: f32,
    pub(crate) hold: f32,
    pub(crate) decay: f32,
    pub(crate) sustain: f32,
    pub(crate) release: f32,
    pub(crate) keynum_to_hold: f32,
    pub(crate) keynum_to_decay: f32,
}
#[derive(Clone, Copy, Default, Debug)]
pub(crate) struct Region {
    pub(crate) loop_mode: i32,
    pub(crate) sample_rate: u32,
    pub(crate) lokey: u8,
    pub(crate) hikey: u8,
    pub(crate) lovel: u8,
    pub(crate) hivel: u8,
    pub(crate) group: u32,
    pub(crate) offset: u32,
    pub(crate) end: u32,
    pub(crate) loop_start: u32,
    pub(crate) loop_end: u32,
    pub(crate) transpose: i32,
    pub(crate) tune: i32,
    pub(crate) pitch_keycenter: i32,
    pub(crate) pitch_keytrack: i32,
    pub(crate) attenuation: f32,
    pub(crate) pan: f32,
    pub(crate) ampenv: Envelope,
    pub(crate) modenv: Envelope,
    pub(crate) initial_filter_q: i32,
    pub(crate) initial_filter_fc: i32,
    pub(crate) mod_env_to_pitch: i32,
    pub(crate) mod_env_to_filter_fc: i32,
    pub(crate) mod_lfo_to_filter_fc: i32,
    pub(crate) mod_lfo_to_volume: i32,
    pub(crate) delay_mod_lfo: f32,
    pub(crate) freq_mod_lfo: i32,
    pub(crate) mod_lfo_to_pitch: i32,
    pub(crate) delay_vib_lfo: f32,
    pub(crate) freq_vib_lfo: i32,
    pub(crate) vib_lfo_to_pitch: i32,
}
impl Region {
    pub(crate) fn clear(relative: bool) -> Self {
        let mut r = Self {
            hikey: 127,
            hivel: 127,
            pitch_keycenter: 60,
            ..Self::default()
        };
        if !relative {
            r.pitch_keycenter = -1;
            r.pitch_keytrack = 100;
            r.initial_filter_fc = 13500;
            r.delay_mod_lfo = -12000.;
            r.delay_vib_lfo = -12000.;
            for e in [&mut r.ampenv, &mut r.modenv] {
                e.delay = -12000.;
                e.attack = -12000.;
                e.hold = -12000.;
                e.decay = -12000.;
                e.release = -12000.;
            }
        }
        r
    }
    fn float_field(&mut self, op: u16) -> Option<&mut f32> {
        Some(match op {
            17 => &mut self.pan,
            21 => &mut self.delay_mod_lfo,
            23 => &mut self.delay_vib_lfo,
            25 => &mut self.modenv.delay,
            26 => &mut self.modenv.attack,
            27 => &mut self.modenv.hold,
            28 => &mut self.modenv.decay,
            29 => &mut self.modenv.sustain,
            30 => &mut self.modenv.release,
            31 => &mut self.modenv.keynum_to_hold,
            32 => &mut self.modenv.keynum_to_decay,
            33 => &mut self.ampenv.delay,
            34 => &mut self.ampenv.attack,
            35 => &mut self.ampenv.hold,
            36 => &mut self.ampenv.decay,
            37 => &mut self.ampenv.sustain,
            38 => &mut self.ampenv.release,
            39 => &mut self.ampenv.keynum_to_hold,
            40 => &mut self.ampenv.keynum_to_decay,
            48 => &mut self.attenuation,
            _ => return None,
        })
    }
    fn int_field(&mut self, op: u16) -> Option<&mut i32> {
        Some(match op {
            5 => &mut self.mod_lfo_to_pitch,
            6 => &mut self.vib_lfo_to_pitch,
            7 => &mut self.mod_env_to_pitch,
            8 => &mut self.initial_filter_fc,
            9 => &mut self.initial_filter_q,
            10 => &mut self.mod_lfo_to_filter_fc,
            11 => &mut self.mod_env_to_filter_fc,
            13 => &mut self.mod_lfo_to_volume,
            22 => &mut self.freq_mod_lfo,
            24 => &mut self.freq_vib_lfo,
            51 => &mut self.transpose,
            52 => &mut self.tune,
            56 => &mut self.pitch_keytrack,
            _ => return None,
        })
    }
    pub(crate) fn operator(&mut self, op: u16, amount: u16) {
        let signed = amount as i16 as i32;
        if let Some(v) = self.float_field(op) {
            *v = signed as f32;
            return;
        }
        if let Some(v) = self.int_field(op) {
            *v = signed;
            return;
        }
        let delta = if matches!(op, 4 | 12 | 45 | 50) {
            signed.wrapping_mul(32768)
        } else {
            signed
        } as u32;
        match op {
            0 | 4 => self.offset = self.offset.wrapping_add(delta),
            1 | 12 => self.end = self.end.wrapping_add(delta),
            2 | 45 => self.loop_start = self.loop_start.wrapping_add(delta),
            3 | 50 => self.loop_end = self.loop_end.wrapping_add(delta),
            43 => {
                self.lokey = amount as u8;
                self.hikey = (amount >> 8) as u8
            }
            44 => {
                self.lovel = amount as u8;
                self.hivel = (amount >> 8) as u8
            }
            54 => {
                self.loop_mode = match amount & 3 {
                    1 => 1,
                    3 => 2,
                    _ => 0,
                }
            }
            57 => self.group = amount as u32,
            58 => self.pitch_keycenter = signed,
            _ => {}
        }
    }
    pub(crate) fn merge(&mut self, other: &Self) {
        for op in 0..59 {
            let mut o = *other;
            if let (Some(a), Some(b)) = (self.float_field(op), o.float_field(op)) {
                let (factor, min, max) = match op {
                    17 => (0.001, -0.5, 0.5),
                    48 => (0.01, 0., 14.4),
                    29 => (1., 0., 1000.),
                    37 => (1., 0., 1440.),
                    31 | 32 | 39 | 40 => (1., -1200., 1200.),
                    26 | 28 | 30 | 34 | 36 | 38 => (1., -12000., 8000.),
                    _ => (1., -12000., 5000.),
                };
                *a = ((*a + *b) * factor).clamp(min, max);
            } else if let (Some(a), Some(b)) = (self.int_field(op), o.int_field(op)) {
                *a += *b;
                let limits = match op {
                    5..=7 | 10 | 11 => Some((-12000, 12000)),
                    8 => Some((1500, 13500)),
                    9 => Some((0, 960)),
                    13 => Some((-960, 960)),
                    22 | 24 => Some((-16000, 4500)),
                    _ => None,
                };
                if let Some((lo, hi)) = limits {
                    *a = (*a).clamp(lo, hi)
                }
            }
        }
        self.offset = self.offset.wrapping_add(other.offset);
        self.end = self.end.wrapping_add(other.end);
        self.loop_start = self.loop_start.wrapping_add(other.loop_start);
        self.loop_end = self.loop_end.wrapping_add(other.loop_end);
    }
    pub(crate) fn convert(&mut self) {
        self.ampenv.convert(true);
        self.modenv.convert(false);
        self.delay_mod_lfo = seconds(self.delay_mod_lfo);
        self.delay_vib_lfo = seconds(self.delay_vib_lfo);
    }
}
fn seconds(v: f32) -> f32 {
    if v < -11950. {
        0.
    } else {
        2_f32.powf(v / 1200.)
    }
}
impl Envelope {
    fn convert(&mut self, gain: bool) {
        self.delay = seconds(self.delay);
        self.attack = seconds(self.attack);
        self.release = seconds(self.release);
        if self.keynum_to_hold == 0. {
            self.hold = seconds(self.hold)
        }
        if self.keynum_to_decay == 0. {
            self.decay = seconds(self.decay)
        }
        if self.sustain < 0. {
            self.sustain = 0.
        } else if gain {
            let db = -self.sustain / 10.;
            self.sustain = if db > -100. {
                10_f32.powf(db * 0.05)
            } else {
                0.
            }
        } else {
            self.sustain = 1. - self.sustain / 1000.
        }
    }
}
