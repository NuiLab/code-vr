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

use std::collections::HashMap;

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

macro_rules! input_config {
    ($($field:ident: [ $([$key:ident, $sensitivity:expr, $meta:expr ]),* ]),*) => {
        {
            let mut i = InputConfig::new();
            $(
            i.insert(
                String::from(stringify!($field)),
                vec![
                    $(
                        axis_value(String::from(stringify!($key)), $sensitivity, $meta),
                    )*
                    ]
                );
            )*
            i
        }
    };
}

pub fn default_input() -> InputConfig {

    let i = input_config!(
        move_right: [[
            arrow_right, 
            1.0, 
            None
          ],[
              arrow_left,
              -1.0,
              None
          ]],
        move_forward: [[
            arrow_up,
            1.0,
            None
        ],[
            arrow_down,
            -1.0,
            None
        ]]
    );

    i
}

