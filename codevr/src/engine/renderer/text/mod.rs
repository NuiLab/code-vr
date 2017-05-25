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

use vulkano::device::Device;
use vulkano::image::immutable::ImmutableImage;
use vulkano::buffer::{BufferUsage, DeviceLocalBuffer};
use vulkano::sampler::{Filter, Sampler, MipmapMode, SamplerAddressMode};
use vulkano::pipeline::viewport::{Viewport};
use cgmath::{Matrix4, SquareMatrix};

use std::sync::Arc;
use std::collections::HashMap;
use std::path::Path;

/// Core Text Vertex Shader
pub mod vs { include!{concat!(env!("OUT_DIR"), "/shaders/src/engine/renderer/text/shaders/text_vs.glsl")} }

/// Core Text Fragment Sahder
pub mod fs { include!{concat!(env!("OUT_DIR"), "/shaders/src/engine/renderer/text/shaders/text_fs.glsl")} }

/// Core Text Pipeline Layout
/*
mod pipeline_layout {
    use engine::renderer::text::fs;
    pipeline_layout!{
        set0: {
            transforms: UniformBuffer<fs::ty::Block>,
            tex: CombinedImageSampler
        }
    }
}*/

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
  fonts: HashMap<String, Font>,
  device: Arc<Device>
}

impl TextRenderer {

  pub fn new(device: Arc<Device>, queue: u8) -> TextRenderer {

    // Core Shaders
    let vs = vs::Shader::load(&device).expect("failed to create shader module");

    let fs = fs::Shader::load(&device).expect("failed to create shader module");
/*
    // Core Sampler
    let sampler = Sampler::new(&device, Filter::Linear,
                                                 Filter::Linear, MipmapMode::Nearest,
                                                 SamplerAddressMode::Repeat,
                                                 SamplerAddressMode::Repeat,
                                                 SamplerAddressMode::Repeat,
                                                 0.0, 1.0, 0.0, 0.0).unwrap();

    let uniform_buffer = {
        CpuAccessibleBuffer::<fs::ty::Block>::from_data(&device,
                                                        &BufferUsage::all(),
                                                        Some(queue.family()),
                                                        vs::ty::Data {
                                    world : <Matrix4<f32> as SquareMatrix>::identity().into(),
                                    view : (view * scale).into(),
                                    proj : proj.into(),
                                })
                .expect("failed to create Uniform Buffer")
    };

    let pipeline_layout = pipeline_layout::CustomPipeline::new(&device).unwrap();
    
    let set = pipeline_layout::set0::Set::new(&descriptor_pool,
                                            &pipeline_layout,
                                            &pipeline_layout::set0::Descriptors {
                                                   transforms: &uniform_buffer,
                                                   tex: (&sampler, &texture)
                                               });

    let pipeline = vulkano::pipeline::GraphicsPipeline::new(&device, vulkano::pipeline::GraphicsPipelineParams {
        vertex_input: vulkano::pipeline::vertex::SingleBufferDefinition::new(),
        vertex_shader: vs.main_entry_point(),
        input_assembly: vulkano::pipeline::input_assembly::InputAssembly {
            topology: vulkano::pipeline::input_assembly::PrimitiveTopology::TriangleStrip,
            primitive_restart_enable: false,
        },
        tessellation: None,
        geometry_shader: None,
        viewport: vulkano::pipeline::viewport::ViewportsState::Fixed {
            data: vec![(
                Viewport {
                    origin: [0.0, 0.0],
                    depth_range: 0.0 .. 1.0,
                    dimensions: [images[0].dimensions()[0] as f32, images[0].dimensions()[1] as f32],
                },
                vulkano::pipeline::viewport::Scissor::irrelevant()
            )],
        },
        raster: Default::default(),
        multisample: vulkano::pipeline::multisample::Multisample::disabled(),
        fragment_shader: fs.main_entry_point(),
        depth_stencil: vulkano::pipeline::depth_stencil::DepthStencil::disabled(),
        blend: vulkano::pipeline::blend::Blend::pass_through(),
        layout: &pipeline_layout,
        render_pass: vulkano::framebuffer::Subpass::from(&renderpass, 0).unwrap(),
    }).unwrap();

    let pipeline_layout = pipeline_layout::CustomPipeline::new(&device).unwrap();
*/
    TextRenderer {
      fonts: HashMap::new(),
      device
    }
  }

  /// Builds a font and adds it to the renderer.
  pub fn font(&mut self, path: String, name: String, char_range: (u16, u16)) {
    //self.device.
    //let font_img_buffer = msdfgen.run(path, char_range);
    //let font = Font {
      
    //}
    //self.fonts.insert(name, font);
/*
        let pixel_buffer = {
        let image = image::load_from_memory_with_format(include_bytes!("image_img.png"),
                                                        image::ImageFormat::PNG).unwrap().to_rgba();
        let image_data = image.into_raw().clone();

        let image_data_chunks = image_data.chunks(4).map(|c| [c[0], c[1], c[2], c[3]]);

        // TODO: staging buffer instead
        vulkano::buffer::cpu_access::CpuAccessibleBuffer::<[[u8; 4]]>
            ::from_iter(&device, &vulkano::buffer::BufferUsage::all(),
                        Some(queue.family()), image_data_chunks)
                        .expect("failed to create buffer")
    };

    let sampler = vulkano::sampler::Sampler::new(&device, vulkano::sampler::Filter::Linear,
                                                 vulkano::sampler::Filter::Linear, vulkano::sampler::MipmapMode::Nearest,
                                                 vulkano::sampler::SamplerAddressMode::Repeat,
                                                 vulkano::sampler::SamplerAddressMode::Repeat,
                                                 vulkano::sampler::SamplerAddressMode::Repeat,
                                                 0.0, 1.0, 0.0, 0.0).unwrap();


*/
  }

  /// Allocates all fonts in GPU memory. 
  pub fn allocate(&mut self, cmd: u8) {
    // Traverse local font store
    for (string_key, font) in self.fonts.iter() {

      }
  }

}