//! Batch 2 list lines 166-171.
mod interstage;
mod ironoxide5;
mod ironoxideclassic;
mod ironoxideclassic2;
mod isolator;
mod isolator2;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    interstage::ENTRY,
    ironoxide5::ENTRY,
    ironoxideclassic::ENTRY,
    ironoxideclassic2::ENTRY,
    isolator::ENTRY,
    isolator2::ENTRY,
];
