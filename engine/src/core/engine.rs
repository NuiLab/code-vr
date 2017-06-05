use winit::{WindowBuilder, get_available_monitors, get_primary_monitor, Event, WindowEvent, EventsLoop};
use vulkano_win::Window;

use std::clone::Clone;
use std::sync::{Arc, Mutex};

use config::Config;
use config::WindowConfig;
use input::InputSystem;
use renderer::Renderer;
use core::Scene;
use renderer::GraphicsState;

/// The minimum window resolution the engine supports.
pub const MINIMUM_RESOLUTION: [u32; 2] = [640, 480];

/// Game Engine
pub struct Engine {

    /// Engine Configuration
    config: Arc<Config>,

    /// Actor Scene Graph
    scene: Scene,

    /// Input Mappings
    inputs: Arc<Mutex<InputSystem>>,

    /// API Specific Renderer
    renderer: Renderer,

    /// Graphics Data Structures
    gfx: Arc<Mutex<GraphicsState>>,

    /// OS Window
    window: Arc<Window>,

    /// OS Events
    events_loop: Arc<EventsLoop>

}

impl Engine {

    /// Initialize Engine subsystems
    pub fn new(config: Config, scene: Scene) -> Engine {

        let cfg = Arc::new(config.clone());

        let (renderer, window, events_loop) = Renderer::new(create_window(&config.window), cfg.clone());

        let inputs = Arc::new(Mutex::new(InputSystem::new(cfg.clone())));

        let gfx = Arc::new(Mutex::new(GraphicsState::new()));

        Engine {
            window,
            events_loop,
            renderer,
            gfx,
            config: cfg,
            inputs,
            scene,
        }
    }

    /// Handles input/output events from the window and any input middleware.
    pub fn io(&mut self) -> bool {

        let mut running = true;

        self.events_loop.clone().poll_events(|ev| {

            // Pass &ev to Input System

            let mut inputs = self.inputs.lock().unwrap();
            inputs.poll(&ev);

            // Core Events
            match ev {
                Event::WindowEvent { event: WindowEvent::Resized(w, h), .. } => {
                    let mut config_ref = Arc::get_mut(&mut self.config).unwrap();
                    config_ref.window.resolution = [w, h];
                    self.renderer.resize();
                },
               Event::WindowEvent { event: WindowEvent::Closed, .. } => running = false,
                _ => (),
            };
        });

        running
    }

    /// Updates engine subsystems.
    pub fn update(&mut self) {
        
        // Update actors
        self.scene.update(&self.config, &self.gfx, &self.inputs);

        // Grab lock for graphics state
        let gfx = self.gfx.lock().unwrap();
        // Render graphics state
        self.renderer.render(&gfx);
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

