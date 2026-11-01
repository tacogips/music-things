// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
use super::{hydra::Hydra, region::Region, LoadError};
#[derive(Clone, Debug)]
pub(crate) struct Preset {
    pub(crate) preset_name: [u8; 20],
    pub(crate) preset: u16,
    pub(crate) bank: u16,
    pub(crate) regions: Vec<Region>,
    pub(crate) region_num: i32,
}
fn zones<'a>(
    bags: &[usize],
    gens: &'a [(u16, u16)],
    start: usize,
    end: usize,
) -> Result<Vec<&'a [(u16, u16)]>, LoadError> {
    if start > end || end >= bags.len() {
        return Err(LoadError::InvalidHydra);
    }
    let mut result = Vec::new();
    for i in start..end {
        let a = bags[i];
        let b = bags[i + 1];
        if a > b || b > gens.len() {
            return Err(LoadError::InvalidHydra);
        }
        result.push(&gens[a..b]);
    }
    Ok(result)
}
pub(crate) fn resolve(h: &Hydra, count: u32) -> Result<Vec<Preset>, LoadError> {
    if h.phdr.len() < 2 || h.inst.len() < 2 || h.shdr.len() < 2 {
        return Err(LoadError::InvalidHydra);
    }
    let mut result = Vec::new();
    for index in 0..h.phdr.len() - 1 {
        let hdr = &h.phdr[index];
        let mut preset = Preset {
            preset_name: hdr.name,
            preset: hdr.number,
            bank: hdr.bank,
            regions: Vec::new(),
            region_num: 0,
        };
        let mut global = Region::clear(true);
        for (pi, pzone) in zones(&h.pbag, &h.pgen, hdr.bag, h.phdr[index + 1].bag)?
            .iter()
            .enumerate()
        {
            let mut pr = global;
            let mut has_inst = false;
            for &(op, amount) in *pzone {
                if op != 41 {
                    pr.operator(op, amount);
                    continue;
                }
                let ix = amount as usize;
                if ix + 1 >= h.inst.len() {
                    continue;
                }
                has_inst = true;
                let mut iglobal = Region::clear(false);
                for (ii, izone) in zones(&h.ibag, &h.igen, h.inst[ix], h.inst[ix + 1])?
                    .iter()
                    .enumerate()
                {
                    let mut r = iglobal;
                    let mut has_sample = false;
                    for &(iop, ia) in *izone {
                        if iop != 53 {
                            r.operator(iop, ia);
                            continue;
                        }
                        if r.hikey < pr.lokey
                            || r.lokey > pr.hikey
                            || r.hivel < pr.lovel
                            || r.lovel > pr.hivel
                        {
                            continue;
                        }
                        let s = h.shdr.get(ia as usize).ok_or(LoadError::InvalidHydra)?;
                        r.lokey = r.lokey.max(pr.lokey);
                        r.hikey = r.hikey.min(pr.hikey);
                        r.lovel = r.lovel.max(pr.lovel);
                        r.hivel = r.hivel.min(pr.hivel);
                        r.merge(&pr);
                        r.convert();
                        r.offset = r.offset.wrapping_add(s.start);
                        r.end = r.end.wrapping_add(s.end);
                        r.loop_start = r.loop_start.wrapping_add(s.loop_start);
                        r.loop_end = r.loop_end.wrapping_add(s.loop_end);
                        if s.loop_end > 0 {
                            r.loop_end = r.loop_end.wrapping_sub(1)
                        }
                        r.loop_end = r.loop_end.min(count);
                        if r.pitch_keycenter == -1 {
                            r.pitch_keycenter = s.pitch as i32
                        }
                        r.tune += s.correction as i32;
                        r.sample_rate = s.rate;
                        r.end = if r.end != 0 && r.end < count {
                            r.end + 1
                        } else {
                            count
                        };
                        preset.regions.push(r);
                        has_sample = true;
                    }
                    if ii == 0 && !has_sample {
                        iglobal = r
                    }
                }
            }
            if pi == 0 && !has_inst {
                global = pr
            }
        }
        preset.region_num = preset.regions.len() as i32;
        result.push(preset);
    }
    result.sort_by_key(|p| (p.bank, p.preset));
    Ok(result)
}
