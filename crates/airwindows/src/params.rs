/// Metadata for a normalized parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParamInfo {
    pub name: &'static str,
    pub label: &'static str,
    pub default: f32,
}
/// Metadata for an effect.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PluginInfo {
    pub name: &'static str,
    pub upstream_name: &'static str,
    pub description: &'static str,
    pub params: &'static [ParamInfo],
}
pub(crate) fn sanitize_param(value: f32) -> Option<f32> {
    if value.is_nan() {
        None
    } else {
        Some(value.clamp(0.0, 1.0))
    }
}
