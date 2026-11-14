//! Batch 3 list lines 325-329.

use crate::registry::PluginEntry;

mod zlowpass2;
mod znotch;
mod znotch2;
mod zregion;
mod zregion2;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    zlowpass2::ENTRY,
    znotch::ENTRY,
    znotch2::ENTRY,
    zregion::ENTRY,
    zregion2::ENTRY,
];
