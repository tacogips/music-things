//! Batch 1 list lines 49-56.
use crate::registry::PluginEntry;
mod channel4;
mod channel5;
mod channel6;
mod channel7;
mod channel8;
mod channel9;
mod chorus;
mod chorusensemble;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    channel4::ENTRY,
    channel5::ENTRY,
    channel6::ENTRY,
    channel7::ENTRY,
    channel8::ENTRY,
    channel9::ENTRY,
    chorus::ENTRY,
    chorusensemble::ENTRY,
];
