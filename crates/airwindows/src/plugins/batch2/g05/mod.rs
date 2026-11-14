//! Batch 2 list lines 128-133.
mod ensemble;
mod eq;
mod everytrim;
mod exciter;
mod facet;
mod fathomfive;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    ensemble::ENTRY,
    eq::ENTRY,
    everytrim::ENTRY,
    exciter::ENTRY,
    facet::ENTRY,
    fathomfive::ENTRY,
];
