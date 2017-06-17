use vulkano::buffer::CpuAccessibleBuffer;
use std::sync::Arc;

pub struct Mesh {
  primitives: u32,
  indices: Arc<CpuAccessibleBuffer>
}