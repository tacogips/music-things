//! Batch 3 list lines 269-276.

use crate::registry::PluginEntry;

mod studiotan;
mod subsonly;
mod surge;
mod surgetide;
mod swell;
mod tape;
mod tapedelay;
mod tapedelay2;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    studiotan::ENTRY,
    subsonly::ENTRY,
    surge::ENTRY,
    surgetide::ENTRY,
    swell::ENTRY,
    tape::ENTRY,
    tapedelay::ENTRY,
    tapedelay2::ENTRY,
];
