//! Batch 2 list lines 146-152.
mod gringer;
mod groovewear;
mod guitarconditioner;
mod hardvacuum;
mod hermepass;
mod hermetrim;
mod highglossdither;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    gringer::ENTRY,
    groovewear::ENTRY,
    guitarconditioner::ENTRY,
    hardvacuum::ENTRY,
    hermepass::ENTRY,
    hermetrim::ENTRY,
    highglossdither::ENTRY,
];
