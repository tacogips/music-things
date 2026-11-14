//! Batch 3 list lines 277-284.

use crate::registry::PluginEntry;

mod tapedither;
mod tapedust;
mod tapefat;
mod texturize;
mod texturizems;
mod thunder;
mod toneslant;
mod totape5;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    tapedither::ENTRY,
    tapedust::ENTRY,
    tapefat::ENTRY,
    texturize::ENTRY,
    texturizems::ENTRY,
    thunder::ENTRY,
    toneslant::ENTRY,
    totape5::ENTRY,
];
