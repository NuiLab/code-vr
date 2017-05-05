#[derive(Serialize, Deserialize, Clone)]
pub struct GraphicsConfig {
  #[serde(default = "vsync_default")]
  pub vsync: bool,
  #[serde(default = "aa_default")]
  pub antialiasing: u8,
}

fn vsync_default() -> bool {
  false
}

fn aa_default() -> u8 {
  0
}