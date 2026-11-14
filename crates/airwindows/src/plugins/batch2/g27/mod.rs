//! Batch 2 list lines 194-196.
mod naturalizedither;
mod ncseventeen;
mod neverland;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    naturalizedither::ENTRY,
    ncseventeen::ENTRY,
    neverland::ENTRY,
];
