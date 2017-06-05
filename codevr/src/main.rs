extern crate engine;

#[cfg(test)] 
mod tests;
mod app;

use engine::bootstrap;
use app::app;

fn main() {
    bootstrap(app());
}