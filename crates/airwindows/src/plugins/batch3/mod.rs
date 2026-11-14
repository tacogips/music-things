//! Owned by the batch-3 run.

mod g01;
mod g02;
mod g03;
mod g04;
mod g05;
mod g06;
mod g07;
mod g08;
mod g09;
mod g10;
mod g11;
mod g12;
mod g13;
mod g14;

use crate::registry::PluginEntry;

pub(crate) const GROUPS: &[&[PluginEntry]] = &[
    g01::ENTRIES,
    g02::ENTRIES,
    g03::ENTRIES,
    g04::ENTRIES,
    g05::ENTRIES,
    g06::ENTRIES,
    g07::ENTRIES,
    g08::ENTRIES,
    g09::ENTRIES,
    g10::ENTRIES,
    g11::ENTRIES,
    g12::ENTRIES,
    g13::ENTRIES,
    g14::ENTRIES,
];
