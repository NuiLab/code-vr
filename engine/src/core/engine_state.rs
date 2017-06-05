use config::Config;
use std::sync::Arc;
use input::InputSystem;
use renderer::GraphicsState;

/// Proxy to the current state of the Engine
pub struct EngineState {
    /// Unique ID for actor recieving engine state.
    pub id: u64,

    pub input: Arc<InputSystem>,

    /// Graphics State (Cameras, Geometry, etc.)
    pub gfx: Arc<GraphicsState>,

    /// Game Engine State (Resolution, quality, inputs, etc.)
    pub config: Arc<Config>,
}

impl EngineState {

    /// Get the value of an input axis
    pub fn input_axis(&self, key: String) -> f32 {
        self.input.inputs.get(&key).unwrap_or(&0.0).clone()
    }
}
