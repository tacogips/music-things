//! Batch 1 list lines 33-40.
use crate::registry::PluginEntry;
mod brassrider;
mod brightambience;
mod brightambience2;
mod brightambience3;
mod buildatpdf;
mod busscolors4;
mod buttercomp;
mod buttercomp2;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    brassrider::ENTRY,
    brightambience::ENTRY,
    brightambience2::ENTRY,
    brightambience3::ENTRY,
    buildatpdf::ENTRY,
    busscolors4::ENTRY,
    buttercomp::ENTRY,
    buttercomp2::ENTRY,
];
