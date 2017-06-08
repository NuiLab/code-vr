use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::descriptor::descriptor_set::DescriptorSet;

pub struct VulkanGraphicsState {
  pub nodes: Vec<Node>,
  pub cameras: Vec<Camera>,
  pub primitives: Vec<Primitive>,
  pub materials: Vec<Material>
}

pub struct CameraUbo {
  pub view: [[f32; 4]; 4],
  pub projection: [[f32; 4]; 4]
}

pub struct Camera {
  /// Camera view/projection matricies
  pub ubo: CpuAccessibleBuffer<CameraUbo>
}

pub struct Node {
  /// Model position/rotation/scale matrix
  pub ubo: CpuAccessibleBuffer<u32>,
  pub transform_descriptor: Box<DescriptorSet>
}

pub struct Primitive {
  vbo: CpuAccessibleBuffer<f32>,
  ibo: CpuAccessibleBuffer<i16>
}

pub struct Material {
  pipeline: u32
}

impl VulkanGraphicsState {
  pub fn new() -> VulkanGraphicsState {
    VulkanGraphicsState {
      primitives: Vec::new(),
      nodes: Vec::new(),
      cameras: Vec::new(),
      materials: Vec::new()
    }
  }
}