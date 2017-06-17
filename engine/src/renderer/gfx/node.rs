use cgmath::Matrix4;
use cgmath::SquareMatrix;

pub struct Node {
    pub model: Matrix4<f32>,
    pub mesh: u32,
    pub children: Vec<u32>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            model: Matrix4::identity(),
            mesh: 0,
            children: Vec::new(),
        }
    }
}
