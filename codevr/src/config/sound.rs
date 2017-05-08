#[derive(Serialize, Deserialize, Clone)]
pub struct SoundConfig {
    #[serde(default = "master_default")]
    pub master: f32,
}

impl SoundConfig {
    pub fn new() -> SoundConfig {
        SoundConfig { master: 100.0 }
    }
}

fn master_default() -> f32 {
    100.0
}
