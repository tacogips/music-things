//! Batch 2 list lines 111-118.
mod dithermetimbers;
mod doublelay;
mod doublepaul;
mod drive;
mod drumslam;
mod dubcenter;
mod dubly;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    dithermetimbers::ENTRY,
    doublelay::ENTRY,
    doublepaul::ENTRY,
    drive::ENTRY,
    drumslam::ENTRY,
    dubcenter::ENTRY,
    dubly::ENTRY,
];
