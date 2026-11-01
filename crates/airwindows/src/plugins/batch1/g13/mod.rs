//! Batch 1 list lines 97-104.
use crate::registry::PluginEntry;
mod density;
mod density2;
mod derez;
mod derez2;
mod desk;
mod desk4;
mod digitalblack;
mod dirt;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    density::ENTRY,
    density2::ENTRY,
    derez::ENTRY,
    derez2::ENTRY,
    desk::ENTRY,
    desk4::ENTRY,
    digitalblack::ENTRY,
    dirt::ENTRY,
];
