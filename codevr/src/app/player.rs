use engine::Actor;
use std::sync::Arc;
pub struct Player {

}

impl Player {
    pub fn new() -> Arc<Player> {
    Arc::new(Player {})
  }
}

impl Actor for Player {
  fn update(&mut self) {

  }
}