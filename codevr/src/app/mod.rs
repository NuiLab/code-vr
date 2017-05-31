use engine::Scene;

mod player;

use self::player::Player;

pub fn app() -> Scene {
  vec![
    Player::new()
  ]
}