//! Batch 1 list lines 25-32.
mod biquaddouble;
mod biquadonehalf;
mod biquadplus;
mod biquadtriple;
mod bite;
mod bitglitter;
mod bitshiftgain;
mod blockparty;
use crate::registry::PluginEntry;
pub(crate) const ENTRIES: &[PluginEntry] = &[
    biquaddouble::ENTRY,
    biquadonehalf::ENTRY,
    biquadplus::ENTRY,
    biquadtriple::ENTRY,
    bite::ENTRY,
    bitglitter::ENTRY,
    bitshiftgain::ENTRY,
    blockparty::ENTRY,
];
