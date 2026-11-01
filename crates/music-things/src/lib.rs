//! Facade crate for the music-things workspace.
//!
//! Each DSP family lives in its own crate under `crates/`; this crate
//! re-exports them so hosts can depend on a single package. Member crates
//! are dependency-free, host-agnostic and build for `wasm32-unknown-unknown`.
//!
//! Optional features `airwindows`, `soundfont`, and `pitch` enable the
//! corresponding crate re-exports. No features are enabled by default.
//!
//! ```
//! # #[cfg(feature = "airwindows")]
//! assert!(music_things::airwindows::catalog().next().is_some());
//! ```

#[cfg(feature = "airwindows")]
pub use music_things_airwindows as airwindows;
#[cfg(feature = "pitch")]
pub use music_things_pitch as pitch;
#[cfg(feature = "soundfont")]
pub use music_things_soundfont as soundfont;
