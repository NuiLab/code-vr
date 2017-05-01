/// Creates a window builder with a given window configuration.
// pub fn create_window(config: WindowConfig) -> WindowBuilder {

//     let window_manager = winit::WindowBuilder::new()
//         .with_title("CodeVR")
//         .with_dimensions(config.resolution[0], config.resolution[1]);

//     let mut w = get_primary_monitor();

//     if config.fullscreen {
//         for (i, monitor) in get_available_monitors().enumerate() {
//             if i == config.display as usize {
//                 w = monitor;
//             }
//         }
//     }

//     window_manager.with_fullscreen(w)
// }

pub struct Engine {

}

impl Engine {
    pub fn new() -> Engine {
        Engine {}
    }

    pub fn io(&self) {

    }

    pub fn update(&self) -> bool {
        false
    }

    pub fn render(&self) {

    }
}