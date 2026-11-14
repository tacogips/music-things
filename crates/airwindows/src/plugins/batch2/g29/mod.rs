//! Batch 2 list lines 198-199.
mod nodedither;
mod noise;
use crate::registry::PluginEntry;
#[rustfmt::skip]
pub(crate) const ENTRIES: &[PluginEntry] = &[
    nodedither::ENTRY,
    noise::ENTRY,
];
