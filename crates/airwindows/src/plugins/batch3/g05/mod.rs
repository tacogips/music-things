//! Batch 3 list lines 253-260.

use crate::registry::PluginEntry;
mod slew;
mod slew2;
mod slew3;
mod slewonly;
mod smooth;
mod softgate;
mod spatializedither;
mod spiral;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    slew::ENTRY,
    slew2::ENTRY,
    slew3::ENTRY,
    slewonly::ENTRY,
    smooth::ENTRY,
    softgate::ENTRY,
    spatializedither::ENTRY,
    spiral::ENTRY,
];
