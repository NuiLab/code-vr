/*!
# Engine System

The engine for CodeVR is composed of a number of subsystems:

- **Config** - The engine configuration that subsystems can query for reinitialization of important constructs. 
- **Vulkan Renderer** - Given a scene graph description, the renderer creates vulkan constructs and renders to the window's surface.
- **Input System** - Maps input devices to scene actors.

With more incoming.

*/
mod input;
mod renderer;
mod config;

use winit::{WindowBuilder, get_available_monitors, get_primary_monitor, Event};
use vulkano_win::Window;

use std::clone::Clone;
use std::sync::Arc;
use std::time::Duration;
use std::collections::HashMap;

pub use self::config::read as read_config;
use self::config::Config;
use self::config::WindowConfig;
use self::input::InputSystem;
use self::renderer::Renderer;

pub struct Engine {
    config: Arc<Config>,
    window: Arc<Window>,
    renderer: Renderer,
    inputs: InputSystem
}

impl Engine {

    pub fn new(config: Config) -> Engine {

        let cfg = Arc::new(config.clone());

        let (renderer, window) = Renderer::new(create_window(&config.window), cfg.clone());

        let inputs = InputSystem::new(cfg.clone());

        Engine {
            window,
            renderer,
            config: cfg,
            inputs,
        }
    }

    /// Handles input/output events from the window and any input middleware.
    pub fn io(&mut self) -> bool {

        for ev in self.window.window().poll_events() {

            // Pass &ev to Input System
            self.inputs.poll(&ev); 

            // Core Events
            match &ev
            {
                &Event::Resized(w, h) => {
                    //self.config.window.resolution = [w, h];
                    self.renderer.resize();
                }
                &Event::Closed => return false,
                _ => (),
            };
        }

        true
    }

    /// Recursively updates application tree.
    pub fn update(&mut self) {

        // @TODO - Traverse Scene Graph

        self.renderer.render();
    }
}


/// Creates a window builder with a given window configuration.
fn create_window(config: &WindowConfig) -> WindowBuilder {

    let mut w = get_primary_monitor();
    let dimensions = w.get_dimensions();

    // Set resolution
    let resolution = if config.resolution[0] <= 240 || config.resolution[1] <= 240 {
        [dimensions.0, dimensions.1]
    } else {
        config.resolution.clone()
    };

    // Set fullscreen
    if config.fullscreen {
        for (i, monitor) in get_available_monitors().enumerate() {
            if i == config.display as usize {
                w = monitor;
            }
        }
    }

    // Create manager
    let window_manager = WindowBuilder::new()
        .with_title("CodeVR")
        .with_dimensions(resolution[0], resolution[1]);

    if config.fullscreen {
        return window_manager.with_fullscreen(w);
    }


    window_manager
}
