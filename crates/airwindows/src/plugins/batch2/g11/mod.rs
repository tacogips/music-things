//! Batch 2 list lines 153-159.
use crate::registry::PluginEntry;
mod highimpact;
mod highpass;
mod highpass2;
mod holt;
mod holt2;
mod hombre;
mod hull;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    highimpact::ENTRY,
    highpass::ENTRY,
    highpass2::ENTRY,
    holt::ENTRY,
    holt2::ENTRY,
    hombre::ENTRY,
    hull::ENTRY,
];
