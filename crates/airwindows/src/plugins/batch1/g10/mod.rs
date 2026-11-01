//! Batch 1 list lines 73-80.
use crate::registry::PluginEntry;
mod console7buss;
mod console7cascade;
mod console7channel;
mod console7crunch;
mod console8busout;
mod console8busshype;
mod console8bussin;
mod console8channelhype;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    console7buss::ENTRY,
    console7cascade::ENTRY,
    console7channel::ENTRY,
    console7crunch::ENTRY,
    console8busout::ENTRY,
    console8busshype::ENTRY,
    console8bussin::ENTRY,
    console8channelhype::ENTRY,
];
