use cgmath::Matrix4;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::device::{Device, Queue};
use std::sync::Arc;

pub struct CameraUbo {
    pub view: [[f32; 4]; 4],
    pub projection: [[f32; 4]; 4],
}

pub struct Camera {
    /// Camera view/projection matricies
    pub ubo: Arc<CpuAccessibleBuffer<CameraUbo>>,
}

impl Camera {
    pub fn new(
        device: &Arc<Device>,
        queue: &Arc<Queue>,
        view: Matrix4<f32>,
        projection: Matrix4<f32>,
    ) -> Camera {
        Camera {
            ubo: CpuAccessibleBuffer::<CameraUbo>::from_data(
                device.clone(),
                BufferUsage::all(),
                Some(queue.family()),
                CameraUbo {
                    view: view.into(),
                    projection: projection.into(),
                },
            ).expect("Failed to create Camera UBO"),
        }
    }
}
