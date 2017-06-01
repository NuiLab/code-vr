use engine::Scene;

mod player;

use self::player::Player;

/// First Engine scene
pub fn app() -> Scene {
  vec![
    Player::new()
  ]
}