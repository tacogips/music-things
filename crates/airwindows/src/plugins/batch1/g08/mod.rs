//! Batch 1 list lines 57-64.
mod chromeoxide;
mod cider;
mod cliponly;
mod cliponly2;
mod clipsoftly;
mod coils;
mod coils2;
mod cojones;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    chromeoxide::ENTRY,
    cider::ENTRY,
    cliponly::ENTRY,
    cliponly2::ENTRY,
    clipsoftly::ENTRY,
    coils::ENTRY,
    coils2::ENTRY,
    cojones::ENTRY,
];
