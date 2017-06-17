use vulkano::pipeline::GraphicsPipelineAbstract;
use std::sync::Arc;

pub struct Material {
  pipeline: Arc<GraphicsPipelineAbstract>
}