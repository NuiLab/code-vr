use engine::{Actor, EngineState};
use engine::gfx::{Camera, CameraProps, ProjectionMode};
use std::sync::Arc;

/// A Player in the CodeVR scene, handles its own movement and editing the CodeVR scene.
pub struct Player {
    // Local State
    health: [i32; 2],
    // Engine State
    camera: Option<Camera>,
    engine: Option<EngineState>,
}

/// Local Logic
impl Player {
    pub fn new() -> Player {
        Player {
            health: [100, 100],
            camera: None,
            engine: None,
        }
    }
}

/// Actor Logic
impl Actor for Player {

    // Mount engine state
    fn start(&mut self, engine: EngineState) {

        self.engine = Some(engine);

        // Add reference to camera
        self.camera = Some(self.engine.unwrap().gfx.camera(CameraProps {
                                                               projection_mode: ProjectionMode::Perspective,
                                                               to: [4., 4., 4.],
                                                               from: [0., 0., 0.],
                                                               fov: 75.0,
                                                           }));

    }

    // Update Engine State
    fn update(&mut self) {

        if let Some(engine) = self.engine {

            // Destroy self if we're out of health
            if self.health[0] < 1 {
                //engine.scene.destroy(|actor| true);
            }

            // Check inputs
            let cam_x = engine.input_axis(String::from("look_right"));
            let cam_y = engine.input_axis(String::from("look_up"));
            //self.camera.unwrap().rotate(cam_x, cam_y, 0.);

        }

    }
}
