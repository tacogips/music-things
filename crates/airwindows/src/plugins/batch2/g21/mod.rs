//! Batch 2 list lines 186-187.
mod midside;
mod mojo;

use crate::registry::PluginEntry;
#[rustfmt::skip]
pub(crate) const ENTRIES: &[PluginEntry] = &[
    midside::ENTRY,
    mojo::ENTRY,
];
