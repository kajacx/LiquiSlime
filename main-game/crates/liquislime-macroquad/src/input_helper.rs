use liquislime_core::{Position, Screen, TilePosition};
use macroquad::{input, miniquad};

pub struct InputHelper;

impl InputHelper {
    pub fn get_mouse_screen_position() -> Position {
        let (x, y) = input::mouse_position();
        Position { x, y }
    }
    pub fn get_mouse_world_position(screen: &Screen) -> Position {
        screen.screen_position_to_world(Self::get_mouse_screen_position())
    }

    pub fn get_mouse_tile_position(screen: &Screen) -> TilePosition {
        Self::get_mouse_world_position(screen).to_tile_position()
    }

    pub fn screen_size() -> Position {
        let (x, y) = miniquad::window::screen_size();
        Position { x, y }
    }
}
