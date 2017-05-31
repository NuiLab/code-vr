enum ProjectionMode {
  Perspective,
  Orthographic
}

struct CameraProps  {
  active: bool,
  projection_mode: ProjectionMode,
  to: [f32; 3],
  from: [f32; 3],
  rotation: [f32; 3],
  fov: f32,
}

pub struct Camera {
    //descriptor_set: Option<u32>
}

impl Camera {
  pub fn rotate() {
    
  }
}