//! Batch 1 list lines 105-110.
use crate::registry::PluginEntry;
mod distance;
mod distance2;
mod distortion;
mod ditherbox;
mod ditherfloat;
mod dithermediskers;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    distance::ENTRY,
    distance2::ENTRY,
    distortion::ENTRY,
    ditherbox::ENTRY,
    ditherfloat::ENTRY,
    dithermediskers::ENTRY,
];
