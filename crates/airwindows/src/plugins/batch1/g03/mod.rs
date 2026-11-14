//! Batch 1 list lines 17-24.
use crate::registry::PluginEntry;
mod bassdrive;
mod basskit;
mod baxandall;
mod baxandall2;
mod beam;
mod bigamp;
mod biquad;
mod biquad2;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    bassdrive::ENTRY,
    basskit::ENTRY,
    baxandall::ENTRY,
    baxandall2::ENTRY,
    beam::ENTRY,
    bigamp::ENTRY,
    biquad::ENTRY,
    biquad2::ENTRY,
];
