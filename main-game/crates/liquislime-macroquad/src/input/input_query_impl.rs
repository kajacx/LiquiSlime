use crate::input::InputHelper;
use liquislime_core::*;

pub struct InputQueryImpl;

impl InputQuery for InputQueryImpl {
    fn is_key_pressed(&self, key_code: InputKey) -> bool {
        InputHelper::is_key_pressed(key_code)
    }

    fn is_key_down(&self, key_code: InputKey) -> bool {
        InputHelper::is_key_down(key_code)
    }

    fn is_key_released(&self, key_code: InputKey) -> bool {
        InputHelper::is_key_released(key_code)
    }

    fn get_mouse_screen_position(&self) -> Position {
        InputHelper::get_mouse_screen_position()
    }
}
