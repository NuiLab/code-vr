pub struct App {

}

impl App {
  pub fn new() -> App {
    App {}
  }

  fn update(&self) {
    /*
    // Inputmap has Iterator trait to match appropriate inptus.
    for input in self.inputmap {
      match input {
        ("move_right", x) => {
          self.hp += 1.;
        },
        ("move_down", y) => ()
      }
    }
    */
  }

  fn render(&self) {
    // Option 1 - Create command buffers here.
    // Option 2 create description of what you want to render w/ gltf like declarations.
    // The renderer is responsable for taking the render tree and splitting it into threads to create command buffers to render.
  }
}