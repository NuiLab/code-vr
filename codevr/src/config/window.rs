#[derive(Serialize, Deserialize)]
pub struct WindowConfig {
    #[serde(default = "display_default")]
    display: u32,
    #[serde(default = "resolution_default")]
    resolution: [u32; 2],
    #[serde(default = "fullscreen_default")]
    fullscreen: bool,
}

fn fullscreen_default() -> bool {
    true
}

fn resolution_default() -> [u32; 2] {
    [1920, 1080]
}

fn display_default() -> u32 {
    0
}