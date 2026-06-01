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

    fn get_mouse_wheel() -> i32 {
        let (_x, y) = input::mouse_wheel();
        (y / 120.0) as i32 // TODO: why 120???
    }

    pub fn is_key_pressed(key: liquislime_core::InputKey) -> bool {
        Self::key_to_keycode(key).iter().any(|k| match k {
            KeyType::Keyboard(key_code) => input::is_key_pressed(*key_code),
            KeyType::Mouse(mouse_button) => input::is_mouse_button_pressed(*mouse_button),
            KeyType::MouseWheelUp => Self::get_mouse_wheel() > 0,
            KeyType::MouseWheelDown => Self::get_mouse_wheel() < 0,
        })
    }

    pub fn is_key_down(key: liquislime_core::InputKey) -> bool {
        Self::key_to_keycode(key).iter().any(|k| match k {
            KeyType::Keyboard(key_code) => input::is_key_down(*key_code),
            KeyType::Mouse(mouse_button) => input::is_mouse_button_down(*mouse_button),
            KeyType::MouseWheelUp => Self::get_mouse_wheel() > 0,
            KeyType::MouseWheelDown => Self::get_mouse_wheel() < 0,
        })
    }

    pub fn is_key_released(key: liquislime_core::InputKey) -> bool {
        Self::key_to_keycode(key).iter().any(|k| match k {
            KeyType::Keyboard(key_code) => input::is_key_released(*key_code),
            KeyType::Mouse(mouse_button) => input::is_mouse_button_released(*mouse_button),
            KeyType::MouseWheelUp => Self::get_mouse_wheel() > 0,
            KeyType::MouseWheelDown => Self::get_mouse_wheel() < 0,
        })
    }

    fn key_to_keycode(key: liquislime_core::InputKey) -> &'static [KeyType] {
        match key {
            liquislime_core::InputKey::Left => &[
                KeyType::Keyboard(input::KeyCode::Left),
                KeyType::Keyboard(input::KeyCode::A),
            ],
            liquislime_core::InputKey::Right => &[
                KeyType::Keyboard(input::KeyCode::Right),
                KeyType::Keyboard(input::KeyCode::D),
            ],
            liquislime_core::InputKey::Up => &[
                KeyType::Keyboard(input::KeyCode::Up),
                KeyType::Keyboard(input::KeyCode::W),
            ],
            liquislime_core::InputKey::Down => &[
                KeyType::Keyboard(input::KeyCode::Down),
                KeyType::Keyboard(input::KeyCode::S),
            ],
            liquislime_core::InputKey::LeftMouse => &[KeyType::Mouse(input::MouseButton::Left)],
            liquislime_core::InputKey::RightMouse => &[KeyType::Mouse(input::MouseButton::Right)],
            liquislime_core::InputKey::MouseWheelUp => &[KeyType::MouseWheelUp],
            liquislime_core::InputKey::MouseWheelDown => &[KeyType::MouseWheelDown],
        }
    }
}

enum KeyType {
    Keyboard(input::KeyCode),
    Mouse(input::MouseButton),
    MouseWheelUp,
    MouseWheelDown,
}
