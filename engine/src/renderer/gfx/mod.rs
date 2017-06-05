/*!
# CodeVR Graphics

CodeVR is powered by a data driven renderer. Actors in the engine add graphics objects 
to the RenderState, which generate vulkan data structures that can be mutated by actors.

The renderer then traverses the render state, and generates command buffers that are
sent to Vulkan's rendering queue.

An actor accesses the `gfx` state from their EngineState struct, and calls
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

use vulkano_win::{Window};
use std::sync::{Arc, Mutex};
use config::Config;
pub use self::camera::*;

/// Centralized Graphics Store
pub struct GraphicsState {
  pub buffers: Vec<u32>,
  pub buffer_views: Vec<u32>,
  pub images: Vec<u32>,
  pub textures: Vec<u32>,
  pub shaders: Vec<u32>,
  pub pipelines: Vec<u32>,
  pub cameras: Vec<Arc<Mutex<Camera>>>,
  pub nodes: Vec<u32>,
  pub meshes: Vec<u32>,
}

impl GraphicsState {
  pub fn new() -> GraphicsState {
    GraphicsState {
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
}