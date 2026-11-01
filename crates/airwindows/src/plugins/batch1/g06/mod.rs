//! Batch 1 list lines 41-48.
mod c5rawbuss;
mod c5rawchannel;
mod cabs;
mod cabs_coeff;
mod calibre;
mod capacitor;
mod capacitor2;
mod chamber;
mod chamber2;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    c5rawbuss::ENTRY,
    c5rawchannel::ENTRY,
    cabs::ENTRY,
    calibre::ENTRY,
    capacitor::ENTRY,
    capacitor2::ENTRY,
    chamber::ENTRY,
    chamber2::ENTRY,
];
