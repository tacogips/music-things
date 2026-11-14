//! Each plugin module defines const INFO and ENTRY, and a construct(sample_rate, seed) function.
//! ENTRY references INFO; arrays longer than 4096 elements use Box<[T]>.
use crate::registry::PluginEntry;
mod batch1;
mod batch2;
mod batch3;
pub(crate) const BATCHES: &[&[&[PluginEntry]]] = &[batch1::GROUPS, batch2::GROUPS, batch3::GROUPS];
