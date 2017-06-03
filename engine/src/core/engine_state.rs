use config::Config;
use std::sync::Arc;
use input::InputSystem;
use renderer::RenderState;

/// Shared pointers to Engine state and Actor specific metadata.
pub struct EngineState {

  // Unique ID for actor recieving engine state.
  pub id: u64,

  pub input: Arc<InputSystem>,

  // Graphics State (Cameras, Geometry, etc.)
  pub gfx: Arc<RenderState>,

  // Audio State 
  //audio: Arc<u32>,

  // Physics (World settings, Collision data, etc.)
  //physics: Arc<u32>,

  // Game Engine State (Resolution, quality, inputs, etc.)
  pub config: Arc<Config>,

}

/*
@TODO, Proxys to the internal engine state 

struct Input {

}

struct Graphics {

}
*/
