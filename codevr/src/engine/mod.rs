use winit::{WindowBuilder, get_available_monitors, get_primary_monitor};
use vulkano_win::{Window, VkSurfaceBuild, required_extensions};
use vulkano::instance::Instance;

use config::Config;
use config::WindowConfig;

pub struct Engine {
    window: Window,
}

impl Engine {
    pub fn new(config: Config) -> Engine {

        let window_builder = create_window(config.window);

        // @TODO - Pass window to renderer specific initializer...

        let instance = {
            let extensions = required_extensions();
            Instance::new(None, &extensions, None).expect("Failed to create Vulkan instance.")
        };

        let window = window_builder.build_vk_surface(&instance).unwrap();

        Engine { window }
    }

    pub fn io(&self) {}

    pub fn update(&self) -> bool {
        false
    }

    pub fn render(&self) {}
}

/// Creates a window builder with a given window configuration.
pub fn create_window(config: WindowConfig) -> WindowBuilder  {

    let window_manager = WindowBuilder::new()
        .with_title("CodeVR")
        .with_dimensions(config.resolution[0], config.resolution[1]);

    let mut w = get_primary_monitor();

    if config.fullscreen {
        for (i, monitor) in get_available_monitors().enumerate() {
            if i == config.display as usize {
                w = monitor;
            }
        }
    }

    window_manager.with_fullscreen(w)
}
