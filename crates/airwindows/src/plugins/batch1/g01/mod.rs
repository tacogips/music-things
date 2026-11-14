//! Batch 1 list lines 1-8.
use crate::registry::PluginEntry;
mod acceleration;
mod acceleration2;
mod adclip7;
mod adt;
mod air;
mod air2;
mod apicolypse;
mod aquickvoiceclip;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    acceleration::ENTRY,
    acceleration2::ENTRY,
    adclip7::ENTRY,
    adt::ENTRY,
    air::ENTRY,
    air2::ENTRY,
    apicolypse::ENTRY,
    aquickvoiceclip::ENTRY,
];
