use liquislime_core::{Position, TilePosition};
use macroquad::input;

pub struct InputHelper;

impl InputHelper {
    pub fn get_mouse_position() -> Position {
        let (x, y) = input::mouse_position();
        Position { x, y }
    }

    pub fn get_mouse_tile_position() -> TilePosition {
        Self::get_mouse_position().to_tile_position()
    }
}
