#[derive(Serialize, Deserialize, Clone)]
pub struct SoundConfig {
  #[serde(default = "master_default")]
  pub master: f32
}

fn master_default() -> f32 {
  100.0
}