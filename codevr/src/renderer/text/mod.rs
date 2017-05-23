/*!
# Vulkano Font Renderer

A method of using multi-channel signed distance fields to render text in Vulkan, built on top of a fork of [msdfgen](https://github.com/Chlumsky/msdfgen).

Given a:
1. msdf bitmap + metadata of that bitmap (UTF8 char to vec2 map, dimensions of characters)
2. A UTF-8 String
4. Transformation/Color data

We generate a VBO, which can be drawn with our graphics pipeline implementation or your own.

## Usage:

```rust,ignore
extern crate vulkano_glyphs;

use vulkano_glyphs::{TextRenderer};

fn main() {

  // ... After initializing vulkan constructs

  // Create a text renderer to handle 
  // font textures/shaders/descriptor set layouts

  let text_renderer = TextRenderer::new(device, queue);


  // Load font with path of file
  // The renderer creates a msdf texture + char to vec2 map.

  text_renderer.font("arial.ttf", "Arial", (32, 127));


  // Send texture data to device memory, fences GPU memory.
  text_renderer.allocate(cmd);

  // Create a VBO Generator for your font
  let builder = text_renderer.builder("Arial", cmd);

  // Start creating VBOs + Command Buffer
  // You recieve the VBO's lifetime
  let vbo = builder.text("Hello World");

  // Submit that command buffer to render that text.
  queue.submit(cmd);

}
```

*/

use vulkano::image::immutable::ImmutableImage;
use vulkano::buffer::{BufferUsage, DeviceLocalBuffer};
use vulkano::sampler::Sampler;


use std::collections::HashMap;
use std::path::Path;

mod vs { include!{concat!(env!("OUT_DIR"), "/renderer/text/shaders/text_vs.glsl")} }
mod fs { include!{concat!(env!("OUT_DIR"), "/renderer/text/shaders/text_fs.glsl")} }

struct Font {
  texture: Vec<u8>,
  char_to_vec2: HashMap<char, [u16; 2]>,
  char_size: [u8; 2]
}

struct VulkanFont {
  sampler: Sampler,
  pipeline: u8,
  vbo: Vec<f32>,
}

pub struct TextRenderer {
  fonts: HashMap<String, Font>
}

impl TextRenderer {

  pub fn new(device: u8, queue: u8) -> TextRenderer {

    // Create Frag/Vert vulkan data

    TextRenderer {
      fonts: HashMap::new()
    }
  }

  /// Builds a font and adds it to the renderer.
  pub fn font(path: String, name: String, char_range: (u16, u16)) {

  }

  /// Allocates all fonts in GPU memory.
  pub fn allocate(cmd: u8) {

  }

}