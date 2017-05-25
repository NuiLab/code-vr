extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate vulkano;
extern crate winit;
extern crate vulkano_win;
extern crate image;
extern crate cgmath;

mod engine;
mod app;

use engine::{Engine, read_config};
use app::App;

fn main() {

    // Initialize app state
    let state = read_config();

    // Initialize app tree
    let app = App::new();

    // Start engine
    let mut engine = Engine::new(state/*, app*/);

    // Render loop
    while engine.io()
    {
        engine.update();
    }

}