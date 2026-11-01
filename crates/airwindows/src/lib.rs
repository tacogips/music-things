//! Host-independent Airwindows effects ported from the MIT-licensed upstream project.
//! Display formatting uses the clean-room D1 specification; seeded randomness follows D5.
//! Only float processing is provided (D6).
#![allow(dead_code)]
mod display;
mod dither;
mod effect;
mod params;
mod plugins;
mod rand;
mod registry;
pub use effect::AirwindowsEffect;
pub use params::{ParamInfo, PluginInfo};
pub use registry::{catalog, create, create_with_seed, DEFAULT_SEED};
