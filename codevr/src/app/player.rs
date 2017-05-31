use engine::Actor;
use std::sync::Arc;
 
pub struct Player {
  camera: Option<u32>
}

impl Player {
    pub fn new() -> Arc<Player> {
    Arc::new(Player {
      camera: None
    })
  }
}

impl Actor for Player {

  fn start(&mut self) {
    /*
    let Engine { gfx } = self.engine();

    // Add reference to camera

    self.camera = Some(gfx.camera(
      CameraProps {
        ...
      }
    ));
    */
  }

  fn update(&mut self) {
    println!("Updating the player");
    /*
    let Engine { input, scene } = self.engine();

    match input {
      ("look_right", x) => {
        self.camera.ok().rotate_x(x);
      },
      ("look_up", y) => {
        self.camera.ok().rotate_y(y);
      },
      ("add_block", z) => {
        scene.add(Block::new());
      },
      _ => ()
    }
    */
  }
  /*
  fn render(&mut self, gfx: GraphicsState) {

  }
  */
}