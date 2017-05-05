#[derive(Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    #[serde(default = "display_default")]
    pub display: u32,
    #[serde(default = "resolution_default")]
    pub resolution: [u32; 2],
    #[serde(default = "fullscreen_default")]
    pub fullscreen: bool,
}

fn fullscreen_default() -> bool {
    true
}

fn resolution_default() -> [u32; 2] {
    [0, 0]
}

fn display_default() -> u32 {
    0
}