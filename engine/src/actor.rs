pub trait Actor {

  /// Called when adding the actor to the scene, you can access the engine state.
  fn start(&mut self) {

  }
  /// Update the state of the Actor and check engine state
  fn update(&mut self) {

  }

  /// Access the state of the engine
  fn engine(&mut self) {

  }

}