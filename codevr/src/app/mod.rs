use engine::Scene;

mod player;

use self::player::Player;
use std::sync::Arc;

/// First Engine scene
pub fn app() -> Scene {
  Scene::new(
    vec![
    Player::new()
    ]
  )
}