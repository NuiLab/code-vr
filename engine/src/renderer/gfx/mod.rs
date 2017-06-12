/*!
# CodeVR Graphics

CodeVR is powered by a data driven renderer. Actors in the engine add graphics objects 
to the GraphicsState, which in turn generates an API specific graphics state and
communicates between this intermediary data layer.

An actor accesses the `gfx` state from their EngineState struct, and calls
methods that return shared pointers with mutexes to the resource that they want to mutate.

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

During the engine's polling step, it checks if there's only one pointer to a resource, 
if there's only one, it will deallocate that resource, otherwise 
*/
mod camera;
mod mesh;
mod text;

use std::sync::{Arc, Mutex};
pub use self::camera::*;
use std::collections::HashMap;

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
