//! Batch 1 list lines 89-96.
mod curve;
mod dark;
mod darknoise;
mod dcvoltage;
mod debess;
mod deckwrecka;
mod deess;
mod dehiss;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    curve::ENTRY,
    dark::ENTRY,
    darknoise::ENTRY,
    dcvoltage::ENTRY,
    debess::ENTRY,
    deckwrecka::ENTRY,
    deess::ENTRY,
    dehiss::ENTRY,
];
