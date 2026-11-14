//! SoundFont loading from SF2 bytes. Internal `Region`, `Preset`, and `Envelope`
//! mirror TinySoundFont structures for the synth module owned by bop-16c.
// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
mod hydra;
mod preset;
mod region;
mod riff;
pub(crate) use preset::Preset;
#[allow(unused_imports)]
pub(crate) use region::{Envelope, Region};
use std::fmt::Write;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    NotRiff,
    NotSfbk,
    MissingChunk(&'static str),
    Truncated,
    InvalidHydra,
}
impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for LoadError {}
#[derive(Debug)]
pub struct SoundFont {
    pub(crate) presets: Vec<Preset>,
    pub(crate) samples: Vec<f32>,
    pub(crate) sample_count: u32,
}
impl SoundFont {
    pub fn load(bytes: &[u8]) -> Result<Self, LoadError> {
        if bytes.len() < 4 || &bytes[..4] != b"RIFF" {
            return Err(LoadError::NotRiff);
        }
        if bytes.len() < 12 {
            return Err(LoadError::Truncated);
        }
        if &bytes[8..12] != b"sfbk" {
            return Err(LoadError::NotSfbk);
        }
        let size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
        if size < 4 || size.checked_add(8).is_none_or(|n| n > bytes.len()) {
            return Err(LoadError::Truncated);
        }
        let chunks = riff::chunks(&bytes[12..size + 8])?;
        let mut smpl = None;
        let mut pdta = None;
        for (id, data) in chunks {
            if &id != b"LIST" || data.len() < 4 {
                continue;
            }
            let (kind, body) = data.split_at(4);
            if kind == b"sdta" {
                for (sub, d) in riff::chunks(body)? {
                    if &sub == b"smpl" {
                        smpl = Some(d)
                    }
                }
            } else if kind == b"pdta" {
                pdta = Some(body)
            }
        }
        let raw = smpl.ok_or(LoadError::MissingChunk("smpl"))?;
        let samples: Vec<f32> = raw
            .chunks_exact(2)
            .map(|s| (i16::from_le_bytes([s[0], s[1]]) as f64 / 32767.0) as f32)
            .collect();
        let sample_count = samples.len() as u32;
        let hydra = hydra::Hydra::parse(pdta.ok_or(LoadError::MissingChunk("pdta"))?)?;
        let presets = preset::resolve(&hydra, sample_count)?;
        Ok(Self {
            presets,
            samples,
            sample_count,
        })
    }
    pub fn preset_count(&self) -> usize {
        self.presets.len()
    }
    pub fn preset_index(&self, bank: i32, number: i32) -> Option<usize> {
        self.presets
            .iter()
            .position(|p| i32::from(p.bank) == bank && i32::from(p.preset) == number)
    }
    pub fn preset_name(&self, index: usize) -> Option<&str> {
        self.presets.get(index).map(|p| name(&p.preset_name))
    }
    pub fn bank_preset_name(&self, bank: i32, number: i32) -> Option<&str> {
        self.preset_index(bank, number)
            .and_then(|i| self.preset_name(i))
    }
    #[doc(hidden)]
    pub fn debug_dump(&self) -> String {
        let mut out = String::new();
        writeln!(out, "presets {}", self.presets.len()).ok();
        for (i, p) in self.presets.iter().enumerate() {
            writeln!(
                out,
                "preset {} {} {} {} {}",
                i,
                p.bank,
                p.preset,
                name(&p.preset_name),
                p.region_num
            )
            .ok();
            for r in &p.regions {
                write!(
                    out,
                    "region {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
                    r.loop_mode,
                    r.sample_rate,
                    r.lokey,
                    r.hikey,
                    r.lovel,
                    r.hivel,
                    r.group,
                    r.offset,
                    r.end,
                    r.loop_start,
                    r.loop_end,
                    r.transpose,
                    r.tune,
                    r.pitch_keycenter,
                    r.pitch_keytrack
                )
                .ok();
                for v in [r.attenuation, r.pan] {
                    write!(out, " {:08x}", v.to_bits()).ok();
                }
                for e in [r.ampenv, r.modenv] {
                    for v in [
                        e.delay,
                        e.attack,
                        e.hold,
                        e.decay,
                        e.sustain,
                        e.release,
                        e.keynum_to_hold,
                        e.keynum_to_decay,
                    ] {
                        write!(out, " {:08x}", v.to_bits()).ok();
                    }
                }
                writeln!(
                    out,
                    " {} {} {} {} {} {} {:08x} {} {} {:08x} {} {}",
                    r.initial_filter_q,
                    r.initial_filter_fc,
                    r.mod_env_to_pitch,
                    r.mod_env_to_filter_fc,
                    r.mod_lfo_to_filter_fc,
                    r.mod_lfo_to_volume,
                    r.delay_mod_lfo.to_bits(),
                    r.freq_mod_lfo,
                    r.mod_lfo_to_pitch,
                    r.delay_vib_lfo.to_bits(),
                    r.freq_vib_lfo,
                    r.vib_lfo_to_pitch
                )
                .ok();
            }
        }
        let mut hash = 0xcbf29ce484222325_u64;
        for s in &self.samples {
            for b in s.to_le_bytes() {
                hash = (hash ^ u64::from(b)).wrapping_mul(0x100000001b3)
            }
        }
        writeln!(out, "samples {} {:016x}", self.sample_count, hash).ok();
        out
    }
}

fn name(bytes: &[u8; 20]) -> &str {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    std::str::from_utf8(&bytes[..end]).unwrap_or("")
}
