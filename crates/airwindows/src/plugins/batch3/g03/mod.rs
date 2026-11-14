//! Batch 3 list lines 237-244.

use crate::registry::PluginEntry;

mod purestwarm;
mod purestwarm2;
mod pyewacket;
mod rawglitters;
mod rawtimbers;
mod recurve;
mod remap;
mod reseq;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    purestwarm::ENTRY,
    purestwarm2::ENTRY,
    pyewacket::ENTRY,
    rawglitters::ENTRY,
    rawtimbers::ENTRY,
    recurve::ENTRY,
    remap::ENTRY,
    reseq::ENTRY,
];
