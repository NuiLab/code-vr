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
mod core;

pub use self::core::*;
pub use self::renderer::gfx;

/// Starts the CodeVR Game Engine
pub fn bootstrap(scene: Scene) {

    // Initialize app state
    let config = config::read();

    // Start engine
    let mut engine = Engine::new(config, scene);

    // Render loop
    while engine.io()
    {
        engine.update();
    }
}