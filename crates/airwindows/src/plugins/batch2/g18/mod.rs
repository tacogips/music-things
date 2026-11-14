//! Batch 2 list lines 176-180.
mod loud;
mod lowpass;
mod lowpass2;
mod lrfliptimer;
mod luxor;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    loud::ENTRY,
    lowpass::ENTRY,
    lowpass2::ENTRY,
    lrfliptimer::ENTRY,
    luxor::ENTRY,
];
