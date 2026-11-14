//! Batch 1 list lines 65-72.
mod compresaturator;
mod console4buss;
mod console4channel;
mod console5buss;
mod console5channel;
mod console5darkch;
mod console6buss;
mod console6channel;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    compresaturator::ENTRY,
    console4buss::ENTRY,
    console4channel::ENTRY,
    console5buss::ENTRY,
    console5channel::ENTRY,
    console5darkch::ENTRY,
    console6buss::ENTRY,
    console6channel::ENTRY,
];
