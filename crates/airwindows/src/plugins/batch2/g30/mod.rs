//! Batch 2 list lines 200.
use crate::registry::PluginEntry;
mod nonlinearspace;
pub(crate) const ENTRIES: &[PluginEntry] = &[nonlinearspace::ENTRY];
