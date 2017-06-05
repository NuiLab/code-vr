use winit::{ Event, VirtualKeyCode, ElementState, MouseButton, WindowEvent };

// @TODO - We need a better way of abstracting this.
pub fn string_to_wevent(s: &String, event: &Event) -> Option<f32> {

    match s.as_ref() {
        "arrow_left" => {
            match event {
                &Event::WindowEvent { event: WindowEvent::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Left), _), ..} => {
                    Some(-1.0)
                }
                &Event::WindowEvent { event: WindowEvent::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Left), _), ..} => {
                    Some(0.0)
                }
                _ => None,
            }
        }
        "arrow_right" => {
            match event {
                &Event::WindowEvent { event: WindowEvent::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Right), _), ..} => {
                    Some(1.0)
                }
                &Event::WindowEvent { event: WindowEvent::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Right), _), ..} => {
                    Some(0.0)
                }
                _ => None,
            }
        }
        "arrow_up" => {
            match event {
                &Event::WindowEvent { event: WindowEvent::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Up), _), ..} => {
                    Some(1.0)
                }
                &Event::WindowEvent { event: WindowEvent::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Up), _), ..} => {
                    Some(0.0)
                }
                _ => None,
            }
        }
        "arrow_down" => {
            match event {
                &Event::WindowEvent { event: WindowEvent::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Down), _), ..} => {
                    Some(-1.0)
                }
                &Event::WindowEvent { event: WindowEvent::KeyboardInput(ElementState::Released, _, Some(VirtualKeyCode::Down), _), ..} => {
                    Some(0.0)
                }
                _ => None,
            }
        }
        _ => None,
    }
}
