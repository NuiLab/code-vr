/**!
Inputs are configured via a map of axises and coresponding outputs.

For example:

```json
"input": {
  "move_right": [
    {
      "key": "left_arrow",
      "scale": -1.0
    },
    {
      "key": "right_arrow",
      "scale": 1.0
    },
    {
      "key": "gamepad_left",
      "scale": -1.0,
      "meta": {
        "deadzone": 0.25,
        "sensitivity": 1.0
        "invert": false
      }
    }
  ]
}
```

This is serialized and converted to a map of Strings and Axis'.

These are mapped to winit::Event instances, and updated when those events occur by the engine.

*/

use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use std::collections::HashMap;
use std::str::FromStr;

pub type InputConfig = HashMap<String, Axis>;

pub type Axis = Vec<AxisValue>;

#[derive(Serialize, Deserialize, Clone)]
pub struct AxisValue {
    #[serde(default = "key_default")]
    pub key: String,
    #[serde(default = "scale_default")]
    pub scale: f32,
    #[serde(default = "meta_default")]
    pub meta: Option<AxisMeta>,
}

fn key_default() -> String {
    //@TODO make unique generated hash based off current system time.
    String::from("unknown")
}

fn scale_default() -> f32 {
    1.0
}

fn meta_default() -> Option<AxisMeta> {
    None
}

fn axis_value(key: String, scale: f32, meta: Option<AxisMeta>) -> AxisValue {
    AxisValue { key, scale, meta }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AxisMeta {
    #[serde(default = "deadzone_default")]
    pub deadzone: f32,
    #[serde(default = "sensitivity_default")]
    pub sensitivity: f32,
    #[serde(default = "invert_default")]
    pub invert: bool,
}

fn deadzone_default() -> f32 {
    0.25
}

fn sensitivity_default() -> f32 {
    1.0
}

fn invert_default() -> bool {
    false
}

pub fn default_input() -> InputConfig {
    /*
    @TODO - Make macro to generate this map faster.
    input_config!({
      "move_right" : {
          key : "right_arrow", 
          sensitivity: 1.0, 
          meta: None
          }
        });
    */
    
    let mut i = InputConfig::new();
    i.insert(String::from("move_right"),
             vec![axis_value(String::from("right_arrow"), 1.0, None),
                  axis_value(String::from("left_arrow"), -1.0, None)]);
    i.insert(String::from("move_forward"), 
            vec![axis_value(String::from("up_arrow"), 1.0, None),
                  axis_value(String::from("down_arrow"), -1.0, None)]);
    i
}

