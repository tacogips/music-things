//! Batch 2 list lines 207-212.
mod pdbuss;
mod pdchannel;
mod peaksonly;
mod phasenudge;
mod pitchdelay;
mod pitchnasty;

use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    pdbuss::ENTRY,
    pdchannel::ENTRY,
    peaksonly::ENTRY,
    phasenudge::ENTRY,
    pitchdelay::ENTRY,
    pitchnasty::ENTRY,
];
