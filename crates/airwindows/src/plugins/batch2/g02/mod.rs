//! Batch 2 list lines 119-125.
mod dustbunny;
mod dynamics;
mod dyno;
mod edge;
mod edisdim;
mod elation;
mod electrohat;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    dustbunny::ENTRY,
    dynamics::ENTRY,
    dyno::ENTRY,
    edge::ENTRY,
    edisdim::ENTRY,
    elation::ENTRY,
    electrohat::ENTRY,
];
