use cgmath::Matrix4;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::descriptor::descriptor_set::DescriptorSet;
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::pipeline::GraphicsPipelineAbstract;
use std::sync::Arc;
use renderer::vulkan::CameraUbo;

pub struct NodeUbo {
    pub model: [[f32; 4]; 4],
}

pub struct Node {
    pub ubo: Arc<CpuAccessibleBuffer<NodeUbo>>,
    pub transform_descriptor: Arc<DescriptorSet>,
}

impl Node {
    pub fn new(
        device: &Arc<Device>,
        queue: &Arc<Queue>,
        model: Matrix4<f32>,
        pipeline: Arc<GraphicsPipelineAbstract>,
        cam_ubo: Arc<CpuAccessibleBuffer<CameraUbo>>,
    ) -> Node {

        let ubo = CpuAccessibleBuffer::<NodeUbo>::from_data(
            device.clone(),
            BufferUsage::all(),
            Some(queue.family()),
            NodeUbo { model: model.into() },
        ).expect("Failed to create Camera UBO");
        Node {
            transform_descriptor: Arc::new(simple_descriptor_set!(pipeline.clone(), 0, {
                model: ubo.clone(),
                camera: cam_ubo.clone()
            })),
            ubo,
        }
    }
}
