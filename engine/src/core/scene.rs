use core::Actor;
use core::EngineState;
use config::Config;
use input::InputSystem;
use renderer::GraphicsState;
use std::sync::Arc;

pub struct Scene {
    pub created: Vec<Arc<Actor>>,
    pub active: Vec<Arc<Actor>>,
    //pub destroy: Vec<Fn(&Actor) -> bool>,
}

impl Scene {
    pub fn new(actors: Vec<Arc<Actor>>) -> Scene {
        Scene {
            created: actors,
            active: Vec::new(),
            //destroy: Vec::new(),
        }
    }

    /// Add actor to the scene
    pub fn add(&mut self, actor: Arc<Actor>) {
        self.created.push(actor);
    }

    // Updates the scene. Spawns new actors, updates current actors, and destroys other actors.
    pub fn update(&mut self, config: &Arc<Config>, gfx: &Arc<GraphicsState>, input: &Arc<InputSystem>) {

        while let Some(mut new_actor) = self.created.pop() {

            // Mount component to scene
            {
                let a = Arc::get_mut(&mut new_actor).unwrap();

                a.start(
                    EngineState {
                        id: 0, // @TODO - Generate Hash
                        config: config.clone(),
                        gfx: gfx.clone(),
                        input: input.clone()
                });
            }        
               
            self.active.push(new_actor);

        }

        for actor in &mut self.active {
            let a = Arc::get_mut(actor).unwrap();
            a.update();
        }

        // @TODO - Faster algorithm/data structure
        //while let Some(actor_killer) = self.destroy.pop() {
        //    for actor in self.active {
        //        actor_killer(target_actor);
        //    }
        //}
    }

    // Queue actors to be destroyed that satisfy callback.
    //pub fn destroy(&mut self, callback: Fn(Actor) -> bool) {
    //    self.destroy(callback);
    //}
}

