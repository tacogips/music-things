//! Facade crate for the music-things workspace.
//!
//! Each DSP family lives in its own crate under `crates/`; this crate
//! re-exports them so hosts can depend on a single package. Member crates
//! are dependency-free, host-agnostic and build for `wasm32-unknown-unknown`.
//!
//! Optional features enable the corresponding crate re-exports: `dsp`,
//! `synth` and `fx` (the kernels extracted from vactr) and `airwindows`,
//! `soundfont` and `pitch` (ports of the permissive DSP bundled with the
//! Behringer KICK and IGNITE firmware). No features are enabled by default.
//!
//! ```
//! # #[cfg(feature = "airwindows")]
//! assert!(music_things::airwindows::catalog().next().is_some());
//! ```

#[cfg(feature = "airwindows")]
pub use music_things_airwindows as airwindows;
#[cfg(feature = "dsp")]
pub use music_things_dsp as dsp;
#[cfg(feature = "fx")]
pub use music_things_fx as fx;
#[cfg(feature = "pitch")]
pub use music_things_pitch as pitch;
#[cfg(feature = "soundfont")]
pub use music_things_soundfont as soundfont;
#[cfg(feature = "synth")]
pub use music_things_synth as synth;
