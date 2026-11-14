// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
use super::LoadError;
#[derive(Clone)]
pub(crate) struct Header {
    pub name: [u8; 20],
    pub number: u16,
    pub bank: u16,
    pub bag: usize,
}
#[derive(Clone, Copy)]
pub(crate) struct Sample {
    pub start: u32,
    pub end: u32,
    pub loop_start: u32,
    pub loop_end: u32,
    pub rate: u32,
    pub pitch: u8,
    pub correction: i8,
}
pub(crate) struct Hydra {
    pub phdr: Vec<Header>,
    pub pbag: Vec<usize>,
    pub pgen: Vec<(u16, u16)>,
    pub inst: Vec<usize>,
    pub ibag: Vec<usize>,
    pub igen: Vec<(u16, u16)>,
    pub shdr: Vec<Sample>,
}
fn u16at(d: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([d[i], d[i + 1]])
}
fn u32at(d: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([d[i], d[i + 1], d[i + 2], d[i + 3]])
}
fn records<'a>(
    c: &[([u8; 4], &'a [u8])],
    id: &'static str,
    size: usize,
) -> Result<Vec<&'a [u8]>, LoadError> {
    let d = c
        .iter()
        .find(|(k, _)| k == id.as_bytes())
        .map(|(_, v)| *v)
        .ok_or(LoadError::MissingChunk(id))?;
    if d.len() % size != 0 || d.is_empty() {
        return Err(LoadError::InvalidHydra);
    }
    Ok(d.chunks_exact(size).collect())
}
impl Hydra {
    pub(crate) fn parse(data: &[u8]) -> Result<Self, LoadError> {
        let c = super::riff::chunks(data)?;
        let phdr = records(&c, "phdr", 38)?
            .iter()
            .map(|d| Header {
                name: {
                    let mut n = [0; 20];
                    n[..19].copy_from_slice(&d[..19]);
                    n
                },
                number: u16at(d, 20),
                bank: u16at(d, 22),
                bag: u16at(d, 24) as usize,
            })
            .collect();
        let pbag = records(&c, "pbag", 4)?
            .iter()
            .map(|d| u16at(d, 0) as usize)
            .collect();
        let _ = records(&c, "pmod", 10)?;
        let pgen = records(&c, "pgen", 4)?
            .iter()
            .map(|d| (u16at(d, 0), u16at(d, 2)))
            .collect();
        let inst = records(&c, "inst", 22)?
            .iter()
            .map(|d| u16at(d, 20) as usize)
            .collect();
        let ibag = records(&c, "ibag", 4)?
            .iter()
            .map(|d| u16at(d, 0) as usize)
            .collect();
        let _ = records(&c, "imod", 10)?;
        let igen = records(&c, "igen", 4)?
            .iter()
            .map(|d| (u16at(d, 0), u16at(d, 2)))
            .collect();
        let shdr = records(&c, "shdr", 46)?
            .iter()
            .map(|d| Sample {
                start: u32at(d, 20),
                end: u32at(d, 24),
                loop_start: u32at(d, 28),
                loop_end: u32at(d, 32),
                rate: u32at(d, 36),
                pitch: d[40],
                correction: d[41] as i8,
            })
            .collect();
        Ok(Self {
            phdr,
            pbag,
            pgen,
            inst,
            ibag,
            igen,
            shdr,
        })
    }
}
