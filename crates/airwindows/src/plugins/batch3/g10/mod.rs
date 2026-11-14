//! Batch 3 list lines 293-300.

use crate::registry::PluginEntry;

mod tube;
mod tube2;
mod tubedesk;
mod ulawdecode;
mod ulawencode;
mod ultrasonic;
mod ultrasoniclite;
mod ultrasonicmed;

pub(crate) const ENTRIES: &[PluginEntry] = &[
    tube::ENTRY,
    tube2::ENTRY,
    tubedesk::ENTRY,
    ulawdecode::ENTRY,
    ulawencode::ENTRY,
    ultrasonic::ENTRY,
    ultrasoniclite::ENTRY,
    ultrasonicmed::ENTRY,
];
