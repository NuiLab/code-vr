extern crate pyro;

#[cfg(test)] 
mod tests;
mod app;

use pyro::bootstrap;
use app::app;

fn main() {
    bootstrap(app());
}