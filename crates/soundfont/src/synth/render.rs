// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
use super::{cents_hz, db_gain, envelope::DONE, voice::Voice, OutputMode, Synth};
impl Synth {
    pub fn render_float(&mut self, out: &mut [f32], frames: usize, mixing: bool) {
        let channels = if self.mode == OutputMode::Mono { 1 } else { 2 };
        let frames = frames.min(out.len() / channels);
        if !mixing {
            out[..frames * channels].fill(0.);
        }
        for v in &mut self.voices {
            if v.preset != -1 {
                render(v, &self.font.samples, self.mode, self.rate, out, frames);
            }
        }
    }
}
fn render(
    v: &mut Voice,
    input: &[f32],
    mode: OutputMode,
    rate: f32,
    out: &mut [f32],
    frames: usize,
) {
    let r = v.region;
    let dynamic_filter = r.mod_lfo_to_filter_fc != 0 || r.mod_env_to_filter_fc != 0;
    let dynamic_pitch =
        r.mod_lfo_to_pitch != 0 || r.mod_env_to_pitch != 0 || r.vib_lfo_to_pitch != 0;
    let dynamic_gain = r.mod_lfo_to_volume != 0;
    let looping = v.loop_start < v.loop_end;
    let mut pos = v.position;
    let mut cursor = 0;
    while cursor < frames {
        let block = (frames - cursor).min(64);
        if dynamic_filter {
            let fres = r.initial_filter_fc as f32
                + v.mod_lfo.level * r.mod_lfo_to_filter_fc as f32
                + v.modulation.level * r.mod_env_to_filter_fc as f32;
            let fc = if fres <= 13500. {
                cents_hz(fres) / rate
            } else {
                1.
            };
            v.filter.active = fc < 0.499;
            if v.filter.active {
                v.filter.setup(fc);
            }
        }
        let pitch = if dynamic_pitch {
            (v.mod_lfo.level * r.mod_lfo_to_pitch as f32
                + v.vib_lfo.level * r.vib_lfo_to_pitch as f32
                + v.modulation.level * r.mod_env_to_pitch as f32) as f64
        } else {
            0.
        };
        let ratio = 2_f64.powf((v.pitch_input + pitch) / 1200.) * v.pitch_factor;
        let gain = db_gain(
            v.gain_db
                + if dynamic_gain {
                    v.mod_lfo.level * (r.mod_lfo_to_volume as f32 * 0.1)
                } else {
                    0.
                },
        ) * v.amp.level;
        v.amp.process(block as i32, rate);
        if r.mod_env_to_pitch != 0 || r.mod_env_to_filter_fc != 0 {
            v.modulation.process(block as i32, rate);
        }
        if v.mod_lfo.delta != 0.
            && (r.mod_lfo_to_pitch != 0 || r.mod_lfo_to_filter_fc != 0 || r.mod_lfo_to_volume != 0)
        {
            v.mod_lfo.process(block as i32);
        }
        if v.vib_lfo.delta != 0. && r.vib_lfo_to_pitch != 0 {
            v.vib_lfo.process(block as i32);
        }
        for n in 0..block {
            if pos >= r.end as f64 {
                break;
            }
            let sample = pos as usize;
            let next = if sample >= v.loop_end as usize && looping {
                v.loop_start as usize
            } else {
                sample + 1
            };
            if sample >= input.len() || next >= input.len() {
                v.preset = -1;
                return;
            }
            let alpha = (pos - sample as f64) as f32;
            let mut value = input[sample] * (1. - alpha) + input[next] * alpha;
            if v.filter.active {
                value = v.filter.process(value);
            }
            match mode {
                OutputMode::Mono => out[cursor + n] += value * gain,
                OutputMode::StereoInterleaved => {
                    out[2 * (cursor + n)] += value * (gain * v.pan_left);
                    out[2 * (cursor + n) + 1] += value * (gain * v.pan_right);
                }
                OutputMode::StereoUnweaved => {
                    out[cursor + n] += value * (gain * v.pan_left);
                    out[frames + cursor + n] += value * (gain * v.pan_right);
                }
            }
            pos += ratio;
            if looping && pos >= v.loop_end as f64 + 1. {
                pos -= (v.loop_end - v.loop_start + 1) as f64;
            }
        }
        if pos >= r.end as f64 || v.amp.segment == DONE {
            v.preset = -1;
            return;
        }
        cursor += block;
    }
    v.position = pos;
}
