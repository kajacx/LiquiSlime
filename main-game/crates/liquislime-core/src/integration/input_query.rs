use crate::{InputKey, Position, Screen};

pub trait InputQuery {
    fn is_key_pressed(&self, key_code: InputKey) -> bool;

    fn is_key_down(&self, key_code: InputKey) -> bool;

    fn is_key_released(&self, key_code: InputKey) -> bool;

    fn get_mouse_screen_position(&self) -> Position;

    fn get_mouse_world_position(&self, screen: &Screen) -> Position {
        screen.screen_position_to_world(self.get_mouse_screen_position())
    }
}
