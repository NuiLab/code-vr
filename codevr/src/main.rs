extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate vulkano;
extern crate winit;
extern crate vulkano_win;
extern crate image;

mod config;
mod app;
mod engine;
mod renderer;

fn main() {

    // Initialize app state
    let state = config::read();

    // Initialize app tree
    let app = app::App::new();

    // Start engine
    let mut engine = engine::Engine::new(state/*, app*/);

    // Render loop
    while engine.io()
    {
        engine.update();
        engine.render();
    }

}