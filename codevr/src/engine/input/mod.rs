/*!
# Input Module

This module takes care of all input events processed by the engine

*/
mod events;

use config::InputConfig;
use std::clone::Clone;
use std::collections::HashMap;
use config::Config;
use winit::Event;

/// Core Input System
pub struct InputSystem {
    axis_map: InputConfig,
    pub inputs: HashMap<String, f32>
}

//Input System
impl InputSystem {
    
    pub fn new(axis_map: InputConfig) -> InputSystem {

        let mut inputs: HashMap<String, f32> =
            axis_map.keys().map(|k| (k.clone(), 0.0)).collect();

        InputSystem {
            inputs,
            axis_map
        }
    }

    /// Polls window events
    pub fn poll(&mut self, ev: &Event) {

        println!("POLL");
        
        // Axis Map
        for (string_key, axis) in self.axis_map.iter() {

            for (i, axis_value) in axis.iter().enumerate() {

                let out = events::string_to_wevent(&axis_value.key, &ev);

                // @TODO - Check axis_value.meta for additional checks.

                // Write to axis map
                match out {
                    Some(x) => {
                        println!("SOME {}", i);
                        *self.inputs.get_mut(string_key).unwrap() = x;
                        continue;
                        },
                    None => continue,
                };
                
            }
        }
    }
}
