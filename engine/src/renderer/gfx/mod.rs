/*!
# CodeVR Graphics

CodeVR is powered by a data driven renderer. Actors in the engine add graphics objects 
to the RenderState, which generate vulkan data structures and can be mutated by actors.

The renderer then traverses the render state, and generates command buffers that are
sent to Vulkan's rendering queue.

An actor accesses the `gfx` state from their `.engine()` accessor method, and calls
methods that return shared pointers to the resource that they can mutate.

```
// Add a camera
let cam = gfx.camera(
  CameraProps {
    ...
  }
);

let img = gfx.image("texture.png");
let tex = gfx.texture(img);
let text = gfx.text("Hello World");
```
*/
mod camera;
mod mesh;
mod text;

use std::sync::Arc;
use config::Config;
use self::camera::Camera;

pub struct RenderState {
  config: Arc<Config>,
  buffers: Vec<u32>,
  buffer_views: Vec<u32>,
  images: Vec<u32>,
  textures: Vec<u32>,
  shaders: Vec<u32>,
  pipelines: Vec<u32>,
  cameras: Vec<Camera>,
  nodes: Vec<u32>,
  meshes: Vec<u32>,
}

impl RenderState {
  pub fn new(config: Arc<Config>) -> RenderState {
    RenderState {
      config,
      buffers: Vec::new(),
      buffer_views: Vec::new(),
      images: Vec::new(),
      textures: Vec::new(),
      shaders: Vec::new(),
      pipelines: Vec::new(),
      cameras: Vec::new(),
      nodes: Vec::new(),
      meshes: Vec::new(),
    }
  }

  pub fn render(&mut self)  {
    for camera in self.cameras {
      // Set a camera descriptor set with it's data...
      
    }
  }
}