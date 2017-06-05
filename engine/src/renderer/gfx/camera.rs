use renderer::GraphicsState;
use cgmath::{perspective, ortho, Rad, Vector3, Point3, Deg};
use cgmath::Matrix4;

enum ProjectionMode {
    Perspective,
    Orthographic,
}

pub struct CameraProps {
    projection_mode: ProjectionMode,
    to: [f32; 3],
    from: [f32; 3],
    fov: f32,
}

pub struct Camera {
  view: Matrix4<f32>,
  projection: Matrix4<f32>
}

impl Camera {
    pub fn rotate(euler: [f32; 3]) {}
}

impl GraphicsState {
    /// Creates a camera
    pub fn camera(&mut self, props: CameraProps) {

        let CameraProps {
            projection_mode,
            to,
            from,
            fov,
        } = props;

        let view = Matrix4::look_at(
          Point3::new(from[0], from[1], from[2]), 
          Point3::new(to[0], to[1], to[2]),
          Vector3::new(0.0, 1.0, 0.0)
        );

        let projection = match projection_mode {

            ProjectionMode::Perspective => {
              // @TODO - fetch aspect ratio from window
              // Update camera aspect ratio in graphics state traversal.
              perspective(Deg(fov), 1.6, 1.0, 10000.)
            },
            
            ProjectionMode::Orthographic => {
              ortho(2.0, 2.0, 2.0, 2.0, 1.0, 10000.)
            }
        };

        self.cameras.push(
            Camera {
                view,
                projection
                })
    }
}
