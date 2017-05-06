/**!
Inputs are configured via a map of events and coresponding outputs.

For example:

```json
"input": {
  "move_right": [
    {
      "event": "left_arrow",
      "value": -1.0
    },
    {
      "event": "right_arrow",
      "value": 1.0
    },
    {
      "event": "gamepad_left",
      "value": -1.0
    }
  ]
}
```

This is serialized and converted to a map of event &str and EventValues.

These are mapped to winit::Event instances, and updated when those events occur.

 */

use std::collections::HashMap;

struct EventValue {
  event: &str,
  value: f32
}

pub type Input: HashMap<&str, winit::Event>;