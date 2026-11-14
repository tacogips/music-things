//! Batch 3 list lines 229-236.

use crate::registry::PluginEntry;

mod purestconsole2channel;
mod purestconsolebuss;
mod purestconsolechannel;
mod purestdrive;
mod purestecho;
mod purestfade;
mod purestgain;
mod purestsquish;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    purestconsole2channel::ENTRY,
    purestconsolebuss::ENTRY,
    purestconsolechannel::ENTRY,
    purestdrive::ENTRY,
    purestecho::ENTRY,
    purestfade::ENTRY,
    purestgain::ENTRY,
    purestsquish::ENTRY,
];
