//! Facade crate for the music-things workspace.
//!
//! Each DSP family lives in its own crate under `crates/`; this crate
//! re-exports them so hosts can depend on a single package. Member crates
//! are dependency-free, host-agnostic and build for `wasm32-unknown-unknown`.
