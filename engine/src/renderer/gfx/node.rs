use cgmath::Matrix4;

pub struct Node {
  model: Matrix4<f32>,
  children: u32
}

impl Node {
  pub fn new() -> Node {
    Node {
      model: Matrix4::identity();
    }
  }
}