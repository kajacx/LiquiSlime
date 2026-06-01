use crate::{FactionId, GameState, Position, SlimeAmount, TilePosition};

pub struct GameInteraction<'a> {
    state: &'a mut GameState,
}

impl<'a> GameInteraction<'a> {
    pub fn new(state: &'a mut GameState) -> Self {
        Self { state }
    }

    pub fn set_slime(
        &mut self,
        faction_id: FactionId,
        position: TilePosition,
        amount: SlimeAmount,
    ) {
        self.state.grids.set_amount(faction_id, position, amount);
    }

    pub fn pan_camera_by(&mut self, delta: Position) {
        self.state.screen.camera.pan_by(delta);
    }
}
