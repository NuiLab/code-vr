/*!
# Vulkano Glyphs

A method of using multi-channel signed distance fields to render text in Vulkan, built on top of a fork of [msdfgen](https://github.com/Chlumsky/msdfgen).

Given a:
1. String
2. msdf bitmap
3. metadata of that bitmap (UTF8 char to vec2 hashmap, dimensions of characters)
4. Optional Textbox (color, alignment, transformation) 

We generate a VBO, which can be drawn with our graphics pipeline implementation or your own.

## Usage:

```rust,ignore
extern crate vulkano_glyphs;

use vulkano_glyphs::{TextBuilder};

fn main() {

  // ... After initializing vulkan constructs

  // Create a text builder
  let builder = TextBuilder::new(msdf, None);

  // Start adding text
  let cmds = builder
      .text("Hello World!")
      .text("Haha!")
      .build();

  queue.submit(cmds);

}
```

*/

use vulkano::image::immutable::ImmutableImage;

struct TextBuilder {

}

impl TextBuilder {
  pub fn new(msdf: u8, meta: Option<u8>) -> TextBuilder {
    TextBuilder {}
  }

  /// Build VBO from string and TextBuilder data
  pub fn text(string: String) {

  }

  /// Builds command buffers from text input.
  pub fn build() {

  }
}

/// Convert a .ttf font to a distance field bitmap that can be cached or saved as an image.
pub fn load_font(file_path: String, range: (u32, u32)) {
  // @TODO, msdfgen ffi?
}

/// Takes a distance field image and places it in GPU memory, giving a handle to that texture
fn push_dfimage(msdf: u8, cmd: vulkano::command_buffer::PrimaryCommandBufferBuilder) {
  //let texture = ImmutableImage::new();
  //cmd.copy_buffer_to_color_image();
}
