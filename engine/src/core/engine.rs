use winit::{WindowBuilder, get_available_monitors, get_primary_monitor, Event};
use vulkano_win::Window;

use std::clone::Clone;
use std::sync::Arc;

use config::Config;
use config::WindowConfig;
use input::InputSystem;
use renderer::Renderer;
use core::Scene;

const MINIMUM_RESOLUTION: [u32; 2] = [640, 480];

pub struct Engine {
    pub config: Arc<Config>,
    window: Arc<Window>,
    pub renderer: Renderer,
    pub inputs: Arc<InputSystem>,
    scene: Scene,
}

impl Engine {
    pub fn new(config: Config, scene: Scene) -> Engine {

        let cfg = Arc::new(config.clone());

        let (renderer, window) = Renderer::new(create_window(&config.window), cfg.clone());

        let inputs = Arc::new(InputSystem::new(cfg.clone()));

        Engine {
            window,
            renderer,
            config: cfg,
            inputs,
            scene,
        }
    }

    /// Handles input/output events from the window and any input middleware.
    pub fn io(&mut self) -> bool {

        for ev in self.window.window().poll_events() {

            // Pass &ev to Input System
            let inputs = Arc::get_mut(&mut self.inputs).unwrap();
            inputs.poll(&ev);

            // Core Events
            match &ev {

                &Event::Resized(w, h) => {
                    let mut config_ref = Arc::get_mut(&mut self.config).unwrap();
                    config_ref.window.resolution = [w, h];
                    self.renderer.resize();
                }

                &Event::Closed => return false,
                _ => (),
            };
        }

        true
    }

    /// Updates the scene's actors.
    pub fn update(&mut self) {
        // Update actors
        self.scene.update(&self.config, &self.renderer.gfx, &self.inputs);
        // Render graphics state
        self.renderer.render();
    }
}


/// Creates a window builder with a given window configuration.
fn create_window(config: &WindowConfig) -> WindowBuilder {

    let mut w = get_primary_monitor();
    let dimensions = w.get_dimensions();

    // Set resolution
    let resolution = if config.resolution[0] <= MINIMUM_RESOLUTION[0] ||
                        config.resolution[1] <= MINIMUM_RESOLUTION[1] {
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

