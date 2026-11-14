//! Batch 2 list lines 181-184.
mod mackeq;
mod mackity;
mod matrixverb;
mod melt;

use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    mackeq::ENTRY,
    mackity::ENTRY,
    matrixverb::ENTRY,
    melt::ENTRY,
];
