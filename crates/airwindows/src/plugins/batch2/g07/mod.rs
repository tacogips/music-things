//! Batch 2 list lines 135-139.
use crate::registry::PluginEntry;
mod flipity;
mod floor;
mod flutter;
mod focus;
mod fracture;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    flipity::ENTRY,
    floor::ENTRY,
    flutter::ENTRY,
    focus::ENTRY,
    fracture::ENTRY,
];
