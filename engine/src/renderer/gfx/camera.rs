use renderer::GraphicsState;
use cgmath::{perspective, ortho, Vector3, Point3, Deg, Quaternion, Euler};
use cgmath::Matrix4;
use std::sync::{Arc, Mutex};

pub enum ProjectionMode {
    Perspective,
    Orthographic,
}

pub struct CameraProps {
    pub projection_mode: ProjectionMode,
    pub to: [f32; 3],
    pub from: [f32; 3],
    pub fov: f32,
}

pub struct Camera {
    pub view: Matrix4<f32>,
    pub projection: Matrix4<f32>,
}

impl Camera {
    pub fn rotate(&mut self, euler: [f32; 3]) {
        let rot = Matrix4::from(Euler::new(Deg(euler[0]), Deg(euler[1]), Deg(euler[2])));
        self.view = rot * self.view;
    }
}

impl GraphicsState {
    /// Creates a camera
    pub fn camera(&mut self, props: CameraProps) -> Arc<Mutex<Camera>> {

        let CameraProps {
            projection_mode,
            to,
            from,
            fov,
        } = props;

        let view = Matrix4::look_at(Point3::new(from[0], from[1], from[2]),
                                    Point3::new(to[0], to[1], to[2]),
                                    Vector3::new(0.0, 1.0, 0.0));

        let projection = match projection_mode {

            ProjectionMode::Perspective => {
                // @TODO - fetch aspect ratio from window
                // Update camera aspect ratio in graphics state traversal.
                perspective(Deg(fov), 1.6, 1.0, 10000.)
            }

            ProjectionMode::Orthographic => ortho(2.0, 2.0, 2.0, 2.0, 1.0, 10000.),
        };

        let camera = Arc::new(Mutex::new(Camera { view, projection }));

        self.cameras.push(camera.clone());

        camera
    }
}

