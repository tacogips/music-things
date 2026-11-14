//! Batch 3 list lines 285-292.

use crate::registry::PluginEntry;

mod totape6;
mod tovinyl4;
mod tpdfdither;
mod tpdfwide;
mod transdesk;
mod tremolo;
mod tremosquare;
mod triplespread;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    totape6::ENTRY,
    tovinyl4::ENTRY,
    tpdfdither::ENTRY,
    tpdfwide::ENTRY,
    transdesk::ENTRY,
    tremolo::ENTRY,
    tremosquare::ENTRY,
    triplespread::ENTRY,
];
