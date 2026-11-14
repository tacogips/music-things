//! Batch 2 list lines 160-165.
mod hype;
mod hypersonic;
mod hypersonx;
mod infinity;
mod infinity2;
mod infrasonic;

use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    hype::ENTRY,
    hypersonic::ENTRY,
    hypersonx::ENTRY,
    infinity::ENTRY,
    infinity2::ENTRY,
    infrasonic::ENTRY,
];
