//! Batch 3 list lines 261-268.

use crate::registry::PluginEntry;

mod spiral2;
mod srsly;
mod srsly2;
mod starchild;
mod stereochorus;
mod stereodoubler;
mod stereoensemble;
mod stereofx;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    spiral2::ENTRY,
    srsly::ENTRY,
    srsly2::ENTRY,
    starchild::ENTRY,
    stereochorus::ENTRY,
    stereodoubler::ENTRY,
    stereoensemble::ENTRY,
    stereofx::ENTRY,
];
