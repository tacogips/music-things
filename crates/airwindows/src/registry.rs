use crate::{AirwindowsEffect, PluginInfo};
pub(crate) struct PluginEntry {
    pub(crate) info: &'static PluginInfo,
    pub(crate) construct: fn(f32, u32) -> Box<dyn AirwindowsEffect>,
}
/// Default deterministic seed.
pub const DEFAULT_SEED: u32 = 0x00c0_ffee;
/// Construct an effect using the default seed.
pub fn create(name: &str, sample_rate: f32) -> Option<Box<dyn AirwindowsEffect>> {
    create_with_seed(name, sample_rate, DEFAULT_SEED)
}
/// Construct an effect by either its catalog or upstream name.
pub fn create_with_seed(
    name: &str,
    sample_rate: f32,
    seed: u32,
) -> Option<Box<dyn AirwindowsEffect>> {
    crate::plugins::BATCHES
        .iter()
        .flat_map(|g| g.iter())
        .flat_map(|e| e.iter())
        .find(|e| e.info.name == name || e.info.upstream_name == name)
        .map(|e| (e.construct)(sample_rate, seed))
}
/// Iterate all registered metadata in batch and group order.
pub fn catalog() -> impl Iterator<Item = &'static PluginInfo> {
    crate::plugins::BATCHES
        .iter()
        .flat_map(|g| g.iter())
        .flat_map(|e| e.iter())
        .map(|e| e.info)
}
