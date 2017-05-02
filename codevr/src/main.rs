extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate vulkano;
extern crate winit;
extern crate vulkano_win;

use vulkano_win::VkSurfaceBuild;

mod config;
mod app;
mod engine;

fn main() {

    // Initialize application state
    let state = config::read();

    // Initialize app tree
    let app = app::App::new();

    // Start engine
    let engine = engine::Engine::new(state/*, app*/);

    while engine.update()
    {
        engine.render();
        engine.io();   
    }

}