//! Batch 3 list lines 309-316.

mod voicetrick;
mod wider;
mod xbandpass;
mod xhighpass;
mod xlowpass;
mod xnotch;
mod xregion;
mod ybandpass;

use crate::registry::PluginEntry;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    voicetrick::ENTRY,
    wider::ENTRY,
    xbandpass::ENTRY,
    xhighpass::ENTRY,
    xlowpass::ENTRY,
    xnotch::ENTRY,
    xregion::ENTRY,
    ybandpass::ENTRY,
];
