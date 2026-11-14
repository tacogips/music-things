//! Batch 3 list lines 245-252.

use crate::registry::PluginEntry;

mod reverb;
mod righteous4;
mod rightomono;
mod shape;
mod sidedull;
mod sidepass;
mod silhouette;
mod singleendedtriode;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    reverb::ENTRY,
    righteous4::ENTRY,
    rightomono::ENTRY,
    shape::ENTRY,
    sidedull::ENTRY,
    sidepass::ENTRY,
    silhouette::ENTRY,
    singleendedtriode::ENTRY,
];
