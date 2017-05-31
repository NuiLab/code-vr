/*!
# CodeVR Graphics

CodeVR is powered by a data driven renderer. Actors in the engine add graphics objects 
to the RenderState, which generate vulkan data structures and can be mutated by actors.

The renderer then traverses the render state, and generates command buffers that are
sent to Vulkan's rendering queue.

*/
mod camera;
mod text;

use self::camera::Camera;

struct RenderState {
  camera: Camera,
  shaders: u32,
  textures: u32
}

impl RenderState {

  /// Creates a camera
  pub fn camera() {

  }

}