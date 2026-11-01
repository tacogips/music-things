//! Batch 1 list lines 81-88.
mod console8channelin;
mod console8channelout;
mod console8subhype;
mod console8subin;
mod console8subout;
mod crunchygroovewear;
mod crystal;
mod cstrip;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    console8channelin::ENTRY,
    console8channelout::ENTRY,
    console8subhype::ENTRY,
    console8subin::ENTRY,
    console8subout::ENTRY,
    crunchygroovewear::ENTRY,
    crystal::ENTRY,
    cstrip::ENTRY,
];
