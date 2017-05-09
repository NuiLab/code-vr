use winit::{Event, VirtualKeyCode, ElementState, MouseButton};

pub fn string_to_wevent(s: &String, event: &Event) -> Option<f32> {
  match s.as_ref() {
    "arrow_left" => {
      match event {
        &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Left)) => Some(1.0),
        &Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Left)) => Some(0.0),
        _ => None
        }
    },
    "arrow_right" => {
      match event {
        &Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Right)) => Some(1.0),
        &Event::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Right)) => Some(0.0),
        _ => None
        }
    }
    _ => None
  }
}
