use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::descriptor::descriptor_set::DescriptorSet;
use vulkano::buffer::BufferUsage;
use vulkano::device::{Device, Queue};

use std::sync::Arc;

mod camera;
mod node;

pub use self::camera::*;
pub use self::node::*;

pub struct VulkanGraphicsState {
    pub nodes: Vec<Node>,
    pub cameras: Vec<Camera>,
    pub primitives: Vec<Primitive>,
    pub materials: Vec<Material>,
}

pub struct Primitive {
    vbo: CpuAccessibleBuffer<f32>,
    ibo: CpuAccessibleBuffer<i16>,
}

pub struct Material {
    pipeline: u32,
}

impl VulkanGraphicsState {
    pub fn new() -> VulkanGraphicsState {
        VulkanGraphicsState {
            primitives: Vec::new(),
            nodes: Vec::new(),
            cameras: Vec::new(),
            materials: Vec::new(),
        }
    }
}
