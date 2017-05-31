#[macro_use] extern crate serde_derive;
#[macro_use] extern crate vulkano;
extern crate cgmath;
extern crate image;
extern crate serde_json;
extern crate serde;
extern crate vulkano_win;
extern crate winit;

mod engine;
mod app;
#[cfg(test)] mod tests;

use engine::bootstrap;
use app::app;

fn main() {
    bootstrap(app());
}