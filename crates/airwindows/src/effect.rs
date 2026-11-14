use crate::PluginInfo;
/// A stereo Airwindows effect using the upstream float processing path.
pub trait AirwindowsEffect: Send {
    /// Metadata for this effect.
    fn info(&self) -> &'static PluginInfo;
    /// Number of normalized parameters.
    fn param_count(&self) -> usize {
        self.info().params.len()
    }
    /// Metadata for a parameter, if present.
    fn param_info(&self, index: usize) -> Option<&'static crate::ParamInfo> {
        self.info().params.get(index)
    }
    /// Current normalized value.
    fn get_param(&self, index: usize) -> f32;
    /// Set a normalized value.
    fn set_param(&mut self, index: usize, value: f32);
    /// Display the parameter value.
    fn param_display(&self, index: usize) -> String;
    /// Unit label for a parameter.
    fn param_label(&self, index: usize) -> &'static str {
        self.param_info(index).map_or("", |p| p.label)
    }
    /// Restore the initialized state.
    fn reset(&mut self);
    /// Process the minimum of the four slice lengths, without allocation, matching processReplacing.
    fn process(&mut self, in_l: &[f32], in_r: &[f32], out_l: &mut [f32], out_r: &mut [f32]);
}
