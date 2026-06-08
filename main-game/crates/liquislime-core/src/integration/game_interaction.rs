use crate::*;

pub struct GameInteraction<'a> {
    game: &'a mut GameState,
    inputs: &'a dyn InputQuery,
}

impl<'a> GameInteraction<'a> {
    pub fn new(game: &'a mut GameState, inputs: &'a dyn InputQuery) -> Self {
        Self { game, inputs }
    }

    pub fn set_slime(
        &mut self,
        faction_id: FactionId,
        position: TilePosition,
        amount: SlimeAmount,
    ) {
        self.game.grids.set_amount(faction_id, position, amount);
    }

    pub fn add_slime(
        &mut self,
        faction_id: FactionId,
        position: TilePosition,
        amount: SlimeAmount,
    ) {
        self.game.grids.add_amount(faction_id, position, amount);
    }

    pub fn pan_camera_by(&mut self, delta: Position) {
        self.game.screen.camera.pan_by(delta);
    }

    pub fn is_key_pressed(&self, key_code: InputKey) -> bool {
        self.inputs.is_key_pressed(key_code)
    }

    pub fn is_key_down(&self, key_code: InputKey) -> bool {
        self.inputs.is_key_down(key_code)
    }

    pub fn is_key_released(&self, key_code: InputKey) -> bool {
        self.inputs.is_key_released(key_code)
    }

    pub fn get_mouse_screen_position(&self) -> Position {
        self.inputs.get_mouse_screen_position()
    }

    pub fn get_mouse_world_position(&self) -> Position {
        self.game
            .screen
            .screen_position_to_world(self.get_mouse_screen_position())
    }

    /**
     * Safety: This will just remove the lifetime at compile time, the caller must
     * make sure to not use the returned reference for longer than they can hold the original reference.
     */
    pub unsafe fn with_static_lifetime(&mut self) -> &mut GameInteraction<'static> {
        &mut *(self as *mut GameInteraction<'a> as *mut GameInteraction<'static>)
    }
}
