use config::Config;
use std::sync::Arc;
use input::InputSystem;
use renderer::RenderState;

/// Proxy to the current state of the Engine
pub struct EngineState {

  /// Unique ID for actor recieving engine state.
  pub id: u64,

  pub input: Arc<InputSystem>,

  /// Graphics State (Cameras, Geometry, etc.)
  pub gfx: Arc<RenderState>,

  /// Game Engine State (Resolution, quality, inputs, etc.)
  pub config: Arc<Config>,

}