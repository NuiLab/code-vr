use engine::{Actor, EngineState};
use engine::gfx::{Camera};
use std::sync::Arc;

 /// A Player in the CodeVR scene, handles its own movement and editing the CodeVR scene.
pub struct Player {
  // Local State
  health: [i32; 2],
  // Engine State
  camera: Option<Arc<Camera>>,
  engine: Option<EngineState>
}

/// Local Logic
impl Player {
  pub fn new() -> Player {
    Player {
      health: [100, 100],
      camera: None,
      engine: None
    }
  }
}

/// Actor Logic
impl Actor for Player {

  // Mount engine state  
  fn start(&mut self, engine: EngineState) {
  
    self.engine = Some(engine);
  
    // Add reference to camera
    self.camera = Some(self.engine.gfx.camera());

  }

  // Update Engine State
  fn update(&mut self) {

    if let Some(engine) = self.engine {
      
      // Destroy self if we're out of health
      if self.health[0] < 1 {
        engine.scene.destroy(|actor| true );
      }

      // Check inputs
      for input in engine.input.iter() {

        match input {
          ("look_right", x) => {
            if let Some(camera) = self.camera {
              camera.rotate_x(x);
            }
          },
          ("look_up", y) => {
            if let Some(camera) = self.camera {
              camera.ok().rotate_y(y);
            }
          },
          ("add_block", z) => {
            //scene.add(Block::new());
          },
          _ => ()
        }

      }

    }

  }
}