// TinySoundFont, Copyright (C) 2017 Bernhard Schelling, MIT license.
// Ported from tsf.h at 853a0a171759f1ddba0de1442133a75912bbeffa.
use super::LoadError;
type Chunk<'a> = ([u8; 4], &'a [u8]);
pub(crate) fn chunks(mut data: &[u8]) -> Result<Vec<Chunk<'_>>, LoadError> {
    let mut result = Vec::new();
    while !data.is_empty() {
        if data.len() < 8 {
            return Err(LoadError::Truncated);
        }
        let id = [data[0], data[1], data[2], data[3]];
        let size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
        data = &data[8..];
        if size > data.len() {
            return Err(LoadError::Truncated);
        }
        result.push((id, &data[..size]));
        let advance = size.checked_add(size & 1).ok_or(LoadError::Truncated)?;
        if advance > data.len() {
            return Err(LoadError::Truncated);
        }
        data = &data[advance..];
    }
    Ok(result)
}
