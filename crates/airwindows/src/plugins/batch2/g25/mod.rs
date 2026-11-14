//! Batch 2 list lines 191-192.
mod monoam;
mod multibanddistortion;
use crate::registry::PluginEntry;
#[rustfmt::skip]
pub(crate) const ENTRIES: &[PluginEntry] = &[
    monoam::ENTRY,
    multibanddistortion::ENTRY,
];
