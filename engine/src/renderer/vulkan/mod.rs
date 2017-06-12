use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::descriptor::descriptor_set::DescriptorSet;
use vulkano::buffer::BufferUsage;
use vulkano::device::{Device, Queue};

use std::sync::Arc;

pub struct VulkanGraphicsState {
    pub nodes: Vec<Node>,
    pub cameras: Vec<Camera>,
    pub primitives: Vec<Primitive>,
    pub materials: Vec<Material>,
}

pub struct CameraUbo {
    pub view: [[f32; 4]; 4],
    pub projection: [[f32; 4]; 4],
}

pub struct Camera {
    /// Camera view/projection matricies
    pub ubo: Arc<CpuAccessibleBuffer<CameraUbo>>,
}

impl Camera {
    pub fn new(device: &Arc<Device>,
               queue: &Arc<Queue>,
               view: [[f32; 4]; 4],
               projection: [[f32; 4]; 4])
               -> Camera {
        Camera {
            ubo: CpuAccessibleBuffer::<CameraUbo>::from_data(device.clone(),
                                                             BufferUsage::all(),
                                                             Some(queue.family()),
                                                             CameraUbo { view, projection })
                .expect("Failed to create Camera UBO"),
        }
    }
}

pub struct Node {
    /// Model position/rotation/scale matrix
    pub ubo: CpuAccessibleBuffer<u32>,
    pub transform_descriptor: Box<DescriptorSet>,
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
