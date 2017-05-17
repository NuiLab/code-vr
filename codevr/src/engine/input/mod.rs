/*!
# Input Module

This module takes care of all input events processed by the engine

*/
mod events;

use std::collections::HashMap;
use config::Config;
use winit::{Event};

pub struct InputSystem {
    inputs: HashMap<String, (f32, u8)>
}

//Input System
impl InputSystem {
    pub fn new(config: &Config) -> InputSystem {

        let mut inputs: HashMap<String, (f32, u8)> = config
            .input
            .keys()
            .map(|k| (k.clone(), (0.0,0)))
            .collect();

            InputSystem {
                inputs
            }
    }

    pub fn update(self, ev:&Event) {

    }

    pub fn inputs(self) {
        // Give an iterator depending on what input is being queried...
    }
}