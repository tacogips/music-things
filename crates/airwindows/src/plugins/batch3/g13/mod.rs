//! Batch 3 list lines 317-324.

use crate::registry::PluginEntry;

mod yhighpass;
mod ylowpass;
mod ynotch;
mod zbandpass;
mod zbandpass2;
mod zhighpass;
mod zhighpass2;
mod zlowpass;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    yhighpass::ENTRY,
    ylowpass::ENTRY,
    ynotch::ENTRY,
    zbandpass::ENTRY,
    zbandpass2::ENTRY,
    zhighpass::ENTRY,
    zhighpass2::ENTRY,
    zlowpass::ENTRY,
];
