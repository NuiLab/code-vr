/*!
# Engine System

The engine for CodeVR is composed of a number of subsystems:

- **Config** - The engine configuration that subsystems can query for reinitialization of important constructs. 
- **Vulkan Renderer** - Given a scene graph description, the renderer creates vulkan constructs and renders to the window's surface.
- **Input System** - Maps input devices to scene actors.

With more incoming.

*/

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate vulkano;
extern crate cgmath;
extern crate image;
extern crate serde_json;
extern crate serde;
extern crate vulkano_win;
extern crate winit;

#[cfg(test)] mod tests;
mod input;
mod renderer;
mod config;
mod actor;

use winit::{WindowBuilder, get_available_monitors, get_primary_monitor, Event};
use vulkano_win::Window;

use std::clone::Clone;
use std::sync::Arc;

pub use self::actor::Actor;
use self::config::Config;
use self::config::read as read_config;
use self::config::WindowConfig;
use self::input::InputSystem;
use self::renderer::Renderer;

pub type Scene = Vec<Arc<Actor>>;

pub struct Engine {
    config: Arc<Config>,
    window: Arc<Window>,
    renderer: Renderer,
    inputs: InputSystem,
    scene: Scene
}

impl Engine {

    pub fn new(config: Config, scene: Scene) -> Engine {

        let cfg = Arc::new(config.clone());

        let (renderer, window) = Renderer::new(create_window(&config.window), cfg.clone());

        let inputs = InputSystem::new(cfg.clone());

        Engine {
            window,
            renderer,
            config: cfg,
            inputs,
            scene
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
                    let mut config_ref = Arc::get_mut(&mut self.config).unwrap();
                    config_ref.window.resolution = [w, h];
                    self.renderer.resize();
                },
                &Event::Closed => return false,
                _ => (),
            };
        }

        true
    }

    /// Updates the scene's actors.
    pub fn update(&mut self) {

        // @TODO - Check scene for destroyed or created elements.
        // Call their respective lifetime functions.
        // actor.start(engine);
        // actor.on_destroy();

        for mut actor in &mut self.scene {
            let a = Arc::get_mut(actor).unwrap();
            a.update();
        }

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

/// Starts the CodeVR Game Engine
pub fn bootstrap(scene: Scene) {
    // Initialize app state
    let config = read_config();

    // Start engine
    let mut engine = Engine::new(config, scene);

    // Render loop
    while engine.io()
    {
        engine.update();
    }
}