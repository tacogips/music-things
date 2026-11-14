//! Batch 3 list lines 221-228.

use crate::registry::PluginEntry;

mod powersag;
mod powersag2;
mod precious;
mod preponderant;
mod pressure4;
mod pressure5;
mod purestair;
mod purestconsole2buss;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    powersag::ENTRY,
    powersag2::ENTRY,
    precious::ENTRY,
    preponderant::ENTRY,
    pressure4::ENTRY,
    pressure5::ENTRY,
    purestair::ENTRY,
    purestconsole2buss::ENTRY,
];
