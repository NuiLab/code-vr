#[derive(Serialize, Deserialize)]
pub struct GraphicsConfig {
  vsync: bool,
  antialiasing: u8,
}