//! Batch 2 list lines 214-220.
mod pockey;
mod pockey2;
mod podcast;
mod podcastdeluxe;
mod point;
mod pop;
mod pop2;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    pockey::ENTRY,
    pockey2::ENTRY,
    podcast::ENTRY,
    podcastdeluxe::ENTRY,
    point::ENTRY,
    pop::ENTRY,
    pop2::ENTRY,
];
