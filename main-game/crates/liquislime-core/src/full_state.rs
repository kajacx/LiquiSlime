use crate::*;

pub struct FullState {
    pub game_state: GameState,
    pub input_query: Box<dyn InputQuery>,
    pub adaptors: Vec<Box<dyn BehaviourAdaptor>>,
}

impl FullState {
    pub fn new(game_state: GameState, input_query: Box<dyn InputQuery>) -> Self {
        Self {
            game_state,
            input_query,
            adaptors: Vec::new(),
        }
    }

    pub fn update(&mut self, time_passed: TimeInterval) {
        self.game_state.update(time_passed);
        let mut game_interaction = GameInteraction::new(&mut self.game_state, &*self.input_query);
        for adaptor in &mut self.adaptors {
            adaptor.update(&mut game_interaction, time_passed);
        }
    }
}
