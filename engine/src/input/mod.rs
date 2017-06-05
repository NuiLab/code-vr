/*!
# Input Module

This module takes care of all input events processed by the engine.
It's modeled similarly to the axis system in Unreal Engine 4.
*/
mod events;

use winit::Event;
use config::Config;
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

        let inputs: HashMap<String, f32> =
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
    
            for axis_value in axis {

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
