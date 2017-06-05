use config::Config;
use std::sync::{Arc, Mutex};
use input::InputSystem;
use renderer::GraphicsState;

/// Proxy to the current state of the Engine
#[derive(Clone)]
pub struct EngineState {
    /// Unique ID for actor recieving engine state.
    id: u64,

    input: Arc<Mutex<InputSystem>>,

    /// Graphics State (Cameras, Geometry, etc.)
    pub gfx: Arc<Mutex<GraphicsState>>,

    /// Game Engine State (Resolution, quality, inputs, etc.)
    config: Arc<Config>
}

impl EngineState {

    pub fn new(id: u64, config: Arc<Config>, input: Arc<Mutex<InputSystem>>, gfx: Arc<Mutex<GraphicsState>> ) -> EngineState {
        EngineState {
            id,
            input,
            gfx,
            config
        }
    }

    pub fn id(self) -> u64 {
        self.id
    }

    /// Get the value of an input axis
    pub fn input_axis(&self, key: String) -> f32 {
        match self.input.lock() {
            Ok(input_guard) => {
                let input_system = &*input_guard;
                input_system.inputs.get(&key).unwrap_or(&0.0).clone()
                },
            _ => 0.0
        } 
    }
}
