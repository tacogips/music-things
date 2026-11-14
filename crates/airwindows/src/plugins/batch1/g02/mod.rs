//! Batch 1 list lines 9-16.
use crate::registry::PluginEntry;
mod atmospherebuss;
mod atmospherechannel;
mod aura;
mod autopan;
mod average;
mod avermatrix;
mod balanced;
mod bassamp;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    atmospherebuss::ENTRY,
    atmospherechannel::ENTRY,
    aura::ENTRY,
    autopan::ENTRY,
    average::ENTRY,
    avermatrix::ENTRY,
    balanced::ENTRY,
    bassamp::ENTRY,
];
