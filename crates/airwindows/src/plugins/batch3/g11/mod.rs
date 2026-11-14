//! Batch 3 list lines 301-308.

use crate::registry::PluginEntry;

mod ultrasonx;
mod unbox;
mod varimu;
mod verbity;
mod verbity2;
mod vibrato;
mod vinyldither;
mod voiceofthestarship;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    ultrasonx::ENTRY,
    unbox::ENTRY,
    varimu::ENTRY,
    verbity::ENTRY,
    verbity2::ENTRY,
    vibrato::ENTRY,
    vinyldither::ENTRY,
    voiceofthestarship::ENTRY,
];
