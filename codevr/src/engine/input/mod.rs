/*!
# Input Module

This module takes care of all input events processed by the engine

*/
mod events;

use winit::Event;
use engine::config::Config;
use std::clone::Clone;
use std::collections::HashMap;
use std::sync::Arc;


/// Core Input System
pub struct InputSystem {
    config: Arc<Config>,
    pub inputs: HashMap<String, f32>,
}

//Input System
impl InputSystem {
    
    pub fn new(config: Arc<Config>) -> InputSystem {

        let mut inputs: HashMap<String, f32> =
            config.input.keys().map(|k| (k.clone(), 0.0)).collect();

        InputSystem {
            inputs,
            config
        }
    }

    /// Polls window events
    pub fn poll(&mut self, ev: &Event) {
        // Axis Map
        for (string_key, axis) in self.config.input.iter() {
    
            for (i, axis_value) in axis.iter().enumerate() {

                let out = events::string_to_wevent(&axis_value.key, &ev);

                // @TODO - Check axis_value.meta for additional checks.

                // Write to axis map
                match out {
                    Some(x) => *self.inputs.get_mut(string_key).unwrap() = x,
                    None => (),
                };
                
            }
        }
    }
}
