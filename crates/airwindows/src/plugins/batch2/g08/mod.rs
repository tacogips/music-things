//! Batch 2 list lines 140-144.
mod fromtape;
mod galactic;
mod gatelope;
mod glitchshifter;
mod golem;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    fromtape::ENTRY,
    galactic::ENTRY,
    gatelope::ENTRY,
    glitchshifter::ENTRY,
    golem::ENTRY,
];
